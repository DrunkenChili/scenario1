mod handler;
mod server;
mod router;
mod server:Server;

fn main(){
    let server = Server::new("localhost:3000");
    server.run();
}