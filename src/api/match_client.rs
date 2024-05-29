use std::env;
use dotenv::dotenv;
use reqwest::header::{AUTHORIZATION, ORIGIN};
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
    pub homeTeam: Team,
    pub awayTeam: Team,
    pub score: Score,
    pub status: String,
    pub homeScore: Option<isize>,
    pub awayScore: Option<isize>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Team {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub shortName: Option<String>,
    pub tla: Option<String>,
    #[serde(rename = "crest")]
    pub flag: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Score {
    pub winner: Option<String>,
    pub duration: String,
    pub fullTime: ScoreDetail,
    pub halfTime: ScoreDetail,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScoreDetail {
    pub home: Option<isize>,
    pub away: Option<isize>,
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

    pub async fn save_matches_to_sqlite(matches: &mut Vec<Match>) {
        for single_match in matches {
            let resp = reqwest::Client::new()
                .post("http://localhost:4321/api/match/import")
                .header(ORIGIN, "RUST_APPLICATION")
                // .header(AUTHORIZATION, ApiData::get_api_token())
                .json(single_match)
                .send()
                .await
                .unwrap();

            let response_text = resp.text().await.unwrap();

            if response_text.contains("Error") {
                // Err(response_text)
            } else {
                // Ok(response_text)
            }
        }
    }
}