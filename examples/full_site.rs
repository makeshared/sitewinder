//! Example: Runs the sitewinder binary to generate a full-featured site, demonstrating all
// the features supported by sitewinder.
//
// Run with: cargo run --example full_site
//
// The page template and include files can be found in the full_site folder.
//

use std::path::Path;
use std::process::Command;
use webbrowser::open;

fn main() {
    let webroot = std::fs::canonicalize(Path::new("examples/full_site")).unwrap();

    // Use cargo to run the sitewinder binary, this ensures that the binary is built and
    // that we don't have to worry about the binary's location or potential file extension.
    Command::new("cargo")
        .args(["run", "--bin", "sitewinder", "--"])
        .arg(webroot)
        .status()
        .expect("Failed to run sitewinder");

    // Open the index file in the user's default web browser
    if let Err(e) = open("examples/full_site/italy.html") {
        eprintln!("Failed to open page in browser: {}", e);
        std::process::exit(1);
    }
}
