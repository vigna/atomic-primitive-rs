/// Forward a method to an inherent method
macro_rules! forward {
    () => {};
    (fn $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; $($rest:tt)*) => {
        #[doc = forward_doc!($method)]
        #[inline]
        fn $method(self $( , $arg : $ty )* ) -> $ret {
            Self::$method(self $( , $arg )* )
        }
        forward! { $($rest)* }
    };
    (fn $method:ident ( &self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; $($rest:tt)*) => {
        #[doc = forward_doc!($method)]
        #[inline]
        fn $method(&self $( , $arg : $ty )* ) -> $ret {
            Self::$method(self $( , $arg )* )
        }
        forward! { $($rest)* }
    };
    (fn $method:ident ( &mut self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; $($rest:tt)*) => {
        #[doc = forward_doc!($method)]
        #[inline]
        fn $method(&mut self $( , $arg : $ty )* ) -> $ret {
            Self::$method(self $( , $arg )* )
        }
        forward! { $($rest)* }
    };
    (fn $method:ident ( $( $arg:ident : $ty:ty ),* ) -> $ret:ty ; $($rest:tt)*) => {
        #[doc = forward_doc!($method)]
        #[inline]
        fn $method($( $arg : $ty ),* ) -> $ret {
            Self::$method($( $arg ),* )
        }
        forward! { $($rest)* }
    };
}

/// A string suitable for method `#[doc = ...]`
macro_rules! forward_doc {
    ($method:ident) => {
        concat!(
            "See the inherent [`",
            stringify!($method),
            "`][Self::",
            stringify!($method),
            "] method."
        )
    };
}
