use crate::AppResult;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use crate::config::APPLICATION_CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(user_id: &str) -> Claims {
        let current_time = OffsetDateTime::now_utc();
        let expiration_time = current_time + Duration::minutes(APPLICATION_CONFIG.jwt.exp as i64);
        let timestamp = expiration_time
            .to_offset(current_time.offset())
            .unix_timestamp();
        Claims {
            sub: user_id.to_owned(),
            exp: timestamp as usize,
        }
    }

    pub fn encode(&self) -> String {
        let encoding_key = EncodingKey::from_secret(APPLICATION_CONFIG.jwt.secrect_key.as_bytes());
        encode(&Header::default(), self, &encoding_key).unwrap()
    }

    pub fn decode(jwt: &str) -> AppResult<Claims> {
        let decoding_key = DecodingKey::from_secret(APPLICATION_CONFIG.jwt.secrect_key.as_bytes());
        let token_data = decode::<Claims>(jwt, &decoding_key, &Validation::default())?;
        Ok(token_data.claims)
    }
}
