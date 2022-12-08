use dotenvy::var;
use jwt_simple::prelude::*;
use std::{error::Error, fs};

use crate::user::role::Role;

use super::role_claims::RoleClaims;

pub struct TokenManager {
    private_key: RS384KeyPair,
    public_key: RS384PublicKey,
    token_duration: u64,
}

impl TokenManager {
    pub fn new() -> TokenManager {
        let (private_key, public_key) = match get_keys() {
            Ok((private, public)) => (private, public),
            Err(e) => panic!("{}", e),
        };
        let token_duration: u64 = var("TOKEN_DURATION_HOURS")
            .expect("Environment var should not be null!")
            .trim()
            .parse()
            .expect("number required");

        return TokenManager {
            private_key,
            public_key,
            token_duration,
        };
    }

    pub async fn create_token(&self, role: Role, role_id: i32, ) -> String {
        let custom_claims = RoleClaims { role, id: role_id};
        let claims =
            Claims::with_custom_claims(custom_claims, Duration::from_hours(self.token_duration));
        return self
            .private_key
            .sign(claims)
            .expect("Claims should be valid");
    }

    pub async fn verify_token(
        &self,
        token: String,
    ) -> Result<JWTClaims<RoleClaims>, Box<dyn Error + Send + Sync>> {
        let claims = self.public_key.verify_token::<RoleClaims>(&token, None)?;
        Ok(claims)
    }
}

fn get_keys() -> Result<(RS384KeyPair, RS384PublicKey), Box<dyn Error>> {
    let private_key_content = fs::read_to_string(var("KEY_DIR").expect("Environment var should not be null!") + "/private.pem")?;
    let public_key_content = fs::read_to_string(var("KEY_DIR").expect("Environment var should not be null!") + "/public.pem")?;

    let private_key = RS384KeyPair::from_pem(&private_key_content)?;
    let public_key = RS384PublicKey::from_pem(&public_key_content)?;

    return Ok((private_key, public_key));
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_get_keys() {
        let success = match get_keys() {
            Ok(_) => true,
            Err(_) => false
        };

        assert_eq!(success, true);
    }

    #[tokio::test]
    async fn test_create_token() {
        let regex = Regex::new(r"^[A-Za-z0-9-_=]+\.[A-Za-z0-9-_=]+\.?[A-Za-z0-9-_.+/=]*$").unwrap();
        let manager = TokenManager::new();
        let token = manager.create_token(Role::CUSTOMER, 1).await;
        assert!(regex.is_match(&token[..]))
    }
}
