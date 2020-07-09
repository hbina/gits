fn main() {
    let result = std::process::Command::new("git")
        .args(&["-c","color.status=always","status"])
        .output()
        .expect("failed to execute process")
        .stdout;

    println!(
        "{}",
        String::from_utf8(result).expect("unable to parse string.")
    );
}
