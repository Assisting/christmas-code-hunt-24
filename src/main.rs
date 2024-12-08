use poem::{get, Route};
use shuttle_poem::ShuttlePoem;

mod challenge0;
mod challenge1;

use challenge0::{hello_bird, seek_redirect};
use challenge1::{ipv4_encryption_reverser, ipv4_encryption_validation, ipv6_encryption_reverser, ipv6_encryption_validation};

#[shuttle_runtime::main]
async fn poem() -> ShuttlePoem<impl poem::Endpoint> {
    let app = Route::new()
                            .at("/2/dest", get(ipv4_encryption_validation))
                            .at("/2/key", get(ipv4_encryption_reverser))
                            .at("/2/v6/dest", get(ipv6_encryption_validation))
                            .at("/2/v6/key", get(ipv6_encryption_reverser))
                            .at("/-1/seek", get(seek_redirect))
                            .at("/", get(hello_bird));

    Ok(app.into())
}