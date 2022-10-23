use chrono::prelude::*;
use vergen::{Config, vergen, ShaKind};

fn main() {
    /*
      build info:
        version (1.0.0)
        commit (hash)
        build timestamp (time)

    */
    let dt = Local::now();
    println!("cargo:rustc-env=BUILD_INFO={}", dt.format("%Y-%m-%dT%H:%M:%S%.f%:z"));

    let mut config = Config::default();

    // Change the SHA output to the short variant
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    vergen(config).expect("kjlkjl");
}
