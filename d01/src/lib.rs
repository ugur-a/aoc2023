use core::marker::PhantomData;

struct WeirdNumber<P>(u32, PhantomData<P>);

pub mod p1;
pub mod p2;
