use anyhow::Result;
use chrono::Duration;
use humantime::parse_duration;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

const SECRET: &str = "my_secret_key";

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct JwtClaims {
    pub sub: String,
    pub aud: String,
    pub iss: String,
    pub iat: u64,
    pub exp: u64,
}

impl JwtClaims {
    fn new(sub: &str, aud: &str, exp: &str, iss: &str) -> Self {
        let now = get_current_timestamp();
        let duration = parse_human_duration(exp).unwrap();
        let t = now + duration.num_seconds() as u64;
        Self {
            sub: sub.to_owned(),
            aud: aud.to_owned(),
            exp: t,
            iat: now,
            iss: iss.to_owned(),
        }
    }
}

pub struct JwtHS256 {
    secret: String,
}

impl JwtHS256 {
    fn new(s: &str) -> Self {
        Self {
            secret: s.to_owned(),
        }
    }

    fn sign(&self, claims: &JwtClaims) -> Result<String> {
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;
        Ok(token)
    }

    fn verify(&self, token: &str) -> Result<JwtClaims> {
        let mut validator = Validation::default();
        // 要设置验证过期时间 但是不验证目标
        validator.validate_aud = false;
        validator.validate_exp = true;
        let data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validator,
        )?;
        // println!("{:?}", data.header);
        // println!("{:?}", data.claims);
        Ok(data.claims)
    }
}

pub fn process_jwt_sign(sub: &str, aud: &str, exp: &str, iss: &str) -> Result<String> {
    let my_claims = JwtClaims::new(sub, aud, exp, iss);
    let jwt = JwtHS256::new(SECRET);
    let token = jwt.sign(&my_claims)?;
    Ok(token)
}

pub fn process_jwt_verify(token: &str) -> Result<JwtClaims> {
    let jwt = JwtHS256::new(SECRET);
    let claims = jwt.verify(token)?;
    Ok(claims)
}

fn parse_human_duration(text: &str) -> Result<Duration> {
    let std_duration = parse_duration(text)?;
    let chrono_duration = Duration::from_std(std_duration)?;
    // println!("duration {:?}", chrono_duration);
    Ok(chrono_duration)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jwt() -> Result<()> {
        let claims = JwtClaims::new("jwt", "device1", "14d", "abc.com");
        let jwt = JwtHS256::new("testkey");

        let token = jwt.sign(&claims)?;
        let result = jwt.verify(&token)?;
        assert_eq!(result, claims);
        Ok(())
    }
}
