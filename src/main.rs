extern crate core;

mod api;
mod service;

use crate::api::match_client::MatchClient;
use crate::service::daily_winner::DailyWinnerService;
use crate::service::date_handler::DateHandler;
use crate::service::firebase_connector::FirebaseConnector;
use crate::service::ranking::Ranking;

#[tokio::main]
async fn main() {
    let mut api_result = MatchClient::get_matches().await;
    DateHandler::convert_date_from_utc_to_cet(api_result.matches.as_mut());

    let firebase_writer = FirebaseConnector::init().await;
    let saved_matches = firebase_writer.write_matches(api_result).await;
    let tips = firebase_writer.get_tips().await;
    let user_ranking = Ranking::get_user_ranking(saved_matches.clone().matches, tips).await;
    let saved_ranking = firebase_writer.write_ranking(user_ranking).await;
    let daily_winners = DailyWinnerService::get_daily_winners(saved_ranking, saved_matches.matches);
    let _saved_daily_winners = firebase_writer.write_daily_winner(daily_winners).await;
}
