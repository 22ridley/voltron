extern crate rocket;

use voltron::build_server;

#[rocket::main]
async fn main() {
    if let Err(e) = build_server()
        .launch()
        .await 
    {
        println!("didn't launch properly");
        drop(e);
    };
}
