mod simple {
    use profile::profile;
    #[profile(Copyable)]
    #[iso(#[derive(Clone)])]
    #[on(#[derive(Copy)])]
    struct Base<T>(T);
}

mod specified_base {
    use profile::profile;
    #[profile(Copyable)]
    #[derive(Clone)]
    #[iso_toggle]
    #[on(Base, #[derive(Copy)])]
    struct Base<T>(T);
}
mod multiprofile {
    use profile::profile;
    #[profile(A B)]
    #[iso_toggle]
    #[derive(Clone)]
    #[iso_toggle]
    #[on(Base, #[derive(Copy)])]
    #[on(Base, pub)]
    struct Base;
}
mod classic {
    use profile::profile;

    #[profile(A)]
    #[iso_toggle]
    #[derive(Clone, PartialEq)]
    #[iso_toggle]
    #[on(#[derive(Eq)])]
    struct Base<T> {
        b: T,
    }
}

fn main() {}
