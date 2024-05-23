/*
*     _____     ___
*    / ___/__  / _/_ __
*   / /__/ _ \/ _/ // /
*   \___/\___/_/ \_, /
*               /___/
*
*   http_parse.rs: http request/response manipulation
*/

use http::StatusCode;
use serde_json::to_string;
use std::collections::HashMap;

type StringMap = HashMap<String, String>;

pub fn parse_http_request(request: &str) -> Option<(String, StringMap)> {
    let parts: Vec<_> = request.splitn(2, ' ').collect();
    if parts.len() != 2 {
        return None;
    }

    let (path_and_query, _version) = (parts[1].split_once(' ')?.0, parts[0]);

    let mut query_params = HashMap::new();

    let path = if let Some(index) = path_and_query.find('?') {
        let query = path_and_query.get(index + 1..)?;

        for param in query.split('&') {
            let param = param.replace("%20", " ");
            let key_value: Vec<_> = param.split('=').collect();
            if key_value.len() == 2 {
                query_params.insert(key_value[0].to_owned(), key_value[1].to_owned());
            }
        }

        path_and_query.get(..index)?.to_owned()
    } else {
        path_and_query.to_owned()
    };

    Some((path, query_params))
}

pub fn new_response(code: u16, result: &StringMap) -> String {
    let data = StatusCode::from_u16(code);
    let resp = data.unwrap_or(StatusCode::NOT_FOUND);
    format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\n\r\n{}",
        resp,
        to_string(&result).unwrap_or_else(|_| "{}".to_owned())
    )
}

pub fn new_str_response(code: u16, result: &str) -> String {
    let data = StatusCode::from_u16(code).unwrap_or(StatusCode::NOT_FOUND);
    format!("HTTP/1.1 {data}\r\nContent-Type: application/json\r\n\r\n{result}")
}
