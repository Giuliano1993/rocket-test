use rocket::tokio::time::{sleep, Duration};
use rocket::fs::FileServer;


#[macro_use] extern crate rocket;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}

#[get("/ciaone")]              // <- route attribute
fn ciaone() -> &'static str {  // <- request handler
    "ciaone, mondo!"
}

#[get("/superciao")]              // <- route attribute
fn superciao() -> &'static str {  // <- request handler
    "superciao, mondo!"
}
#[get("/mediociao")]              // <- route attribute
fn mediociao() -> &'static str {  // <- request handler
    "mediociao, mondo!"
}


#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    
    rocket::build().mount("/", routes![index,world,ciaone,superciao, mediociao,delay]).mount("/public", FileServer::from("./www/static"))
}



