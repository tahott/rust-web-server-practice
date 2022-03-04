use oauth2::AuthorizationCode;

#[derive(Clone)]
pub enum OAuthProvider {
  Github,
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

pub struct Authentication {
  pub provider: OAuthProvider,
  pub auth_code: AuthorizationCode,
}

impl Authentication {
  pub fn new(provider: OAuthProvider, auth_code: AuthorizationCode) -> Self {
    Self {
      provider,
      auth_code,
    }
  }
}