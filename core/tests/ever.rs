use regex::Regex;
use std::process::Command;

#[test]
fn test_ever_never() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("ever-tests-noarg")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, world!\n");
}

fn is_match(pattern: &str, input: &str) {
    let regex = Regex::new(pattern).unwrap();
    assert!(regex.is_match(input), "regex check failed `{}`", input);
}

fn is_ever(info: &str) {
    let mut lines = info.lines();

    is_match(
        r"^ever-tests-(arg|noarg) [^\s]+ \(.+\): crate to test ever$",
        lines.next().unwrap(),
    );
    assert!(lines.next().unwrap().is_empty());

    is_match(r"date:\s+[^\s]*", lines.next().unwrap());
    is_match(r"commit:\s+[a-f0-9]+(-dirty)?$", lines.next().unwrap());
    is_match(r"user:\s+[^\s]*", lines.next().unwrap());
    is_match(r"host:\s+[^\s]*", lines.next().unwrap());
    is_match(r"builddir:\s+[^\s]*", lines.next().unwrap());
    is_match(
        r"rustc:\s+ [^\s]+(-nightly)? \([^\s]* \d+-\d+-\d+\)",
        lines.next().unwrap(),
    );
}

const PACKAGES: &[(&str, &str)] = &[
    ("ever-tests-noarg", "EVER"),
    ("ever-tests-arg", "MY_VERSION"),
];
const VALUES: &[&str] = &["1", "true"];

#[test]
fn test_ever() {
    for (package, env) in PACKAGES {
        for val in VALUES {
            let output = Command::new("cargo")
                .arg("run")
                .arg("-p")
                .arg(package)
                .env(env, val)
                .output()
                .unwrap();
            assert_eq!(output.status.code().unwrap(), 1);
            let info = String::from_utf8_lossy(&output.stdout).to_string();
            is_ever(&info);
        }
    }
}

#[test]
fn test_ever_dump_lock() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("ever-tests-noarg")
        .env("EVER", "dump_lock")
        .output()
        .unwrap();
    assert_eq!(output.status.code().unwrap(), 1);
    let lock = String::from_utf8_lossy(&output.stdout).to_string();
    let _: cargo_lock::Lockfile = lock.parse().unwrap();
}
