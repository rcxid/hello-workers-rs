use crate::ServerState;
use rand::rng;
use rand::seq::IndexedRandom;
use worker::{Error, Headers, Request, Response, Result, RouteContext};

/// 基于R2对象存储的随机图片实现
pub async fn random_image(_: Request, ctx: RouteContext<ServerState>) -> Result<Response> {
    let bucket = ctx.env.bucket("dev_data")?;
    let objects = bucket.list().execute().await?;
    let object_list = objects
        .objects()
        .iter()
        .map(|x| x.key())
        .collect::<Vec<_>>();
    let mut rng = rng();
    let key = object_list.choose(&mut rng).ok_or(Error::Infallible)?;
    let object = bucket.get(key).execute().await?.ok_or(Error::Infallible)?;
    let bytes = object.body().ok_or(Error::Infallible)?.bytes().await?;
    let content_type = object
        .http_metadata()
        .content_type
        .unwrap_or_else(|| "application/octet-stream".to_string());
    let headers = Headers::new();
    headers.set("Content-Type", content_type.as_str())?;
    Response::from_bytes(bytes).map(|resp| resp.with_headers(headers))
}
