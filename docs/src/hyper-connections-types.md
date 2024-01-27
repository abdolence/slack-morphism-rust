# Different hyper connection types and proxy support

In some cases you may need to configure hyper connection types. 

Common examples:
- Need to use a proxy server
- Have different initialisation for certs/TLS

To do that there is additional initialisation method in `SlackClientHyperConnector`.

For example for proxy server config it might be used as:

```rust,noplaypen

    let proxy = {
        let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_only()
            .enable_http1()
            .build();

        let proxy_uri = "http://proxy.unfortunate.world.example.net:3128"
            .parse()
            .unwrap();
        let proxy = Proxy::new(Intercept::Https, proxy_uri);
        ProxyConnector::from_proxy(https_connector, proxy).unwrap()
    };

    let _client = SlackClient::new(
        SlackClientHyperConnector::with_connector(proxy)
    );
    
```

Please note that this configuration available only for Slack Client, and doesn't work for Socket Mode (WS) mode.
