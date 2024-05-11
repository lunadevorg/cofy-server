use http::StatusCode;
use serde_json::to_string;
use std::{collections::HashMap, future::Future, task::Poll};

pub type StringMap = HashMap<String, String>;

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

pub struct HandlerResult {
    pub code: u16,
    pub result: StringMap,
}

impl HandlerResult {
    pub fn get_response(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\n\r\n{}",
            self.code,
            StatusCode::from_u16(self.code).unwrap().as_str(),
            to_string(&self.result).unwrap()
        )
    }
}

impl Future for HandlerResult {
    type Output = String;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.result.contains_key("detail") {
            Poll::Ready(self.get_response())
        } else {
            Poll::Pending
        }
    }
}
