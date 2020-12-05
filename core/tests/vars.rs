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
