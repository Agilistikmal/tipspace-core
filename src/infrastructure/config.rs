use dotenv::{dotenv, var};

pub struct Config {
    pub database_url: String,
}

pub fn load() -> Config {
    dotenv().ok();

    let config = Config {
        database_url: var("DATABASE_URL").expect("DATABASE_URL not found"),
    };

    return config;
}
