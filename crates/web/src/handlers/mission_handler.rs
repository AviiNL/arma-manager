// a single function to upload a mission file

use api_schema::response::SimpleResponse;
use axum::response::IntoResponse;
use axum_extra::extract::Multipart;

use crate::response::{ApiResponse, ApiResult};

pub async fn upload_mission(mut multipart: Multipart) -> ApiResult<impl IntoResponse> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}
