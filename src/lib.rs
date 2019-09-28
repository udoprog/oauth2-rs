#![warn(missing_docs)]
//!
//! A simple implementation of the OAuth2 flow, trying to adhere as much as possible to
//! [RFC 6749](https://tools.ietf.org/html/rfc6749).
//!
//! # Getting started: Authorization Code Grant
//!
//! This is the most common OAuth2 flow.
//!
//! ## Example
//!
//! ```
//! extern crate base64;
//! extern crate oauth2;
//! extern crate rand;
//! extern crate url;
//!
//! use oauth2::prelude::*;
//! use oauth2::{
//!     AuthorizationCode,
//!     AuthUrl,
//!     ClientId,
//!     ClientSecret,
//!     CsrfToken,
//!     RedirectUrl,
//!     Scope,
//!     TokenResponse,
//!     TokenUrl
//! };
//! use oauth2::basic::BasicClient;
//! use url::Url;
//!
//! # fn err_wrapper() -> Result<(), Box<std::error::Error>> {
//! // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
//! // token URL.
//! let client =
//!     BasicClient::new(
//!         ClientId::new("client_id".to_string()),
//!         Some(ClientSecret::new("client_secret".to_string())),
//!         AuthUrl::new(Url::parse("http://authorize")?),
//!         Some(TokenUrl::new(Url::parse("http://token")?))
//!     )
//!         // Set the desired scopes.
//!         .add_scope(Scope::new("read".to_string()))
//!         .add_scope(Scope::new("write".to_string()))
//!
//!         // Set the URL the user will be redirected to after the authorization process.
//!         .set_redirect_url(RedirectUrl::new(Url::parse("http://redirect")?));
//!
//! // Generate the full authorization URL.
//! let (auth_url, csrf_token) = client.authorize_url(CsrfToken::new_random);
//!
//! // This is the URL you should redirect the user to, in order to trigger the authorization
//! // process.
//! println!("Browse to: {}", auth_url);
//!
//! // Once the user has been redirected to the redirect URL, you'll have access to the
//! // authorization code. For security reasons, your code should verify that the `state`
//! // parameter returned by the server matches `csrf_state`.
//!
//! // Now you can trade it for an access token.
//! let token_result =
//!     client.exchange_code(AuthorizationCode::new("some authorization code".to_string()));
//!
//! // Unwrapping token_result will either produce a Token or a RequestTokenError.
//! # Ok(())
//! # }
//! # fn main() {}
//! ```
//!
//! # Implicit Grant
//!
//! This flow fetches an access token directly from the authorization endpoint. Be sure to
//! understand the security implications of this flow before using it. In most cases, the
//! Authorization Code Grant flow is preferable to the Implicit Grant flow.
//!
//! ## Example:
//!
//! ```
//! extern crate base64;
//! extern crate oauth2;
//! extern crate rand;
//! extern crate url;
//!
//! use oauth2::prelude::*;
//! use oauth2::{
//!     AuthUrl,
//!     ClientId,
//!     ClientSecret,
//!     CsrfToken,
//!     RedirectUrl,
//!     Scope
//! };
//! use oauth2::basic::BasicClient;
//! use url::Url;
//!
//! # fn err_wrapper() -> Result<(), Box<std::error::Error>> {
//! let client =
//!     BasicClient::new(
//!         ClientId::new("client_id".to_string()),
//!         Some(ClientSecret::new("client_secret".to_string())),
//!         AuthUrl::new(Url::parse("http://authorize")?),
//!         None
//!     );
//!
//! // Generate the full authorization URL.
//! let (auth_url, csrf_token) = client.authorize_url_implicit(CsrfToken::new_random);
//!
//! // This is the URL you should redirect the user to, in order to trigger the authorization
//! // process.
//! println!("Browse to: {}", auth_url);
//!
//! // Once the user has been redirected to the redirect URL, you'll have the access code.
//! // For security reasons, your code should verify that the `state` parameter returned by the
//! // server matches `csrf_state`.
//!
//! # Ok(())
//! # }
//! # fn main() {}
//! ```
//!
//! # Resource Owner Password Credentials Grant
//!
//! You can ask for a *password* access token by calling the `Client::exchange_password` method,
//! while including the username and password.
//!
//! ## Example
//!
//! ```
//! extern crate base64;
//! extern crate oauth2;
//! extern crate rand;
//! extern crate url;
//!
//! use oauth2::prelude::*;
//! use oauth2::{
//!     AuthUrl,
//!     ClientId,
//!     ClientSecret,
//!     ResourceOwnerPassword,
//!     ResourceOwnerUsername,
//!     Scope,
//!     TokenResponse,
//!     TokenUrl
//! };
//! use oauth2::basic::BasicClient;
//! use url::Url;
//!
//! # fn err_wrapper() -> Result<(), Box<std::error::Error>> {
//! let client =
//!     BasicClient::new(
//!         ClientId::new("client_id".to_string()),
//!         Some(ClientSecret::new("client_secret".to_string())),
//!         AuthUrl::new(Url::parse("http://authorize")?),
//!         Some(TokenUrl::new(Url::parse("http://token")?))
//!     )
//!         .add_scope(Scope::new("read".to_string()));
//!
//! let token_result =
//!     client.exchange_password(
//!         &ResourceOwnerUsername::new("user".to_string()),
//!         &ResourceOwnerPassword::new("pass".to_string())
//!     );
//! # Ok(())
//! # }
//! # fn main() {}
//! ```
//!
//! # Client Credentials Grant
//!
//! You can ask for a *client credentials* access token by calling the
//! `Client::exchange_client_credentials` method.
//!
//! ## Example:
//!
//! ```
//! extern crate oauth2;
//! extern crate url;
//!
//! use oauth2::prelude::*;
//! use oauth2::{
//!     AuthUrl,
//!     ClientId,
//!     ClientSecret,
//!     Scope,
//!     TokenResponse,
//!     TokenUrl
//! };
//! use oauth2::basic::BasicClient;
//! use url::Url;
//!
//! # fn err_wrapper() -> Result<(), Box<std::error::Error>> {
//! let client =
//!     BasicClient::new(
//!         ClientId::new("client_id".to_string()),
//!         Some(ClientSecret::new("client_secret".to_string())),
//!         AuthUrl::new(Url::parse("http://authorize")?),
//!         Some(TokenUrl::new(Url::parse("http://token")?))
//!     )
//!         .add_scope(Scope::new("read".to_string()));
//!
//! let token_result = client.exchange_client_credentials();
//! # Ok(())
//! # }
//! # fn main() {}
//! ```
//!
//! # Other examples
//!
//! More specific implementations are available as part of the examples:
//!
//! - [Google](https://github.com/ramosbugs/oauth2-rs/blob/master/examples/google.rs)
//! - [Github](https://github.com/ramosbugs/oauth2-rs/blob/master/examples/github.rs)
//!

