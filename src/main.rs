use std::env;
use tiny_http::{Request, Response, ResponseBox, Server};

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = match args.iter().position(|x| x == "--port") {
        Some(i) => args[i + 1].clone(),
        None => "2331".to_string(),
    };

    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on http://{}", addr);
    let server = Server::http(addr).expect("Failed to start server");

    for request in server.incoming_requests() {
        let response = get_ip(&request);
        match request.respond(response) {
            Ok(_) => {}
            Err(e) => eprintln!("Error responding to request: {}", e),
        };
    }
}

fn get_ip(request: &Request) -> ResponseBox {
    match request.remote_addr() {
        Some(ip) => Response::from_string(ip.to_string()).boxed(),
        None => Response::empty(400).boxed(),
    }
}
