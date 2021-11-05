use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Error { success: bool, error: String },
    Success { success: bool },
}

impl Response {
    pub fn error(error: String) -> Response {
        Response::Error {
            success: false,
            error,
        }
    }

    pub fn success() -> Response {
        Response::Success { success: true }
    }
}
