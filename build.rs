fn main() {
    let version = std::env::var("CORD_VERSION").unwrap_or_else(|_| {
        let content = std::fs::read_to_string(".github/server-metadata.txt")
            .expect("server-metadata.txt not found");
        content
            .split('|')
            .next()
            .unwrap_or(&content)
            .trim()
            .to_string()
    });
    println!("cargo:rustc-env=CORD_VERSION={}", version);
}
