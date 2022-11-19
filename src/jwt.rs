use jwt_simple::prelude::*;
use std::{fs, error::Error};

#[derive(Deserialize, Serialize)]
pub struct RoleClaims {
    role_id: u64,
    id: u64
}

pub struct TokenManager {
    private_key: RS384KeyPair,
    public_key: RS384PublicKey,
    token_duration: u64
}

impl TokenManager {
    pub fn new() -> TokenManager {
        let (private_key, public_key) = match get_keys() {
            Ok((private, public)) => (private, public),
            Err(e) => panic!("{}", e)
        };
        let token_duration: u64 = dotenv!("TOKEN_DURATION_HOURS")
            .trim()
            .parse()
            .expect("number required");

        return TokenManager {
            private_key,
            public_key,
            token_duration
        };
    }

    pub async fn create_token(&self) -> String {
        let custom_claims = RoleClaims {
            role_id: 0,
            id: 0
        };
        let claims = Claims::with_custom_claims(custom_claims, Duration::from_hours(self.token_duration));
        return self.private_key.sign(claims).expect("Claims should be valid");
    }

    pub async fn verify_token(&self, token: String) -> Result<JWTClaims<RoleClaims>, Box<dyn Error + Send + Sync>> {
        let claims = self.public_key.verify_token::<RoleClaims>(&token, None)?;
        Ok(claims)
    }
}

pub fn get_keys() -> Result<(RS384KeyPair, RS384PublicKey), Box<dyn Error>> {
    let private_key_content = fs::read_to_string("../keys/private.pem")?;
    let public_key_content = fs::read_to_string("../keys/public.pem")?;

    let private_key = RS384KeyPair::from_pem(&private_key_content)?;
    let public_key = RS384PublicKey::from_pem(&public_key_content)?;

    return Ok((private_key, public_key));
}