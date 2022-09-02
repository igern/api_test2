use server::server;
mod common;
mod server;
mod test;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
