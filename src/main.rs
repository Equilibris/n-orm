use profile::profile;

#[profile(Copyable)]
#[iso_default]
#[derive(Clone)]
#[on(derive(Copy))]
struct Base<T>(T);

fn main() {
    println!("Hello, world!");
}
