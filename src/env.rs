use std::env;

pub struct Env {
    pub port: u16,
    pub host: String,
    pub database_url: String,
}

impl Env {
    pub fn init() -> Self {
        dotenvy::dotenv().expect("Failed to load enviroment variables");

        let port = env::var("PORT")
            .expect("Failed to load 'port' var")
            .parse()
            .expect("Failed to parse 'port' var");

        let host = env::var("HOST").expect("Unable to load 'host' var");
        let database_url = env::var("DATABASE_URL").expect("Unable to load 'host' var");

        Env {
            port,
            host,
            database_url,
        }
    }
}
