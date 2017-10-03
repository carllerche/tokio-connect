extern crate futures;
extern crate tokio_io;

use futures::Future;
use tokio_io::{AsyncRead, AsyncWrite};

/// Create asynchronous byte streams.
pub trait Connect {
    type Connected: AsyncRead + AsyncWrite;
    type Error;
    type Future: Future<Item = Self::Connected, Error = Self::Error>;

    fn connect(&self) -> Self::Future;
}
