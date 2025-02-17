// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Defines temporal kernels for time and date related functions.

use chrono::{Datelike, Timelike};

use crate::array::*;
use crate::datatypes::*;
use crate::error::{ArrowError, Result};
use crate::temporal_conversions::*;

use super::arity::unary;

/// Extracts the hours of a given temporal array as an array of integers
pub fn hour(array: &dyn Array) -> Result<PrimitiveArray<u32>> {
    let final_data_type = DataType::UInt32;
    match array.data_type() {
        DataType::Time32(TimeUnit::Second) => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i32>>()
                .unwrap();
            Ok(unary(array, |x| time32s_to_time(x).hour(), final_data_type))
        }
        DataType::Time32(TimeUnit::Microsecond) => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i32>>()
                .unwrap();
            Ok(unary(
                array,
                |x| time32ms_to_time(x).hour(),
                final_data_type,
            ))
        }
        DataType::Time64(TimeUnit::Microsecond) => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i64>>()
                .unwrap();
            Ok(unary(
                array,
                |x| time64us_to_time(x).hour(),
                final_data_type,
            ))
        }
        DataType::Time64(TimeUnit::Nanosecond) => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i64>>()
                .unwrap();
            Ok(unary(
                array,
                |x| time64ns_to_time(x).hour(),
                final_data_type,
            ))
        }
        DataType::Date32 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i32>>()
                .unwrap();
            Ok(unary(
                array,
                |x| date32_to_datetime(x).hour(),
                final_data_type,
            ))
        }
        DataType::Date64 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i64>>()
                .unwrap();
            Ok(unary(
                array,
                |x| date64_to_datetime(x).hour(),
                final_data_type,
            ))
        }
        DataType::Timestamp(time_unit, None) => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i64>>()
                .unwrap();
            let op = match time_unit {
                TimeUnit::Second => |x| timestamp_s_to_datetime(x).hour(),
                TimeUnit::Millisecond => |x| timestamp_ms_to_datetime(x).hour(),
                TimeUnit::Microsecond => |x| timestamp_us_to_datetime(x).hour(),
                TimeUnit::Nanosecond => |x| timestamp_ns_to_datetime(x).hour(),
            };
            Ok(unary(array, op, final_data_type))
        }
        dt => Err(ArrowError::NotYetImplemented(format!(
            "\"hour\" does not support type {:?}",
            dt
        ))),
    }
}

/// Checks if an array of type `datatype` can perform hour operation
///
/// # Examples
/// ```
/// use arrow2::compute::temporal::can_hour;
/// use arrow2::datatypes::{DataType, TimeUnit};
///
/// let data_type = DataType::Time32(TimeUnit::Second);
/// assert_eq!(can_hour(&data_type), true);

/// let data_type = DataType::Int8;
/// assert_eq!(can_hour(&data_type), false);
/// ```
pub fn can_hour(data_type: &DataType) -> bool {
    matches!(
        data_type,
        DataType::Time32(TimeUnit::Second)
            | DataType::Time32(TimeUnit::Microsecond)
            | DataType::Time64(TimeUnit::Microsecond)
            | DataType::Time64(TimeUnit::Nanosecond)
            | DataType::Date32
            | DataType::Date64
            | DataType::Timestamp(_, None)
    )
}

/// Extracts the hours of a given temporal array as an array of integers
pub fn year(array: &dyn Array) -> Result<PrimitiveArray<i32>> {
    let final_data_type = DataType::Int32;
    match array.data_type() {
        DataType::Date32 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i32>>()
                .unwrap();
            Ok(unary(
                array,
                |x| date32_to_datetime(x).year(),
                final_data_type,
            ))
        }
        DataType::Date64 => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i64>>()
                .unwrap();
            Ok(unary(
                array,
                |x| date64_to_datetime(x).year(),
                final_data_type,
            ))
        }
        DataType::Timestamp(time_unit, None) => {
            let array = array
                .as_any()
                .downcast_ref::<PrimitiveArray<i64>>()
                .unwrap();
            let op = match time_unit {
                TimeUnit::Second => |x| timestamp_s_to_datetime(x).year(),
                TimeUnit::Millisecond => |x| timestamp_ms_to_datetime(x).year(),
                TimeUnit::Microsecond => |x| timestamp_us_to_datetime(x).year(),
                TimeUnit::Nanosecond => |x| timestamp_ns_to_datetime(x).year(),
            };
            Ok(unary(array, op, final_data_type))
        }
        dt => Err(ArrowError::NotYetImplemented(format!(
            "\"year\" does not support type {:?}",
            dt
        ))),
    }
}

/// Checks if an array of type `datatype` can perform year operation
///
/// # Examples
/// ```
/// use arrow2::compute::temporal::can_year;
/// use arrow2::datatypes::{DataType};
///
/// let data_type = DataType::Date32;
/// assert_eq!(can_year(&data_type), true);

/// let data_type = DataType::Int8;
/// assert_eq!(can_year(&data_type), false);
/// ```
pub fn can_year(data_type: &DataType) -> bool {
    matches!(
        data_type,
        DataType::Date32 | DataType::Date64 | DataType::Timestamp(_, None)
    )
}
