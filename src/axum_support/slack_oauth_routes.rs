use crate::axum_support::SlackEventsAxumListener;
use crate::hyper_tokio::hyper_ext::HyperExtensions;
use crate::listener::UserCallbackFunction;
use crate::prelude::SlackOAuthListenerConfig;
use axum::response::Response;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use http::Request;
use hyper::client::connect::Connect;
use hyper::Body;
use rvstruct::ValueStruct;
use std::future::Future;
use tracing::*;

use crate::api::*;
use crate::errors::*;
use crate::hyper_tokio::SlackClientHyperConnector;
use crate::SlackClientHttpApiUri;

impl<H: 'static + Send + Sync + Connect + Clone> SlackEventsAxumListener<H> {
    pub fn slack_oauth_install(
        &self,
        config: &SlackOAuthListenerConfig,
    ) -> impl Fn(Request<Body>) -> BoxFuture<'static, Response<Body>> + 'static + Send + Clone {
        let environment = self.environment.clone();
        let config = config.clone();
        move |_| {
            let config = config.clone();
            let environment = environment.clone();
            async move {
                let full_uri = SlackClientHttpApiUri::create_url_with_params(
                    SlackOAuthListenerConfig::OAUTH_AUTHORIZE_URL_VALUE,
                    &vec![
                        ("client_id", Some(config.client_id.value())),
                        ("scope", Some(&config.bot_scope)),
                        (
                            "redirect_uri",
                            Some(config.to_redirect_url()?.as_str().to_string()).as_ref(),
                        ),
                    ],
                );
                debug!("Redirecting to Slack OAuth authorize: {}", &full_uri);
                Ok(HyperExtensions::hyper_redirect_to(&full_uri.to_string())?)
            }
            .map(|res| Self::handle_error(environment, res))
            .boxed()
        }
    }

    pub fn slack_oauth_callback(
        &self,
        config: &SlackOAuthListenerConfig,
        install_service_fn: UserCallbackFunction<
            SlackOAuthV2AccessTokenResponse,
            impl Future<Output = ()> + 'static + Send,
            SlackClientHyperConnector<H>,
        >,
    ) -> impl Fn(Request<Body>) -> BoxFuture<'static, Response<Body>> + 'static + Send + Clone {
        let environment = self.environment.clone();
        let config = config.clone();
        move |req| {
            let config = config.clone();
            let environment = environment.clone();
            let err_environment = environment.clone();
            let err_config = config.clone();

            async move {
                let params = HyperExtensions::parse_query_params(&req);
                debug!("Received Slack OAuth callback: {:?}", &params);

                match (params.get("code"), params.get("error")) {
                    (Some(code), None) => {
                        let oauth_access_resp = environment
                            .client
                            .oauth2_access(
                                &SlackOAuthV2AccessTokenRequest::from(
                                    SlackOAuthV2AccessTokenRequestInit {
                                        client_id: config.client_id.clone(),
                                        client_secret: config.client_secret.clone(),
                                        code: code.into(),
                                    },
                                )
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
                                install_service_fn(
                                    oauth_resp,
                                    environment.client.clone(),
                                    environment.user_state.clone(),
                                )
                                .await;
                                HyperExtensions::hyper_redirect_to(&config.redirect_installed_url)
                            }
                            Err(err) => {
                                error!("Slack OAuth error: {}", &err);
                                (environment.clone().error_handler)(
                                    Box::new(err),
                                    environment.client.clone(),
                                    environment.user_state.clone(),
                                );
                                HyperExtensions::hyper_redirect_to(
                                    &config.redirect_error_redirect_url,
                                )
                            }
                        }
                    }
                    (None, Some(err)) => {
                        info!("Slack OAuth cancelled with the reason: {}", err);
                        (environment.error_handler)(
                            Box::new(SlackClientError::ApiError(SlackClientApiError::new(
                                err.clone(),
                            ))),
                            environment.client.clone(),
                            environment.user_state.clone(),
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
                        (environment.error_handler)(
                            Box::new(SlackClientError::SystemError(
                                SlackClientSystemError::new()
                                    .with_message("OAuth cancelled with unknown reason".into()),
                            )),
                            environment.client.clone(),
                            environment.user_state.clone(),
                        );
                        HyperExtensions::hyper_redirect_to(&config.redirect_error_redirect_url)
                    }
                }
            }
            .map(move |res| match res {
                Ok(result) => result,
                Err(err) => {
                    error!("Slack OAuth system error: {}", err);
                    (err_environment.error_handler)(
                        Box::new(SlackClientError::SystemError(
                            SlackClientSystemError::new()
                                .with_message(format!("OAuth cancelled system error: {}", err)),
                        )),
                        err_environment.client.clone(),
                        err_environment.user_state.clone(),
                    );
                    HyperExtensions::hyper_redirect_to(&err_config.redirect_error_redirect_url)
                        .unwrap()
                }
            })
            .boxed()
        }
    }
}
