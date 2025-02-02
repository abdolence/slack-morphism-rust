use crate::models::SlackSigningSecret;
use hmac::{Hmac, Mac};
use rsb_derive::Builder;
use rvstruct::*;
use sha2::Sha256;
use std::error::Error;
use std::fmt::{Display, Formatter};
use subtle::ConstantTimeEq;

#[derive(Clone)]
pub struct SlackEventSignatureVerifier {
    secret_bytes: Vec<u8>,
}

impl SlackEventSignatureVerifier {
    pub const SLACK_SIGNED_HASH_HEADER: &'static str = "x-slack-signature";
    pub const SLACK_SIGNED_TIMESTAMP: &'static str = "x-slack-request-timestamp";

    pub fn new(secret: &SlackSigningSecret) -> Self {
        let secret_bytes = secret.value().as_bytes().to_vec();
        SlackEventSignatureVerifier { secret_bytes }
    }

    fn sign<'a, 'b>(&'a self, body: &'b str, ts: &'b str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.secret_bytes)
            .expect("HMAC can take key of any size");
        mac.update(b"v0:");
        mac.update(ts.as_bytes());
        mac.update(b":");
        mac.update(body.as_bytes());
        let result = mac.finalize().into_bytes();
        format!("v0={}", hex::encode(result))
    }

    pub fn verify<'b>(
        &self,
        hash: &'b str,
        body: &'b str,
        ts: &'b str,
    ) -> Result<(), SlackEventSignatureVerifierError> {
        if self.secret_bytes.is_empty() {
            Err(SlackEventSignatureVerifierError::CryptoInitError(
                SlackEventSignatureCryptoInitError::new("secret key is empty".into()),
            ))
        } else {
            let hash_to_check = self.sign(body, ts);
            if hash_to_check.as_bytes().ct_eq(hash.as_bytes()).unwrap_u8() == 1 {
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

#[test]
fn check_signature_success() {
    use sha2::Digest;

    let key_str: String = hex::encode(Sha256::digest("test-key"));

    let verifier = SlackEventSignatureVerifier::new(&key_str.into());

    const TEST_BODY: &str = "test-body";
    const TEST_TS: &str = "test-ts";

    let hash = verifier.sign(TEST_BODY, TEST_TS);
    verifier
        .verify(&hash, TEST_BODY, TEST_TS)
        .expect("signature verification failed");
}

#[test]
fn test_precoded_data() {
    const TEST_SECRET: &str = "d058b0b8f3f91e4446ad981890c9b6c16b2acc85367e30a2d76b8a95e525c02a";
    const TEST_HASH: &str = "v0=37ca0519af8b621f18b13586fc72488ebb159fc730a5d1718dd823dec69dea95";
    const TEST_BODY: &str = "test-body";
    const TEST_TS: &str = "test-ts";

    let verifier = SlackEventSignatureVerifier::new(&TEST_SECRET.to_string().into());

    verifier
        .verify(TEST_HASH, TEST_BODY, TEST_TS)
        .expect("signature verification failed");
}

#[test]
fn check_empty_secret_error_test() {
    match SlackEventSignatureVerifier::new(&"".to_string().into()).verify(
        "test-hash",
        "test-body",
        "test-ts",
    ) {
        Err(SlackEventSignatureVerifierError::CryptoInitError(ref err)) => {
            assert!(!err.message.is_empty())
        }
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
    const TEST_TS: &str = "test-ts";

    let hash = verifier_malicious.sign(TEST_BODY, TEST_TS);
    let err = verifier_correct
        .verify(&hash, TEST_BODY, TEST_TS)
        .unwrap_err();
    match err {
        SlackEventSignatureVerifierError::WrongSignatureError(_) => {}
        _ => panic!("unexpected error, {}", err),
    }
}
