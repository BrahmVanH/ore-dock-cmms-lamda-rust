//! Database module containing table definitions and database operations.
//!
//! This module is organized into separate files for different functional areas:
//! - `common.rs` - Shared utilities and helper functions
//! - `user_tables.rs` - User and access management tables
//! - `asset_tables.rs` - Asset management and maintenance tables
//! - `notification_tables.rs` - Notification and alerting system tables
//! - `security_tables.rs` - Security and permissions system tables
//! - `vendor_tables.rs` - Vendor and supply chain management tables
//! - `misc_tables.rs` - Miscellaneous system tables
//! - `ensure_table_exists.rs` - Main orchestration for table creation

pub mod init;
pub mod local;
pub mod connect;
pub mod common;
pub mod user_tables;
pub mod asset_tables;
pub mod notification_tables;
pub mod security_tables;
pub mod vendor_tables;
pub mod misc_tables;
pub mod ensure_table_exists;

// Re-export commonly used items
pub use common::build;
pub use ensure_table_exists::ensure_all_tables_exist;
