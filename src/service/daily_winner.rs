use chrono::{DateTime, FixedOffset, Utc};
use serde_derive::{Deserialize, Serialize};
use crate::api::match_client::Match;
use crate::service::ranking::{UserRanking};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DailyWinners {
    dailyWinners: Vec<DailyWinner>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DailyWinner {
    date: String,
    user: Vec<String>,
    points: isize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AllDailies {
    dailies: Vec<Daily>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Daily {
    date: String,
    dailyPoints: Vec<DailyPoints>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DailyPoints {
    user: String,
    points: isize,
}

pub struct DailyWinnerService {}

impl DailyWinnerService {
    pub fn get_daily_winners(ranking: UserRanking, matches: Vec<Match>) -> DailyWinners {
        let mut daily_winners: Vec<DailyWinner> = vec![];

        let finished_matches = get_finished_games(matches);

        let mut all_dailies = AllDailies { dailies: vec![] };
        for finished_match in finished_matches {
            let date = finished_match
                .utcDate
                .parse::<DateTime<Utc>>()
                .unwrap()
                .with_timezone(&FixedOffset::west_opt(-7200).unwrap())
                .date_naive();

            let daily_index = all_dailies.dailies.iter().position(|p| p.date == date.to_string());

            let mut is_new_daily = true;

            let daily = &mut Daily {
                date: date.to_string(),
                dailyPoints: vec![],
            };

            if daily_index.is_some() {
                is_new_daily = false;
            };

            for user in ranking.user.clone() {
                let tip = user.tips.iter().find(|u| u.matchId == finished_match.id);

                if tip.is_some() {
                    let points = tip.unwrap().score;

                    if is_new_daily {
                        let index = daily.dailyPoints.iter().position(|u| u.user == user.name);

                        if index.is_some() {
                            daily.dailyPoints[index.unwrap()].points += points;
                        } else {
                            daily.dailyPoints.push(
                                DailyPoints {
                                    user: user.name,
                                    points,
                                }
                            );
                        }
                    } else {
                        let index = all_dailies.dailies[daily_index.unwrap()].dailyPoints.iter().position(|u| u.user == user.name);

                        if index.is_some() {
                            all_dailies.dailies[daily_index.unwrap()].dailyPoints[index.unwrap()].points += points;
                        } else {
                            all_dailies.dailies[daily_index.unwrap()].dailyPoints.push(
                                DailyPoints {
                                    user: user.name,
                                    points,
                                }
                            );
                        }
                    }
                }
            }

            if is_new_daily {
                all_dailies.dailies.push(daily.clone());
            }
        }

        for mut daily in all_dailies.dailies {
            daily.dailyPoints.sort_by(|a, b| b.points.cmp(&a.points));

            let mut daily_winner = DailyWinner {
                date: daily.date.to_string(),
                user: vec![],
                points: 0,
            };

            let mut highest_score = 0;
            for daily_point in daily.dailyPoints {
                if highest_score == 0 {
                    highest_score = daily_point.points;
                    daily_winner.user.push(daily_point.user);
                } else if highest_score == daily_point.points {
                    daily_winner.user.push(daily_point.user);
                } else {
                    break;
                }
            }

            daily_winner.points = highest_score;

            if !daily_winner.user.is_empty() {
                daily_winners.push(daily_winner);
            }
        }

        DailyWinners {
            dailyWinners: daily_winners
        }
    }
}

fn get_finished_games(matches: Vec<Match>) -> Vec<Match> {
    let mut finished_matches: Vec<Match> = vec![];
    for single_match in matches {
        if single_match.status == "FINISHED" {
            finished_matches.push(single_match);
        }
    }

    finished_matches
}