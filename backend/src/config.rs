#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port_http: u16,
    pub port_https: u16,
    pub enable_https: bool,
    pub email_verification: bool,
    pub host_url: String,
}

impl Config {

    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let enable_https =std::env::var("ENABLE_HTTPS").expect("ENABLE_HTTPS must be set");
        let email_verification =std::env::var("VERIFY_EMAIL").expect("VERIFY_EMAIL must be set");
        let host_url =std::env::var("HOST_URL").expect("HOST_URL must be set");

        Config {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            port_https: 8080,
            port_http: 8000,
            enable_https: enable_https.parse::<bool>().unwrap_or(false),
            email_verification: email_verification.parse::<bool>().unwrap(),
            host_url,
        }
    }
    
}
