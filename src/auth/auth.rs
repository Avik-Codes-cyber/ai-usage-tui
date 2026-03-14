use serde::Deserialize;

// Top Level JSON Object
#[derive(Deserialize)]
pub struct CredentialsFile {
    #[serde(rename = "claudeAIOauth")]
    pub claude_ai_oauth: OAuthData,
}

#[derive(Deserialize)]
pub struct OAuthData {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,

    #[serde(rename = "expiresAt")]
    pub expires_at: Option<u64>,

    #[serde(rename = "subscriptionType")]
    pub subscription_type: Option<String>,
}
