use actix_files::Files;
use actix_web::{HttpServer, App};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ip_addr = "localhost";
    let port = 1313;
    match HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind((ip_addr, port)){
        Ok(server) => {
            println!("Servidor iniciado en http://{}:{}", ip_addr, port);
            println!("desde el directorio ./static");
            server
        },
        Err(e) => {
            println!("Error: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error"));
        }
    }
    .run()
    .await
}