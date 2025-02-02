use crate::models::SlackSigningSecret;
use futures_util::TryFutureExt;
use hmac::digest::Key;
use hmac::{Hmac, Mac};
use rsb_derive::Builder;
use rvstruct::*;
use sha2::Sha256;
use std::error::Error;
use std::fmt::{Display, Formatter};
use subtle::ConstantTimeEq;

#[derive(Clone)]
pub struct SlackEventSignatureVerifier {
    secret: SlackSigningSecret,
}

impl SlackEventSignatureVerifier {
    pub const SLACK_SIGNED_HASH_HEADER: &'static str = "x-slack-signature";
    pub const SLACK_SIGNED_TIMESTAMP: &'static str = "x-slack-request-timestamp";

    // 5 minutes is the maximum allowed timestamp age for Slack events
    pub const MAX_TIMESTAMP_AGE_SECONDS: i64 = 60 * 5;

    pub fn new(secret: &SlackSigningSecret) -> Self {
        SlackEventSignatureVerifier {
            secret: secret.clone(),
        }
    }

    pub fn verify<'b>(
        &self,
        hash: &'b str,
        body: &'b str,
        ts: &'b str,
    ) -> Result<(), SlackEventSignatureVerifierError> {
        self.verify_at_time(hash, body, ts, chrono::Utc::now().timestamp())
    }

    fn verify_at_time<'b>(
        &self,
        hash: &'b str,
        body: &'b str,
        ts: &'b str,
        time: i64,
    ) -> Result<(), SlackEventSignatureVerifierError> {
        let hash_to_check = self.sign(body, ts)?;
        if hash_to_check.as_bytes().ct_eq(hash.as_bytes()).unwrap_u8() == 1 {
            Self::validate_ts(ts, time)?;
            Ok(())
        } else {
            Err(SlackEventSignatureVerifierError::WrongSignatureError(
                SlackEventWrongSignatureErrorInit {
                    body_len: body.len(),
                    ts: ts.into(),
                    received_hash: hash.into(),
                    generated_hash: hash_to_check,
                }
                .into(),
            ))
        }
    }

    fn sign<'a, 'b>(
        &'a self,
        body: &'b str,
        ts: &'b str,
    ) -> Result<String, SlackEventSignatureVerifierError> {
        if self.secret.value().is_empty() {
            Err(SlackEventSignatureVerifierError::CryptoInitError(
                SlackEventSignatureCryptoInitError::new("Secret key is empty.".into()),
            ))
        } else {
            let mut mac =
                Hmac::<Sha256>::new_from_slice(self.secret.value().as_bytes()).map_err(|e| {
                    SlackEventSignatureVerifierError::CryptoInitError(
                        SlackEventSignatureCryptoInitErrorInit {
                            message: format!("HMAC init error: {}.", e),
                        }
                        .into(),
                    )
                })?;
            mac.update(b"v0:");
            mac.update(ts.as_bytes());
            mac.update(b":");
            mac.update(body.as_bytes());
            let result = mac.finalize().into_bytes();
            Ok(format!("v0={}", hex::encode(result)))
        }
    }

    fn validate_ts(ts: &str, time: i64) -> Result<(), SlackEventSignatureVerifierError> {
        let ts_int = ts.parse::<i64>().map_err(|e| {
            SlackEventSignatureVerifierError::IncorrectOrExpiredTimestampError(
                SlackEventTimestampErrorInit {
                    ts: ts.into(),
                    message: format!("Timestamp is not a valid integer: {}", e),
                }
                .into(),
            )
        })?;
        if (time - ts_int).abs() > Self::MAX_TIMESTAMP_AGE_SECONDS {
            Err(
                SlackEventSignatureVerifierError::IncorrectOrExpiredTimestampError(
                    SlackEventTimestampErrorInit {
                        ts: ts.into(),
                        message: format!(
                            "Timestamp is either incorrect or expired: now: {}, ts: {}. It should be within {} seconds.",
                            time, ts_int, Self::MAX_TIMESTAMP_AGE_SECONDS
                        ),
                    }
                    .into(),
                ),
            )
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub enum SlackEventSignatureVerifierError {
    CryptoInitError(SlackEventSignatureCryptoInitError),
    AbsentSignatureError(SlackEventAbsentSignatureError),
    WrongSignatureError(SlackEventWrongSignatureError),
    IncorrectOrExpiredTimestampError(SlackEventTimestampError),
}

impl Display for SlackEventSignatureVerifierError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            SlackEventSignatureVerifierError::CryptoInitError(ref err) => err.fmt(f),
            SlackEventSignatureVerifierError::AbsentSignatureError(ref err) => err.fmt(f),
            SlackEventSignatureVerifierError::WrongSignatureError(ref err) => err.fmt(f),
            SlackEventSignatureVerifierError::IncorrectOrExpiredTimestampError(ref err) => {
                err.fmt(f)
            }
        }
    }
}

