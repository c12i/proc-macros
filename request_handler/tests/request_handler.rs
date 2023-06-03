#![allow(unused)]
use request_handler::request_handler;

// dummy structs to represent a Request and Response

struct Request;
struct Response;

#[request_handler(GET, "/")]
fn index(req: Request, res: Response) {
    //.. handle req
		// .. return res
}

#[request_handler(GET, "/")]
fn posts(req: Request, res: Response) {
    //.. handle req
		// .. return res
}