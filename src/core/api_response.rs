#![allow(dead_code)]

use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    code: u16,
    message: String,
    data: Option<serde_json::Value>,
}

/// default api response
impl Default for ApiResponse {
    fn default() -> Self {
        ApiResponse {
            code: 0,
            message: "ok".to_string(),
            data: Some(serde_json::Value::Null),
        }
    }
}

impl ApiResponse {
    /// create new `ApiResponse`
    #[inline]
    pub fn builder() -> Self {
        ApiResponse::default()
    }

    /// set code to current `ApiResponse`
    pub fn with_code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    /// set message to current `ApiResponse`
    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    /// set data to current `ApiResponse`
    pub fn with_data<T: Serialize>(mut self, ret: T) -> Self {
        self.data = Some(serde_json::to_value(ret).unwrap());
        self
    }

    pub fn build(self) -> Self {
        Self {
            code: self.code,
            message: self.message,
            data: self.data,
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
