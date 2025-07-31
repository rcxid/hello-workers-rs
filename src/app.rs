use crate::components::counter::Counter;
use crate::components::hash::HashCalc;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // let (value, _set_value) = signal(0);
    view! {
        <Router>
            <nav>
              <ul class="flex gap-x-4">
                <li><a href="/">首页</a></li>
                <li><a href="/counter">计数器</a></li>
                <li><a href="/hash">hash性能测试</a></li>
              </ul>
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/counter") view=Counter />
                    <Route path=path!("/hash") view=HashCalc />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="text-red-500">Hello Leptos!</div>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Stylesheet href="/style/main.css"/>
                <Stylesheet href="/style/tailwind.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