use std::{borrow::Cow, convert::Into, fmt, ops::Deref, time::Duration};

use failure::{Error, Fail};
use rand::{thread_rng, Rng};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

const CONTENT_TYPE_JSON: &str = "application/json";

///
/// Indicates whether requests to the authorization server should use basic authentication or
/// include the parameters in the request body for requests in which either is valid.
///
/// The default AuthType is *BasicAuth*, following the recommendation of
/// [Section 2.3.1 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-2.3.1).
///
#[derive(Clone, Copy, Debug)]
pub enum AuthType {
    /// The client_id and client_secret will be included as part of the request body.
    RequestBody,
    /// The client_id and client_secret will be included using the basic auth authentication scheme.
    BasicAuth,
}

macro_rules! new_type {
    // Convenience pattern without an impl.
    (
        $(#[$attr:meta])*
        pub struct $name:ident(
            $(#[$type_attr:meta])*
            $type:ty
        );
    ) => {
        new_type! {
            @new_type $(#[$attr])*,
            $name(
                $(#[$type_attr])*
                $type
            ),
            concat!("Create a new `", stringify!($name), "` to wrap the given `", stringify!($type), "`."),
        }
    };

    // Actual implementation, after stringifying the #[doc] attr.
    (
        @new_type $(#[$attr:meta])*,
        $name:ident(
            $(#[$type_attr:meta])*
            $type:ty
        ),
        $new_doc:expr,
    ) => {
        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name(
            $(#[$type_attr])*
            $type
        );
        impl $name {
            #[doc = $new_doc]
            pub fn new(s: $type) -> Self {
                $name(s)
            }
        }

        impl Deref for $name {
            type Target = $type;

            fn deref(&self) -> &$type {
                &self.0
            }
        }

        impl Into<$type> for $name {
            fn into(self) -> $type {
                self.0
            }
        }
    }
}

macro_rules! new_secret_type {
    (
        $(#[$attr:meta])*
        pub struct $name:ident($type:ty);
    ) => {
        new_secret_type! {
            $(#[$attr])*,
            $name($type),
            concat!("Create a new `", stringify!($name), "` to wrap the given `", stringify!($type), "`."),
            concat!("Get the secret contained within this `", stringify!($name), "`."),
        }
    };

    (
        $(#[$attr:meta])*,
        $name:ident($type:ty),
        $new_doc:expr,
        $secret_doc:expr,
    ) => {
        $(
            #[$attr]
        )*
        #[derive(Clone, PartialEq)]
        pub struct $name($type);
        impl $name {
            #[doc = $new_doc]
            pub fn new(s: $type) -> Self {
                $name(s)
            }

            #[doc = $secret_doc]
            ///
            /// # Security Warning
            ///
            /// Leaking this value may compromise the security of the OAuth2 flow.
            ///
            pub fn secret(&self) -> &$type { &self.0 }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "([redacted])"))
            }
        }
    };
}

new_type! {
    /// Client identifier issued to the client during the registration process described by
    /// [Section 2.2](https://tools.ietf.org/html/rfc6749#section-2.2).
    #[derive(Deserialize, Serialize)]
    pub struct ClientId(String);
}

new_type! {
    /// URL of the authorization server's authorization endpoint.
    #[derive(Deserialize, Serialize)]
    pub struct AuthUrl(
        #[serde(
            deserialize_with = "helpers::deserialize_url",
            serialize_with = "helpers::serialize_url"
        )]
        Url
    );
}

new_type! {
    /// URL of the authorization server's token endpoint.
    #[derive(Deserialize, Serialize)]
    pub struct TokenUrl(
        #[serde(
            deserialize_with = "helpers::deserialize_url",
            serialize_with = "helpers::serialize_url"
        )]
        Url
    );
}

new_type! {
    /// URL of the client's redirection endpoint.
    #[derive(Deserialize, Serialize)]
    pub struct RedirectUrl(
        #[serde(
            deserialize_with = "helpers::deserialize_url",
            serialize_with = "helpers::serialize_url"
        )]
        Url
    );
}

new_type! {
    /// Authorization endpoint response (grant) type defined in
    /// [Section 3.1.1](https://tools.ietf.org/html/rfc6749#section-3.1.1).
    #[derive(Deserialize, Serialize)]
    pub struct ResponseType(String);
}

new_type! {
    /// Resource owner's username used directly as an authorization grant to obtain an access
    /// token.
    pub struct ResourceOwnerUsername(String);
}

new_type! {
    /// Access token scope, as defined by the authorization server.
    #[derive(Deserialize, Serialize)]
    pub struct Scope(String);
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        self
    }
}

