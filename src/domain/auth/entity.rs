use std::{env, time::SystemTime};

use dotenv::dotenv;
use jsonwebtoken::{encode, Header, EncodingKey};
use log::info;
use oauth2::{AuthorizationCode, basic::{BasicClient, BasicErrorResponseType, BasicTokenType}, StandardErrorResponse, StandardTokenResponse, EmptyExtraTokenFields, StandardTokenIntrospectionResponse, StandardRevocableToken, RevocationErrorResponseType, ClientId, ClientSecret, AuthUrl, TokenUrl, reqwest::{async_http_client}, TokenResponse};
use oauth2::RequestTokenError::{ServerResponse, Request, Parse, Other};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum OAuthProvider {
  Github,
}

pub enum TokenError {
  ServerReponse,
  Request,
  Parse,
  Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub exp: u128,
  pub aud: Option<String>,
  pub iss: Option<String>,
}

impl TryFrom<String> for OAuthProvider {
  type Error = ();

  fn try_from(t: String) -> Result<Self, Self::Error> {
      match t.as_str() {
          "Github" => Ok(Self::Github),
          _ => Err(()),
      }
  }
}

impl From<OAuthProvider> for String {
  fn from(t: OAuthProvider) -> Self {
      String::from(match t {
          OAuthProvider::Github => "Github",
      })
  }
}

#[derive(Debug)]
pub struct Authentication {
  pub client: oauth2::Client<StandardErrorResponse<BasicErrorResponseType>, StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, BasicTokenType, StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>, StandardRevocableToken, StandardErrorResponse<RevocationErrorResponseType>>,
  pub auth_code: AuthorizationCode,
}

impl Authentication {
  pub fn new(provider: OAuthProvider, auth_code: AuthorizationCode) -> Self {
    dotenv().ok();
    info!("provider is {:?}", provider);
    let client = BasicClient::new(
      ClientId::new(env::var("github_client_id").expect("").to_string()),
      Some(ClientSecret::new(env::var("github_client_secret").expect("").to_string())),
      AuthUrl::new(env::var("github_auth_url").expect("").to_string()).expect(""),
      Some(TokenUrl::new(env::var("github_token_url").expect("").to_string()).expect("")),
    );

    Self {
      client,
      auth_code,
    }
  }

  pub fn create_jwt(&self) -> String {
    let exp  = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("error");

    let my_claims = Claims {
      exp: exp.as_millis() + (60 * 1000 * 60 * 8), // 8hour
      aud: Some("".to_string()),
      iss: Some("DECAFO".to_string()),
    };

    let jwt = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).expect("msg");

    jwt
  }

  pub async fn get_access_token(&self) -> Result<String, TokenError> {
    let code = self.auth_code.clone();

    let token_response = self.client
      .exchange_code(code)
      .request_async(async_http_client).await;

    match token_response {
      Ok(token) => {
        let token = token.access_token().secret().to_owned();

        Ok(token)
      },
      Err(err) => {
        match err {
          ServerResponse(e) => {
            Err(TokenError::ServerReponse)
          },
          Request(e) => Err(TokenError::Request),
          Parse(e, _) => Err(TokenError::Parse),
          Other(e) => Err(TokenError::Other),
        }
      },
    }
  }
}