#![cfg_attr(not(feature = "std"), no_std)]

#[macro_export]
macro_rules! derive_test {
    ($type:ty: $($trait:ident),* $(,)?) => {
        paste::paste! {
            $(
                #[test]
                fn [<test_ $type:snake _derives_ $trait:snake>]() {
                    derive_test!(@ident_map $trait, $type);
                }
            )*
        }
    };

    (@assert $type:ty: $trait:path) => {
        // This function will only compile if $type implements $trait
        fn assert_implements_trait<T: $trait>() {}
        assert_implements_trait::<$type>();
    };

    // Map derive macro names to their full trait paths.
    (@ident_map Clone, $type:ty) => {
        derive_test!(@assert $type: core::clone::Clone);
    };
    (@ident_map Copy, $type:ty) => {
        derive_test!(@assert $type: core::marker::Copy);
    };
    (@ident_map Debug, $type:ty) => {
        derive_test!(@assert $type: core::fmt::Debug);
    };
    (@ident_map Default, $type:ty) => {
        derive_test!(@assert $type: core::default::Default);
    };
    (@ident_map Eq, $type:ty) => {
        derive_test!(@assert $type: core::cmp::Eq);
    };
    (@ident_map Hash, $type:ty) => {
        derive_test!(@assert $type: core::hash::Hash);
    };
    (@ident_map Ord, $type:ty) => {
        derive_test!(@assert $type: core::cmp::Ord);
    };
    (@ident_map PartialEq, $type:ty) => {
        derive_test!(@assert $type: core::cmp::PartialEq);
    };
    (@ident_map PartialOrd, $type:ty) => {
        derive_test!(@assert $type: core::cmp::PartialOrd);
    };

    (@ident_map Deserialize, $type:ty) => {
        #[cfg(feature = "serde")]
        derive_test!(@assert $type: serde::Deserialize<'static>);
    };
    (@ident_map Serialize, $type:ty) => {
        #[cfg(feature = "serde")]
        derive_test!(@assert $type: serde::Serialize);
    };
}

#[cfg(test)]
mod core_tests {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Foo {
        x: usize,
        y: usize,
    }

    derive_test!(
        Foo: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd
    );

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum FooEnum {
        #[default]
        A,
        #[expect(dead_code, reason = "It's just for testing...")]
        B,
    }

    derive_test!(
        FooEnum: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd
    );
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct Foo {
        x: usize,
        y: usize,
    }

    derive_test!(Foo: Serialize, Deserialize);

    #[derive(Deserialize, Serialize)]
    enum FooEnum {
        A,
        B,
    }

    derive_test!(FooEnum: Serialize, Deserialize);
}
