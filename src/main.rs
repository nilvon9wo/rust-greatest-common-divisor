extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use std::str::FromStr;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

fn main() {
    let router = create_router();
    serve(router);
}

fn create_router() -> Router {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    router
}

fn serve(router: Router) {
    let domain = "localhost:3000";
    println!("Serving on http://{}", domain);
    Iron::new(router).http(domain).unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text / Html; Charset = Utf8));
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>"#);
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) =>
            map
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) =>
            nums
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' parameter not a number: {:?}\n",
                            unparsed));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n); }
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = greatest_common_divisor(d, *m);
    }
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
                numbers, d));
    Ok(response)
}

fn greatest_common_divisor(mut integer1: u64, mut integer2: u64) -> u64 {
    assert!(integer1 != 0 && integer2 != 0);
    while integer2 != 0 {
        if integer2 < integer1 {
            let temp = integer2;
            integer2 = integer1;
            integer1 = temp;
        }
        integer2 = integer2 % integer1;
    }
    integer1
}

#[test]
fn test_greatest_common_divisor() {
    assert_eq!(greatest_common_divisor(14, 15), 1);
    assert_eq!(greatest_common_divisor(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