new_type! {
    /// Code Challenge used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection via the
    /// `code_challenge` parameter.
    #[derive(Deserialize, Serialize)]
    pub struct PkceCodeChallengeS256(String);
}

new_type! {
    /// Code Challenge Method used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection
    /// via the `code_challenge_method` parameter.
    #[derive(Deserialize, Serialize)]
    pub struct PkceCodeChallengeMethod(String);
}

new_secret_type! {
    /// Client password issued to the client during the registration process described by
    /// [Section 2.2](https://tools.ietf.org/html/rfc6749#section-2.2).
    #[derive(Deserialize, Serialize)]
    pub struct ClientSecret(String);
}

new_secret_type! {
    /// Value used for [CSRF]((https://tools.ietf.org/html/rfc6749#section-10.12)) protection
    /// via the `state` parameter.
    #[must_use]
    #[derive(Deserialize, Serialize)]
    pub struct CsrfToken(String);
}

impl CsrfToken {
    ///
    /// Generate a new random, base64-encoded 128-bit CSRF token.
    ///
    pub fn new_random() -> Self {
        CsrfToken::new_random_len(16)
    }
    ///
    /// Generate a new random, base64-encoded CSRF token of the specified length.
    ///
    /// # Arguments
    ///
    /// * `num_bytes` - Number of random bytes to generate, prior to base64-encoding.
    ///
    pub fn new_random_len(num_bytes: u32) -> Self {
        let random_bytes: Vec<u8> = (0..num_bytes).map(|_| thread_rng().gen::<u8>()).collect();
        CsrfToken::new(base64::encode_config(
            &random_bytes,
            base64::URL_SAFE_NO_PAD,
        ))
    }
}

new_secret_type! {
    /// Code Verifier used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection via the
    /// `code_verifier` parameter. The value must have a minimum length of 43 characters and a
    /// maximum length of 128 characters.  Each character must be ASCII alphanumeric or one of
    /// the characters "-" / "." / "_" / "~".
    #[derive(Deserialize, Serialize)]
    pub struct PkceCodeVerifierS256(String);
}

impl PkceCodeVerifierS256 {
    ///
    /// Generate a new random, base64-encoded code verifier.
    ///
    pub fn new_random() -> Self {
        PkceCodeVerifierS256::new_random_len(32)
    }
    ///
    /// Generate a new random, base64-encoded code verifier.
    ///
    /// # Arguments
    ///
    /// * `num_bytes` - Number of random bytes to generate, prior to base64-encoding.
    ///   The value must be in the range 32 to 96 inclusive in order to generate a verifier
    ///   with a suitable length.
    ///
    pub fn new_random_len(num_bytes: u32) -> Self {
        // The RFC specifies that the code verifier must have "a minimum length of 43
        // characters and a maximum length of 128 characters".
        // This implies 32-96 octets of random data to be base64 encoded.
        assert!(num_bytes >= 32 && num_bytes <= 96);
        let random_bytes: Vec<u8> = (0..num_bytes).map(|_| thread_rng().gen::<u8>()).collect();
        let code = base64::encode_config(&random_bytes, base64::URL_SAFE_NO_PAD);
        assert!(code.len() >= 43 && code.len() <= 128);
        PkceCodeVerifierS256::new(code)
    }
    ///
    /// Return the code challenge for the code verifier.
    ///
    pub fn code_challenge(&self) -> PkceCodeChallengeS256 {
        let digest = Sha256::digest(self.secret().as_bytes());
        PkceCodeChallengeS256::new(base64::encode_config(&digest, base64::URL_SAFE_NO_PAD))
    }

    ///
    /// Return the code challenge method for this code verifier.
    ///
    pub fn code_challenge_method() -> PkceCodeChallengeMethod {
        PkceCodeChallengeMethod::new("S256".to_string())
    }

    ///
    /// Return the extension params used for authorize_url.
    ///
    pub fn authorize_url_params(&self) -> Vec<(&'static str, String)> {
        vec![
            (
                "code_challenge_method",
                PkceCodeVerifierS256::code_challenge_method().into(),
            ),
            ("code_challenge", self.code_challenge().into()),
        ]
    }
}

