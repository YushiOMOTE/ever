#[test]
fn test_package_name() {
    assert_eq!("ever", ever::package_name!());
}

#[test]
fn test_package_version() {
    assert_eq!("0.1.0", ever::package_version!());
}

#[test]
fn test_package_description() {
    assert_eq!(
        "Print the build-time information of your program with minimal boilerplate",
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
fn test_build_dir() {
    let s = std::path::Path::new(ever::build_dir!());
    assert!(s.is_dir());
    assert_eq!(s.file_name().unwrap(), "ever");
}
