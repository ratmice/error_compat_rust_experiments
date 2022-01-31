use core::fmt::Debug;
use core::num::NonZeroI32;
use core::ops::ControlFlow;
use core::ops::FromResidual;
use core::ops::Try;
use core::result::Result as CoreResult;
use strum::EnumCount;
use strum_macros::EnumCount;

/*
 * Not certain we would want this?
 */
impl From<CoreResult<(), Error>> for Error {
    fn from(result: CoreResult<(), Error>) -> Error {
        match result {
            CoreResult::Ok(()) => Error::NoError,
            CoreResult::Err(error) => error,
        }
    }
}

impl From<Error> for CoreResult<(), Error> {
    fn from(it: Error) -> Self {
        match NonZeroI32::new(it as i32) {
            None => CoreResult::Ok(()),
            Some(_) => CoreResult::Err(it),
        }
    }
}

impl Try for Error {
    type Output = ();
    type Residual = Error;

    fn branch(self) -> ControlFlow<Self::Residual, ()> {
        match NonZeroI32::new(self as i32) {
            None => ControlFlow::Continue(()),
            Some(_) => ControlFlow::Break(self),
        }
    }

    fn from_output((): ()) -> Self {
        Error::NoError
    }
}

impl FromResidual<Error> for Error {
    fn from_residual(e: Error) -> Self {
        e
    }
}

#[derive(Copy, Clone, Debug, EnumCount, Eq, PartialEq)]
#[repr(i32)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
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
    /* NumErrors is not actually needed in rust use Error::COUNT
     * if you add anything you'll need to replace the previous last error
     * with the newly added one in the test_count() test below.
     */
}

#[test]
fn test_count() {
    assert_eq!(Error::COUNT as usize, Error::NotEnoughMemory as usize + 1)
}

/*
 * I do not believe that we would want
 */
/*
impl From<()> for Error {
  fn from((): ()) -> Error {
     Error::NoError
  }
}

// Because using it hides that this returns NoError.
fn _test_ok_3_() -> Error {
    Error::from(Error::from(())?)
}
*/

fn _test_ok_1_() -> Error {
    let () = Error::NoError?;
    Error::NoError
}

#[test]
fn _test_ok_2_() -> Error {
    let () = Error::from(CoreResult::Ok(()))?;
    Error::NoError
}
fn _test_invalid_argument_() -> Error {
    Error::InvalidArgument?;
    Error::NoError
}

fn _test_invalid_capability_() -> Error {
    Error::InvalidCapability?;
    Error::NoError
}

fn _test_illegal_operation_() -> Error {
    Error::IllegalOperation?;
    Error::NoError
}

fn _test_range_error_() -> Error {
    Error::RangeError?;
    Error::NoError
}

fn _test_alignment_error_() -> Error {
    Error::AlignmentError?;
    Error::NoError
}

fn _test_failed_lookup_() -> Error {
    Error::FailedLookup?;
    Error::NoError
}

fn _test_truncated_message_() -> Error {
    Error::TruncatedMessage?;
    Error::NoError
}

fn _test_delete_first_() -> Error {
    Error::DeleteFirst?;
    Error::NoError
}

fn _test_revoke_first_() -> Error {
    Error::RevokeFirst?;
    Error::NoError
}

fn _test_not_enough_memory_() -> Error {
    Error::NotEnoughMemory?;
    Error::NoError
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
    fn test_ok_2() -> Error {
        _test_ok_2_()
    }

    #[test]
    fn test_invalid_argument() {
        assert_eq!(_test_invalid_argument_(), Error::InvalidArgument);
    }

    #[test]
    fn test_invalid_capability() {
        assert_eq!(_test_invalid_capability_(), Error::InvalidCapability);
    }

    #[test]
    fn test_illegal_operation() {
        assert_eq!(_test_illegal_operation_(), Error::IllegalOperation);
    }
    #[test]
    fn test_range_error() {
        assert_eq!(_test_range_error_(), Error::RangeError,);
    }
    #[test]
    fn test_alignment_error() {
        assert_eq!(_test_alignment_error_(), Error::AlignmentError,);
    }
    #[test]
    fn test_failed_lookup() {
        assert_eq!(_test_failed_lookup_(), Error::FailedLookup,);
    }
    #[test]
    fn test_truncated_message() {
        assert_eq!(_test_truncated_message_(), Error::TruncatedMessage,);
    }
    #[test]
    fn test_delete_first() {
        assert_eq!(_test_delete_first_(), Error::DeleteFirst);
    }
    #[test]
    fn test_revoke_first() {
        assert_eq!(_test_revoke_first_(), Error::RevokeFirst);
    }
    #[test]
    fn test_not_enough_memory() {
        assert_eq!(_test_not_enough_memory_(), Error::NotEnoughMemory);
    }
}