new_secret_type! {
    /// Authorization code returned from the authorization endpoint.
    #[derive(Deserialize, Serialize)]
    pub struct AuthorizationCode(String);
}

new_secret_type! {
    /// Refresh token used to obtain a new access token (if supported by the authorization server).
    #[derive(Deserialize, Serialize)]
    pub struct RefreshToken(String);
}

new_secret_type! {
    /// Access token returned by the token endpoint and used to access protected resources.
    #[derive(Deserialize, Serialize)]
    pub struct AccessToken(String);
}

new_secret_type! {
    /// Resource owner's password used directly as an authorization grant to obtain an access
    /// token.
    pub struct ResourceOwnerPassword(String);
}

///
/// Stores the configuration for an OAuth2 client.
///
#[derive(Clone, Debug)]
pub struct Client {
    client: reqwest::r#async::Client,
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    auth_url: AuthUrl,
    auth_type: AuthType,
    token_url: Option<TokenUrl>,
    scopes: Vec<Scope>,
    redirect_url: Option<RedirectUrl>,
}

impl Client {
    ///
    /// Initializes an OAuth2 client with the fields common to most OAuth2 flows.
    ///
    /// # Arguments
    ///
    /// * `client_id` -  Client ID
    /// * `client_secret` -  Optional client secret. A client secret is generally used for private
    ///   (server-side) OAuth2 clients and omitted from public (client-side or native app) OAuth2
    ///   clients (see [RFC 8252](https://tools.ietf.org/html/rfc8252)).
    /// * `auth_url` -  Authorization endpoint: used by the client to obtain authorization from
    ///   the resource owner via user-agent redirection. This URL is used in all standard OAuth2
    ///   flows except the [Resource Owner Password Credentials
    ///   Grant](https://tools.ietf.org/html/rfc6749#section-4.3) and the
    ///   [Client Credentials Grant](https://tools.ietf.org/html/rfc6749#section-4.4).
    /// * `token_url` - Token endpoint: used by the client to exchange an authorization grant
    ///   (code) for an access token, typically with client authentication. This URL is used in
    ///   all standard OAuth2 flows except the
    ///   [Implicit Grant](https://tools.ietf.org/html/rfc6749#section-4.2). If this value is set
    ///   to `None`, the `exchange_*` methods will return `Err(RequestTokenError::Other(_))`.
    ///
    pub fn new(
        client_id: ClientId,
        client_secret: Option<ClientSecret>,
        auth_url: AuthUrl,
        token_url: Option<TokenUrl>,
    ) -> Result<Self, Error> {
        let client = reqwest::r#async::Client::builder()
            .redirect(reqwest::RedirectPolicy::none())
            .build()?;

        Ok(Client {
            client,
            client_id,
            client_secret,
            auth_url,
            auth_type: AuthType::BasicAuth,
            token_url,
            scopes: Vec::new(),
            redirect_url: None,
        })
    }

    ///
    /// Appends a new scope to the authorization URL.
    ///
    pub fn add_scope(mut self, scope: Scope) -> Self {
        self.scopes.push(scope);

        self
    }

    ///
    /// Configures the type of client authentication used for communicating with the authorization
    /// server.
    ///
    /// The default is to use HTTP Basic authentication, as recommended in
    /// [Section 2.3.1 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-2.3.1).
    ///
    pub fn set_auth_type(mut self, auth_type: AuthType) -> Self {
        self.auth_type = auth_type;

        self
    }

    ///
    /// Sets the the redirect URL used by the authorization endpoint.
    ///
    pub fn set_redirect_url(mut self, redirect_url: RedirectUrl) -> Self {
        self.redirect_url = Some(redirect_url);

        self
    }

    ///
    /// Produces the full authorization URL used by the
    /// [Authorization Code Grant](https://tools.ietf.org/html/rfc6749#section-4.1) flow, which
    /// is the most common OAuth2 flow.
    ///
    /// # Arguments
    ///
    /// * `state_fn` - A function that returns an opaque value used by the client to maintain state
    ///   between the request and callback. The authorization server includes this value when
    ///   redirecting the user-agent back to the client.
    ///
    /// # Security Warning
    ///
    /// Callers should use a fresh, unpredictable `state` for each authorization request and verify
    /// that this value matches the `state` parameter passed by the authorization server to the
    /// redirect URI. Doing so mitigates
    /// [Cross-Site Request Forgery](https://tools.ietf.org/html/rfc6749#section-10.12)
    ///  attacks. To disable CSRF protections (NOT recommended), use `insecure::authorize_url`
    ///  instead.
    ///
    pub fn authorize_url<F>(&self, state_fn: F) -> (Url, CsrfToken)
    where
        F: FnOnce() -> CsrfToken,
    {
        let state = state_fn();
        (self.authorize_url_impl("code", Some(&state)), state)
    }

