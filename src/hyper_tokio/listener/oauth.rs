use crate::hyper_tokio::connector::SlackClientHyperConnector;
use crate::hyper_tokio::hyper_ext::HyperExtensions;
use crate::hyper_tokio::{Body, SlackClientEventsHyperListener};

use crate::api::*;
use crate::errors::*;
use crate::listener::*;
use crate::{AnyStdResult, SlackClient, SlackClientHttpApiUri};

use futures::future::{BoxFuture, FutureExt};
use hyper::body::Incoming;
use hyper::{Method, Request, Response};
use hyper_util::client::legacy::connect::Connect;
use rvstruct::*;
use std::future::Future;
use std::sync::Arc;
use tracing::*;

impl<H: 'static + Send + Sync + Connect + Clone> SlackClientEventsHyperListener<H> {
    pub(crate) async fn slack_oauth_install_service(
        _: Request<Incoming>,
        config: &SlackOAuthListenerConfig,
    ) -> AnyStdResult<Response<Body>> {
        let full_uri = SlackClientHttpApiUri::create_url_with_params(
            SlackOAuthListenerConfig::OAUTH_AUTHORIZE_URL_VALUE.parse()?,
            &vec![
                ("client_id", Some(config.client_id.value())),
                ("scope", Some(&config.bot_scope)),
                ("user_scope", Some(&config.user_scope)),
                (
                    "redirect_uri",
                    Some(config.to_redirect_url()?.as_str().to_string()).as_ref(),
                ),
            ],
        )?;
        debug!("Redirecting to Slack OAuth authorize: {}", &full_uri);
        HyperExtensions::hyper_redirect_to(full_uri.as_ref())
    }

    pub(crate) async fn slack_oauth_callback_service(
        req: Request<Incoming>,
        config: &SlackOAuthListenerConfig,
        client: Arc<SlackClient<SlackClientHyperConnector<H>>>,
        user_state_storage: SlackClientEventsUserState,
        install_service_fn: UserCallbackFunction<
            SlackOAuthV2AccessTokenResponse,
            impl Future<Output = ()> + 'static + Send,
            SlackClientHyperConnector<H>,
        >,
        error_handler: BoxedErrorHandler<SlackClientHyperConnector<H>>,
    ) -> AnyStdResult<Response<Body>> {
        let params = HyperExtensions::parse_query_params(req.uri());
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
                        .with_redirect_uri(config.to_redirect_url()?),
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
                        HyperExtensions::hyper_redirect_to(&config.redirect_installed_url)
                    }
                    Err(err) => {
                        error!("Slack OAuth error: {}", &err);
                        error_handler(Box::new(err), client, user_state_storage);
                        HyperExtensions::hyper_redirect_to(&config.redirect_error_redirect_url)
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
                HyperExtensions::hyper_redirect_to(&redirect_error_url)
            }
            _ => {
                error!("Slack OAuth cancelled with unknown reason");
                error_handler(
                    Box::new(SlackClientError::SystemError(
                        SlackClientSystemError::new()
                            .with_message("OAuth cancelled with unknown reason".into()),
                    )),
                    client,
                    user_state_storage,
                );
                HyperExtensions::hyper_redirect_to(&config.redirect_error_redirect_url)
            }
        }
    }

    pub fn oauth_service_fn<'a, D, F>(
        &self,
        config: Arc<SlackOAuthListenerConfig>,
        install_service_fn: UserCallbackFunction<
            SlackOAuthV2AccessTokenResponse,
            impl Future<Output = ()> + 'static + Send,
            SlackClientHyperConnector<H>,
        >,
    ) -> impl Fn(Request<Incoming>, D) -> BoxFuture<'a, AnyStdResult<Response<Body>>> + 'a + Send + Clone
    where
        D: Fn(Request<Incoming>) -> F + 'a + Send + Sync + Clone,
        F: Future<Output = AnyStdResult<Response<Body>>> + 'a + Send,
    {
        let client = self.environment.client.clone();
        let listener_error_handler = self.environment.error_handler.clone();
        let user_state_storage = self.environment.user_state.clone();

        move |req: Request<Incoming>, chain: D| {
            let cfg = config.clone();
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
                            install_service_fn,
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
