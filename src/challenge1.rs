use poem::{handler, Request, Response, http::StatusCode};
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Deserialize)]
struct ValidationRequest {
    from: String,
    key: String,
}

#[derive(Deserialize)]
struct ReverseRequest {
    from: String,
    to: String,
}

#[handler]
pub fn ipv4_encryption_validation(req: &Request) -> Response {
    let params = req.params::<ValidationRequest>();
    if params.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Error retrieving query parameters");
    }
    let validation_request = params.unwrap();
    let from_address = validation_request.from.parse::<Ipv4Addr>();
    let key_address = validation_request.key.parse::<Ipv4Addr>();

    if from_address.is_err() || key_address.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Error retrieving IP address(es) from query parameters");
    }

    let from_octets = (from_address.unwrap()).octets();
    let key_octets = (key_address.unwrap()).octets();

    let mut dest_octets = [0u8; 4];
    let mut i = 0;
    while i < 4 {
        (dest_octets[i], _) = u8::overflowing_add(from_octets[i], key_octets[i]);
        i += 1;
    }
    let dest = ipv4_from_octets(dest_octets);
    
    Response::builder()
            .status(StatusCode::OK)
            .body(format!("{}", dest))
}

#[handler]
pub fn ipv4_encryption_reverser(req: &Request) -> Response {
    let params = req.params::<ReverseRequest>();
    if params.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Error retrieving query parameters");
    }
    let reverse_request = params.unwrap();
    let from_address = reverse_request.from.parse::<Ipv4Addr>();
    let to_address = reverse_request.to.parse::<Ipv4Addr>();

    if from_address.is_err() || to_address.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("One or more query parameter not representable as IP v6 address");
    }

    let from_octets = (from_address.unwrap()).octets();
    let to_octets = (to_address.unwrap()).octets();

    let mut key_octets = [0u8; 4];
    let mut i = 0;
    while i < 4 {
        (key_octets[i], _) = u8::overflowing_sub(to_octets[i], from_octets[i]);
        i += 1;
    }

    let dest = ipv4_from_octets(key_octets);
    Response::builder()
            .status(StatusCode::OK)
            .body(format!("{}", dest))
}

#[handler]
pub fn ipv6_encryption_validation(req: &Request) -> Response {
    let params = req.params::<ValidationRequest>();
    if params.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Error retrieving query parameters");
    }
    let validation_request = params.unwrap();
    let from_address = validation_request.from.parse::<Ipv6Addr>();
    let key_address = validation_request.key.parse::<Ipv6Addr>();

    if from_address.is_err() || key_address.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("One or more query parameter not representable as IP v6 address");
    }

    let from_bits = (from_address.unwrap()).to_bits();
    let key_bits = (key_address.unwrap()).to_bits();

    let dest_bits = from_bits ^ key_bits;

    let dest = Ipv6Addr::from_bits(dest_bits);

    Response::builder()
            .status(StatusCode::OK)
            .body(format!("{}", dest))
}

#[handler]
pub fn ipv6_encryption_reverser(req: &Request) -> Response {
    let params = req.params::<ReverseRequest>();
    if params.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Error retrieving query parameters");
    }
    let reverse_request = params.unwrap();
    let from_address = reverse_request.from.parse::<Ipv6Addr>();
    let to_address = reverse_request.to.parse::<Ipv6Addr>();

    if from_address.is_err() || to_address.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("One or more query parameter not representable as IP v4 address");
    }

    let from_bits = (from_address.unwrap()).to_bits();
    let to_bits = (to_address.unwrap()).to_bits();

    let key_bits = from_bits ^ to_bits;

    let key = Ipv6Addr::from_bits(key_bits);

    Response::builder()
            .status(StatusCode::OK)
            .body(format!("{}", key))
}

// We currently can't use #![feature(ip_from)] in Shuttle
// Or we can and I just don't know how :^)
const fn ipv4_from_octets(octets: [u8; 4]) -> Ipv4Addr {
    Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
}