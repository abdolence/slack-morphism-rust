use crate::connector::SlackClientHyperConnector;
use crate::listener::SlackClientEventsHyperListener;

use slack_morphism::api::*;
use slack_morphism::errors::*;
use slack_morphism::listener::*;
use slack_morphism::{SlackClient, SlackClientHttpApiUri};

use futures::future::{BoxFuture, FutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response};
use log::*;
use std::future::Future;
use std::sync::{Arc, RwLock};

impl SlackClientEventsHyperListener {
    async fn slack_oauth_install_service(
        _: Request<Body>,
        config: &SlackOAuthListenerConfig,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        let full_uri = SlackClientHttpApiUri::create_url_with_params(
            SlackOAuthListenerConfig::OAUTH_AUTHORIZE_URL_VALUE,
            &vec![
                ("client_id", Some(&config.client_id)),
                ("scope", Some(&config.bot_scope)),
                ("redirect_uri", Some(&config.to_redirect_url())),
            ],
        );
        debug!("Redirecting to Slack OAuth authorize: {}", &full_uri);
        SlackClientHyperConnector::hyper_redirect_to(&full_uri.to_string())
    }

    async fn slack_oauth_callback_service<I, IF>(
        req: Request<Body>,
        config: &SlackOAuthListenerConfig,
        client: Arc<SlackClient<SlackClientHyperConnector>>,
        user_state_storage: Arc<RwLock<SlackClientEventsUserStateStorage>>,
        install_service_fn: I,
        error_handler: BoxedErrorHandler<SlackClientHyperConnector>,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>>
    where
        I: Fn(
                SlackOAuthV2AccessTokenResponse,
                Arc<SlackClient<SlackClientHyperConnector>>,
                Arc<RwLock<SlackClientEventsUserStateStorage>>,
            ) -> IF
            + 'static
            + Send
            + Sync
            + Clone,
        IF: Future<Output = ()> + 'static + Send,
    {
        let params = SlackClientHyperConnector::parse_query_params(&req);
        debug!("Received Slack OAuth callback: {:?}", &params);

        match (params.get("code"), params.get("error")) {
            (Some(code), None) => {
                let oauth_access_resp = client
                    .oauth2_access(
                        &SlackOAuthV2AccessTokenRequest::from(SlackOAuthV2AccessTokenRequestInit {
                            client_id: config.client_id.clone().into(),
                            client_secret: config.client_secret.clone().into(),
                            code: code.into(),
                        })
                        .with_redirect_uri(config.to_redirect_url()),
                    )
                    .await;

                match oauth_access_resp {
                    Ok(oauth_resp) => {
                        info!(
                            "Received slack OAuth access resp for: {} / {} / {}",
                            &oauth_resp.team.id,
                            &oauth_resp
                                .team
                                .name
                                .as_ref()
                                .cloned()
                                .unwrap_or_else(|| "".into()),
                            &oauth_resp.authed_user.id
                        );
                        install_service_fn(oauth_resp, client, user_state_storage).await;
                        SlackClientHyperConnector::hyper_redirect_to(&config.redirect_installed_url)
                    }
                    Err(err) => {
                        error!("Slack OAuth error: {}", &err);
                        error_handler(err, client, user_state_storage);
                        SlackClientHyperConnector::hyper_redirect_to(
                            &config.redirect_error_redirect_url,
                        )
                    }
                }
            }
            (None, Some(err)) => {
                info!("Slack OAuth cancelled with the reason: {}", err);
                error_handler(
                    Box::new(SlackClientError::ApiError(SlackClientApiError::new(
                        err.clone(),
                    ))),
                    client,
                    user_state_storage,
                );
                let redirect_error_url = format!(
                    "{}{}",
                    &config.redirect_error_redirect_url,
                    req.uri().query().map_or("".into(), |q| format!("?{}", &q))
                );
                SlackClientHyperConnector::hyper_redirect_to(&redirect_error_url)
            }
            _ => {
                error!("Slack OAuth cancelled with unknown reason");
                error_handler(
                    Box::new(SlackClientError::SystemError(SlackClientSystemError::new(
                        "OAuth cancelled with unknown reason".into(),
                    ))),
                    client,
                    user_state_storage,
                );
                SlackClientHyperConnector::hyper_redirect_to(&config.redirect_error_redirect_url)
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
                SlackOAuthV2AccessTokenResponse,
                Arc<SlackClient<SlackClientHyperConnector>>,
                Arc<RwLock<SlackClientEventsUserStateStorage>>,
            ) -> IF
            + 'static
            + Send
            + Sync
            + Clone,
        IF: Future<Output = ()> + 'static + Send,
    {
        let client = self.environment.client.clone();
        let listener_error_handler = self.environment.error_handler.clone();
        let user_state_storage = self.environment.user_state_storage.clone();

        move |req: Request<Body>, chain: D| {
            let cfg = config.clone();
            let install_fn = install_service_fn.clone();
            let sc = client.clone();
            let error_handler = listener_error_handler.clone();
            let thread_user_state_storage = user_state_storage.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, url) if url == cfg.install_path => {
                        Self::slack_oauth_install_service(req, &cfg).await
                    }
                    (&Method::GET, url) if url == cfg.redirect_callback_path => {
                        Self::slack_oauth_callback_service(
                            req,
                            &cfg,
                            sc,
                            thread_user_state_storage,
                            install_fn,
                            error_handler,
                        )
                        .await
                    }
                    _ => chain(req).await,
                }
            }
            .boxed()
        }
    }
}
