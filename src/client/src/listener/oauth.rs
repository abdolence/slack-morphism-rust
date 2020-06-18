use rsb_derive::Builder;
use rvstruct::ValueStruct;

use crate::api::*;
use crate::errors::*;
use crate::{SlackClient, SlackClientHttpApi};

use crate::listener::SlackClientEventsListener;
use futures::future::{BoxFuture, FutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response};
use log::*;
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackOAuthListenerConfig {
    pub client_id: String,
    pub client_secret: String,
    pub bot_scope: String,
    pub redirect_callback_host: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_INSTALL_PATH_VALUE.into()"]
    pub install_path: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_CALLBACK_PATH_VALUE.into()"]
    pub redirect_callback_path: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_INSTALLED_URL_VALUE.into()"]
    pub redirect_installed_url: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_CANCELLED_URL_VALUE.into()"]
    pub redirect_cancelled_url: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_ERROR_URL_VALUE.into()"]
    pub redirect_error_redirect_url: String,
}

impl SlackOAuthListenerConfig {
    const DEFAULT_INSTALL_PATH_VALUE: &'static str = "/auth/install";
    const DEFAULT_CALLBACK_PATH_VALUE: &'static str = "/auth/callback";
    const DEFAULT_INSTALLED_URL_VALUE: &'static str = "/installed";
    const DEFAULT_CANCELLED_URL_VALUE: &'static str = "/cancelled";
    const DEFAULT_ERROR_URL_VALUE: &'static str = "/error";

    const OAUTH_AUTHORIZE_URL_VALUE: &'static str = "https://slack.com/oauth/v2/authorize";

    pub fn to_redirect_url(&self) -> String {
        format!(
            "{}{}",
            &self.redirect_callback_host, &self.redirect_callback_path
        )
    }
}

impl SlackClientEventsListener {
    async fn slack_oauth_install_service(
        _: Request<Body>,
        config: &SlackOAuthListenerConfig,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        let full_uri = SlackClientHttpApi::create_url_with_params(
            SlackOAuthListenerConfig::OAUTH_AUTHORIZE_URL_VALUE,
            &vec![
                ("client_id", Some(&config.client_id)),
                ("scope", Some(&config.bot_scope)),
                ("redirect_uri", Some(&config.to_redirect_url())),
            ],
        );
        debug!("Redirecting to Slack OAuth authorize: {}", &full_uri);
        SlackClientHttpApi::hyper_redirect_to(&full_uri.to_string())
    }

    async fn slack_oauth_callback_service<'a, I, IF>(
        req: Request<Body>,
        config: &'a SlackOAuthListenerConfig,
        client: Arc<SlackClient>,
        install_service_fn: I,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>>
    where
        I: Fn(
                Result<
                    SlackOAuthV2AccessTokenResponse,
                    Box<dyn std::error::Error + Send + Sync + 'static>,
                >,
                Arc<SlackClient>,
            ) -> IF
            + 'static
            + Send
            + Sync
            + Clone,
        IF: Future<Output = ()> + 'static + Send,
    {
        let params = SlackClientHttpApi::parse_query_params(&req);
        debug!("Received Slack OAuth callback: {:?}", &params);

        match (params.get("code"), params.get("error")) {
            (Some(code), None) => {
                let oauth_access_resp = client
                    .oauth2_access(
                        &SlackOAuthV2AccessTokenRequest::from(SlackOAuthV2AccessTokenRequestInit {
                            client_id: config.client_id.clone(),
                            client_secret: config.client_secret.clone(),
                            code: code.into(),
                        })
                        .with_redirect_uri(config.to_redirect_url()),
                    )
                    .await;

                match oauth_access_resp {
                    Ok(oauth_resp) => {
                        info!(
                            "Received slack OAuth access resp for: {} / {} / {}",
                            &oauth_resp.team.id.value(),
                            &oauth_resp
                                .team
                                .name
                                .as_ref()
                                .map(|n| n.clone())
                                .unwrap_or("".into()),
                            &oauth_resp.authed_user.id
                        );
                        install_service_fn(Ok(oauth_resp), client).await;
                        SlackClientHttpApi::hyper_redirect_to(&config.redirect_installed_url)
                    }
                    Err(err) => {
                        error!("Slack OAuth error: {}", &err);
                        install_service_fn(Err(err), client).await;
                        SlackClientHttpApi::hyper_redirect_to(&config.redirect_error_redirect_url)
                    }
                }
            }
            (None, Some(err)) => {
                info!("Slack OAuth cancelled with the reason: {}", err);
                install_service_fn(
                    Err(Box::new(SlackClientError::ApiError(
                        SlackClientApiError::new(err.clone()),
                    ))),
                    client,
                );
                let redirect_error_url = format!(
                    "{}{}",
                    &config.redirect_error_redirect_url,
                    req.uri().query().map_or("".into(), |q| format!("?{}", &q))
                );
                SlackClientHttpApi::hyper_redirect_to(&redirect_error_url)
            }
            _ => {
                error!("Slack OAuth cancelled with unknown reason");
                install_service_fn(
                    Err(Box::new(SlackClientError::SystemError(
                        SlackClientSystemError::new("OAuth cancelled with unknown reason".into()),
                    ))),
                    client,
                );
                SlackClientHttpApi::hyper_redirect_to(&config.redirect_error_redirect_url)
            }
        }
    }

    pub fn oauth_service_fn<'a, D, F, I, IF>(
        &self,
        config: Arc<SlackOAuthListenerConfig>,
        install_service_fn: I,
    ) -> impl Fn(
        Request<Body>,
        D,
    ) -> BoxFuture<
        'a,
        Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>,
    >
           + 'a
           + Send
           + Clone
    where
        D: Fn(Request<Body>) -> F + 'a + Send + Sync + Clone,
        F: Future<Output = Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
            + 'a
            + Send,
        I: Fn(
                Result<
                    SlackOAuthV2AccessTokenResponse,
                    Box<dyn std::error::Error + Send + Sync + 'static>,
                >,
                Arc<SlackClient>,
            ) -> IF
            + 'static
            + Send
            + Sync
            + Clone,
        IF: Future<Output = ()> + 'static + Send,
    {
        let client = self.client.clone();
        move |req: Request<Body>, chain: D| {
            let cfg = config.clone();
            let c = chain.clone();
            let install_fn = install_service_fn.clone();
            let sc = client.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, url) if url == cfg.install_path => {
                        Self::slack_oauth_install_service(req, &cfg).await
                    }
                    (&Method::GET, url) if url == cfg.redirect_callback_path => {
                        Self::slack_oauth_callback_service(req, &cfg, sc, install_fn).await
                    }
                    _ => c(req).await,
                }
            }
            .boxed()
        }
    }
}