    ///
    /// Produces the full authorization URL used by the
    /// [Implicit Grant](https://tools.ietf.org/html/rfc6749#section-4.2) flow.
    ///
    /// # Arguments
    ///
    /// * `state_fn` - A function that returns an opaque value used by the client to maintain state
    ///   between the request and callback. The authorization server includes this value when
    ///   redirecting the user-agent back to the client.
    ///
    /// # Security Warning
    ///
    /// Callers should use a fresh, unpredictable `state` for each authorization request and verify
    /// that this value matches the `state` parameter passed by the authorization server to the
    /// redirect URI. Doing so mitigates
    /// [Cross-Site Request Forgery](https://tools.ietf.org/html/rfc6749#section-10.12)
    ///  attacks. To disable CSRF protections (NOT recommended), use
    /// `insecure::authorize_url_implicit` instead.
    ///
    pub fn authorize_url_implicit<F>(&self, state_fn: F) -> (Url, CsrfToken)
    where
        F: FnOnce() -> CsrfToken,
    {
        let state = state_fn();
        (self.authorize_url_impl("token", Some(&state)), state)
    }

    fn authorize_url_impl(&self, response_type: &str, state_opt: Option<&CsrfToken>) -> Url {
        let scopes = self
            .scopes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let mut url: Url = (*self.auth_url).clone();

        {
            let mut query = url.query_pairs_mut();

            query.append_pair("response_type", response_type);
            query.append_pair("client_id", &self.client_id);

            if let Some(ref redirect_url) = self.redirect_url {
                query.append_pair("redirect_uri", redirect_url.as_str());
            }

            if !scopes.is_empty() {
                query.append_pair("scope", &scopes);
            }

            if let Some(state) = state_opt {
                query.append_pair("state", state.secret());
            }
        }

        url
    }

    ///
    /// Exchanges a code produced by a successful authorization process with an access token.
    ///
    /// Acquires ownership of the `code` because authorization codes may only be used to retrieve
    /// an access token from the authorization server.
    ///
    /// See https://tools.ietf.org/html/rfc6749#section-4.1.3
    ///
    pub fn exchange_code<'a>(&'a self, code: AuthorizationCode) -> RequestBuilder<'a> {
        self.request_token()
            .param("grant_type", "authorization_code")
            .param("code", code.secret().to_string())
    }

    ///
    /// Requests an access token for the *password* grant type.
    ///
    /// See https://tools.ietf.org/html/rfc6749#section-4.3.2
    ///
    pub fn exchange_password<'a>(
        &'a self,
        username: &ResourceOwnerUsername,
        password: &ResourceOwnerPassword,
    ) -> RequestBuilder<'a> {
        let mut builder = self
            .request_token()
            .param("grant_type", "password")
            .param("username", username.to_string())
            .param("password", password.secret().to_string());

        // Generate the space-delimited scopes String before initializing params so that it has
        // a long enough lifetime.
        if !self.scopes.is_empty() {
            let scopes = self
                .scopes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            builder = builder.param("scope", scopes);
        }

        builder
    }

    ///
    /// Requests an access token for the *client credentials* grant type.
    ///
    /// See https://tools.ietf.org/html/rfc6749#section-4.4.2
    ///
    pub fn exchange_client_credentials<'a>(&'a self) -> RequestBuilder<'a> {
        let mut builder = self
            .request_token()
            .param("grant_type", "client_credentials");

        // Generate the space-delimited scopes String before initializing params so that it has
        // a long enough lifetime.
        if !self.scopes.is_empty() {
            let scopes = self
                .scopes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            builder = builder.param("scopes", scopes);
        }

        builder
    }

    ///
    /// Exchanges a refresh token for an access token
    ///
    /// See https://tools.ietf.org/html/rfc6749#section-6
    ///
    pub fn exchange_refresh_token(&self, refresh_token: &RefreshToken) -> RequestBuilder {
        self.request_token()
            .param("grant_type", "refresh_token")
            .param("refresh_token", refresh_token.secret().to_string())
    }

    /// Construct a request builder for the token URL.
    fn request_token(&self) -> RequestBuilder<'_> {
        RequestBuilder {
            client: &self.client,
            token_url: self.token_url.as_ref(),
            auth_type: self.auth_type,
            client_id: &self.client_id,
            client_secret: self.client_secret.as_ref(),
            redirect_url: self.redirect_url.as_ref(),
            params: vec![],
        }
    }
}

