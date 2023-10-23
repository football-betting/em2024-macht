use crate::api::match_client::Match;
use chrono::{DateTime, FixedOffset, Utc};

pub struct DateHandler {}

impl DateHandler  {
    pub fn convert_date_from_utc_to_cet(matches: &mut Vec<Match>) {
        for single_match in matches {
            let utc_date_time = single_match.utcDate.parse::<DateTime<Utc>>().unwrap();

            let converted_to_cet: DateTime<FixedOffset> = utc_date_time.with_timezone(&FixedOffset::west_opt(-7200).unwrap());

            single_match.cetDate = converted_to_cet.to_string();
        }
    }
}