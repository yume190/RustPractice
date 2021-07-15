## diesel

[blog](https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104)

```bash
cargo install diesel_cli --no-default-features --features "postgres"

diesel setup

diesel migration generate create_posts
```

https://lankydan.dev/2018/05/20/creating-a-rusty-rocket-fuelled-with-diesel

## actix

## auto reload

cargo install cargo-watch

> cargo watch -x 'run --bin app'

## State

## Mut State

```rust
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            // Note: using app_data instead of data
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## scope

```rust
let scope = web::scope("/users").service(show_users);
App::new().service(scope);
```

## https://actix.rs/docs/server/

- Multi-threading
- ssl
- Keep-Alive
- Graceful shutdown

## input (extraction)

### Path

```rust
use actix_web::{get, web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

// web::Path<Info>
// web::Path((user_id, friend)): web::Path<(u32, String)>
```

### Query

> web::Query<Info>

### JSON

> web::Json<Info>

### Form

`application/x-www-form-urlencoded`
encoding: `chunked`

> web::Form<FormData>

### Multipart

### Streaming

```rust
use actix_web::{get, web, Error, HttpResponse};
use futures::StreamExt;

#[get("/")]
async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    Ok(HttpResponse::Ok().finish())
}
```