/// A token request that is in progress.
pub struct RequestBuilder<'a> {
    client: &'a reqwest::r#async::Client,
    token_url: Option<&'a TokenUrl>,
    auth_type: AuthType,
    client_id: &'a ClientId,
    client_secret: Option<&'a ClientSecret>,
    /// Configured redirect URL.
    redirect_url: Option<&'a RedirectUrl>,
    /// Extra parameters.
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> RequestBuilder<'a> {
    /// Set an additional request param.
    pub fn param(mut self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        self.params.push((key.into(), value.into()));
        self
    }

    /// Execute the token request.
    pub async fn execute<T>(self) -> Result<T, RequestTokenError>
    where
        T: TokenResponse,
    {
        use reqwest::{header, Method};

        let token_url = self
            .token_url
            .ok_or_else(||
                // Arguably, it could be better to panic in this case. However, there may be
                // situations where the library user gets the authorization server's configuration
                // dynamically. In those cases, it would be preferable to return an `Err` rather
                // than panic. An example situation where this might arise is OpenID Connect
                // discovery.
                RequestTokenError::Other("token_url must not be `None`".into()))
            .unwrap();

        let mut request = self
            .client
            .request(Method::POST, &token_url.to_string()[..]);

        // Section 5.1 of RFC 6749 (https://tools.ietf.org/html/rfc6749#section-5.1) only permits
        // JSON responses for this request. Some providers such as GitHub have off-spec behavior
        // and not only support different response formats, but have non-JSON defaults. Explicitly
        // request JSON here.
        request = request.header(
            header::ACCEPT,
            header::HeaderValue::from_static(CONTENT_TYPE_JSON),
        );

        let request = {
            let mut form = url::form_urlencoded::Serializer::new(String::new());

            // FIXME: add support for auth extensions? e.g., client_secret_jwt and private_key_jwt
            match self.auth_type {
                AuthType::RequestBody => {
                    form.append_pair("client_id", self.client_id.as_str());

                    if let Some(client_secret) = self.client_secret {
                        form.append_pair("client_secret", client_secret.secret().as_str());
                    }
                }
                AuthType::BasicAuth => {
                    // Section 2.3.1 of RFC 6749 requires separately url-encoding the id and secret
                    // before using them as HTTP Basic auth username and password. Note that this is
                    // not standard for ordinary Basic auth, so curl won't do it for us.
                    let username = url_encode(self.client_id.as_str());

                    let password = match self.client_secret {
                        Some(client_secret) => Some(url_encode(client_secret.secret().as_str())),
                        None => None,
                    };

                    request = request.basic_auth(&username, password.as_ref());
                }
            }

            for (key, value) in self.params {
                form.append_pair(key.as_ref(), value.as_ref());
            }

            if let Some(ref redirect_url) = self.redirect_url {
                form.append_pair("redirect_uri", redirect_url.as_str());
            }

            request = request.header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            );

            request.body(form.finish().into_bytes())
        };

        let res = request.send().await.map_err(RequestTokenError::Client)?;
        let status = res.status();
        let body = res.bytes().await.map_err(RequestTokenError::Client)?;

        if !status.is_success() {
            if body.is_empty() {
                return Err(RequestTokenError::Other(
                    "Server returned empty error response".into(),
                ));
            } else {
                let error = match serde_json::from_slice::<ErrorResponse>(body.as_ref()) {
                    Ok(error) => RequestTokenError::ServerResponse(error),
                    Err(error) => RequestTokenError::Parse(error, body.as_ref().to_vec()),
                };
                return Err(error);
            }
        }

        if body.is_empty() {
            Err(RequestTokenError::Other(
                "Server returned empty response body".into(),
            ))
        } else {
            serde_json::from_slice(body.as_ref())
                .map_err(|e| RequestTokenError::Parse(e, body.as_ref().to_vec()))
        }
    }
}

fn url_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect::<String>()
}

/// Basic OAuth2 authorization token types.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    /// Bearer token
    /// ([OAuth 2.0 Bearer Tokens - RFC 6750](https://tools.ietf.org/html/rfc6750)).
    Bearer,
    /// MAC ([OAuth 2.0 Message Authentication Code (MAC)
    /// Tokens](https://tools.ietf.org/html/draft-ietf-oauth-v2-http-mac-05)).
    Mac,
}

/// Common methods shared by all OAuth2 token implementations.
///
/// The methods in this trait are defined in
/// [Section 5.1 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-5.1). This trait exists
/// separately from the `StandardTokenResponse` struct to support customization by clients,
/// such as supporting interoperability with non-standards-complaint OAuth2 providers.
pub trait TokenResponse: Clone + fmt::Debug + DeserializeOwned + PartialEq + Serialize {
    /// REQUIRED. The access token issued by the authorization server.
    fn access_token(&self) -> &AccessToken;

    /// REQUIRED. The type of the token issued as described in
    /// [Section 7.1](https://tools.ietf.org/html/rfc6749#section-7.1).
    /// Value is case insensitive and deserialized to the generic `TokenType` parameter.
    fn token_type(&self) -> &TokenType;

    /// RECOMMENDED. The lifetime in seconds of the access token. For example, the value 3600
    /// denotes that the access token will expire in one hour from the time the response was
    /// generated. If omitted, the authorization server SHOULD provide the expiration time via
    /// other means or document the default value.
    fn expires_in(&self) -> Option<Duration>;

    /// OPTIONAL. The refresh token, which can be used to obtain new access tokens using the same
    /// authorization grant as described in
    /// [Section 6](https://tools.ietf.org/html/rfc6749#section-6).
    fn refresh_token(&self) -> Option<&RefreshToken>;

    /// OPTIONAL, if identical to the scope requested by the client; otherwise, REQUIRED. The
    /// scipe of the access token as described by
    /// [Section 3.3](https://tools.ietf.org/html/rfc6749#section-3.3). If included in the response,
    /// this space-delimited field is parsed into a `Vec` of individual scopes. If omitted from
    /// the response, this field is `None`.
    fn scopes(&self) -> Option<&Vec<Scope>>;
}

