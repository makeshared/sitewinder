use std::path::Path;
use std::process::Command;
use std::fs;

#[test]
fn hello_world_example_produces_expected_output() {
    let files_to_test = [
        "hello_world.html",
    ];

    // Delete any existing output files to ensure a clean test run
    for filename in files_to_test.iter() {
        let path = format!("examples/hello_world/{}", filename);
        if fs::metadata(&path).is_ok() {
            fs::remove_file(path).expect("Failed to remove existing output file");
        }
    }

    // Build the site using the example's input
    let webroot = std::fs::canonicalize(Path::new("examples/hello_world")).unwrap();
    let sitewinder = env!("CARGO_BIN_EXE_sitewinder");
    let output = Command::new(sitewinder)
        .args([webroot.to_str().unwrap()])
        .output()
        .expect("Failed to run sitewinder");

    assert!(output.status.success(), "sitewinder failed with status: {}", output.status);

    // Compare generated files to our reference files
    for filename in files_to_test.iter() {
        let generated = fs::read_to_string(format!("examples/hello_world/{}", filename)).unwrap();
        let expected = fs::read_to_string(format!("tests/fixtures/examples/hello_world/{}", filename)).unwrap();
        assert_eq!(generated, expected, "{} does not match reference", filename);
    }
}

#[test]
fn prev_next_example_produces_expected_output() {
    let files_to_test = [
        "post_1.html",
        "post_2.html",
        "post_3.html",
    ];

    // Delete any existing output files to ensure a clean test run
    for filename in files_to_test.iter() {
        let path = format!("examples/groups/{}", filename);
        if fs::metadata(&path).is_ok() {
            fs::remove_file(path).expect("Failed to remove existing output file");
        }
    }

    // Build the site using the example's input
    let webroot = std::fs::canonicalize(Path::new("examples/groups")).unwrap();
    let sitewinder = env!("CARGO_BIN_EXE_sitewinder");
    let output = Command::new(sitewinder)
        .args([webroot])
        .output()
        .expect("Failed to run sitewinder");

    assert!(output.status.success(), "sitewinder failed with status: {}", output.status);

    // Compare generated files to our reference files
    for filename in files_to_test.iter() {
        let generated = fs::read_to_string(format!("examples/groups/{}", filename)).unwrap();
        let expected = fs::read_to_string(format!("tests/fixtures/examples/groups/{}", filename)).unwrap();
        assert_eq!(generated, expected, "{} does not match reference", filename);
    }
}

#[test]
fn tags_example_produces_expected_output() {
    let files_to_test = [
        "post_1.html",
        "post_2.html",
        "post_3.html",
        "tags/2024.html",
        "tags/hiking.html",
        "tags/holidays.html",
        "tags/italy.html",
        "tags/posts.html",
        "tags/spain.html",
    ];

    // Delete any existing output files to ensure a clean test run
    for filename in files_to_test.iter() {
        let path = format!("examples/tags/{}", filename);
        if fs::metadata(&path).is_ok() {
            fs::remove_file(path).expect("Failed to remove existing output file");
        }
    }

    // Build the site using the example's input
    let webroot = std::fs::canonicalize(Path::new("examples/tags")).unwrap();
    let sitewinder = env!("CARGO_BIN_EXE_sitewinder");
    let output = Command::new(sitewinder)
        .args([webroot])
        .output()
        .expect("Failed to run sitewinder");

    assert!(output.status.success(), "sitewinder failed with status: {}", output.status);

    // Compare generated files to our reference files
    for filename in files_to_test.iter() {
        let generated = fs::read_to_string(format!("examples/tags/{}", filename)).unwrap();
        let expected = fs::read_to_string(format!("tests/fixtures/examples/tags/{}", filename)).unwrap();
        assert_eq!(generated, expected, "{} does not match reference", filename);
    }
}

#[test]
fn all_features_example_produces_expected_output() {
    let files_to_test = [
        "about.html",
        "denmark.html",
        "hongkong.html",
        "india.html",
        "italy.html",
        "parsi.html",
        "tags/asia.html",
        "tags/cantonese.html",
        "tags/coffee.html",
        "tags/culture.html",
        "tags/curries.html",
        "tags/denmark.html",
        "tags/dim sum.html",
        "tags/fusion.html",
        "tags/hong kong.html",
        "tags/india.html",
        "tags/italy.html",
        "tags/parsi.html",
        "tags/persia.html",
        "tags/smørrebrød.html",
        "tags/spices.html",
        "tags/street food.html",
    ];

    // Delete any existing output files to ensure a clean test run
    for filename in files_to_test.iter() {
        let path = format!("examples/full_site/{}", filename);
        if fs::metadata(&path).is_ok() {
            fs::remove_file(path).expect("Failed to remove existing output file");
        }
    }

    // Build the site using the example's input
    let webroot = std::fs::canonicalize(Path::new("examples/full_site")).unwrap();
    let sitewinder = env!("CARGO_BIN_EXE_sitewinder");
    let output = Command::new(sitewinder)
        .args([webroot])
        .output()
        .expect("Failed to run sitewinder");

    assert!(output.status.success(), "sitewinder failed with status: {}", output.status);

    // Compare generated files to our reference files
    for filename in files_to_test.iter() {
        let generated = fs::read_to_string(format!("examples/full_site/{}", filename)).unwrap();
        let expected = fs::read_to_string(format!("tests/fixtures/examples/full_site/{}", filename)).unwrap();
        assert_eq!(generated, expected, "{} does not match reference", filename);
    }
}
