// mod simple {
//     use profile::profile;
//     #[profile(Copyable)]
//     #[derive(Clone)]
//     #[iso_default]
//     #[on(#[derive(Copy)])]
//     struct Base<T>(T);
// }

mod specified_base {
    use profile::profile;
    #[profile(Copyable)]
    #[derive(Clone)]
    #[iso_default]
    #[on(Base, #[derive(Copy)])]
    struct Base<T>(T);
}
// mod multiprofile {
//     use profile::profile;
//     #[profile(A B)]
//     #[iso_default]
//     #[derive(Clone)]
//     #[iso_default]
//     #[on(Base, #[derive(Copy)])]
//     #[on(Base, pub)]
//     struct Base;
// }

fn main() {}
