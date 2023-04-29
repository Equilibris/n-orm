#![feature(async_fn_in_trait)]
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
        a: i32,
        d: String,
        c: T,
    }
}

mod transforming {
    use profile::profile;

    #[profile(Product)]
    #[iso(#[derive(Debug)])]
    struct Base<T: Copy + std::ops::Mul<Output = T>> {
        a: T,
        b: T,

        #[on(Product)]
        #[transform(Base Product, a * b)]
        product: T,
    }
}
mod transforming_unnamed {
    use profile::profile;

    #[profile(Product)]
    #[iso(#[derive(Debug, PartialEq, Eq, Clone)])]
    struct Base<T: Copy + std::ops::Mul<Output = T>>(
        #[on(Product)]
        #[transform(Base Product, e1 * e2)]
        T,
        T,
        T,
    );

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn transforming() {
            let v = Base(1, 10);

            let z: Product<_> = v.clone().into();

            assert_eq!(z, Product(10, 1, 10));
            assert_eq!(v, Base::from(z));
        }
    }
}

mod collection {
    use collection::Document;
    use mongodb::bson::oid::ObjectId;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Serialize, Deserialize, Document)]
    #[coll(UserColl users)]
    #[coll(index(compound tag_name, sparse))]
    #[coll(option(collection_sharing))]
    struct User {
        #[serde(rename = "_id")]
        id: ObjectId,

        #[coll(index(single email, unique, type=Text))]
        email: String,

        #[coll(index(compound tag_name))]
        name: String,
        #[coll(index(compound tag_name))]
        tag: u16,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basic_tests() {
            todo!()
        }
    }
}

fn main() {}
