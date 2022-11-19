mod rest;
mod jwt;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dotenv_codegen;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .manage(jwt::TokenManager::new())
        .mount("/orders", routes![rest::login])
        .launch()
        .await?;

    Ok(())
}
