use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResp<'a> {
    pub message: &'a str,
}
