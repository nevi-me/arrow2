//! Definition of basic div operations with primitive arrays
use std::ops::Div;

use num_traits::{CheckedDiv, NumCast, Zero};

use crate::datatypes::DataType;
use crate::{
    array::{Array, PrimitiveArray},
    compute::{
        arithmetics::{ArrayCheckedDiv, ArrayDiv, NotI128},
        arity::{binary, binary_checked, unary, unary_checked},
    },
    error::{ArrowError, Result},
    types::NativeType,
};
use strength_reduce::{
    StrengthReducedU16, StrengthReducedU32, StrengthReducedU64, StrengthReducedU8,
};

/// Divides two primitive arrays with the same type.
/// Panics if the divisor is zero of one pair of values overflows.
///
/// # Examples
/// ```
/// use arrow2::compute::arithmetics::basic::div::div;
/// use arrow2::array::Int32Array;
///
/// let a = Int32Array::from(&[Some(10), Some(6)]);
/// let b = Int32Array::from(&[Some(5), Some(6)]);
/// let result = div(&a, &b).unwrap();
/// let expected = Int32Array::from(&[Some(2), Some(1)]);
/// assert_eq!(result, expected)
/// ```
pub fn div<T>(lhs: &PrimitiveArray<T>, rhs: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>>
where
    T: NativeType + Div<Output = T>,
{
    if lhs.data_type() != rhs.data_type() {
        return Err(ArrowError::InvalidArgumentError(
            "Arrays must have the same logical type".to_string(),
        ));
    }

    binary(lhs, rhs, lhs.data_type().clone(), |a, b| a / b)
}

/// Checked division of two primitive arrays. If the result from the division
/// overflows, the result for the operation will change the validity array
/// making this operation None
///
/// # Examples
/// ```
/// use arrow2::compute::arithmetics::basic::div::checked_div;
/// use arrow2::array::Int8Array;
///
/// let a = Int8Array::from(&[Some(-100i8), Some(10i8)]);
/// let b = Int8Array::from(&[Some(100i8), Some(0i8)]);
/// let result = checked_div(&a, &b).unwrap();
/// let expected = Int8Array::from(&[Some(-1i8), None]);
/// assert_eq!(result, expected);
/// ```
pub fn checked_div<T>(lhs: &PrimitiveArray<T>, rhs: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>>
where
    T: NativeType + CheckedDiv<Output = T> + Zero,
{
    if lhs.data_type() != rhs.data_type() {
        return Err(ArrowError::InvalidArgumentError(
            "Arrays must have the same logical type".to_string(),
        ));
    }

    let op = move |a: T, b: T| a.checked_div(&b);

    binary_checked(lhs, rhs, lhs.data_type().clone(), op)
}

// Implementation of ArrayDiv trait for PrimitiveArrays
impl<T> ArrayDiv<PrimitiveArray<T>> for PrimitiveArray<T>
where
    T: NativeType + Div<Output = T> + NotI128,
{
    type Output = Self;

    fn div(&self, rhs: &PrimitiveArray<T>) -> Result<Self::Output> {
        div(self, rhs)
    }
}

// Implementation of ArrayCheckedDiv trait for PrimitiveArrays
impl<T> ArrayCheckedDiv<PrimitiveArray<T>> for PrimitiveArray<T>
where
    T: NativeType + CheckedDiv<Output = T> + Zero + NotI128,
{
    type Output = Self;

    fn checked_div(&self, rhs: &PrimitiveArray<T>) -> Result<Self::Output> {
        checked_div(self, rhs)
    }
}

/// Divide a primitive array of type T by a scalar T.
/// Panics if the divisor is zero.
///
/// # Examples
/// ```
/// use arrow2::compute::arithmetics::basic::div::div_scalar;
/// use arrow2::array::Int32Array;
///
/// let a = Int32Array::from(&[None, Some(6), None, Some(6)]);
/// let result = div_scalar(&a, &2i32);
/// let expected = Int32Array::from(&[None, Some(3), None, Some(3)]);
/// assert_eq!(result, expected)
/// ```
pub fn div_scalar<T>(lhs: &PrimitiveArray<T>, rhs: &T) -> PrimitiveArray<T>
where
    T: NativeType + Div<Output = T> + NumCast,
{
    let rhs = *rhs;
    match T::DATA_TYPE {
        DataType::UInt64 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<u64>>().unwrap();
            let rhs = rhs.to_u64().unwrap();

            let reduced_div = StrengthReducedU64::new(rhs);
            // Safety: we just proved that `lhs` is `PrimitiveArray<u64>` which means that
            // T = u64
            unsafe {
                std::mem::transmute::<PrimitiveArray<u64>, PrimitiveArray<T>>(unary(
                    lhs,
                    |a| a / reduced_div,
                    lhs.data_type().clone(),
                ))
            }
        }
        DataType::UInt32 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<u32>>().unwrap();
            let rhs = rhs.to_u32().unwrap();

            let reduced_div = StrengthReducedU32::new(rhs);
            // Safety: we just proved that `lhs` is `PrimitiveArray<u32>` which means that
            // T = u32
            unsafe {
                std::mem::transmute::<PrimitiveArray<u32>, PrimitiveArray<T>>(unary(
                    lhs,
                    |a| a / reduced_div,
                    lhs.data_type().clone(),
                ))
            }
        }
        DataType::UInt16 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<u16>>().unwrap();
            let rhs = rhs.to_u16().unwrap();

            let reduced_div = StrengthReducedU16::new(rhs);
            // Safety: we just proved that `lhs` is `PrimitiveArray<u16>` which means that
            // T = u16
            unsafe {
                std::mem::transmute::<PrimitiveArray<u16>, PrimitiveArray<T>>(unary(
                    lhs,
                    |a| a / reduced_div,
                    lhs.data_type().clone(),
                ))
            }
        }
        DataType::UInt8 => {
            let lhs = lhs.as_any().downcast_ref::<PrimitiveArray<u8>>().unwrap();
            let rhs = rhs.to_u8().unwrap();

            let reduced_div = StrengthReducedU8::new(rhs);
            // Safety: we just proved that `lhs` is `PrimitiveArray<u8>` which means that
            // T = u8
            unsafe {
                std::mem::transmute::<PrimitiveArray<u8>, PrimitiveArray<T>>(unary(
                    lhs,
                    |a| a / reduced_div,
                    lhs.data_type().clone(),
                ))
            }
        }
        _ => unary(lhs, |a| a / rhs, lhs.data_type().clone()),
    }
}