/// Standard OAuth2 token response.
///
/// This struct includes the fields defined in
/// [Section 5.1 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-5.1), as well as
/// extensions defined by the `EF` type parameter.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StandardTokenResponse {
    access_token: AccessToken,
    #[serde(deserialize_with = "helpers::deserialize_untagged_enum_case_insensitive")]
    token_type: TokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<RefreshToken>,
    #[serde(rename = "scope")]
    #[serde(deserialize_with = "helpers::deserialize_space_delimited_vec")]
    #[serde(serialize_with = "helpers::serialize_space_delimited_vec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    scopes: Option<Vec<Scope>>,
}

impl TokenResponse for StandardTokenResponse {
    /// REQUIRED. The access token issued by the authorization server.
    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    /// REQUIRED. The type of the token issued as described in
    /// [Section 7.1](https://tools.ietf.org/html/rfc6749#section-7.1).
    /// Value is case insensitive and deserialized to the generic `TokenType` parameter.
    fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    /// RECOMMENDED. The lifetime in seconds of the access token. For example, the value 3600
    /// denotes that the access token will expire in one hour from the time the response was
    /// generated. If omitted, the authorization server SHOULD provide the expiration time via
    /// other means or document the default value.
    fn expires_in(&self) -> Option<Duration> {
        self.expires_in.map(Duration::from_secs)
    }

    /// OPTIONAL. The refresh token, which can be used to obtain new access tokens using the same
    /// authorization grant as described in
    /// [Section 6](https://tools.ietf.org/html/rfc6749#section-6).
    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }

    /// OPTIONAL, if identical to the scope requested by the client; otherwise, REQUIRED. The
    /// scipe of the access token as described by
    /// [Section 3.3](https://tools.ietf.org/html/rfc6749#section-3.3). If included in the response,
    /// this space-delimited field is parsed into a `Vec` of individual scopes. If omitted from
    /// the response, this field is `None`.
    fn scopes(&self) -> Option<&Vec<Scope>> {
        self.scopes.as_ref()
    }
}

/// These error types are defined in
/// [Section 5.2 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-5.2).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorField {
    /// The request is missing a required parameter, includes an unsupported parameter value
    /// (other than grant type), repeats a parameter, includes multiple credentials, utilizes
    /// more than one mechanism for authenticating the client, or is otherwise malformed.
    InvalidRequest,
    /// Client authentication failed (e.g., unknown client, no client authentication included,
    /// or unsupported authentication method).
    InvalidClient,
    /// The provided authorization grant (e.g., authorization code, resource owner credentials)
    /// or refresh token is invalid, expired, revoked, does not match the redirection URI used
    /// in the authorization request, or was issued to another client.
    InvalidGrant,
    /// The authenticated client is not authorized to use this authorization grant type.
    UnauthorizedClient,
    /// The authorization grant type is not supported by the authorization server.
    UnsupportedGrantType,
    /// The requested scope is invalid, unknown, malformed, or exceeds the scope granted by the
    /// resource owner.
    InvalidScope,
    /// Other error type.
    Other(String),
}

impl fmt::Display for ErrorField {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ErrorField::*;

        match *self {
            InvalidRequest => "invalid_request".fmt(fmt),
            InvalidClient => "invalid_client".fmt(fmt),
            InvalidGrant => "invalid_grant".fmt(fmt),
            UnauthorizedClient => "unauthorized_client".fmt(fmt),
            UnsupportedGrantType => "unsupported_grant_type".fmt(fmt),
            InvalidScope => "invalid_scope".fmt(fmt),
            Other(ref value) => value.fmt(fmt),
        }
    }
}

/// Error response returned by server after requesting an access token.
///
/// The fields in this structure are defined in
/// [Section 5.2 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-5.2).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ErrorResponse {
    /// A single ASCII error code.
    pub error: ErrorField,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Human-readable ASCII text providing additional information, used to assist
    /// the client developer in understanding the error that occurred.
    pub error_description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A URI identifying a human-readable web page with information about the error,
    /// used to provide the client developer with additional information about the error.
    pub error_uri: Option<String>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = self.error.to_string();

        if let Some(error_description) = self.error_description.as_ref() {
            formatted.push_str(": ");
            formatted.push_str(error_description);
        }

        if let Some(error_uri) = self.error_uri.as_ref() {
            formatted.push_str(" / See ");
            formatted.push_str(error_uri);
        }

        write!(f, "{}", formatted)
    }
}

