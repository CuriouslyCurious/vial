use vial::prelude::*;

vial! {
    GET "/hi/world" => |_| "Hello, world!";
    GET "/hey/:place" => |req|
        format!("Heyo, {}!", req.arg("place").unwrap_or("?"));

    GET "/" => welcome;
}

fn welcome(_req: Request) -> impl Responder {
    Response::from_file("examples/welcome.html")
}

fn main() {
    vial::run!().unwrap();
}
