use slack_morphism::prelude::*;

use hyper_proxy::{Intercept, Proxy, ProxyConnector};

async fn test_proxy_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let proxy = {
        let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();

        let proxy_uri = "http://proxy.domain.unfortunate.world.example.net:3128"
            .parse()
            .unwrap();
        let proxy = Proxy::new(Intercept::Https, proxy_uri);
        ProxyConnector::from_proxy(https_connector, proxy).unwrap()
    };

    let _client = SlackClient::new(SlackClientHyperConnector::with_connector(proxy));

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    test_proxy_client().await?;

    Ok(())
}
