//^
//^ HEAD
//^

//> HEAD -> CORE
use core::range::{
    Range,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive
};

//> HEAD -> ENUM_DISPATCH
use enum_dispatch::enum_dispatch;


//^
//^ SPAN
//^

//> SPAN -> ENUM
#[enum_dispatch(RangeBounds)]
#[derive(Debug)]
pub enum Span {
    Range(Range<usize>),
    RangeFrom(RangeFrom<usize>),
    RangeFull,
    RangeInclusive(RangeInclusive<usize>),
    RangeTo(RangeTo<usize>),
    RangeToInclusive(RangeToInclusive<usize>)
}

//> SPAN -> DEFAULT
const impl Default for Span {
    fn default() -> Self {return Span::RangeFull(..)}
}