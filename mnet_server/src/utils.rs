use std::env;

use anyhow::{anyhow, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn random_string(len: usize) -> String {
    use rand::distributions::{Alphanumeric, DistString};
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

fn jwt_key() -> Result<Hmac<Sha256>> {
    let jwt_secret = env::var("JWT_SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())?;
    Ok(key)
}

pub fn create_jwt(id: &str) -> Result<String> {
    use std::collections::BTreeMap;

    use jwt::SignWithKey;

    let key = jwt_key()?;
    let mut claims = BTreeMap::new();
    claims.insert("key", id);

    let jwt_string = claims.sign_with_key(&key)?;
    Ok(jwt_string)
}

pub fn decode_jwt(jwt_string: &str) -> Result<String> {
    use std::collections::BTreeMap;

    use jwt::VerifyWithKey;

    let key = jwt_key()?;
    let claims: BTreeMap<String, String> = jwt_string.verify_with_key(&key)?;
    let id = claims
        .get("key")
        .ok_or(anyhow!("Jwt does not contain id field"))?;
    Ok(id.to_owned())
}
