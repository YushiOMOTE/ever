use regex::Regex;
use semver::Version;

fn is_match(pattern: &str, input: &str) {
    let regex = Regex::new(pattern).unwrap();
    assert!(regex.is_match(input), "regex check failed: `{}`", input);
}

#[test]
fn test_package_name() {
    assert_eq!("ever", ever::package_name!());
}

#[test]
fn test_package_version() {
    Version::parse(ever::package_version!()).unwrap();
}

#[test]
fn test_package_description() {
    assert_eq!(
        "Print the build information of your program with minimal boilerplate",
        ever::package_description!()
    );
}

#[test]
fn test_build_date() {
    chrono::DateTime::parse_from_str(ever::build_date!(), "%c %z").unwrap();
}

#[test]
fn test_build_mode() {
    match ever::build_mode!() {
        "debug" | "release" => {}
        e => panic!("{}", e),
    }
}

#[test]
fn test_build_commit_hash() {
    is_match(r"^[a-f0-9]+(-dirty)?$", ever::build_commit_hash!());
}

#[test]
fn test_build_dir() {
    let s = std::path::Path::new(ever::build_dir!());
    assert!(s.is_dir());
    assert_eq!(s.file_name().unwrap(), "ever");
}

#[test]
fn test_rustc_version() {
    let version = ever::rustc_version!();
    let mut tokens = version.splitn(2, " ");

    Version::parse(tokens.next().unwrap()).unwrap();
    is_match(
        r"^\([a-f0-9]+ [0-9]+-[0-9]+-[0-9]+\)$",
        tokens.next().unwrap(),
    );
}
