# Rate control, throttling and retrying Slack API method requests

## Enable rate control
Slack API defines [rate limits](https://api.slack.com/docs/rate-limits) to which all of your applications must follow.

By default, throttler *isn't* enabled, so you should enable it explicitly:

```rust,noplaypen
use slack_morphism::prelude::*;
use slack_morphism_hyper::*;

let client = SlackClient::new(
    SlackClientHyperConnector::new()
        .with_rate_control(
            SlackApiRateControlConfig::new()
        )
);
    
```

The example above creates a Slack API Client that follows the official rate limits from Slack.
Because the Slack rate limits apply per workspaces (separately),
to use throttling and limits properly you *have to specify* team id in tokens:

```
let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
let team_id: SlackTeamId = config_env_var("SLACK_TEST_TEAM_ID")?.into();
let token: SlackApiToken = SlackApiToken::new(token_value).with_team_id(team_id);

let session = client.open_session(&token);
```

## Rate control params

You can also customise rate control params using `SlackApiRateControlConfig`:
- To global rate limit all APIs and for all teams use:  `SlackApiRateControlConfig.global_max_rate_limit`. Default is not limited.
- To rate limit all APIs and each team separately: `SlackApiRateControlConfig.team_max_rate_limit`. Default is not limited.
- To change default tiers limits use `SlackApiRateControlConfig.tiers_limits`. Defaults are following the Slack recommendations (almost, there are slight differences to optimize bursting for Tier1).

## Enable automatic retry for rate exceeded requests

To enable automatic retry of Slack Web API method requests,
you need to specify `max_retries` in rate control params (default value is `0`):

```rust,noplaypen

    let client = SlackClient::new(
        SlackClientHyperConnector::new()
            .with_rate_control(
                SlackApiRateControlConfig::new().with_max_retries(5)
            ),
    );       
```
