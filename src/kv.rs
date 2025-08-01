use worker::{Error, Request, Response, RouteContext};

pub struct Kv;

impl Kv {
    /// kv get
    /// http get
    pub async fn get(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
        let key = ctx.param("key").ok_or(Error::Infallible)?;
        let kv_store = ctx.env.kv("KV_BINDING")?;
        let value = kv_store
            .get(key.as_str())
            .text()
            .await?
            .unwrap_or(format!("kv key: {} not exist!", key));
        Response::ok(value)
    }
}
