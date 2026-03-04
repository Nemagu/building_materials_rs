//! Building materials library for structural design according to Russian standards.
//!
//! Provides types and design properties for:
//! - [`concrete`] — concrete classification (compression, tension, frost resistance, water resistance)
//!   based on GOST and SP requirements
//! - [`rebar`] — reinforcing bar classes and surface profiles per GOST 34028-2016
//! - [`standard`] — normative design characteristics for materials per specific codes,
//!   currently supporting SP 63.13330.2018 (concrete and reinforced concrete structures)
//!
//! # Feature flags
//! - `serde` — enables serialization/deserialization of all types via [`serde`]

pub mod concrete;
pub mod rebar;
pub mod standard;
