use std::collections::HashMap;

use crate::{config::oauth, models::identity::Provider};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use secrecy::ExposeSecret;

pub fn oauth_client(settings: &oauth::ProviderSettings) -> BasicClient {
    BasicClient::new(
        ClientId::new(settings.client_id.to_owned()),
        Some(ClientSecret::new(
            settings.client_secret.expose_secret().to_string(),
        )),
        AuthUrl::new(settings.auth_url.to_string()).unwrap(),
        Some(TokenUrl::new(settings.token_url.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::from_url(settings.redirect_url.to_owned()))
}

//TODO: change this ugly code
pub fn get_oauth_clients(
    settings: &crate::config::oauth::Settings,
) -> HashMap<Provider, BasicClient> {
    let mut clients: HashMap<Provider, BasicClient> = HashMap::new();

    if let Some(discord) = &settings.discord {
        clients.insert(Provider::Discord, oauth_client(&discord));
    };
    if let Some(github) = &settings.github {
        clients.insert(Provider::Github, oauth_client(&github));
    };
    if let Some(apple) = &settings.apple {
        clients.insert(Provider::Apple, oauth_client(&apple));
    };
    if let Some(google) = &settings.google {
        clients.insert(Provider::Google, oauth_client(&google));
    };

    clients
}
