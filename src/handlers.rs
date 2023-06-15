use futures::Future;

// pub trait Handler<Args> {
//     fn call(&self, args: Args);
// }

// ----
pub trait Handler<Args>: Clone + 'static {
    type Output;
    type Future: Future<Output = Self::Output>;
    fn call(&self, args: Args) -> Self::Future;
}

// ---

// impl<Func> Handler<()> for Func
// where
//     Func: Fn(),
// {
//     fn call(&self, _: ()) {
//         (self)();
//     }
// }

impl<Func, Fut> Handler<()> for Func
where
    Func: Fn() -> Fut + Clone + 'static,
    Fut: Future,
{
    type Output = Fut::Output;
    type Future = Fut;
    fn call(&self, _: ()) -> Self::Future {
        (self)()
    }
}

// 1 Argument
/// `handle_func` macro is expanding to this but for
/// 2 or more arguments
// impl<Func, Arg1> Handler<(Arg1,)> for Func
// where
//     Func: Fn(Arg1),
// {
//     fn call(&self, (arg1,): (Arg1,)) {
//         (self)(arg1);
//     }
// }

impl<Func, Arg1, Fut> Handler<(Arg1,)> for Func
where
    Func: Fn(Arg1) -> Fut + Clone + 'static,
    Fut: Future,
{
    type Output = Fut::Output;
    type Future = Fut;
    fn call(&self, (arg1,): (Arg1,)) -> Self::Future {
        (self)(arg1)
    }
}

// macro_rules! handler_func{
//     ($($T: ident),*) => {
//         impl<Func, $($T),+> Handler<($($T),+)> for Func where Func: Fn($($T),+),
//         {
//             #[allow(non_snake_case)]
//             fn call(&self, ($($T),+): ($($T),+)) {
//                 (self)($($T),+);
//             }
//         }
//     };
// }

macro_rules! handler_func{
    ($($T: ident),*) => {
        impl<Func, $($T),+, Fut> Handler<($($T),+)> for Func where Func: Fn($($T),+) -> Fut + Clone + 'static,
        Fut: Future,
        {
            type Output = Fut::Output;
            type Future = Fut;

            #[allow(non_snake_case)]
            fn call(&self, ($($T),+): ($($T),+)) -> Self::Future {
                (self)($($T),+)
            }
        }
    };
}

handler_func! {Arg1, Arg2 }
handler_func! {Arg1, Arg2, Arg3 }
handler_func! {Arg1, Arg2, Arg3, Arg4 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16 }
handler_func! {Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17 }
