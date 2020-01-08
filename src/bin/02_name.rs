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

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    #[test]
    fn ascii_name() {
        let test_server = TestServer::new(router()).unwrap();
        let client = test_server.client();

        let response = client.
            get("http://localhost/Pesho").
            perform().
            unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_utf8_body().unwrap();
        assert_eq!(body, "Hello, Pesho");
    }

    #[test]
    fn wrong_url() {
        let test_server = TestServer::new(router()).unwrap();
        let client = test_server.client();

        let response = client.
            get("http://localhost/other/url").
            perform().
            unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.read_utf8_body().unwrap();
        assert_eq!(body, "");
    }
}
