use std::env;

use actix_web::{HttpResponse};
use dotenv::dotenv;
use oauth2::{ClientSecret, TokenUrl, AuthUrl, PkceCodeChallenge, CsrfToken, Scope, ClientId, basic::BasicClient, RedirectUrl};

pub async fn authorization_code() -> HttpResponse {
  dotenv().ok();

  let client_id = ClientId::new(env::var("GITHUB_CLIENT_ID").unwrap());
  let client_secret = Some(ClientSecret::new(env::var("GITHUB_CLIENT_SECRET").unwrap()));
  let auth_url = AuthUrl::new(env::var("GITHUB_AUTH_URL").unwrap()).unwrap();
  let token_url = Some(TokenUrl::new(env::var("GITHUB_TOKEN_URL").unwrap()).unwrap());
  let redirect_url = RedirectUrl::new(env::var("GITHUB_REDIRECT_URL").unwrap()).unwrap();

  let client = BasicClient::new(
    client_id,
    client_secret,
    auth_url,
    token_url,
  ).set_redirect_uri(redirect_url);

  let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
  let (auth_url, _csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new("read:user".to_string()))
    .add_scope(Scope::new("user:email".to_string()))
    .set_pkce_challenge(pkce_challenge)
    .url();

  HttpResponse::Ok().json(auth_url)
}