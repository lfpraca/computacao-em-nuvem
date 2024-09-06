use std::{error::Error, fs, path::PathBuf};

use rsa::{pkcs1v15::SigningKey, pkcs8::DecodePrivateKey, sha2::Sha256, RsaPrivateKey};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigBuilder {
    database_url: String,
    bind_addr: String,
    oci_tenancy_oid: String,
    oci_user_oid: String,
    oci_cert_fingerprint: String,
    oci_namespace: String,
    oci_bucket: String,
    oci_cert_path: PathBuf,
}

impl ConfigBuilder {
    pub fn build(self) -> Result<Config, Box<dyn Error>> {
        let private_key_pem = fs::read_to_string(self.oci_cert_path)?;

        let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)?;
        let oci_signing_key = SigningKey::<Sha256>::new(private_key);

        Ok(Config {
            database_url: self.database_url,
            bind_addr: self.bind_addr,
            oci_tenancy_oid: self.oci_tenancy_oid,
            oci_user_oid: self.oci_user_oid,
            oci_cert_fingerprint: self.oci_cert_fingerprint,
            oci_namespace: self.oci_namespace,
            oci_bucket: self.oci_bucket,
            oci_signing_key,
        })
    }
}

pub struct Config {
    database_url: String,
    bind_addr: String,
    oci_tenancy_oid: String,
    oci_user_oid: String,
    oci_cert_fingerprint: String,
    oci_namespace: String,
    oci_bucket: String,
    oci_signing_key: SigningKey<Sha256>,
}

impl Config {
    pub fn database_url(&self) -> &String {
        &self.database_url
    }

    pub fn bind_addr(&self) -> &String {
        &self.bind_addr
    }

    pub fn oci_tenancy_oid(&self) -> &String {
        &self.oci_tenancy_oid
    }

    pub fn oci_user_oid(&self) -> &String {
        &self.oci_user_oid
    }

    pub fn oci_cert_fingerprint(&self) -> &String {
        &self.oci_cert_fingerprint
    }

    pub fn oci_namespace(&self) -> &String {
        &self.oci_namespace
    }

    pub fn oci_bucket(&self) -> &String {
        &self.oci_bucket
    }

    pub fn oci_signing_key(&self) -> &SigningKey<Sha256> {
        &self.oci_signing_key
    }
}
