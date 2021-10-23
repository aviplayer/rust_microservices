use crate::models::errors::ProducerError;
use crate::config::env::EnvConfig;

pub async fn get_users(since: u8, per_page: u8) -> Result<reqwest::Response, ProducerError> {
    reqwest::Client::new()
        .get("https://api.github.com/users")
        .header("user-agent", "producer")
        .query(
            &[
                ("since", since),
                ("per_page", per_page)
            ]
        )
        .send()
        .await
        .map_err(|e| ProducerError::BackendError(format!("Server unavailable: {}", e)))
}
