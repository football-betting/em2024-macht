extern crate core;

mod api;
mod service;

use std::env;
use getopts::Options;
use crate::api::match_client::{ApiResult, MatchClient};
use crate::service::score_helper::ScoreHelper;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("f", "full", "full import");
    let flag_matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    let mut api_result: ApiResult;
    if flag_matches.opt_present("f") {
        api_result = MatchClient::get_matches(None).await;
    } else {
        api_result = MatchClient::get_matches(Some(chrono::offset::Utc::now().to_string())).await;
    }

    if api_result.matches.is_some() {
        ScoreHelper::set_home_and_away_score(api_result.matches.as_mut().unwrap());
        MatchClient::save_matches_to_sqlite(api_result.matches.as_mut().unwrap()).await;
    }
}
