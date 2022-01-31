use core::convert::Infallible;
use core::fmt::Debug;
use core::num::NonZeroI32;
use core::ops::ControlFlow;
use core::ops::FromResidual;
use core::ops::Try;
use core::result::Result;
use strum::EnumCount;
use strum_macros::EnumCount;

use strum::EnumMetadata;
use strum_macros::EnumMetadata;

use enum_extra::NonZeroRepr;
use enum_extra_derive::NonZeroRepr;

// Not sure if we actually want this one.
impl From<Result<(), Error>> for Error {
    fn from(result: Result<(), Error>) -> Error {
        match result {
            Result::Ok(()) => Error::NoError,
            Result::Err(error) => error,
        }
    }
}

impl From<Error> for Result<(), Error> {
    fn from(it: Error) -> Self {
        match NonZeroI32::new(it as i32) {
            None => Result::Ok(()),
            Some(_) => Result::Err(it),
        }
    }
}

impl From<Error> for Result<(), JustError> {
    fn from(it: Error) -> Self {
        match NonZeroI32::new(it as i32) {
            // it as i32 == 0.
            None => Result::Ok(()),
            Some(error_code) => {
                let error_code: i32 = i32::from(error_code);
                // Safety:
                //
                // 1. All non-zero error_codes are valid and equivalent JustError codes.
                // 2. We've determined that the error_code is non-zero.
                Result::Err(unsafe { core::mem::transmute(error_code) })
            }
        }
    }
}

impl Try for Error {
    type Output = ();
    type Residual = Result<Infallible, JustError>;

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match NonZeroI32::new(self as i32) {
            // self as i32 == 0.
            None => ControlFlow::Continue(()),
            Some(error_code) => {
                let error_code: i32 = i32::from(error_code);
                ControlFlow::Break(
                    // Safety:
                    //
                    // 1. All non-zero error_codes are valid and equivalent JustError codes.
                    // 2. We've determined that the error_code is non-zero.
                    Err(unsafe { core::mem::transmute(error_code) }),
                )
            }
        }
    }

    fn from_output((): ()) -> Self {
        Error::NoError
    }
}

impl FromResidual<Result<Infallible, JustError>> for Error {
    fn from_residual(result: Result<Infallible, JustError>) -> Self {
        match result {
            // This should never happen because this is along the Continue branch.
            // But is required in accordance with the Expected laws.
            Ok(_) => Error::NoError,
            Err(just_error) => {
                let error_code = just_error as i32;
                // Safety: All JustError error codes are valid and equivalent Error error codes.
                unsafe { core::mem::transmute(error_code) }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, EnumCount, Eq, PartialEq)]
#[repr(i32)]
#[allow(clippy::enum_variant_names)]
enum Error {
    NoError = 0,
    InvalidArgument,
    InvalidCapability,
    IllegalOperation,
    RangeError,
    AlignmentError,
    FailedLookup,
    TruncatedMessage,
    DeleteFirst,
    RevokeFirst,
    NotEnoughMemory,
    // NumErrors is not actually needed in rust use Error::COUNT
    // if you add anything you'll need to replace the previous last error
    // with the newly added one in the test_count() test below.
}

// In theory I should just write a derive macro to derive this enum from
// Error without any duplication.  This would not be hard to do, I just have not done it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumMetadata, NonZeroRepr)]
#[repr(i32)]
enum JustError {
    InvalidArgument = 1,
    InvalidCapability,
    IllegalOperation,
    RangeError,
    AlignmentError,
    FailedLookup,
    TruncatedMessage,
    DeleteFirst,
    RevokeFirst,
    NotEnoughMemory,
    // NumErrors is not actually needed in rust use Error::COUNT
    // if you add anything you'll need to replace the previous last error
    // with the newly added one in the test_count() test below.
}

// Compile time check that ConcreteError is one less than Error.
const _: () = if Error::COUNT == JustError::COUNT + 1 {
} else {
    panic!()
};

#[test]
fn test_count() {
    assert_eq!(Error::COUNT as usize, Error::NotEnoughMemory as usize + 1)
}

fn _test_ok_1_() -> Error {
    let () = Error::NoError?;
    Error::NoError
}

#[test]
fn _test_ok_2_() -> Result<(), JustError> {
    Ok(Error::from(Result::Ok(()))?)
}
fn _test_invalid_argument_() -> Result<(), JustError> {
    Ok(Error::InvalidArgument?)
}

fn _test_invalid_capability_() -> Result<(), JustError> {
    Ok(Error::InvalidCapability?)
}

fn _test_illegal_operation_() -> Result<(), JustError> {
    Ok(Error::IllegalOperation?)
}

fn _test_range_error_() -> Result<(), JustError> {
    Ok(Error::RangeError?)
}

fn _test_alignment_error_() -> Result<(), JustError> {
    Ok(Error::AlignmentError?)
}

fn _test_failed_lookup_() -> Result<(), JustError> {
    Ok(Error::FailedLookup?)
}

fn _test_truncated_message_() -> Result<(), JustError> {
    Ok(Error::TruncatedMessage?)
}

fn _test_delete_first_() -> Result<(), JustError> {
    Ok(Error::DeleteFirst?)
}

fn _test_revoke_first_() -> Result<(), JustError> {
    Ok(Error::RevokeFirst?)
}

fn _test_not_enough_memory_() -> Result<(), JustError> {
    Ok(Error::NotEnoughMemory?)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::process::Termination;
    impl Termination for Error {
        fn report(self) -> i32 {
            self as i32
        }
    }
    #[test]
    fn test_ok_1() -> Error {
        _test_ok_1_()
    }

    #[test]
    fn test_ok_2() -> Result<(), JustError> {
        _test_ok_2_()
    }

    #[test]
    fn test_invalid_argument() {
        assert_eq!(_test_invalid_argument_(), Err(JustError::InvalidArgument));
    }

    #[test]
    fn test_invalid_capability() {
        assert_eq!(
            _test_invalid_capability_(),
            Err(JustError::InvalidCapability)
        );
    }

    #[test]
    fn test_illegal_operation() {
        assert_eq!(_test_illegal_operation_(), Err(JustError::IllegalOperation));
    }
    #[test]
    fn test_range_error() {
        assert_eq!(_test_range_error_(), Err(JustError::RangeError),);
    }
    #[test]
    fn test_alignment_error() {
        assert_eq!(_test_alignment_error_(), Err(JustError::AlignmentError),);
    }
    #[test]
    fn test_failed_lookup() {
        assert_eq!(_test_failed_lookup_(), Err(JustError::FailedLookup),);
    }
    #[test]
    fn test_truncated_message() {
        assert_eq!(_test_truncated_message_(), Err(JustError::TruncatedMessage),);
    }
    #[test]
    fn test_delete_first() {
        assert_eq!(_test_delete_first_(), Err(JustError::DeleteFirst));
    }
    #[test]
    fn test_revoke_first() {
        assert_eq!(_test_revoke_first_(), Err(JustError::RevokeFirst));
    }
    #[test]
    fn test_not_enough_memory() {
        assert_eq!(_test_not_enough_memory_(), Err(JustError::NotEnoughMemory));
    }
}
