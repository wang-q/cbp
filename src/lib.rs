//! Cross-platform Binary Package manager library
//!
//! This library provides core functionality for the CBP package manager:
//! - Directory structure management
//! - Package installation and removal
//! - File operations and utilities

pub mod libs;

pub use crate::libs::dirs::CbpDirs;
pub use crate::libs::utils::*;
