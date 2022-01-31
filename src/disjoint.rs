use core::fmt::Debug;
use core::num::NonZeroI32;
use core::ops::ControlFlow;
use core::ops::FromResidual;
use core::ops::Try;
use core::result::Result as CoreResult;
use enum_extra::NonZeroRepr;
use enum_extra_derive::NonZeroRepr;
use strum::EnumMetadata;
use strum_macros::EnumMetadata;

//
// We should probably just dustbin this implementation,
// I only leave it in for completeness.
// See lib.rs for more comments.
//

impl From<CoreResult<NonError, Error>> for Result {
    fn from(result: CoreResult<NonError, Error>) -> Result {
        match result {
            CoreResult::Ok(non_error) => Result { non_error },
            CoreResult::Err(error) => Result { error },
        }
    }
}

impl From<Result> for CoreResult<NonError, Error> {
    fn from(it: Result) -> Self {
        match NonZeroI32::new(unsafe { it.error_code }) {
            None => unsafe { CoreResult::Ok(it.non_error) },
            Some(e) if (/* TODO derive Error::FLOOR */ Error::InvalidArgument as i32 .. Error::COUNT as i32).contains(&e.into()) => {
                CoreResult::Err(unsafe { it.error })
            }
            _ => {
                CoreResult::Err(Error::ErrorCodeOutOfRange)
            }
        }
    }
}

impl Try for Result {
    type Output = NonError;
    type Residual = Error;

    fn branch(self) -> ControlFlow<Self::Residual, NonError> {
        match NonZeroI32::new(unsafe { self.error_code }) {
				None => { ControlFlow::Continue(unsafe { self.non_error}) }
				Some(e) if (/* TODO derive Error::FLOOR */ Error::InvalidArgument as i32 .. Error::COUNT as i32).contains(&e.into()) => {
					ControlFlow::Break(unsafe { self.error } )
				}
				_ => { ControlFlow::Break(Error::ErrorCodeOutOfRange) }
        	}
    }

    fn from_output(ok: NonError) -> Self {
        Result { non_error: ok }
    }
}

impl FromResidual<Error> for Result {
    fn from_residual(e: Error) -> Self {
        Result { error: e }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
union Result {
    non_error: NonError,
    error: Error,
    error_code: i32,
}

impl Debug for Result {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "error code: {}", unsafe { self.error_code })
    }
}

impl PartialEq for Result {
    fn eq(&self, other: &Self) -> bool {
        let me: i32 = unsafe { self.error_code };
        let it: i32 = unsafe { other.error_code };
        me == it
    }
}

#[derive(Copy, Clone, Debug, EnumMetadata)]
#[repr(i32)]
enum NonError {
    Ok = 0,
}

#[derive(Copy, Clone, Debug, NonZeroRepr, EnumMetadata)]
#[repr(i32)]
#[allow(clippy::enum_variant_names)]
enum Error {
    InvalidArgument = (NonError::Ok as i32) + 1_i32,
    InvalidCapability,
    IllegalOperation,
    RangeError,
    AlignmentError,
    FailedLookup,
    TruncatedMessage,
    DeleteFirst,
    RevokeFirst,
    NotEnoughMemory,
    /* This has been added */
    ErrorCodeOutOfRange,
    /* NumErrors has been moved out of the enum into a Error::COUNT */
}

fn _test_ok_1_() -> Result {
    // OK so it is a little more verbose than Ok(Result::Ok?)
    Result {
        non_error: Result {
            non_error: NonError::Ok,
        }?,
    }
}

#[test]
fn _test_ok_2_() -> Result {
    Result {
        non_error: Result::from(CoreResult::Ok(NonError::Ok))?,
    }
}
fn _test_invalid_argument_() -> Result {
    Result {
        non_error: Result {
            error: Error::InvalidArgument,
        }?,
    }
}

fn _test_invalid_capability_() -> Result {
    Result {
        non_error: Result {
            error: Error::InvalidCapability,
        }?,
    }
}

