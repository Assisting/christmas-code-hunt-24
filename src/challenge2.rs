use cargo_manifest::{Manifest, Package};
use poem::{handler, http::StatusCode, Request, Body, Response};
use serde::Deserialize;
use toml::Value;

#[derive(Deserialize)]
struct Orders {
    orders: Vec<Value>,
}

#[derive(Deserialize)]
struct ToyOrder {
    item: String,
    quantity: u32,
}

#[handler]
pub async fn accept_manifest(req: &Request, body: Body) -> Response {
    let content_type = Request::content_type(req);

    let body_payload = body.into_string().await;

    if body_payload.is_err() {
        return Response::builder()
                .status(StatusCode::NO_CONTENT)
                .finish();
    }

    match content_type {
        Some("application/toml") => return toml_manifest(body_payload.unwrap()),
        Some("application/yaml") => return yaml_manifest(body_payload.unwrap()),
        Some("application/json") => return json_manifest(body_payload.unwrap()),
        _ => return Response::builder()
                    .status(StatusCode::UNSUPPORTED_MEDIA_TYPE)
                    .finish()
    }
}

fn toml_manifest(payload: String) -> Response {
    let toml_manifest = payload.parse::<Manifest>();

    if toml_manifest.is_err() {
        return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid manifest");
    }

    read_manifest(toml_manifest.unwrap())
}

fn yaml_manifest(payload: String) -> Response {
    let yaml_manifest = serde_yaml::from_str::<Manifest>(&payload);

    if yaml_manifest.is_err() {
        return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid manifest");
    }

    read_manifest(yaml_manifest.unwrap())
}

fn json_manifest(payload: String) -> Response {
    let json_manifest = serde_json::from_str::<Manifest>(&payload);

    if json_manifest.is_err() {
        return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid manifest");
    }

    read_manifest(json_manifest.unwrap())
}

fn read_manifest(manifest: Manifest) -> Response {
    let package = manifest.package;

    if package.is_none() {
        return Response::builder()
                .status(StatusCode::NO_CONTENT)
                .finish();
    }

    let manifest_package = package.unwrap();

    let keywords_valid = check_keywords(manifest_package.clone());

    if !keywords_valid {
        return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Magic keyword not provided");
    }

    let metadata = manifest_package.metadata;

    if metadata.is_none() {
        return Response::builder()
                .status(StatusCode::NO_CONTENT)
                .finish();
    }

    let orders = metadata.unwrap().try_into::<Orders>();

    if orders.is_err() {
        return Response::builder()
                .status(StatusCode::NO_CONTENT)
                .finish();
    }

    let mut toy_orders_found = 0;
    let mut toy_order_list: Vec<ToyOrder> = vec![];
    orders.unwrap().orders
        .into_iter()
        .for_each(|o| {
            let current_order = o.try_into::<ToyOrder>();
            if current_order.is_ok() {
                toy_order_list.push(current_order.unwrap());
                toy_orders_found += 1;
            }
        });

    if toy_orders_found == 0 {
        return Response::builder()
                .status(StatusCode::NO_CONTENT)
                .finish();
    }

    let toy_orders = toy_order_list
                        .into_iter()
                        .map(|to| format!("{}: {}", to.item, to.quantity))
                        .collect::<Vec<_>>()
                        .join("\n");

    Response::builder()
        .status(StatusCode::OK)
        .body(format!("{}", toy_orders))
}

fn check_keywords(package: Package) -> bool {
    if package.keywords.is_none() {
        return false;
    }

    let keywords = package.keywords.unwrap().as_local();

    if keywords.is_none() || !keywords.unwrap().into_iter().any(|kw| kw == "Christmas 2024") {
        return false;
    }

    true
}