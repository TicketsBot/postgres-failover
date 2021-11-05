use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Error {
        error: String,
    },
    CheckResponse {
        online: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time: Option<usize>,
    },
}

impl Response {
    pub fn error(error: &str) -> Response {
        Response::Error {
            error: error.to_owned(),
        }
    }

    pub fn check_response(online: bool, response_time: Option<usize>) -> Response {
        Response::CheckResponse {
            online,
            response_time,
        }
    }
}
