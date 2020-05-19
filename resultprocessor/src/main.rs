use bson::UtcDateTime;
use chrono::{DateTime, NaiveDateTime, Utc};
use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use rand::{distributions::Uniform, Rng};
use serde_json::json;

use db::draw_results::DrawResult;

fn main() {
    lambda!(handler)
}

fn handler(_: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let mut rng = rand::thread_rng();

    let range = Uniform::new(0, 10000);
    let num_of_random_numbers = 0..19; //19 random numbers

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

    Ok(json!({}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {
        let request = Request::default();
        let expected = json!({}).into_response();
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
