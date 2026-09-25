macro_rules! welp_text {
    () => {
        "This syntax style was not previously used in the C headers of the official OpenFX \
         specification. `openfx-datagen`'s preprocessor should be updated."
    };
}

macro_rules! welp {
    () => {
        todo!(crate::parsing::welp::welp_text!())
    };
}
macro_rules! welp_assert {
    ($($tt:tt)*) => {
        assert!($($tt)*, welp_text!())
    };
}
macro_rules! welp_assert_eq {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right, welp_text!())
    };
}

pub(crate) use welp;
pub(crate) use welp_assert;
pub(crate) use welp_assert_eq;
pub(crate) use welp_text;
