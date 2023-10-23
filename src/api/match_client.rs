use std::env;
use dotenv::dotenv;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiResult {
    pub matches: Vec<Match>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Match {
    pub id: isize,
    pub utcDate: String,
    #[serde(default = "default")]
    pub cetDate: String,
    pub homeTeam: Team,
    pub awayTeam: Team,
    pub score: Score,
    pub status: String,
}

fn default() -> String {
    "".to_string()
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Team {
    pub id: isize,
    pub name: String,
    pub shortName: String,
    pub tla: String,
    #[serde(rename = "crest")]
    pub flag: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Score {
    pub winner: String,
    pub duration: String,
    pub fullTime: ScoreDetail,
    pub halfTime: ScoreDetail,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScoreDetail {
    pub home: isize,
    pub away: isize,
}

pub struct MatchClient {}

impl MatchClient {
    pub async fn get_matches() -> ApiResult {
        dotenv().ok();

        let uri = match env::var("API_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable API_URI"),
        };

        let token = match env::var("X_AUTH_TOKEN") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable X_AUTH_TOKEN"),
        };

        let client = reqwest::Client::new();

        client
            .get(uri)
            .header("X-Auth-Token", token)
            .send()
            .await
            .unwrap()
            .json::<ApiResult>()
            .await
            .unwrap()
    }
}