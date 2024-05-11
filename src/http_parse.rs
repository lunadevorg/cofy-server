use http::StatusCode;
use serde_json::to_string;
use std::collections::HashMap;

type StringMap = HashMap<String, String>;

pub fn parse_http_request(request: &str) -> Option<(String, StringMap)> {
    let parts: Vec<&str> = request.splitn(2, ' ').collect();
    if parts.len() != 2 {
        return None;
    }

    let (path_and_query, _version) = (parts[1].split_once(' ')?.0, parts[0]);

    let path: String;
    let mut query_params = HashMap::new();

    if let Some(index) = path_and_query.find('?') {
        path = path_and_query.get(..index)?.to_owned();
        let query = &path_and_query.get(index + 1..)?;

        for param in query.split('&') {
            let param = param.replace("%20", " ");
            let key_value: Vec<&str> = param.split('=').collect();
            if key_value.len() == 2 {
                query_params.insert(key_value[0].to_owned(), key_value[1].to_owned());
            }
        }
    } else {
        path = path_and_query.to_owned();
    }

    Some((path, query_params))
}

pub fn construct_response(code: u16, result: StringMap) -> String {
    let data = StatusCode::from_u16(code);
    let resp = data.unwrap_or(StatusCode::NOT_FOUND);
    format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\n\r\n{}",
        resp,
        to_string(&result).unwrap_or_else(|_| "{}".to_owned())
    )
}

/*
pub struct HandlerResult {
    pub code: u16,
    pub result: StringMap,
}

impl HandlerResult {
    pub fn get_response(&self) -> String {
        let data = StatusCode::from_u16(self.code);
        let resp = data.unwrap_or(StatusCode::OK);
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\n\r\n{}",
            self.code,
            resp,
            to_string(&self.result).unwrap_or_else(|_| "{}".to_owned())
        )
    }
}

impl Future for HandlerResult {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.result.contains_key("detail") {
            Poll::Ready(self.get_response())
        } else {
            Poll::Pending
        }
    }
}
*/
