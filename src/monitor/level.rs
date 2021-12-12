use crate::common::fixed_width::FixedWidth;

/// Simple "level" compare logic - Level is derived from the typical use case
/// Monitoring threshold levels for being over or under
pub mod level_simple {
    /// Level return Type
    /// Must be [PartialEq] so it can be compared with ==
    /// Must be [Clone] and [Copy] so work with Cell and is trivial for a simple numeric enum
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum LevelState {
        OVER = 0,
        UNDER = 1,
    }
    /// Check function for a given value to a limit
    /// # Example
    /// ```
    /// # use mylib::monitor::level::level_simple::*;
    /// let limit = 300;
    /// assert_eq!(check(300, limit), LevelState::UNDER);
    /// assert_eq!(check(301, limit), LevelState::OVER);
    /// ```
    pub fn check<T>(val: T, limit: T) -> LevelState
    where
        T: super::FixedWidth,
    {
        if val > limit {
            LevelState::OVER
        } else {
            LevelState::UNDER
        }
    }
    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_over() {
            assert_eq!(check(5, 5), LevelState::UNDER);
        }
        #[test]
        fn test_under() {
            assert_eq!(check(5, 4), LevelState::OVER);
        }
    }
}
/// Builds level logic with hystersis on top of [level_simple]
pub mod level_hyst {

    use super::level_simple;
    use super::level_simple::LevelState;
    use crate::common::{fixed_width::FixedWidth, framework::*};
    use core::cell::Cell;

    /// Object that monitors a level with a given hysteresis
    /// Reasoning for cell and interior mutability: Even in single threaded context this is a strong contract
    /// that this is the only place a value is able to be modified. It is not possible to get Ref to a Cell
    /// so you can be sure that no other part of your code is able to modify that value.
    #[derive(Debug, Clone)]
    pub struct LevelHyst<T>
    where
        T: FixedWidth,
    {
        // Must be a Cell to provide interior mutability for Observer pattern
        level_state: Cell<LevelState>,
        // Limit that leads to state change
        limit: T,
        // Hysteresis that leads to state change
        hyst: T,
    }

    impl<T> LevelHyst<T>
    where
        T: FixedWidth,
    {
        /// Builds a new [LevelHyst] structure from the given arguments
        /// # Example
        /// ```
        /// # use mylib::monitor::level::{level_simple::*, level_hyst::*};
        /// let mut level = LevelHyst::new(100, 150, LevelState::UNDER);
        /// assert_eq!(level.get_state(), LevelState::UNDER);
        ///
        /// ```
        pub fn new(set: T, reset: T, initial: LevelState) -> Self {
            if set < reset {
                Self {
                    level_state: Cell::new(initial),
                    limit: reset,
                    hyst: set,
                }
            } else {
                Self {
                    level_state: Cell::new(initial),
                    limit: set,
                    hyst: reset,
                }
            }
        }
        /// Sets internal [LevelState] to given value
        /// # Example
        /// ```
        /// # use mylib::monitor::level::{level_simple::*, level_hyst::*};
        /// let mut level = LevelHyst::new(100, 150, LevelState::UNDER);
        /// assert_eq!(level.get_state(), LevelState::UNDER);
        /// level.set_state(LevelState::OVER);
        /// assert_eq!(level.get_state(), LevelState::OVER);
        /// ```
        pub fn set_state(&self, state: LevelState) {
            self.level_state.replace(state);
        }
        /// Checks the given value against the limit and hysteresis
        /// # Example
        /// ```
        /// # use mylib::monitor::level::{level_simple::*, level_hyst::*};
        /// let mut level = LevelHyst::new(100, 150, LevelState::OVER);
        /// assert_eq!(level.check(100), LevelState::UNDER);
        /// assert_eq!(level.check(150), LevelState::UNDER);
        /// assert_eq!(level.check(151), LevelState::OVER);
        /// ```
        pub fn check(&self, val: T) -> LevelState {
            let a = match self.level_state.get() {
                LevelState::OVER => level_simple::check(val, self.hyst),
                LevelState::UNDER => level_simple::check(val, self.limit),
            };
            self.level_state.replace(a);
            a
        }
        /// Returns internal state variable of type [LevelState]
        pub fn get_state(&self) -> LevelState {
            self.level_state.get()
        }
    }
    /// Implementing Observer for [LevelHyst]
    impl<T> Observer<T, SensorGroupInner<T>> for LevelHyst<T>
    where
        T: FixedWidth,
    {
        fn dispatch(&self, sender: &SensorGroupInner<T>, val: T) {
            self.level_state.replace(self.check(sender.get_max()));
            let _ = val;
        }
    }

    impl<T> Observer<T, SensorInner<T>> for LevelHyst<T>
    where
        T: FixedWidth,
    {
        fn dispatch(&self, sender: &SensorInner<T>, val: T) {
            self.level_state.replace(self.check(sender.val));
            let _ = val;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_overlimit() {
            let dut = LevelHyst::new(100, 70, LevelState::UNDER);

            assert_eq!(dut.check(100), LevelState::UNDER);
            assert_eq!(dut.level_state.get(), LevelState::UNDER);

            assert_eq!(dut.check(101), LevelState::OVER);
            assert_eq!(dut.level_state.get(), LevelState::OVER);

            assert_eq!(dut.check(71), LevelState::OVER);
            assert_eq!(dut.level_state.get(), LevelState::OVER);

            assert_eq!(dut.check(70), LevelState::UNDER);
            assert_eq!(dut.level_state.get(), LevelState::UNDER);
        }

        #[test]
        fn test_underlimit() {
            let dut = LevelHyst::new(70, 100, LevelState::OVER);

            assert_eq!(dut.check(71), LevelState::OVER);
            assert_eq!(dut.level_state.get(), LevelState::OVER);

            assert_eq!(dut.check(70), LevelState::UNDER);
            assert_eq!(dut.level_state.get(), LevelState::UNDER);

            assert_eq!(dut.check(100), LevelState::UNDER);
            assert_eq!(dut.level_state.get(), LevelState::UNDER);

            assert_eq!(dut.check(101), LevelState::OVER);
            assert_eq!(dut.level_state.get(), LevelState::OVER);
        }
        #[test]
        fn test_observer_pattern() {
            let dut = LevelHyst::new(70, 100, LevelState::OVER);

            struct TestSensorGroup<'a> {
                fp: &'a dyn Observer<i32, SensorGroupInner<i32>>,
                inner: SensorGroupInner<i32>,
            }

            let mut test = TestSensorGroup {
                fp: &dut,
                inner: SensorGroupInner { max: 5, min: 5 },
            };
            // test holds a mut ref to dut
            // no other ref on dut can be made as long as test holds a mut ref
            //
            //assert_eq!(dut.get_state(), LevelState::OVER);

            test.fp.dispatch(&&test.inner, 5);
            assert_eq!(dut.get_state(), LevelState::UNDER);
            test.inner.max = 101;
            test.fp.dispatch(&&test.inner, 5);
            assert_eq!(dut.get_state(), LevelState::OVER);
        }
    }
}
