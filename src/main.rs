use tiny_http::{Response, Server};
fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    for request in server.incoming_requests() {
        let ip = match request.remote_addr() {
            Some(ip) => ip,
            None => {
                let response = Response::from_string("".to_string());
                let response = response.with_status_code(500);
                let _ = request.respond(response);
                continue;
            }
        };
        let response = Response::from_string(ip.to_string());
        let _ = request.respond(response);
    }
}
