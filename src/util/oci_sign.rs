use std::time::SystemTime;

use axum::http::HeaderValue;
use base64::prelude::*;
use httpdate::HttpDate;
use reqwest::Request;
use rsa::signature::{SignatureEncoding, Signer};
use sha2::{Digest, Sha256};

use crate::models::config::Config;

pub trait OciSign {
    fn sign_request(self, config: &Config) -> Self;
}

impl OciSign for Request {
    fn sign_request(mut self, config: &Config) -> Self {
        if self.headers().get("x-date").is_none() {
            self.headers_mut().append(
                "x-date",
                HeaderValue::from_str(&HttpDate::from(SystemTime::now()).to_string()).unwrap(),
            );
        }

        let mut headers = "x-date (request-target) host".to_string();

        let mut signing_string = format!(
            "x-date: {}\n(request-target): {} {}{}{}\nhost: {}",
            self.headers().get("x-date").unwrap().to_str().unwrap(),
            self.method().as_str().to_ascii_lowercase(),
            self.url().path(),
            if self.url().query().is_some() {
                "?"
            } else {
                ""
            },
            self.url().query().unwrap_or(""),
            self.url().host_str().unwrap(),
        );

        if self
            .headers()
            .get("content-type")
            .is_some_and(|x| x.to_str().unwrap() == "application/json")
        {
            let body = self.body().unwrap().as_bytes().unwrap();
            let body_length = body.len().to_string();
            let mut hasher = Sha256::new();
            hasher.update(body);
            let body_sha256 = BASE64_STANDARD.encode(hasher.finalize());

            self.headers_mut().append(
                "content-length",
                HeaderValue::from_str(&body_length).unwrap(),
            );

            headers.push_str(" content-length");
            signing_string.push_str(&format!("\ncontent-length: {}", body_length));

            self.headers_mut().append(
                "x-content-sha256",
                HeaderValue::from_str(&body_sha256).unwrap(),
            );

            headers.push_str(" x-content-sha256");
            signing_string.push_str(&format!("\nx-content-sha256: {}", body_sha256));
        }

        let signature = BASE64_STANDARD.encode(
            config
                .oci_signing_key()
                .sign(signing_string.as_bytes())
                .to_bytes(),
        );

        let auth_value = format!(
            r#"Signature version="1",keyId="{}/{}/{}",algorithm="rsa-sha256",headers="{}",signature="{}""#,
            config.oci_tenancy_oid(),
            config.oci_user_oid(),
            config.oci_cert_fingerprint(),
            headers,
            signature
        );

        self.headers_mut()
            .append("authorization", HeaderValue::from_str(&auth_value).unwrap());

        self
    }
}
