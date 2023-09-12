extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/rand.c")
        .compile("randmy.a");
}