fn _test_illegal_operation_() -> Result {
    Result {
        non_error: Result {
            error: Error::IllegalOperation,
        }?,
    }
}

fn _test_range_error_() -> Result {
    Result {
        non_error: Result {
            error: Error::RangeError,
        }?,
    }
}

fn _test_alignment_error_() -> Result {
    Result {
        non_error: Result {
            error: Error::AlignmentError,
        }?,
    }
}

fn _test_failed_lookup_() -> Result {
    Result {
        non_error: Result {
            error: Error::FailedLookup,
        }?,
    }
}

fn _test_truncated_message_() -> Result {
    Result {
        non_error: Result {
            error: Error::TruncatedMessage,
        }?,
    }
}

fn _test_delete_first_() -> Result {
    Result {
        non_error: Result {
            error: Error::DeleteFirst,
        }?,
    }
}

fn _test_revoke_first_() -> Result {
    Result {
        non_error: Result {
            error: Error::RevokeFirst,
        }?,
    }
}

fn _test_not_enough_memory_() -> Result {
    Result {
        non_error: Result {
            error: Error::NotEnoughMemory,
        }?,
    }
}

fn _test_error_code_out_of_range_() -> Result {
    Result {
        non_error: Result {
            error: Error::ErrorCodeOutOfRange,
        }?,
    }
}

fn _test_error_code_out_of_range_really_() -> Result {
    Result {
        non_error: Result {
            error_code: Error::ErrorCodeOutOfRange as i32 + 1,
        }?,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::process::Termination;
    impl Termination for Result {
        fn report(self) -> i32 {
            unsafe { self.error_code }
        }
    }
    #[test]
    fn test_ok_1() -> Result {
        _test_ok_1_()
    }

    #[test]
    fn test_ok_2() -> Result {
        _test_ok_2_()
    }

    #[test]
    fn test_invalid_argument() {
        assert_eq!(
            _test_invalid_argument_(),
            Result {
                error: Error::InvalidArgument
            }
        );
    }

    #[test]
    fn test_invalid_capability() {
        assert_eq!(
            _test_invalid_capability_(),
            Result {
                error: Error::InvalidCapability
            }
        );
    }

    #[test]
    fn test_illegal_operation() {
        assert_eq!(
            _test_illegal_operation_(),
            Result {
                error: Error::IllegalOperation
            }
        );
    }
    #[test]
    fn test_range_error() {
        assert_eq!(
            _test_range_error_(),
            Result {
                error: Error::RangeError,
            }
        );
    }
    #[test]
    fn test_alignment_error() {
        assert_eq!(
            _test_alignment_error_(),
            Result {
                error: Error::AlignmentError,
            }
        );
    }
    #[test]
    fn test_failed_lookup() {
        assert_eq!(
            _test_failed_lookup_(),
            Result {
                error: Error::FailedLookup,
            }
        );
    }
    #[test]
    fn test_truncated_message() {
        assert_eq!(
            _test_truncated_message_(),
            Result {
                error: Error::TruncatedMessage,
            }
        );
    }
    #[test]
    fn test_delete_first() {
        assert_eq!(
            _test_delete_first_(),
            Result {
                error: Error::DeleteFirst
            }
        );
    }
    #[test]
    fn test_revoke_first() {
        assert_eq!(
            _test_revoke_first_(),
            Result {
                error: Error::RevokeFirst
            }
        );
    }
    #[test]
    fn test_not_enough_memory() {
        assert_eq!(
            _test_not_enough_memory_(),
            Result {
                error: Error::NotEnoughMemory
            }
        );
    }

    #[test]
    fn test_error_code_out_of_range() {
        assert_eq!(
            _test_error_code_out_of_range_(),
            Result {
                error: Error::ErrorCodeOutOfRange
            }
        );
    }

    #[test]
    fn test_error_code_out_of_range_really() {
        assert_eq!(
            _test_error_code_out_of_range_(),
            Result {
                error: Error::ErrorCodeOutOfRange
            }
        );
    }
}
