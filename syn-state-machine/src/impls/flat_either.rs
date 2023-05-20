use std::marker::PhantomData;

use crate::*;

macro_rules! flat_sum {
    ($name:ident, $source:ident, $a:ident; $($gen:ident, $pat:pat),+) => {
        pub struct $name<$($gen: Parsable,)+ A = SmOut<B>>(PhantomData<($($gen),+, A)>)
        where $(SmOut<$gen>: Into<A>),+ ;

        impl<$($gen: Parsable,)+ A> MappedParse for $name<$($gen,)+ A>
        where $(SmOut<$gen>: Into<A>),+
        {
            type Source = $source<$($gen),+>;

            type Output = A;
            type Error = SmErr<Self::Source>;

            fn map(
                src: SmOut<Self::Source>,
            ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
                use $source::*;

                Ok(match src { $($pat => $a.into()),+ })
            }

            fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
                src
            }
        }
    };
}

flat_sum!(FlatSum2, Sum2, a; B, Val0(a), C, Val1(a));
flat_sum!(FlatSum3, Sum3, a; B, Val0(a), C, Val1(a), D, Val2(a));
flat_sum!(FlatSum4, Sum4, a; B, Val0(a), C, Val1(a), D, Val2(a), E, Val3(a));
flat_sum!(FlatSum5, Sum5, a; B, Val0(a), C, Val1(a), D, Val2(a), E, Val3(a), F, Val4(a));
flat_sum!(FlatSum6, Sum6, a; B, Val0(a), C, Val1(a), D, Val2(a), E, Val3(a), F, Val4(a),   G, Val5(a));
flat_sum!(FlatSum7, Sum7, a; B, Val0(a), C, Val1(a), D, Val2(a), E, Val3(a), F, Val4(a),   G, Val5(a), H, Val6(a));
flat_sum!(FlatSum8, Sum8, a; B, Val0(a), C, Val1(a), D, Val2(a), E, Val3(a), F, Val4(a),   G, Val5(a), H, Val6(a), I, Val7(a));
flat_sum!(FlatSum9, Sum9, a; B, Val0(a), C, Val1(a), D, Val2(a), E, Val3(a), F, Val4(a),   G, Val5(a), H, Val6(a), I, Val7(a), J, Val8(a));
flat_sum!(FlatSum10, Sum10, a; B, Val0(a), C, Val1(a), D, Val2(a), E, Val3(a), F, Val4(a), G, Val5(a), H, Val6(a), I, Val7(a), J, Val8(a), K, Val9(a));
flat_sum!(FlatSum11, Sum11, a; B, Val0(a), C, Val1(a), D, Val2(a), E, Val3(a), F, Val4(a), G, Val5(a), H, Val6(a), I, Val7(a), J, Val8(a), K, Val9(a), L, Val10(a));
