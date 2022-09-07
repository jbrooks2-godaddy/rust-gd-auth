use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};
use std::os::raw::c_char;

const TOKEN: &str = "eyJhbGciOiAiUlMyNTYiLCAia2lkIjogIjV5UXU0WlhXMUEifQ.eyJhdXRoIjogImJhc2ljIiwgImZ0YyI6IDEsICJpYXQiOiAxNjYyMTU5ODUzLCAianRpIjogIlhrSjNzOTE5Z3F1NVlZaW9oSW1QTnciLCAidHlwIjogImlkcCIsICJ2YXQiOiAxNjYyMTU5ODUzLCAiZmFjdG9ycyI6IHsia19wdyI6IDE2NjIxNTk4NTN9LCAicGVyIjogdHJ1ZSwgImhiaSI6IDE2NjIxNTk4NTMsICJzaG9wcGVySWQiOiAiMTY4MzE3MyIsICJjaWQiOiAiZTY2NWFlNGEtODE5OS00Yjg5LWI1ZmEtZTExOTFiYWZiZjY3IiwgInBsaWQiOiAiMSIsICJwbHQiOiAxLCAic2hhcmQiOiAiMDEwMiIsICJpZGVudGl0eSI6ICIwYjQ5NjlhMC1mMWI2LTExZTgtODM2OS0wMjQyYzBhOGIwMDIifQ.1c8Gw4I5j5-mqiY1gyMXHEPleAggUhB63-DrLqsRgiuzTe4YQ0Qk5VgDab1RphivU410rDMB2_jTLN4Sw6zYBPxnKcHa-a7rxzDmdzJ6kttxqckHyFhOWeqAwMfuYWtuVsu7mpBbyNroSW8hrMun-pYAoc1uvTvIUIIgfrgY7KBrPXaT6GtjH_Io2yW13ihYq3hO_I5TkzbnTVUBkteIS-t390EJQDb6gMMNQTzx5FI7uSC_klBIGMCQXBnRMXpzLQMEaTdIRafRD0utZ2dA4qrHMwVHmtAlKxRJgfLwsOboqRmWXCaWNAvJphncj_cjw26i_9XLMstYB6YsKNcuOQ";

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
    let token_message = decode::<Claims>(&TOKEN, &key, &validation);
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
