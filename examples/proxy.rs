#![feature(conservative_impl_trait)]

extern crate salt;
extern crate hyper;

use salt::prelude::*;
use hyper::Client;

// Handlers can return a `salt::Response` or an `impl Future<Item = salt::Response>` (
// which `impl FutureResponse` is an alias for). A `Future<Item = hyper::Response>` is
// coercible into a `salt::Response` (which is what is happening in this handler).
fn proxy_google(ctx: Context) -> impl FutureResponse {
    // Proxy google and stream the response back to the client
    // This could easily be expanded into a simple http-proxy
    Client::new(&ctx).get("http://www.google.com".parse().unwrap())
}

fn main() {
    let mut s = Salt::default();

    s.add((Method::Get, "/", proxy_google));

    s.run("127.0.0.1:7878");
}