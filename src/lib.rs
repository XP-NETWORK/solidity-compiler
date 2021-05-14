#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod consts;
pub mod generators;

#[rustfmt::skip]
mod script_templates;

#[cfg(test)]
mod tests;
