use crate::ServerState;
use serde::{Deserialize, Serialize};
use worker::wasm_bindgen::JsValue;
use worker::{Error, Request, Response, RouteContext};

pub struct D1;

#[derive(Debug, Deserialize, Serialize)]
pub struct Kv {
    key: String,
    value: String,
}

impl D1 {
    pub async fn get(_: Request, ctx: RouteContext<ServerState>) -> worker::Result<Response> {
        let key = JsValue::from_str(ctx.param("key").ok_or(Error::Infallible)?);
        let d1 = ctx.env.d1("DB")?;
        let statement = d1.prepare("SELECT * FROM kv WHERE key = ?1;");
        let query = statement.bind(&[key])?;
        let kv = query.first::<Kv>(None).await?;
        match kv {
            Some(kv) => Response::from_json(&kv),
            None => Response::error("Not found", 404),
        }
    }
}