impl Error for SlackEventSignatureVerifierError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SlackEventSignatureVerifierError::CryptoInitError(ref err) => Some(err),
            SlackEventSignatureVerifierError::AbsentSignatureError(ref err) => Some(err),
            SlackEventSignatureVerifierError::WrongSignatureError(ref err) => Some(err),
            SlackEventSignatureVerifierError::IncorrectOrExpiredTimestampError(ref err) => {
                Some(err)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackEventSignatureCryptoInitError {
    pub message: String,
}

impl Display for SlackEventSignatureCryptoInitError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Slack API signature verifier crypto init error: {}",
            self.message
        )
    }
}

impl Error for SlackEventSignatureCryptoInitError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackEventAbsentSignatureError {}

impl Display for SlackEventAbsentSignatureError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Slack API signature is absent")
    }
}

impl Error for SlackEventAbsentSignatureError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackEventWrongSignatureError {
    pub body_len: usize,
    pub ts: String,
    pub received_hash: String,
    pub generated_hash: String,
}

impl Display for SlackEventWrongSignatureError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Slack API signature validation error: Body len: {}, received ts: {}, received hash: {}, generated hash: {}",
            self.body_len,
            self.ts,
            self.received_hash,
            self.generated_hash
        )
    }
}

impl Error for SlackEventWrongSignatureError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct SlackEventTimestampError {
    pub ts: String,
    pub message: String,
}

impl Display for SlackEventTimestampError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Slack API timestamp validation error for ts: '{}'. Reason: {}",
            self.ts, self.message
        )
    }
}

impl Error for SlackEventTimestampError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_signature_success() {
        use sha2::Digest;

        let key_str: String = hex::encode(Sha256::digest("test-key"));

        let verifier = SlackEventSignatureVerifier::new(&key_str.into());

        const TEST_BODY: &str = "test-body";
        let test_ts = chrono::Utc::now().timestamp().to_string();

        let hash = verifier.sign(TEST_BODY, &test_ts).unwrap();
        verifier
            .verify(&hash, TEST_BODY, &test_ts)
            .expect("signature verification failed");
    }

    #[test]
    fn test_precoded_data() {
        const TEST_SECRET: &str = "fa2c05deeef5a4077494e89f54394de0";
        const TEST_HASH: &str =
            "v0=baa91ef62d346e56607bbdd278a8acd570b91226854f15a382f43d33be42f666";
        const TEST_BODY: &str = "test-body";
        const TEST_TS: &str = "1531420618";

        let verifier = SlackEventSignatureVerifier::new(&TEST_SECRET.to_string().into());

        verifier
            .verify_at_time(TEST_HASH, TEST_BODY, TEST_TS, 1531420620)
            .expect("signature verification failed");
    }

    #[test]
    fn check_empty_secret_error_test() {
        match SlackEventSignatureVerifier::new(&"".to_string().into()).verify(
            "test-hash",
            "test-body",
            "1531420618",
        ) {
            Err(SlackEventSignatureVerifierError::CryptoInitError(ref err)) => {
                assert!(!err.message.is_empty())
            }
            Err(e) => panic!("{}", e),
            _ => unreachable!(),
        }
    }

    #[test]
    fn check_signature_error() {
        use sha2::Digest;

        let key_str_correct: String = hex::encode(Sha256::digest("correct-key"));
        let key_str_malicious: String = hex::encode(Sha256::digest("malicious-key"));

        let verifier_correct = SlackEventSignatureVerifier::new(&key_str_correct.into());
        let verifier_malicious = SlackEventSignatureVerifier::new(&key_str_malicious.into());

        const TEST_BODY: &str = "test-body";
        let test_ts = chrono::Utc::now().timestamp().to_string();

        let hash = verifier_malicious.sign(TEST_BODY, &test_ts).unwrap();
        let err = verifier_correct
            .verify(&hash, TEST_BODY, &test_ts)
            .unwrap_err();
        match err {
            SlackEventSignatureVerifierError::WrongSignatureError(_) => {}
            _ => panic!("unexpected error, {}", err),
        }
    }
}
