A basic repo to experiment with Web dev in Rust. Uses:

- [Actix-web](https://actix.rs/) for a web framework
- [Askama](https://docs.rs/askama/0.10.5/askama/) for templating

A simple example is built-up in multiple steps:

- [Step 1: Hello](src/bin/01_hello.rs): A static response to the root URL.
- [Step 2: Hello, {name}](src/bin/02_name.rs): Use a path parameter to modify response. Some testing
- [Step 3: A template](src/bin/03_html.rs): HTML template with some more dynamic content.
- [Step 4: Testing](src/bin/04_testing.rs): Some more testing with a reusable helper.
