extern crate dotenv;
extern crate envy;
extern crate serde;

#[derive(Deserialize, Debug)]
pub struct PlankConfiguration {
    pub app_url: String,
    pub database_url: String,
}

pub fn get_config() -> PlankConfiguration {
    dotenv::dotenv().expect("Failed to read .env file");
    return envy::from_env::<PlankConfiguration>().unwrap();
}
