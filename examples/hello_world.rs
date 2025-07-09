//! Example: Runs the sitewinder binary to generate a simple HTML page, and then opens the
// generated page in the platform's default web browser.
//
// Run with: cargo run --example hello_world
//
// This simple example demonstrates the use of { include "<path>" } and { title } blocks
// to build pages.
//
// The page template and include files can be found in the hello_world folder.
//
// The generated hello_world.html file will be placed alongside the hello_world.sgpage
// template that was used to produce it.
//

use std::path::Path;
use std::process::Command;
use webbrowser::open;

fn main() {
    let webroot = std::fs::canonicalize(Path::new("examples/hello_world")).unwrap();

    // Run sitewinder to produce hello_world.html.
    // Use cargo to run the sitewinder binary, this ensures that the binary is built and
    // that we don't have to worry about the binary's location or potential file extension.
    Command::new("cargo")
        .args(["run", "--bin", "sitewinder", "--"])
        .arg(webroot)
        .status()
        .expect("Failed to run sitewinder");

    // Open hello_world.html in the default web browser
    if let Err(e) = open("examples/hello_world/hello_world.html") {
        eprintln!("Failed to open page in browser: {}", e);
        std::process::exit(1);
    }
}
