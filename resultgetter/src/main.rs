mod lambda_gateway;

use chrono::{DateTime, Utc};
use http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN};
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use serde_json::json;

use db::draw_results::DrawResult;
use lambda_gateway::{LambdaRequest, LambdaResponse, LambdaResponseBuilder};

#[derive(Serialize, Deserialize, Debug)]
struct InputPayload {
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

impl InputPayload {
    fn new() -> Self {
        InputPayload { date: Utc::now() }
    }
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(lambda_handler);
    Ok(())
}

fn lambda_handler(
    req: LambdaRequest<InputPayload>,
    _: Context,
) -> Result<LambdaResponse, HandlerError> {
    let cfg = db::config::Config::new();
    let client = match db::connect(cfg) {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    let payload = req.body();
    let date = payload.date;

    let result: DrawResult = match db::draw_results::get_draw_results(client, date) {
        Some(res) => res,
        None => panic!("no draw result"),
    };

    // Ok(json!({
    //     "code": 200,
    //     "success": true,
    //     "items": result,
    // }))

    let response = LambdaResponseBuilder::new()
        .with_header(ACCESS_CONTROL_ALLOW_ORIGIN.to_string(), String::from("*"))
        .with_header(
            ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
            String::from("Content-Type,X-Amz-Date,Authorization,X-Api-Key,x-requested-with"),
        )
        .with_status(200)
        .with_json(json!({
            "code": 200,
            "success": true,
            "items": result,
        }))
        .build();

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::{oid::ObjectId, UtcDateTime};
    use chrono::NaiveDateTime;

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

        let mut payload: InputPayload = InputPayload::new();
        payload.date = selected_date_time;

        let request = LambdaRequest::new(payload);
        // let mut request = Request::new(Body::from(format!(
        //     r#"{{"date":"{}"}}"#,
        //     selected_date_time_str
        // )));
        // request
        //     .headers_mut()
        //     .insert("Content-Type", HeaderValue::from_static("application/json"));

        let oid =
            ObjectId::with_string("5ec5639500da050c00e537ec").expect("Object id is not converted");
        let all_numbers: Vec<&str> = vec![
            "1897", "9470", "2450", "3045", "7250", "1307", "7414", "5619", "3655", "4837", "7883",
            "9129", "5754", "6649", "8102", "5497", "6536", "2308", "9539", "4105", "2680", "7227",
            "2448",
        ];

        let mut result: DrawResult = DrawResult::new();
        result.id = oid;
        result.result = all_numbers.iter().map(|n| n.to_string()).collect();
        result.result_date = UtcDateTime(selected_date_time);

        let expected = LambdaResponseBuilder::new()
            .with_header(ACCESS_CONTROL_ALLOW_ORIGIN.to_string(), String::from("*"))
            .with_header(
                ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
                String::from("Content-Type,X-Amz-Date,Authorization,X-Api-Key,x-requested-with"),
            )
            .with_json(json!({
                "code": 200,
                "success": true,
                "items": result,
            }));

        let response = lambda_handler(request, Context::default()).expect("expected Ok(_) value");

        assert_eq!(response, expected.build())
    }
}
