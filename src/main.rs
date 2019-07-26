use hyper::{Client, Server, client::HttpConnector};
use hyper::service::service_fn;
use hyper::rt::{self, Future};
use native_tls::{TlsConnector, Protocol};
use hyper_tls::HttpsConnector;
use std::net::{SocketAddr, IpAddr};
use std::env;
use std::str::FromStr;

fn main() {
    let out_ip = env::var("SOURCE_IP").map(|x| IpAddr::from_str(&x).expect("No valid ip given")).expect("No source ip addr given.");
    let in_ip = env::var("LISTEN_IP").map(|x| IpAddr::from_str(&x).expect("No valid ip given")).unwrap_or(IpAddr::V4([0, 0, 0, 0].into()));
    let out_port = env::var("SOURCE_PORT").map(|x| u16::from_str(&x).expect("Port must be numeric")).unwrap_or(8443);
    let in_port = env::var("LISTEN_PORT").map(|x| u16::from_str(&x).expect("Port must be numeric")).unwrap_or(8008);
    let token_main = env::var("TOKEN").expect("Google Cast API Token is required");
    let in_addr = (in_ip, in_port).into();
    let out_addr: SocketAddr = (out_ip, out_port).into();

    let tls = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .max_protocol_version(Some(Protocol::Tlsv12))
        .use_sni(false)
        .build()
        .expect("TLS initialization failed");
    let mut http = HttpConnector::new(4);
    http.enforce_http(false);
    let https = HttpsConnector::from((http, tls));
    let client_main = Client::builder()
        .build::<_, hyper::Body>(https);
            
    let out_addr_clone = out_addr.clone();
    let new_service = move || {
        let client = client_main.clone();
        let token = token_main.clone();

        service_fn(move |mut req| {
            let uri_string = format!("https://{}{}",
                out_addr_clone,
                req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));
            let uri = uri_string.parse().unwrap();
            *req.uri_mut() = uri;
            req.headers_mut().remove("host");
            req.headers_mut().remove("Host");
            req.headers_mut().insert(
                "cast-local-authorization-token",
                token.parse().unwrap()
            );

            println!("Request:\n\t{:?}\n", req);

            client.request(req)
                .inspect(|resp| println!("Response:\n\t{:?}\n", resp))
                .map_err(|err| { println!("Error:\n\t{}:{:?}\n", err, err); err })
        })
    };

    let server = Server::bind(&in_addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", in_addr);
    println!("Proxying on https://{}\n", out_addr);

    rt::run(server);
}
