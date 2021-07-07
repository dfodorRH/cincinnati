#![allow(dead_code, unused_macros)] // Different tests use a different subset of functions

mod input_stream;
#[cfg(feature = "tokio-02")]
mod tokio_02_ext;
#[cfg(feature = "tokio-03")]
mod tokio_03_ext;
#[cfg(feature = "tokio")]
mod tokio_ext;
mod track_closed;
#[macro_use]
mod test_cases;

pub mod algos;
pub mod impls;

pub use self::{input_stream::InputStream, track_closed::TrackClosed};
pub use async_compression::Level;
pub use futures::{executor::block_on, pin_mut, stream::Stream};
pub use std::{future::Future, io::Result, iter::FromIterator, pin::Pin};
