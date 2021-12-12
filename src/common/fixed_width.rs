/// Marker Trait for embedded used fixed width types
/// Acts as a bound for generic parameters that should
/// be in the range of 8 to 32Bit signed and unsigned
pub trait FixedWidth: Copy + PartialOrd {}

impl FixedWidth for u8 {}
impl FixedWidth for u16 {}
impl FixedWidth for u32 {}
impl FixedWidth for i32 {}
impl FixedWidth for i16 {}
impl FixedWidth for i8 {}
