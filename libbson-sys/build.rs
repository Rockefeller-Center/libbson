fn main() {
    let dst = cmake::Config::new("mongo-c-driver")
        .configure_arg("-DENABLE_AUTOMATIC_INIT_AND_CLEANUP=OFF")
        .configure_arg("-DENABLE_EXAMPLES=OFF")
        .configure_arg("-DENABLE_MONGOC=OFF")
        .configure_arg("-DENABLE_TESTS=OFF")
        .build();

    println!("cargo:rustc-link-lib=static=bson-static-1.0");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
}
