use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
    jws::decode,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TokenClaims {
    pub iat: usize,
    pub sub: String,
    pub exp: usize,
}

pub fn create_token(
    user_id: &str,
    secret: &str,
    expires_in_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::Error::into("user_id cannot be empty"));
    }
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;

    let claims = TokenClaims {
        iat,
        sub: user_id.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, Error> {
    let decode = decode::<TokenClaims>(
        token.into(),
        DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    );
    match decode {
        Ok(token) => Ok(token.claims.sub),
        Err(e) => jsonwebtoken::errors::Error::into("Unauthorized!"),
    }
}
