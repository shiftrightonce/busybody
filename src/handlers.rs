pub trait Handler<Args> {
    // type Output;
    fn call(&self, args: Args);
}

impl<Func> Handler<()> for Func
where
    Func: Fn(),
{
    fn call(&self, _arg1: ()) {
        (self)();
    }
}

impl<Func, Arg1> Handler<(Arg1,)> for Func
where
    Func: Fn(Arg1),
{
    fn call(&self, (arg1,): (Arg1,)) {
        (self)(arg1);
    }
}

impl<Func, Arg1, Arg2> Handler<(Arg1, Arg2)> for Func
where
    Func: Fn(Arg1, Arg2),
{
    fn call(&self, (arg1, arg2): (Arg1, Arg2)) {
        (self)(arg1, arg2);
    }
}
