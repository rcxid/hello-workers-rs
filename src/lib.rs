use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    Router::new()
        .get_async("/", root)
        .get_async("/bing", bing)
        .run(req, env)
        .await
}

async fn root(_: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello Workers!")
}

async fn bing(_: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let request = Request::new(
        "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1",
        Method::Get,
    )?;
    let response = Fetch::Request(request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let url_base = response
        .get("images")
        .and_then(|x| x.get(0))
        .and_then(|x| x.get("urlbase"))
        .and_then(|x| x.as_str())
        .ok_or(Error::Infallible)?;

    let url = format!("https://www.bing.com{url_base}_1920x1080.jpg");
    let destination_url = Url::parse(url.as_str())?;
    let status_code = 302;
    Response::redirect_with_status(destination_url, status_code)
}
