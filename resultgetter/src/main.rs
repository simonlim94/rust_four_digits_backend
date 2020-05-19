use chrono::{DateTime, Utc};
use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde::{Deserialize, Serialize};
use serde_json::json;

use db::draw_results::DrawResult;

#[derive(Serialize, Deserialize)]
struct InputPayload {
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

mod date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

fn main() {
    lambda!(handler)
}

fn handler(req: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let cfg = db::config::Config::new();
    let client = match db::connect(cfg) {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    let body = match std::str::from_utf8(req.body()) {
        Ok(b) => b,
        Err(e) => panic!(e),
    };

    println!("body:{}", body);

    let input: InputPayload = match serde_json::from_str(&body) {
        Ok(b) => b,
        Err(e) => panic!(e),
    };

    let result: DrawResult = match db::draw_results::get_draw_results(client, input.date) {
        Some(res) => res,
        None => return Ok(json!({})),
    };

    Ok(json!({
        "code": 200,
        "success": true,
        "payload": result,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::{oid::ObjectId, UtcDateTime};
    use chrono::NaiveDateTime;
    use lambda_http::{http::HeaderValue, Body};

    #[test]
    fn handler_handles() {
        let selected_date_time_str = String::from("2020-05-18 00:00:00");
        let selected_date_time: DateTime<Utc> =
            match NaiveDateTime::parse_from_str(&selected_date_time_str, "%Y-%m-%d %H:%M:%S")
                .map(|ndt| DateTime::<Utc>::from_utc(ndt, Utc))
            {
                Ok(dt) => dt,
                Err(e) => panic!(e),
            };

        let mut request = Request::new(Body::from(format!(
            r#"{{"date":"{}"}}"#,
            selected_date_time_str
        )));
        request
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));

        let oid =
            ObjectId::with_string("5ec2ae3d00398432001409f3").expect("Object id is not converted");
        let all_numbers: Vec<&str> = vec![
            "8317", "2642", "9156", "7162", "5339", "2871", "6894", "7238", "4846", "5100", "6824",
            "5746", "2075", "8079", "0903", "9857", "6843", "0592", "3035",
        ];

        let mut result: DrawResult = DrawResult::new();
        result.id = oid;
        result.result = all_numbers.iter().map(|n| n.to_string()).collect();
        result.result_date = UtcDateTime(selected_date_time);

        let expected = json!({
            "code": 200,
            "success": true,
            "payload": result,
        })
        .into_response();

        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();

        assert_eq!(response.body(), expected.body())
    }
}
