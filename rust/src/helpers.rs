extern crate crypto;
extern crate jwt;

use std::env;
use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{
    Header,
    Registered,
    Token,
};

pub fn generate_jwt_token(email: String) -> String {
    let header: Header = Default::default();
    // TODO: iss, exp, iat etc
    let claims = Registered {
        sub: Some(email),
        ..Default::default()
    };

    let token = Token::new(header, claims);
    // Sign the token
    // TODO: Secret key
    if let Err(e) = env::var("SECRET_KEY") {
        panic!("Your SECRET_KEY environment variable is not set! Please configure it in your .env file.")
    }
    let secret_key: String = env::var("SECRET_KEY").ok().unwrap();
    let gen_jwt = token.signed("secret".as_bytes(), Sha256::new()).unwrap();
    gen_jwt
}