///
/// Error encountered while requesting access token.
///
#[derive(Debug, Fail)]
pub enum RequestTokenError {
    ///
    /// Error response returned by authorization server. Contains the parsed `ErrorResponse`
    /// returned by the server.
    ///
    #[fail(display = "Server returned error response `{}`", _0)]
    ServerResponse(ErrorResponse),
    /// A client error that occured.
    #[fail(display = "Client error: {}", _0)]
    Client(reqwest::Error),
    ///
    /// Failed to parse server response. Parse errors may occur while parsing either successful
    /// or error responses.
    ///
    #[fail(display = "Failed to parse server response")]
    Parse(#[cause] serde_json::error::Error, Vec<u8>),
    ///
    /// Some other type of error occurred (e.g., an unexpected server response).
    ///
    #[fail(display = "Other error: {}", _0)]
    Other(Cow<'static, str>),
}

/// Insecure methods -- not recommended for most applications.
pub mod insecure {
    use url::Url;

    use super::Client;

    ///
    /// Produces the full authorization URL used by the
    /// [Authorization Code Grant](https://tools.ietf.org/html/rfc6749#section-4.1) flow, which
    /// is the most common OAuth2 flow.
    ///
    /// # Security Warning
    ///
    /// The URL produced by this function is vulnerable to
    /// [Cross-Site Request Forgery](https://tools.ietf.org/html/rfc6749#section-10.12) attacks.
    /// It is highly recommended to use the `Client::authorize_url` function instead.
    ///
    pub fn authorize_url(client: &Client) -> Url {
        client.authorize_url_impl("code", None)
    }

    ///
    /// Produces the full authorization URL used by the
    /// [Implicit Grant](https://tools.ietf.org/html/rfc6749#section-4.2) flow.
    ///
    /// # Security Warning
    ///
    /// The URL produced by this function is vulnerable to
    /// [Cross-Site Request Forgery](https://tools.ietf.org/html/rfc6749#section-10.12) attacks.
    /// It is highly recommended to use the `Client::authorize_url_implicit` function instead.
    ///
    pub fn authorize_url_implicit(client: &Client) -> Url {
        client.authorize_url_impl("token", None)
    }
}

///
/// Helper methods used by OAuth2 implementations/extensions.
///
pub mod helpers {
    use serde::{Deserialize, Deserializer, Serializer};
    use url::Url;

    ///
    /// Serde case-insensitive deserializer for an untagged `enum`.
    ///
    /// This function converts values to lowercase before deserializing as the `enum`. Requires the
    /// `#[serde(rename_all = "lowercase")]` attribute to be set on the `enum`.
    ///
    /// # Example
    ///
    /// In example below, the following JSON values all deserialize to
    /// `GroceryBasket { fruit_item: Fruit::Banana }`:
    ///
    ///  * `{"fruit_item": "banana"}`
    ///  * `{"fruit_item": "BANANA"}`
    ///  * `{"fruit_item": "Banana"}`
    ///
    /// Note: this example does not compile automatically due to
    /// [Rust issue #29286](https://github.com/rust-lang/rust/issues/29286).
    ///
    /// ```
    /// # /*
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// #[serde(rename_all = "lowercase")]
    /// enum Fruit {
    ///     Apple,
    ///     Banana,
    ///     Orange,
    /// }
    ///
    /// #[derive(Deserialize)]
    /// struct GroceryBasket {
    ///     #[serde(deserialize_with = "helpers::deserialize_untagged_enum_case_insensitive")]
    ///     fruit_item: Fruit,
    /// }
    /// # */
    /// ```
    ///
    pub fn deserialize_untagged_enum_case_insensitive<'de, T, D>(
        deserializer: D,
    ) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;
        T::deserialize(Value::String(
            String::deserialize(deserializer)?.to_lowercase(),
        ))
        .map_err(Error::custom)
    }

    ///
    /// Serde space-delimited string deserializer for a `Vec<String>`.
    ///
    /// This function splits a JSON string at each space character into a `Vec<String>` .
    ///
    /// # Example
    ///
    /// In example below, the JSON value `{"items": "foo bar baz"}` would deserialize to:
    ///
    /// ```
    /// # struct GroceryBasket {
    /// #     items: Vec<String>,
    /// # }
    /// # fn main() {
    /// GroceryBasket {
    ///     items: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
    /// };
    /// # }
    /// ```
    ///
    /// Note: this example does not compile automatically due to
    /// [Rust issue #29286](https://github.com/rust-lang/rust/issues/29286).
    ///
    /// ```
    /// # /*
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct GroceryBasket {
    ///     #[serde(deserialize_with = "helpers::deserialize_space_delimited_vec")]
    ///     items: Vec<String>,
    /// }
    /// # */
    /// ```
    ///
    pub fn deserialize_space_delimited_vec<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Default + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;
        if let Some(space_delimited) = Option::<String>::deserialize(deserializer)? {
            let entries = space_delimited
                .split(' ')
                .map(|s| Value::String(s.to_string()))
                .collect();
            T::deserialize(Value::Array(entries)).map_err(Error::custom)
        } else {
            // If the JSON value is null, use the default value.
            Ok(T::default())
        }
    }

    ///
    /// Serde space-delimited string serializer for an `Option<Vec<String>>`.
    ///
    /// This function serializes a string vector into a single space-delimited string.
    /// If `string_vec_opt` is `None`, the function serializes it as `None` (e.g., `null`
    /// in the case of JSON serialization).
    ///
    pub fn serialize_space_delimited_vec<T, S>(
        vec_opt: &Option<Vec<T>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        T: AsRef<str>,
        S: Serializer,
    {
        if let Some(ref vec) = *vec_opt {
            let space_delimited = vec.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(" ");

            serializer.serialize_str(&space_delimited)
        } else {
            serializer.serialize_none()
        }
    }

    ///
    /// Serde string deserializer for a `Url`.
    ///
    pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let url_str = String::deserialize(deserializer)?;
        Url::parse(url_str.as_ref()).map_err(Error::custom)
    }

    ///
    /// Serde string serializer for a `Url`.
    ///
    pub fn serialize_url<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(url.as_str())
    }
}
