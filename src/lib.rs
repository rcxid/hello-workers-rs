#[cfg(feature = "ssr")]
use worker::*;

pub mod app;

// #[event(fetch)]
// async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
//     console_error_panic_hook::set_once();
//     Router::new()
//         .get_async("/", root)
//         .get_async("/bing", bing)
//         .run(req, env)
//         .await
// }
//
// async fn root(_: Request, _ctx: RouteContext<()>) -> Result<Response> {
//     Response::ok("Hello Workers!")
// }
//
// async fn bing(_: Request, _ctx: RouteContext<()>) -> Result<Response> {
//     let request = Request::new(
//         "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1",
//         Method::Get,
//     )?;
//     let response = Fetch::Request(request)
//         .send()
//         .await?
//         .json::<serde_json::Value>()
//         .await?;
//
//     let url_base = response
//         .get("images")
//         .and_then(|x| x.get(0))
//         .and_then(|x| x.get("urlbase"))
//         .and_then(|x| x.as_str())
//         .ok_or(Error::Infallible)?;
//
//     let url = format!("https://www.bing.com{url_base}_1920x1080.jpg");
//     let destination_url = Url::parse(url.as_str())?;
//     let status_code = 302;
//     Response::redirect_with_status(destination_url, status_code)
// }

#[cfg(feature = "ssr")]
#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    use tower_service::Service;

    console_error_panic_hook::set_once();

    Ok(axum_router(env).await.call(req).await?)
}

#[cfg(feature = "ssr")]
async fn axum_router(env: Env) -> axum::Router {
    use std::sync::Arc;
    use axum::Extension;
    use axum::routing::get;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use app::*;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    async fn root() -> &'static str {
        "Hello Workers!"
    }

    axum::Router::new()
        // .route("/", get(root))
        // .leptos_routes()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .route("/root", get(root))
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env)))
}

/// maybe useless
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
