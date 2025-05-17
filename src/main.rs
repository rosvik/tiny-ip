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
        let res = ip_res(&req);
        match req.respond(res) {
            Ok(_) => {}
            Err(e) => eprintln!("Error responding to request: {}", e),
        };
    }
}

fn ip_res(req: &Request) -> ResponseBox {
    if let Some(xff) = req
        .headers()
        .iter()
        .find(|h| h.field.equiv("X-Forwarded-For"))
        .map(|h| h.value.as_str())
    {
        return Response::from_string(xff).boxed();
    }

    match req.remote_addr() {
        Some(ip) => Response::from_string(ip.to_string()).boxed(),
        None => Response::empty(400).boxed(),
    }
}
