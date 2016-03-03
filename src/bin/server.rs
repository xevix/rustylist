extern crate rustylist;
extern crate iron;
extern crate router;
extern crate dotenv;

use self::rustylist::services::todos_service;

use dotenv::dotenv;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};

fn main() {
    dotenv().ok();

    let mut router = Router::new();
    router.get("/", handler);

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(_: &mut Request) -> IronResult<Response> {
        let todos = todos_service::fetch_todos();
        Ok(Response::with((status::Ok, todos)))
    }
}
