use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

pub fn hello_web(state: State) -> (State, &'static str) {
    (state, "Hello, Web!")
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(hello_web);
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
    fn receive_hello_router_response() {
        let test_server = TestServer::new(router()).unwrap();
        let client = test_server.client();

        let response = client.
            get("http://localhost").
            perform().
            unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_utf8_body().unwrap();
        assert_eq!(body, "Hello, Web!");
    }
}
