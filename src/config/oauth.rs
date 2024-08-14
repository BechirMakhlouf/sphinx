#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProviderSettings {
    pub client_id: String,
    pub client_secret: secrecy::Secret<String>,
    pub auth_url: url::Url,
    pub token_url: url::Url,
    pub redirect_url: url::Url,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    pub discord: Option<ProviderSettings>,
    pub google: Option<ProviderSettings>,
    pub apple: Option<ProviderSettings>,
    pub github: Option<ProviderSettings>,
}
