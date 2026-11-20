// examples/versions/main.rs : `versions`

fn main() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),);
}
