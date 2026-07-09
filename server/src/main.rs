use tiny_http::{Response, Server};

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("UNL server listening on http://localhost:8080");

    for request in server.incoming_requests() {
        let url = request.url().trim_start_matches('/');

        let path = format!("public/{}", url);
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                let response = Response::from_string(content)
                    .with_header("Content-Type: text/plain".parse::<tiny_http::Header>().unwrap());
                request.respond(response).unwrap();
            }
            Err(_) => {
                let response = Response::from_string("404 not found").with_status_code(404);
                request.respond(response).unwrap();
            }
        }
    }
}
