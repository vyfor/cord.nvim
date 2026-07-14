fn main() {
    let content = std::fs::read_to_string(".github/server-metadata.txt").unwrap();
    let version = content.split('|').next().unwrap_or(&content).trim();
    println!("cargo:rustc-env=CORD_VERSION={}", version);
}
