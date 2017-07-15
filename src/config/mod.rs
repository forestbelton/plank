extern crate dotenv;
extern crate envy;
extern crate serde;

#[derive(Deserialize, Debug)]
pub struct PlankConfiguration {
    database: String
}

pub fn get_config() -> PlankConfiguration {
    dotenv::dotenv().expect("Failed to read .env file");
    return envy::from_env::<PlankConfiguration>().unwrap();
}
