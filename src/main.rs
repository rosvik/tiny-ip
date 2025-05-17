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
    let serv = Server::http(addr).expect("Failed to start server");

    for req in serv.incoming_requests() {
        let res = ip_res(&req);
        match req.respond(res) {
            Ok(_) => {}
            Err(e) => eprintln!("Error responding to request: {}", e),
        };
    }
}

fn ip_res(req: &Request) -> ResponseBox {
    match req.remote_addr() {
        Some(ip) => Response::from_string(ip.to_string()).boxed(),
        None => Response::empty(400).boxed(),
    }
}
