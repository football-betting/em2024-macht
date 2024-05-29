use crate::api::match_client::Match;

pub struct ScoreHelper {}

impl ScoreHelper {
    pub fn set_home_and_away_score(matches: &mut Vec<Match>) {
        for single_match in matches {
            single_match.homeScore = single_match.score.fullTime.home;
            single_match.awayScore = single_match.score.fullTime.away;
        }
    }
}