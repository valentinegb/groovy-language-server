use std::{env::current_exe, process::Command};

fn main() {
    let jar_path = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("groovy-language-server-all.jar");

    Command::new("java")
        .args(["-jar", jar_path.to_str().unwrap()])
        .spawn()
        .unwrap();
}
