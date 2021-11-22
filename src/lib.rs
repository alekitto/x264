//! An x264 wrapper, so that you can safely encode H.264 video.

#![cfg_attr(feature = "no_std", no_std)]
#![warn(missing_docs)]

extern crate x264_sys;

use x264_sys::x264;

mod colorspace;
mod data;
mod encoder;
mod error;
mod image;
mod picture;
mod setup;

pub use colorspace::*;
pub use data::*;
pub use encoder::*;
pub use error::*;
pub use image::*;
pub use picture::*;
pub use setup::*;

#[cfg(not(feature = "no_std"))]
pub(crate) use std::fmt;
#[cfg(not(feature = "no_std"))]
pub(crate) use std::marker::PhantomData;
#[cfg(not(feature = "no_std"))]
pub(crate) use std::mem;
#[cfg(not(feature = "no_std"))]
pub(crate) use std::ptr;
#[cfg(not(feature = "no_std"))]
pub(crate) use std::result;
#[cfg(not(feature = "no_std"))]
pub(crate) use std::slice;

#[cfg(feature = "no_std")]
pub(crate) use core::fmt;
#[cfg(feature = "no_std")]
pub(crate) use core::mem;
#[cfg(feature = "no_std")]
pub(crate) use core::ptr;
#[cfg(feature = "no_std")]
pub(crate) use core::result;
#[cfg(feature = "no_std")]
pub(crate) use core::slice;
#[cfg(feature = "no_std")]
pub(crate) use core::PhantomData;
