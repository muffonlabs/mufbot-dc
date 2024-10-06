use std::env;

fn main() {
    // warn dev if not using sccache
    let rustc_wrapper =
        env::var("RUSTC_WRAPPER")
            .unwrap_or_default();
    if !rustc_wrapper
        .contains("sccache")
    {
        println!(
            "cargo::warning=Usage of sccache is highly recommended for fast builds. \
            Consider setting up sccache."
        );
    }
}
