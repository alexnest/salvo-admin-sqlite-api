use salvo::prelude::*;
use salvo::Writer;

use super::result::Res;
use serde::Serialize;
use std::{
    error::Error,
    fmt::{self, Display},
    io,
};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("io: `{0}`")]
    Io(#[from] io::Error),
    #[error("sqlx error: `{0}`")]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Parse(#[from] salvo::http::ParseError),
    #[error(transparent)]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("my error occurred")]
    MyErr(#[from] MyErr),
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

#[derive(Debug, Serialize)]
pub struct MyErr {
    result: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
}

impl Error for MyErr {}

impl Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MyErr: result={}, code={:?}, msg={:?}",
            self.result, self.code, self.msg
        )
    }
}

impl MyErr {
    pub fn new() -> Self {
        Self {
            result: false,
            code: None,
            msg: None,
        }
    }

    pub fn code(mut self, code: u32) -> Self {
        self.code = Some(code);
        self
    }

    pub fn msg(mut self, msg: &str) -> Self {
        self.msg = Some(msg.to_string());
        self
    }

    pub fn build(self) -> Result<(), AppError> {
        return Err(AppError::MyErr(self));
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            Self::Io(e) => {
                res.render(Json(
                    Res::fail::<()>().code(500).msg(e.to_string().as_str()),
                ));
            }
            Self::Sqlx(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                error!("SQLx error: {:?}", e);
                res.render(Json(
                    Res::fail::<()>().code(500).msg(e.to_string().as_str()),
                ));
            }
            Self::Parse(ref e) => {
                error!("parse error: {:?}", e);
                res.render(Json(
                    Res::fail::<()>().code(500).msg(e.to_string().as_str()),
                ));
            }
            Self::Bcrypt(ref e) => {
                error!("parse error: {:?}", e);
                res.render(Json(
                    Res::fail::<()>().code(500).msg(e.to_string().as_str()),
                ));
            }
            Self::Jwt(e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                error!("Jwt error: {:?}", e);
                res.render(Json(
                    Res::fail::<()>().code(500).msg(e.to_string().as_str()),
                ));
            }
            Self::MyErr(e) => {
                let mut builder = Res::fail::<()>();
                if let Some(code) = e.code {
                    builder = builder.code(code);
                }
                if let Some(msg) = e.msg {
                    builder = builder.msg(msg.as_str());
                }
                res.render(Json(builder));
            }
            _ => {
                res.render(Json(Res::fail::<()>().msg("other error")));
            }
        }
    }
}
