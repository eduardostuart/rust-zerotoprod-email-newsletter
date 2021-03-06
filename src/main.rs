use zero2prod::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    run(addr)?.await
}
