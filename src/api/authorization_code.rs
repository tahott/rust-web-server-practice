use std::env;

use actix_web::{HttpResponse, HttpRequest};
use dotenv::dotenv;
use oauth2::{ClientSecret, TokenUrl, AuthUrl, PkceCodeChallenge, CsrfToken, Scope, ClientId, basic::BasicClient, RedirectUrl};

pub async fn authorization_code(req: HttpRequest) -> HttpResponse {
  dotenv().ok();

  let client = BasicClient::new(
    ClientId::new(env::var("github_client_id").expect("").to_string()),
    Some(ClientSecret::new(env::var("github_client_secret").expect("").to_string())),
    AuthUrl::new(env::var("github_auth_url").expect("").to_string()).expect(""),
    Some(TokenUrl::new(env::var("github_token_url").expect("").to_string()).expect("")),
  ).set_redirect_uri(RedirectUrl::new("".to_string()).expect(""));

  let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
  let (auth_url, _csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    // .add_scope(Scope::new("read:user".to_string()))
    .add_scope(Scope::new("user:email".to_string()))
    .set_pkce_challenge(pkce_challenge)
    .url();

  HttpResponse::Ok().json(auth_url.as_str())
}