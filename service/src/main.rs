/**
* Copyright (C) 2025 NaNdeva
*
* This program is free software: you can use, modify, and distribute it
* under the terms of the Server Side Public License, version 1, as
* published by MongoDB, Inc.
*
* This program is distributed WITHOUT ANY WARRANTY. See the
* Server Side Public License for more details.
*
* You should have received a copy of the SSPL along with this program.
* If not, see https://www.mongodb.com/licensing/server-side-public-license.
**/
mod prelude;

use axum::{routing::get, Router};
use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    return Ok(());
}

async fn hello() -> &'static str {
    "Hello, World!"
}
