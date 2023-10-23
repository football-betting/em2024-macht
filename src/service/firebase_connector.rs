use firestore::{FirestoreDb, FirestoreDbOptions};
use serde_derive::{Deserialize, Serialize};
use crate::api::match_client::ApiResult;
use crate::service::daily_winner::DailyWinners;
use crate::service::ranking::UserRanking;

#[derive(Deserialize, Serialize, Debug)]
pub struct Tip {
    pub id: isize,
    pub score1: isize,
    pub score2: isize,
    pub user: String,
}

pub struct FirebaseConnector {
    db: FirestoreDb,
}

impl FirebaseConnector {
    pub async fn init() -> Self {
        let db = FirestoreDb::with_options_service_account_key_file(
            FirestoreDbOptions::new("football-betting-407bb".to_string()),
            "./tmp/key.json".into()
        ).await.unwrap();

        FirebaseConnector { db }
    }

    pub async fn write_matches(&self, api_result: ApiResult) -> ApiResult {
        self.db.fluent()
            .update()
            .in_col("matches")
            .document_id(&"11223344")
            .object(&api_result)
            .execute()
            .await
            .unwrap()
    }

    pub async fn get_tips(&self) -> Vec<Tip> {
        self.db.fluent()
            .select()
            .from("tip")
            .obj()
            .query()
            .await
            .unwrap()
    }

    pub async fn write_ranking(&self, user_ranking: UserRanking) -> UserRanking {
        self.db.fluent()
            .update()
            .in_col("ranking")
            .document_id(&"11223344")
            .object(&user_ranking)
            .execute()
            .await
            .unwrap()
    }

    pub async fn write_daily_winner(&self, daily_winners: DailyWinners) -> DailyWinners {
        self.db.fluent()
            .update()
            .in_col("daily_winners")
            .document_id(&"11223344")
            .object(&daily_winners)
            .execute()
            .await
            .unwrap()
    }
}