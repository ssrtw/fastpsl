use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PslError {
    #[error("HTTP 請求失敗：{0}")]
    Http(#[from] reqwest::Error),

    #[error("IO 操作失敗：{0}")]
    Io(#[from] io::Error),

    #[error("PSL 解析失敗：{0}")]
    Parse(publicsuffix::Error),
}
