mod server;
mod modules;

pub fn main (){
    server::server_main::main();
    modules::hello::hello();
}