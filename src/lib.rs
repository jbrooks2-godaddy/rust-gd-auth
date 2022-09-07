use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};
use std::os::raw::c_char;

#[derive(Serialize, Deserialize, Debug)]
struct KeyData {
    e: String,
    kid: String,
    kty: String,
    n: String
}

#[derive(Serialize, Deserialize, Debug)]
struct KeyResponse {
    r#type: String,
    id: String,
    code: u32,
    message: String,
    data: KeyData
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Claims {
   auth: String,
   typ: String,
   jti: String,
   iat: u32,
   shopperId: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfig {
    host: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Token {
    claims: Claims
}

fn get_public_key(host: &str, kid: &str) -> Result<KeyData, reqwest::Error> {
    let url = format!("https://sso.{}/v1/api/key/{}", host, kid);
    println!("Requesting: {}", url);
    let resp = reqwest::blocking::get(url)?.json::<KeyResponse>()?;
    println!("Resp: {:#?}", resp);
    return Ok(resp.data);
}

fn parse_token(config: &AuthConfig, token: &str) -> Result<Token, Box<dyn std::error::Error>> {
    let header = decode_header(token)?;
    let kid = match header.kid {
        Some(k) => k,
        None  => return Err("bad kid".into())
    };
    println!("Found kid: {}", kid);
    let key_data = get_public_key(&config.host, &kid)?;

    println!("Verifying signature");
    let key = DecodingKey::from_rsa_components(&key_data.n, &key_data.e)?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.required_spec_claims = HashSet::new();
    let token_message = decode::<Claims>(token, &key, &validation);
    let result = Token {
        claims: token_message?.claims
    };

    return Ok(result);
}

#[no_mangle]
pub unsafe extern "C" fn parse(
    config: *const c_char,
    token: *const c_char,
    output: *mut c_char
) -> i32 {
    let config_json = match cobhan::cbuffer_to_hashmap_json(config) {
        Ok(input_json) => input_json,
        Err(e) => return e,
    };

    let auth_config = AuthConfig {
        host: config_json["host"].as_str().unwrap().to_string()
    };

    let token_str = match cobhan::cbuffer_to_string(token) {
        Ok(tok) => tok,
        Err(e) => return e,
    };

    let parse_result = match parse_token(&auth_config, &token_str) {
        Ok(res) => res,
        Err(_e) => return -1
    };

    let json_result = match serde_json::to_string(&parse_result) {
        Ok(res) => res,
        Err(_e) => return -2
    };

    cobhan::string_to_cbuffer(&json_result, output)
}
