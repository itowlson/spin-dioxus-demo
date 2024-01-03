#![allow(non_snake_case)]

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn handle_dioxus_demo(req: Request) -> Result<Response> {
    let url = req.headers().get("spin-full-url").unwrap().to_str()?;

    let router = rsx! {
        Router {
            initial_url: url.to_owned()
            Menu {}
            Route { to: "/", Home {} }
            Route { to: "/home", Home {} }
            Route { to: "/users", Users {} }
            Route { to: "", NotFound {} }
        }
    };

    let body = dioxus_ssr::render_lazy(router);
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Some(body.into()))?)
}

use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};

fn NotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Oh no! This page doesn't exist" }
        div {
            Link { to: "/home", "Home" }
        }
    })
}

fn Menu(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "SPIN DIOXUS DEMO" }
        hr {}
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Hello from Dioxus running on Spin" }
    })
}

fn Users(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "No users defined!" }  // srs bzness logic
    })
}
