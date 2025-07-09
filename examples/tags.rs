//! Example: Runs the sitewinder binary to generate a set of three HTML pages, each
// sporting a tag cloud - a collection of links to tagged pages - and then opens
// one of the pages in the platform's default web browser.
//
// Run with: cargo run --example tags
//
// This example extends hello_world by demonstrating the use of { tags "<markup>" }
// blocks and the .sgtag template type.
//

use std::path::Path;
use std::process::Command;
use webbrowser::open;

fn main() {
    let webroot = std::fs::canonicalize(Path::new("examples/tags")).unwrap();

    // Use cargo to run the sitewinder binary, this ensures that the binary is built and
    // that we don't have to worry about the binary's location or potential file extension.
    Command::new("cargo")
        .args(["run", "--bin", "sitewinder", "--"])
        .arg(webroot)
        .status()
        .expect("Failed to run sitewinder");

    // Open the first HTML file in the default web browser
    if let Err(e) = open("examples/tags/post_1.html") {
        eprintln!("Failed to open page in browser: {}", e);
        std::process::exit(1);
    }
}
