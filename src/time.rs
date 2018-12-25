pub struct Seconds<T>(pub T);
pub struct MilliSeconds<T>(pub T);
pub struct Ticks<T>(pub T);

pub trait Unit
where
    Self: core::marker::Sized,
{
    fn s(self) -> Seconds<Self>;
    fn ms(self) -> MilliSeconds<Self>;
    fn ticks(self) -> Ticks<Self>;
}

impl Unit for u32 {
    fn s(self) -> Seconds<Self> {
        Seconds(self)
    }

    fn ms(self) -> MilliSeconds<Self> {
        MilliSeconds(self)
    }

    fn ticks(self) -> Ticks<Self> {
        Ticks(self)
    }
}
