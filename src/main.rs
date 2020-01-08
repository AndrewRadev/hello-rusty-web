use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use gotham_derive::{StateData, StaticResponseExtender};
use serde::Deserialize;

pub fn hello_web(state: State) -> (State, &'static str) {
    (state, "Hello, Web!")
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct HelloNamePath {
    name: String,
}

pub fn hello_name(state: State) -> (State, String) {
    let path = HelloNamePath::borrow_from(&state);
    let name = path.name.clone();

    (state, format!("Hello, {}", name))
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(hello_web);

        route.
            get("/:name").
            with_path_extractor::<HelloNamePath>().
            to(hello_name);
    })
}

fn main() {
    let addr = "127.0.0.1:7000";
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router())
}
