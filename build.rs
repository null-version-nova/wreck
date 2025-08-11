use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/lib/");
    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=SDL3");

    let glfw_bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("GLFWwrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    glfw_bindings
        .write_to_file(out_path.join("GLFWbindings.rs"))
        .expect("Couldn't write GLFW bindings!");

    let sdl_bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("SDLwrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    sdl_bindings.write_to_file(out_path.join("SDLbindings.rs")).expect("Couldn't write SDL bindings")
}
