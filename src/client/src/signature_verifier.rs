use ring::hmac;
use rsb_derive::Builder;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct SlackEventSignatureVerifier {
    secret_len: usize,
    key: hmac::Key,
}

impl SlackEventSignatureVerifier {
    pub const SLACK_SIGNED_HASH_HEADER: &'static str = "x-slack-signature";
    pub const SLACK_SIGNED_TIMESTAMP: &'static str = "x-slack-request-timestamp";

    pub fn new(secret: &str) -> Self {
        let secret_bytes = secret.as_bytes();
        SlackEventSignatureVerifier {
            secret_len: secret_bytes.len(),
            key: hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes()),
        }
    }

    fn sign<'a, 'b>(&'a self, body: &'b str, ts: &'b str) -> String {
        let data_to_sign = format!("v0:{}:{}", ts, body);
        format!(
            "v0={}",
            hex::encode(hmac::sign(&self.key, data_to_sign.as_bytes()))
        )
    }

    pub fn verify<'b>(
        &self,
        hash: &'b str,
        body: &'b str,
        ts: &'b str,
    ) -> Result<(), SlackEventSignatureVerifierError> {
        if self.secret_len == 0 {
            Err(SlackEventSignatureVerifierError::CryptoInitError(
                SlackEventSignatureCryptoInitError::new("secret key is empty".into()),
            ))
        } else {
            let hash_to_check = self.sign(body, ts);
            if hash_to_check != hash {
                Err(SlackEventSignatureVerifierError::WrongSignatureError(
                    SlackEventWrongSignatureErrorInit {
                        body_len: body.len(),
                        ts: ts.into(),
                        received_hash: hash.into(),
                        generated_hash: hash_to_check,
                    }
                    .into(),
                ))
            } else {
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum SlackEventSignatureVerifierError {
    CryptoInitError(SlackEventSignatureCryptoInitError),
    AbsentSignatureError(SlackEventAbsentSignatureError),
    WrongSignatureError(SlackEventWrongSignatureError),
}

impl Display for SlackEventSignatureVerifierError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            SlackEventSignatureVerifierError::CryptoInitError(ref err) => err.fmt(f),
            SlackEventSignatureVerifierError::AbsentSignatureError(ref err) => err.fmt(f),
            SlackEventSignatureVerifierError::WrongSignatureError(ref err) => err.fmt(f),
        }
    }
}

impl Error for SlackEventSignatureVerifierError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SlackEventSignatureVerifierError::CryptoInitError(ref err) => Some(err),
            SlackEventSignatureVerifierError::AbsentSignatureError(ref err) => Some(err),
            SlackEventSignatureVerifierError::WrongSignatureError(ref err) => Some(err),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Builder)]
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

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackEventAbsentSignatureError {}

impl Display for SlackEventAbsentSignatureError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Slack API signature is absent")
    }
}

impl Error for SlackEventAbsentSignatureError {}

#[derive(Debug, PartialEq, Clone, Builder)]
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

#[test]
fn check_signature_success() {
    let rng = ring::rand::SystemRandom::new();
    let key_value: [u8; ring::digest::SHA256_OUTPUT_LEN] =
        ring::rand::generate(&rng).unwrap().expose();
    let key_str: String = hex::encode(key_value);

    let verifier = SlackEventSignatureVerifier::new(&key_str);

    const TEST_BODY: &'static str = "test-body";
    const TEST_TS: &'static str = "test-ts";

    let hash = verifier.sign(TEST_BODY, TEST_TS);

    match verifier.verify(&hash, TEST_BODY, TEST_TS) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }
}

#[test]
fn test_precoded_data() {
    const TEST_SECRET: &'static str =
        "d058b0b8f3f91e4446ad981890c9b6c16b2acc85367e30a2d76b8a95e525c02a";
    const TEST_HASH: &'static str =
        "v0=37ca0519af8b621f18b13586fc72488ebb159fc730a5d1718dd823dec69dea95";
    const TEST_BODY: &'static str = "test-body";
    const TEST_TS: &'static str = "test-ts";

    let verifier = SlackEventSignatureVerifier::new(TEST_SECRET);

    match verifier.verify(TEST_HASH, TEST_BODY, TEST_TS) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }
}

#[test]
fn check_empty_secret_error_test() {
    match SlackEventSignatureVerifier::new("").verify("test-hash", "test-body", "test-ts") {
        Err(SlackEventSignatureVerifierError::CryptoInitError(ref err)) => {
            assert!(!err.message.is_empty())
        }
        _ => unreachable!(),
    }
}
