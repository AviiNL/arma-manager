use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub type ApiResult<T> = Result<T, (StatusCode, Json<ErrorResponse>)>;

#[derive(Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    status_code: StatusCode,
    root_key_name: String,

    pub status: String,
    pub data: T,
}

#[allow(unused)]
impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        Self {
            status: "success".into(),
            root_key_name: "data".into(),
            data: data,
            status_code: StatusCode::OK,
        }
    }

    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn with_root_key_name(mut self, root_key_name: impl Into<String>) -> Self {
        self.root_key_name = root_key_name.into();
        self
    }
}

impl<T> Serialize for ApiResponse<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serde_json::Map::new();
        if !self.root_key_name.is_empty() {
            map.insert(self.root_key_name.clone(), serde_json::to_value(&self.data).unwrap());
        }
        map.insert("status".into(), serde_json::Value::String(self.status.clone()));
        serde_json::Value::Object(map).serialize(serializer)
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.status_code, serde_json::json!(self).to_string()).into_response()
    }
}

// impl From<ApiResponse> for (StatusCode, axum::Json<ApiResponse>) {
//     fn from(response: ApiResponse) -> Self {
//         (response.status_code, Json(response))
//     }
// }

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    #[serde(skip)]
    status_code: StatusCode,
    pub status: &'static str,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            status: "error",
            message,
        }
    }

    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}

impl From<ErrorResponse> for (StatusCode, Json<ErrorResponse>) {
    fn from(error: ErrorResponse) -> Self {
        (error.status_code, Json(error))
    }
}
