use std::env;
use tiny_http::{Request, Response, ResponseBox, Server};

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = match args.iter().position(|x| x == "--port") {
        Some(i) => args[i + 1].clone(),
        None => String::from("2331"),
    };

    let addr = format!("0.0.0.0:{}", port);
    let serv = Server::http(addr).expect("Failed to start server");
    println!("Listening on {}", serv.server_addr());

    for req in serv.incoming_requests() {
        let response = ip_response(&req);
        let result = req.respond(response);
        if let Err(e) = result {
            eprintln!("Error responding to request: {}", e);
        }
    }
}

fn ip_response(req: &Request) -> ResponseBox {
    let x_forwarded_for = req
        .headers()
        .iter()
        .find(|header| header.field.equiv("X-Forwarded-For"))
        .map(|header| header.value.as_str());

    if let Some(x_forwarded_for) = x_forwarded_for {
        return Response::from_string(x_forwarded_for).boxed();
    }

    match req.remote_addr() {
        Some(ip) => Response::from_string(ip.to_string()).boxed(),
        None => Response::empty(400).boxed(),
    }
}