/// Checked division of a primitive array of type T by a scalar T. If the
/// divisor is zero then the validity array is changed to None.
///
/// # Examples
/// ```
/// use arrow2::compute::arithmetics::basic::div::checked_div_scalar;
/// use arrow2::array::Int8Array;
///
/// let a = Int8Array::from(&[Some(-100i8)]);
/// let result = checked_div_scalar(&a, &100i8);
/// let expected = Int8Array::from(&[Some(-1i8)]);
/// assert_eq!(result, expected);
/// ```
pub fn checked_div_scalar<T>(lhs: &PrimitiveArray<T>, rhs: &T) -> PrimitiveArray<T>
where
    T: NativeType + CheckedDiv<Output = T> + Zero,
{
    let rhs = *rhs;
    let op = move |a: T| a.checked_div(&rhs);

    unary_checked(lhs, op, lhs.data_type().clone())
}

// Implementation of ArrayDiv trait for PrimitiveArrays with a scalar
impl<T> ArrayDiv<T> for PrimitiveArray<T>
where
    T: NativeType + Div<Output = T> + NotI128 + NumCast,
{
    type Output = Self;

    fn div(&self, rhs: &T) -> Result<Self::Output> {
        Ok(div_scalar(self, rhs))
    }
}

// Implementation of ArrayCheckedDiv trait for PrimitiveArrays with a scalar
impl<T> ArrayCheckedDiv<T> for PrimitiveArray<T>
where
    T: NativeType + CheckedDiv<Output = T> + Zero + NotI128,
{
    type Output = Self;

    fn checked_div(&self, rhs: &T) -> Result<Self::Output> {
        Ok(checked_div_scalar(self, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array::*;

    #[test]
    fn test_div_mismatched_length() {
        let a = Int32Array::from_slice(&[5, 6]);
        let b = Int32Array::from_slice(&[5]);
        div(&a, &b)
            .err()
            .expect("should have failed due to different lengths");
    }

    #[test]
    fn test_div() {
        let a = Int32Array::from(&[Some(5), Some(6)]);
        let b = Int32Array::from(&[Some(5), Some(6)]);
        let result = div(&a, &b).unwrap();
        let expected = Int32Array::from(&[Some(1), Some(1)]);
        assert_eq!(result, expected);

        // Trait testing
        let result = a.div(&b).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_div_panic() {
        let a = Int8Array::from(&[Some(10i8)]);
        let b = Int8Array::from(&[Some(0i8)]);
        let _ = div(&a, &b);
    }

    #[test]
    fn test_div_checked() {
        let a = Int32Array::from(&[Some(5), None, Some(3), Some(6)]);
        let b = Int32Array::from(&[Some(5), Some(3), None, Some(6)]);
        let result = checked_div(&a, &b).unwrap();
        let expected = Int32Array::from(&[Some(1), None, None, Some(1)]);
        assert_eq!(result, expected);

        let a = Int32Array::from(&[Some(5), None, Some(3), Some(6)]);
        let b = Int32Array::from(&[Some(5), Some(0), Some(0), Some(6)]);
        let result = checked_div(&a, &b).unwrap();
        let expected = Int32Array::from(&[Some(1), None, None, Some(1)]);
        assert_eq!(result, expected);

        // Trait testing
        let result = a.checked_div(&b).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_div_scalar() {
        let a = Int32Array::from(&[None, Some(6), None, Some(6)]);
        let result = div_scalar(&a, &1i32);
        let expected = Int32Array::from(&[None, Some(6), None, Some(6)]);
        assert_eq!(result, expected);

        // Trait testing
        let result = a.div(&1i32).unwrap();
        assert_eq!(result, expected);

        // check the strength reduced branches
        let a = UInt64Array::from(&[None, Some(6), None, Some(6)]);
        let result = div_scalar(&a, &1u64);
        let expected = UInt64Array::from(&[None, Some(6), None, Some(6)]);
        assert_eq!(result, expected);

        let a = UInt32Array::from(&[None, Some(6), None, Some(6)]);
        let result = div_scalar(&a, &1u32);
        let expected = UInt32Array::from(&[None, Some(6), None, Some(6)]);
        assert_eq!(result, expected);

        let a = UInt16Array::from(&[None, Some(6), None, Some(6)]);
        let result = div_scalar(&a, &1u16);
        let expected = UInt16Array::from(&[None, Some(6), None, Some(6)]);
        assert_eq!(result, expected);

        let a = UInt8Array::from(&[None, Some(6), None, Some(6)]);
        let result = div_scalar(&a, &1u8);
        let expected = UInt8Array::from(&[None, Some(6), None, Some(6)]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_div_scalar_checked() {
        let a = Int32Array::from(&[None, Some(6), None, Some(6)]);
        let result = checked_div_scalar(&a, &1i32);
        let expected = Int32Array::from(&[None, Some(6), None, Some(6)]);
        assert_eq!(result, expected);

        let a = Int32Array::from(&[None, Some(6), None, Some(6)]);
        let result = checked_div_scalar(&a, &0);
        let expected = Int32Array::from(&[None, None, None, None]);
        assert_eq!(result, expected);

        // Trait testing
        let result = a.checked_div(&0).unwrap();
        assert_eq!(result, expected);
    }
}
