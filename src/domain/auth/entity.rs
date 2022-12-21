use std::{env, time::SystemTime, sync::Arc};

use dotenv::dotenv;
use jsonwebtoken::{encode, Header, EncodingKey};
use oauth2::{AuthorizationCode, basic::{BasicClient, BasicErrorResponseType, BasicTokenType}, StandardErrorResponse, StandardTokenResponse, EmptyExtraTokenFields, StandardTokenIntrospectionResponse, StandardRevocableToken, RevocationErrorResponseType, ClientId, ClientSecret, AuthUrl, TokenUrl, reqwest::{async_http_client}, TokenResponse};
use oauth2::RequestTokenError::{ServerResponse, Request, Parse, Other};
use serde::{Serialize, Deserialize};
use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}, Error};

use crate::{domain::user::{create_user::{self, Request as UserRequest}, fetch_one_user, update_user::{self, Request as UpdateUserRequest}}, repositories::user::PgRepository};


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
pub struct UserProfile {
  pub id: i64,
  pub login: String,
  pub name: Option<String>,
  pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub exp: u128,
  pub aud: Option<String>,
  pub iss: Option<String>,
  pub user: UserProfile,
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
  pub fn new(_provider: OAuthProvider, auth_code: AuthorizationCode) -> Self {
    dotenv().ok();

    let client = BasicClient::new(
      ClientId::new(env::var("GITHUB_CLIENT_ID").expect("").to_string()),
      Some(ClientSecret::new(env::var("GITHUB_CLIENT_SECRET").expect("").to_string())),
      AuthUrl::new(env::var("GITHUB_AUTH_URL").expect("").to_string()).expect(""),
      Some(TokenUrl::new(env::var("GITHUB_TOKEN_URL").expect("").to_string()).expect("")),
    );

    Self {
      client,
      auth_code,
    }
  }

  pub async fn create_jwt(&self, access_token: String) -> Result<String, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("DECAFO"));

    let client = reqwest::Client::builder()
      .default_headers(headers.clone())
      .build()
      .unwrap();

    let request = client.to_owned().get("https://api.github.com/user").headers(headers).bearer_auth(access_token);


    match request.send().await {
      Ok(resp) => {
        let user = resp.json::<UserProfile>().await.unwrap();
        let exp  = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("error");

        let repo = Arc::new(PgRepository::try_new().await);
        let req = UserRequest {
          id: user.id,
          login: user.login.clone(),
          name: user.name.clone().expect(""),
          avatar_url: user.avatar_url.clone(),
        };
        let fetch_request = fetch_one_user::Request {
          id: user.id,
        };

        match fetch_one_user::execute(repo.clone(), fetch_request).await {
          Ok(res) => {
            if res.id == user.id {
              let req = UpdateUserRequest {
                id: req.id,
                name: req.name,
                avatar_url: req.avatar_url,
              };
              update_user::execute(repo, req).await;
            }
          },
          Err(_) => {
            match create_user::execute(repo, req).await {
              Ok(res) => {
                println!("create user:: {:?}", res);
              },
              Err(err) => {
                println!("{:?}", err);
              },
            };
          },
        }

        let my_claims = Claims {
          exp: exp.as_millis() + (60 * 1000) * 60, // 1hour
          aud: Some("".to_string()),
          iss: Some("DECAFO".to_string()),
          user,
        };

        let jwt = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).expect("msg");

        Ok(jwt)
      },
      Err(e) => {
        println!("{:?}", e);
        Err(e)
      },
    }
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
          ServerResponse(_) => {
            Err(TokenError::ServerReponse)
          },
          Request(_) => Err(TokenError::Request),
          Parse(_, _) => Err(TokenError::Parse),
          Other(_) => Err(TokenError::Other),
        }
      },
    }
  }
}