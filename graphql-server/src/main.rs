mod schema;
mod server;

extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    server::start(([127, 0, 0, 1], 3030)).await;
}
