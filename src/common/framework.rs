use super::fixed_width::FixedWidth;

/// # Example of use of pattern
/// ## Example of the use as required trait object to create a holder
/// ```
/// # use crate::mylib::common::framework::*;
/// # use core::cell::RefCell;
/// // Define a inner structure of the holder
/// // call to dispatch takes
/// struct HolderInner{
///     val: i32,
/// }
/// // Define a "Holder" structure that holders the observer trait object
/// struct Holder<'a>{
///     // This is a Ref to the Trait object dispatched via vtable
///     fp: &'a dyn Observer<i32, HolderInner>,
///     // This holds the inner value
///     inner: HolderInner,
/// }
/// // Define a struct that implements the Observer Trait
/// struct Impler {
///     int: RefCell<i32>,
/// }
/// // Impl the Trait for the Impler structure specialised to i32 and [HolderInner]
/// impl<'a> Observer<i32, HolderInner> for Impler {
///    fn dispatch(&self, sender_internals: &HolderInner, val: i32) {
///         // Specific implementation of the reaction on call
///        let _ = val;
///        self.int.replace(sender_internals.val);
///    }
/// }
/// // Use of the types
/// let mut a = Impler { int: RefCell::new(4) };
/// let b = Holder {
///     fp: &a,
///     inner: HolderInner { val: 6 },
/// };
/// // Holder can dispatch a value and  Ref to its inner
/// b.fp.dispatch(&&b.inner, 5);
/// assert_eq!(*a.int.borrow(), 6);
/// ```
/// ## Example with Holder option - Trait object not required for creation
/// ```
/// # use crate::mylib::common::framework::*;
/// # use core::cell::RefCell;
/// // Define a inner structure of the holder
/// struct HolderInner{
///     val: i32,
/// }
/// // Define a struct that implements the Observer Trait
/// struct Impler {
///     int: RefCell<i32>,
/// }
/// // Impl the Trait for the Impler structure specialised to i32 and [HolderInner]
/// impl<'a> Observer<i32, HolderInner> for Impler {
///    fn dispatch(&self, sender_internals: &HolderInner, val: i32) {
///         // Specific implementation of the reaction on call
///        let _ = val;
///        self.int.replace(sender_internals.val);
///    }
/// }
/// struct HolderOption<'a> {
///     // Holder holds now an Option to a Trait object
///     fp: Option<&'a dyn Observer<i32, HolderInner>>,
///     inner: HolderInner,
/// }
/// let mut b = HolderOption {
///     // Now we can create a Holder with None
///     fp: None,
///     inner: HolderInner { val: 5 },
/// };
/// #[should_panic]
/// // If now the trait object is used - this would panic
/// //b.fp.unwrap().dispatch(&&b.inner, 5);
/// match b.fp{
///     // Option should be unrolled with match
///     // None => Do nothing here
///     None => (),
///     // If is is Some it is safe to use
///     Some(fp) => fp.dispatch(&&b.inner, 5),
/// }
/// ```
pub trait Observer<T: FixedWidth, U> {
    fn dispatch(&self, sender_internals: &U, val: T);
}

pub struct SensorGroupInner<T: FixedWidth> {
    pub min: T,
    pub max: T,
}
impl<T> SensorGroupInner<T>
where
    T: FixedWidth + PartialOrd + Copy,
{
    pub fn get_max(&self) -> T {
        self.max
    }

    pub fn get_min(&self) -> T {
        self.min
    }
}

pub struct SensorInner<T: FixedWidth> {
    pub val: T,
}
