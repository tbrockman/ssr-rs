#![doc(html_logo_url = "https://raw.githubusercontent.com/Valerioageno/ssr-rs/main/logo.png")]

//! 
//! This crate aims to enable server side rendering on rust servers in the simplest and lightest way possible.
//!
//! The whole logic is stored inside the [render_to_string()](struct.Ssr.html) function.
//!
//!  # Gettin started
//! ```toml
//! [dependencies]
//! ssr_rs = "0.2.0"
//! ```
//!
//!  # Example
//!
//! ```no_run
//! use ssr_rs::Ssr;
//! use std::fs::read_to_string;
//!
//! fn main() {
//!    let source = read_to_string("./path/to/build.js").unwrap();
//!
//!    let html = Ssr::render_to_string(&source, "entryPoint", None);
//!    
//!    assert_eq!(html, "<!doctype html><html>...</html>".to_string());
//! }
//! ```
//! Check how to use it with actix, rocket, warp and other frameworks <a href="https://github.com/Valerioageno/ssr-rs/tree/main/examples" target="_blank">here</a>.

#[macro_use]
extern crate lazy_static;
mod ssr;

pub use ssr::Ssr;
