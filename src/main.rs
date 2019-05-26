extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use std::collections::HashMap;
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
            <input type="text" name="integer"/>
            <input type="text" name="integer"/>
            <button type="submit">Compute GCD</button>
        </form>"#);
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let form_data = parse_request(request);
    let unparsed_values = extract_values(form_data);
    let numbers = parse_for_numbers(unparsed_values);
    match find_greatest_common_devisor(&numbers) {
        Err(error) => { return Ok(error); }
        Ok(divisor) => {
            let mut response = Response::new();
            response.set_mut(status::Ok);
            response.set_mut(mime!(Text/Html; Charset=Utf8));
            response.set_mut(
                format!(
                    "The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
                    numbers,
                    divisor
                )
            );
            response
        }
    }
}

fn parse_request<'a>(request: &'a mut Request)
                     -> Result<&'a HashMap<String, Vec<String>>, &'a mut Response> {
    match request.get_ref::<UrlEncodedBody>() {
        Err(error) => {
            return Err(
                create_bad_response(format!("Error parsing form data: {:?}\n", error))
            );
        }
        Ok(form_data) => Ok(form_data)
    }
}

fn extract_values<'a>(form_data: Result<&HashMap<String, Vec<String>>, &'a mut Response>)
                      -> Result<&'a Vec<String>, &'a mut Response> {
    match form_data {
        Err(error) => { return Err(error); }
        Ok(input_values) => {
            match input_values.get("integer") {
                None => {
                    return Err(
                        create_bad_response(format!(
                            "form data has no 'integer' parameter\n"
                        ))
                    );
                }
                Some(values) => Ok(values)
            }
        }
    }
}

fn parse_for_numbers<'a>(unparsed_values: Result<&Vec<String>, &'a mut Response>)
                         -> Result<&'a Vec<String>, &'a mut Response> {
    match unparsed_values {
        Err(error) => { return Err(error); }
        Ok(values) => {
            let &mut integers = Vec::new();
            for unparsed in values {
                match u64::from_str(&unparsed) {
                    Err(_) => {
                        return Err(create_bad_response(format!(
                            "Value for 'integer' parameter not a number: {:?}\n",
                            unparsed
                        )));
                    }
                    Ok(n) => { integers.push(n); }
                }
            }
            Ok(integers)
        }
    }
}

fn find_greatest_common_devisor<'a>(numbers: &Result<&Vec<String>, &mut Response>)
                                    -> Result<&'a u64, &'a mut Response> {
    match numbers {
        Err(error) => { return Err(*error); }
        Ok(integers) => {
            let mut divisor = numbers[0];
            for next_integer in &numbers[1..] {
                divisor = greatest_common_divisor(divisor, *next_integer);
            }
            Ok(divisor)
        }
    }
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

fn create_bad_response<'a>(error_message: String) -> &'a mut Response {
    Response::new()
        .set_mut(status::BadRequest)
        .set_mut(error_message)
}
