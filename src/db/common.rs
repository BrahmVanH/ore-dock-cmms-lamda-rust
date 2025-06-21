//! Common utilities for DynamoDB table operations.
//!
//! This module contains shared functions and utilities used across
//! all table creation modules.

use core::fmt;
use crate::error::AppError;

/// Helper function to simplify error handling during DynamoDB resource creation.
///
/// This function wraps the builder pattern results with proper error context.
///
/// # Type Parameters
///
/// * `T` - The success result type from a builder operation
/// * `E` - The error type from a builder operation that implements Display
///
/// # Arguments
///
/// * `builder_result` - Result from a DynamoDB builder operation
/// * `context` - Error context to include in case of failure
///
/// # Returns
///
/// * `Result<T, AppError>` - The original success value or a DatabaseError with context
pub fn build<T, E>(builder_result: Result<T, E>, context: &str) -> Result<T, AppError>
    where E: fmt::Display
{
    builder_result.map_err(|e| AppError::DatabaseError(format!("{}: {:?}", context, e.to_string())))
}
