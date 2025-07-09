//! Example: Runs the sitewinder binary to generate a set of three HTML pages with prev/next
// links between them, and then opens the first page in the sequence in the platform's
// default web browser.
//
// Run with: cargo run --example groups
//
// This example extends hello_world by demonstrating the use of { group "<path>" } blocks
// and metadata to generate prev/next links on each page.
//

use std::path::Path;
use std::process::Command;
use webbrowser::open;

fn main() {
    let webroot = std::fs::canonicalize(Path::new("examples/groups")).unwrap();

    // Use cargo to run the sitewinder binary, this ensures that the binary is built and
    // that we don't have to worry about the binary's location or potential file extension.
    Command::new("cargo")
        .args(["run", "--bin", "sitewinder", "--"])
        .arg(webroot)
        .status()
        .expect("Failed to run sitewinder");

    // Open the first HTML file in the default web browser
    if let Err(e) = open("examples/groups/post_1.html") {
        eprintln!("Failed to open page in browser: {}", e);
        std::process::exit(1);
    }
}
