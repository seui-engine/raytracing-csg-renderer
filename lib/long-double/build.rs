fn main() {
    cc::Build::new()
        .file("long_double.c")
        .compile("long_double");
}
