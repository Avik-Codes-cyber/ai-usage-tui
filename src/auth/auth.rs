use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn load_credentials() -> Result<CredentialsFile, String> {
    let path = credentials_path();
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let credentials: CredentialsFile = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    Ok(credentials)
}
fn credentials_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir()
        .ok_or("Could not find home directory")?;

    Ok(home.join(".claude").join(".credentials.json"))
}

pub fn is_token_expired(oauth: &OAuthData) -> bool {
    match oauth.expires_at {
        None => false,
        Some(expires_at_ms) => {
            let now_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let buffer_ms = 5 * 60 * 1000;
            now_ms + buffer_ms >= expires_at_ms
        }
    }
}
