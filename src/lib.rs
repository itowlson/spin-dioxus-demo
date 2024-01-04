#![allow(non_snake_case)]

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response, IntoResponse},
    http_component,
};

#[http_component]
fn handle_dioxus_demo(req: Request) -> Result<impl IntoResponse> {
    let url = req.header("spin-full-url").unwrap().as_str().unwrap();
    let url: http::uri::Uri = url.parse()?;
    let path = url.path().to_owned();

    let body = dioxus_ssr::render_lazy(rsx! { App { path: path } });
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(body)
        .build())
}

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
fn App(cx: Scope, path: String) -> Element {
    let initial: Route = path.parse().unwrap();
    let config = move || {
        RouterConfig::default().history(MemoryHistory::with_initial_path(initial))
    };
    cx.render(rsx! {
        Menu {}
        Router::<Route> {
            config: config
        }
    })
}

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    #[redirect("/home", || Route::Home {})]
    Home {},
    #[route("/users")]
    Users {},
    #[route("/:.._segments")]
    NotFound { _segments: Vec<String> },
}

#[component]
fn NotFound(cx: Scope, _segments: Vec<String>) -> Element {
    cx.render(rsx! {
        div { "Oh no! This page doesn't exist" }
        div {
            Link { to: Route::Home {}, "Home" }
        }
    })
}

#[component]
fn Menu(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "SPIN DIOXUS DEMO" }
        hr {}
    })
}

#[component]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Hello from Dioxus running on Spin" }
    })
}

#[component]
fn Users(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "No users defined!" }  // srs bzness logic
    })
}
