use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Deserialize)]
pub struct Stat {
    pub user_agent: String,
    pub route: String,
    pub api: String,
    pub token: String,
}
