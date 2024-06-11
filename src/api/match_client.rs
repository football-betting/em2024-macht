use std::env;
use dotenv::dotenv;
use rusqlite::Connection;
use serde_derive::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiResult {
    pub matches: Option<Vec<Match>>,
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
pub struct FoundMatch {
    pub id: isize,
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
    pub async fn get_matches(date: Option<String>) -> ApiResult {
        dotenv().ok();

        let mut uri = match env::var("API_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable API_URI".to_string(),
        };

        uri = match date {
            Some(d) => uri + "?dateFrom=" + d.as_str() + "&dateTo=" + d.as_str(),
            None => uri,
        };

        let token = match env::var("X_AUTH_TOKEN") {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable X_AUTH_TOKEN".to_string(),
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
        let db = Self::get_connection().await;

        for single_match in matches {
            let mut stmt = db.prepare("SELECT * FROM match WHERE id = ?1").unwrap();
            let match_already_exists = stmt.exists(rusqlite::params![single_match.id]).unwrap();

            if match_already_exists {
                db.execute(
                    "UPDATE match set homeTeam = ?1, awayTeam = ?2, status = ?3, utcDate = ?4, score = ?5, \
                    homeScore = ?6, awayScore = ?7 WHERE id = ?8",
                    (
                        to_string(&single_match.homeTeam).unwrap(),
                        to_string(&single_match.awayTeam).unwrap(),
                        &single_match.status,
                        &single_match.utcDate,
                        to_string(&single_match.score).unwrap(),
                        &single_match.homeScore,
                        &single_match.awayScore,
                        &single_match.id,
                    ),
                ).unwrap();
            } else {
                db.execute(
                    "INSERT INTO match (id, homeTeam, awayTeam, status, utcDate, score, homeScore, awayScore) \
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    (
                        &single_match.id,
                        to_string(&single_match.homeTeam).unwrap(),
                        to_string(&single_match.awayTeam).unwrap(),
                        &single_match.status,
                        &single_match.utcDate,
                        to_string(&single_match.score).unwrap(),
                        &single_match.homeScore,
                        &single_match.awayScore,
                    ),
                ).unwrap();
            }
        }
    }

    async fn get_connection() -> Connection {
        let db_path = match env::var("DB_PATH") {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable DB_PATH".to_string(),
        };

        Connection::open(db_path).unwrap()
    }
}