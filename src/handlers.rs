use futures::Future;

pub trait Handler<Args> {
    type Output;
    type Future: Future<Output = Self::Output>;
    fn call(&mut self, args: Args) -> Self::Future;
}

impl<Func, Fut> Handler<()> for Func
where
    Func: FnMut() -> Fut + 'static,
    Fut: Future,
{
    type Output = Fut::Output;
    type Future = Fut;
    fn call(&mut self, _: ()) -> Self::Future {
        (self)()
    }
}

// 1 Argument
impl<Func, Arg1, Fut> Handler<(Arg1,)> for Func
where
    Func: FnMut(Arg1) -> Fut + 'static,
    Fut: Future,
{
    type Output = Fut::Output;
    type Future = Fut;
    fn call(&mut self, (arg1,): (Arg1,)) -> Self::Future {
        (self)(arg1)
    }
}

macro_rules! handler_func{
    ($($T: ident),*) => {
        impl<Func, $($T),+, Fut> Handler<($($T),+)> for Func where Func: FnMut($($T),+) -> Fut + 'static,
        Fut: Future,
        {
            type Output = Fut::Output;
            type Future = Fut;

            #[allow(non_snake_case)]
            fn call(&mut self, ($($T),+): ($($T),+)) -> Self::Future {
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
