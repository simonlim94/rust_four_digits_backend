use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use bson::UtcDateTime;
use chrono::{DateTime, NaiveDateTime, Utc};
use lambda_runtime::{error::HandlerError, lambda, Context};
use rand::{distributions::Uniform, Rng};

use db::draw_results::DrawResult;

const NUMBER_OF_RESULTS: i32 = 23;

fn main() {
    lambda!(handler)
}

fn handler(_: CloudWatchEvent, _: Context) -> Result<(), HandlerError> {
    let mut rng = rand::thread_rng();

    let range = Uniform::new(0, 10000);
    let num_of_random_numbers = 0..NUMBER_OF_RESULTS; 

    //produce 19 random numbers between 0 and 10000
    let vals: Vec<u64> = num_of_random_numbers.map(|_| rng.sample(range)).collect();
    let mut vals_in_str: Vec<String> = Vec::new();
    for val in vals {
        let mut val_in_str = val.to_string();
        let len_of_str = val_in_str.len();

        if len_of_str < 4 {
            // if not 4 digits need to append zero
            val_in_str = format!("{:0>4}", val_in_str);
        }
        vals_in_str.push(val_in_str);
    }

    let current_date = format!("{} 00:00:00", Utc::now().format("%Y-%m-%d").to_string());
    let current_date_time = match NaiveDateTime::parse_from_str(&current_date, "%Y-%m-%d %H:%M:%S")
        .map(|ndt| DateTime::<Utc>::from_utc(ndt, Utc))
    {
        Ok(dt) => dt,
        Err(e) => panic!(e),
    };

    let mut draw_result: DrawResult = DrawResult::new();
    draw_result.result = vals_in_str;
    draw_result.result_date = UtcDateTime(current_date_time);

    let cfg = db::config::Config::new();
    let client = match db::connect(cfg) {
        Ok(c) => c,
        Err(e) => panic!(e),
    };
    db::draw_results::insert_draw_result(client, draw_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    
    #[test]
    fn handler_handles() {
        let request = CloudWatchEvent {
            version: None,
            id: None,
            detail_type: None,
            source: None,
            account_id: None,
            time: Utc::now(),
            region: None,
            resources: vec![],
            detail: Value::Null,
        };
        let response = handler(request, Context::default()).expect("expected Ok(()) value");
        assert_eq!(response, ())
    }
}
