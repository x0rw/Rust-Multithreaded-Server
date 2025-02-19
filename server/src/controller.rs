use std::collections::HashMap;

use crate::{
    builder::{Response, StatusCode},
    error::Result,
    handler::Data,
    Error,
};
pub struct Controller {
    count: u32,
}
impl Controller {
    pub fn EchoController(data: Data) -> Response {
        let s = data.params.unwrap_or_default();
        let header = data.header;
        let serial = serde_json::to_string(&s).unwrap_or_default();
        return Response::JSON(serial, StatusCode::Ok200);
    }
}
