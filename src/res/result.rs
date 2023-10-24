use super::error::AppError;
use salvo::{http::response::Response};
use salvo::writing::Json;
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

pub struct Res;

#[derive(Debug, Serialize)]
pub struct ResBuilder<T> {
    result: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl Res {
    pub fn suc<T: Serialize>() -> ResBuilder<T> {
        ResBuilder {
            result: true,
            code: None,
            code_msg: None,
            msg: None,
            data: None,
        }
    }

    pub fn fail<T: Serialize>() -> ResBuilder<T> {
        ResBuilder {
            result: false,
            code: None,
            code_msg: None,
            msg: None,
            data: None,
        }
    }
}

impl<T: Serialize + std::marker::Send> ResBuilder<T> {
    pub fn code(mut self, code: u32) -> Self {
        self.code = Some(code);
        self
    }

    pub fn msg(mut self, msg: &str) -> Self {
        self.msg = Some(msg.to_string());
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn render(self, res: &mut Response) {
        res.render(Json(self));
    }
}
