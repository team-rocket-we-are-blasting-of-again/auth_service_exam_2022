use rocket::response::Debug;
use std::error::Error;

#[post("/login")]
pub async fn login() -> Result<(), Debug<Box<dyn Error + Send + Sync>>> {
    return Ok(());
}