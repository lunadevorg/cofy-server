use std::collections::HashMap;

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
    pub detail: String,
    pub result: StringMap,
}

impl HandlerResult {
    pub fn new() -> Self {
        Self {
            code: 404,
            detail: "not found".to_owned(),
            result: StringMap::new(),
        }
    }
}
