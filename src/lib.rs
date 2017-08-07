#[macro_export]
macro_rules! bld {
    (@setters $Builder:ident: optional {
        $($no:ident)*
    } ;$($rest:tt)*) => {};
    (@setters $Builder:ident: optional {
        $($no:ident)*
    } ($name:ident: $ty:ty) $(($na:ident: $ta:ty))*;$(($nb:ident: $tb:ty))*) => {
        #[allow(non_camel_case_types)]
        impl<$($nb,)* $($na,)*> $Builder<$($nb,)* (), $($na,)*> {
            fn $name<$name: Into<$ty>>(self, $name: $name)
                                       -> $Builder<$($nb,)* $ty, $($na,)*> {
                $Builder {
                    $($nb: self.$nb,)*
                    $name: $name.into(),
                    $($na: self.$na,)*
                    $($no: self.$no,)*
                }
            }
        }

        bld!(@setters $Builder: optional {
            $($no)*
        } $(($na: $ta))*; $(($nb: $tb))* ($name: $ty));
    };
    ($Builder:ident: required {
        $($nr:ident: $tr:ty),*
    } optional {
        $($no:ident: $to:ty),*
    } $($full_methods:tt)*) => {
        #[allow(non_camel_case_types)]
        struct $Builder<$($nr=()),*> {
            $($nr: $nr,)*
            $($no: Option<$to>,)*
        }

        impl Default for $Builder {
            fn default() -> Self {
                $Builder {
                    $($nr: (),)*
                    $($no: None,)*
                }
            }
        }

        bld!(@setters $Builder: optional { $($no)* } $(($nr: $tr))*;);

        #[allow(non_camel_case_types)]
        impl<$($nr),*> $Builder<$($nr,)*> {
            $(fn $no<$no: Into<$to>>(self, $no: $no) -> Self {
                $Builder { $no: Some($no.into()), ..self }
            })*
        }

        impl $Builder<$($tr),*> {
            $($full_methods)*
        }
    }
}
