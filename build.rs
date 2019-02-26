

fn main() {

    println!("cargo:rustc-link-search=native=native_lib");
    println!("cargo:rustc-link-lib=static=native");

}



