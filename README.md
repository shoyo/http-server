# Custom Web Server

## About
An implementation of a toy web server in Rust. Establishes a TCP connection on `localhost` at port `7878` and accepts HTTP GET requests. Uses a threadpool with an arbitrary number of workers in order to serve multiple requests concurrently. Server can shut down gracefully in order to allow threads to complete their tasks before quitting.

## How to Use
Install Rust with cargo, and run

    $ cargo run

Open your browser and navigate to `localhost:7878/`. This should render the content of `templates/hello.html`. Making a request to any other path should return a `404 NOT FOUND` and render the content of `templates/404.html`. If this behavior doesn't work on your default browser, consider trying
 another browser.

## Credits
This web server was modeled after the code in **The Rust Programming Language**, [chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html).
