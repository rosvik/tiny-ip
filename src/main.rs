use std::env;
use tiny_http::{Response, Server};

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = match args.iter().position(|x| x == "--port") {
        Some(i) => args[i + 1].clone(),
        None => "2331".to_string(),
    };

    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on http://{}", addr);

    let server = Server::http(addr).unwrap();
    for request in server.incoming_requests() {
        let ip = match request.remote_addr() {
            Some(ip) => ip,
            None => {
                let result = request.respond(Response::empty(500));
                if let Err(e) = result {
                    eprintln!("Error responding to request: {}", e);
                }
                continue;
            }
        };
        let response = Response::from_string(ip.to_string());
        println!("{} {}", ip, request.url());
        let _ = request.respond(response);
    }
}
