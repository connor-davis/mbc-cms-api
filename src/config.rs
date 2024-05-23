use std::{env, process::exit};

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub admin_email: String,
    pub admin_password: String,
    pub mfa_issuer: String,
    pub mfa_128_bit_secret: String,
}

impl Config {
    pub fn init() -> Config {
        let environment_result = dotenv();

        if environment_result.is_err() {
            println!("🔥 Failed to load .env file.");
            std::process::exit(1);
        }

        let database_url = match env::var("DATABASE_URL") {
            Ok(jwt_secret) => Some(jwt_secret),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring DATABASE_URL environment variable: {}",
                    error
                );

                exit(0);
            }
        }
        .unwrap();

        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(jwt_secret) => Some(jwt_secret),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring JWT_SECRET environment variable: {}",
                    error
                );

                exit(0);
            }
        }
        .unwrap();

        let admin_email = match env::var("ADMIN_EMAIL") {
            Ok(admin_email) => Some(admin_email),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring ADMIN_EMAIL environment variable: {}",
                    error
                );

                exit(0);
            }
        }
        .unwrap();

        let admin_password = match env::var("ADMIN_PASSWORD") {
            Ok(admin_password) => Some(admin_password),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring ADMIN_PASSWORD environment variable: {}",
                    error
                );

                exit(0);
            }
        }
        .unwrap();

        let mfa_issuer = match env::var("MFA_ISSUER") {
            Ok(mfa_issuer) => Some(mfa_issuer),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring MFA_ISSUER environment variable: {}",
                    error
                );

                exit(0);
            }
        }
        .unwrap();

        let mfa_128_bit_secret = match env::var("MFA_128_BIT_SECRET") {
            Ok(mfa_128_bit_secret) => Some(mfa_128_bit_secret),
            Err(error) => {
                tracing::error!(
                    "Error while acquiring MFA_128_BIT_SECRET environment variable: {}",
                    error
                );

                exit(0);
            }
        }
        .unwrap();

        Config {
            database_url,
            jwt_secret,
            admin_email,
            admin_password,
            mfa_issuer,
            mfa_128_bit_secret,
        }
    }
}
