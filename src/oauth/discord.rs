use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use secrecy::ExposeSecret;

use crate::config::oauth;

pub fn oauth_client(settings: oauth::ProviderSettings) -> BasicClient {
    BasicClient::new(
        ClientId::new(settings.client_id),
        Some(ClientSecret::new(
            settings.client_secret.expose_secret().to_string(),
        )),
        AuthUrl::new(settings.auth_url.to_string()).unwrap(),
        Some(TokenUrl::new(settings.token_url.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::from_url(settings.redirect_url))
}
