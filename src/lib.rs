//! Zero dependency `std::sync` based bidirectional channels. Each side can send
//! and receive with its counterpart.
//!
//! Note, the default `Channel` inherits `!Sync` from `std::sync::mpsc::Receiver`. If you
//! would prefer, a `crossbeam` implementation is available by enabling the `crossbeam` flag. In
//! addition to its desirable performance characteristics, it also drops this `!Sync` constraint.
//!
//! ## Getting Started
//!
//! ```toml
//! bichannel = "1"
//! ```
//!
//! **Example Usage**
//!
//! ```
//! let (left, right) = bichannel::channel();
//!
//! // Send from the left to the right
//! left.send(1).unwrap();
//! assert_eq!(Ok(1), right.recv());
//!
//! // Send from the right to the left
//! right.send(2).unwrap();
//! assert_eq!(Ok(2), left.recv());
//! ```
//!
//! ## License
//! TODO MIT/APACHE
//!
//! ## Contributing
//!  
//! Bug reports, feature requests, and contributions are warmly welcomed.
//!
//! NOTE: This README uses [cargo-readme](https://github.com/livioribeiro/cargo-readme). To
//! update the README, use `cargo readme > README.md`

#[cfg(not(feature = "crossbeam"))]
use std::sync::mpsc::{channel as create_channel, Receiver, Sender};

#[cfg(not(feature = "crossbeam"))]
pub use std::sync::mpsc::RecvError;
#[cfg(not(feature = "crossbeam"))]
pub use std::sync::mpsc::SendError;
#[cfg(not(feature = "crossbeam"))]
pub use std::sync::mpsc::TryRecvError;
#[cfg(not(feature = "crossbeam"))]
pub use std::sync::mpsc::TrySendError;

#[cfg(feature = "crossbeam")]
use crossbeam_channel::{unbounded as create_channel, Receiver, Sender};

#[cfg(feature = "crossbeam")]
pub use crossbeam_channel::RecvError;
#[cfg(feature = "crossbeam")]
pub use crossbeam_channel::SendError;
#[cfg(feature = "crossbeam")]
pub use crossbeam_channel::TryRecvError;
#[cfg(feature = "crossbeam")]
pub use crossbeam_channel::TrySendError;

/// One side of a bidirectional channel. This channel can send to and receive from its
/// counterpart.
///
/// # Examples
///
/// ```
/// let (l, r) = bichannel::channel();
///  
/// l.send(1).unwrap();
/// assert_eq!(Ok(1), r.recv());
///
/// r.send(1).unwrap();
/// assert_eq!(Ok(1), l.recv());
///
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "crossbeam", derive(Clone))]
pub struct Channel<S, R> {
    sender: Sender<S>,
    receiver: Receiver<R>,
}

impl<S, R> Channel<S, R> {
    /// See mpsc::Sender::send
    ///
    /// Attempts to send a value to the other side of this channel, returning it back if it could
    /// not be sent.
    ///
    /// A successful send occurs when it is determined that the other end of
    /// the channel has not hung up already. An unsuccessful send would be one
    /// where the corresponding receiver has already been deallocated. Note
    /// that a return value of [`Err`] means that the data will never be
    /// received, but a return value of [`Ok`] does *not* mean that the data
    /// will be received. It is possible for the corresponding receiver to
    /// hang up immediately after this function returns [`Ok`].
    ///
    /// This method will never block the current thread.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let (l, r) = bichannel::channel();
    ///
    /// // This send is always successful
    /// l.send(1).unwrap();
    ///
    /// // This send will fail because the receiver is gone
    /// drop(l);
    /// assert_eq!(r.send(1).unwrap_err().0, 1);
    /// ```
    pub fn send(&self, s: S) -> Result<(), SendError<S>> {
        self.sender.send(s)
    }

    /// See mpsc::Receiver::recv
    ///
    /// Attempts to wait for a value from the other side, returning an error if the
    /// other side has hung up.
    ///
    /// This function will always block the current thread if there is no data
    /// available and it's possible for more data to be sent. Once a message is
    /// sent from the other side then this will wake up and return that message.
    ///
    /// If the corresponding channel has disconnected, or it disconnects while
    /// this call is blocking, this call will wake up and return [`Err`] to
    /// indicate that no more messages can ever be received on this channel.
    /// However, since channels are buffered, messages sent before the disconnect
    /// will still be properly received.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::thread;
    ///
    /// let (left, right) = bichannel::channel::<u8, u8>();
    ///
    /// let _result = thread::spawn(move || {
    ///     right.send(1u8).unwrap();
    /// }).join().unwrap();
    ///
    /// assert_eq!(Ok(1), left.recv());
    /// ```
    ///
    /// Buffering behavior:
    ///
    /// ```
    /// use std::sync::mpsc;
    /// use std::thread;
    /// use std::sync::mpsc::RecvError;
    ///
    /// let (send, recv) = mpsc::channel();
    /// let handle = thread::spawn(move || {
    ///     send.send(1u8).unwrap();
    ///     send.send(2).unwrap();
    ///     send.send(3).unwrap();
    ///     drop(send);
    /// });
    ///
    /// // wait for the thread to join so we ensure the sender is dropped
    /// handle.join().unwrap();
    ///
    /// assert_eq!(Ok(1), recv.recv());
    /// assert_eq!(Ok(2), recv.recv());
    /// assert_eq!(Ok(3), recv.recv());
    /// assert_eq!(Err(RecvError), recv.recv());
    /// ```
    pub fn recv(&self) -> Result<R, RecvError> {
        self.receiver.recv()
    }

    /// See mpsc::Receiver::try_recv.
    ///
    /// Attempts to return a pending value from the other side without blocking.
    ///
    /// This method will never block the caller in order to wait for data to
    /// become available. Instead, this will always return immediately with a
    /// possible option of pending data on the channel.
    ///
    /// This is useful for a flavor of "optimistic check" before deciding to
    /// block on a receiver.
    ///
    /// Compared with [`recv`], this function has two failure cases instead of one
    /// (one for disconnection, one for an empty buffer).
    ///
    /// [`recv`]: Self::recv
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// let (_, right) = bichannel::channel::<(), ()>();
    ///
    /// assert!(right.try_recv().is_err());
    /// ```
    pub fn try_recv(&self) -> Result<R, TryRecvError> {
        self.receiver.try_recv()
    }
}

/// Creates a bichannelrectional channel returning the left and right
/// sides. Each side can send and receive from its counterpart
///
/// # Examples
///
/// ```
/// let (left, right) = bichannel::channel::<&'static str, &'static str>();
///
/// left.send("ping").unwrap();
///
/// assert_eq!(right.recv().unwrap(), "ping");
/// ```
pub fn channel<T, U>() -> (Channel<T, U>, Channel<U, T>) {
    let (ls, lr) = create_channel();
    let (rs, rr) = create_channel();

    (
        Channel {
            sender: ls,
            receiver: rr,
        },
        Channel {
            sender: rs,
            receiver: lr,
        },
    )
}

#[cfg(test)]
mod examples {

    #[test]
    fn test_threaded_scenario() {
        let (thread, main) = crate::channel();

        let handle = std::thread::spawn(move || loop {
            match main.try_recv() {
                Ok("stop") => break "stopped",
                Err(crate::TryRecvError::Empty) => (),
                _ => main.send("cant stop").unwrap(),
            }
        });

        thread.send("slow down").unwrap();
        assert_eq!(thread.recv().unwrap(), "cant stop");

        thread.send("stop").unwrap();
        assert_eq!(handle.join().unwrap(), "stopped");
    }

    //    #[test]
    //    fn test_arc_scenario() {
    //        let (l, r) = crate::channel::<i8, i8>();
    //
    //        let wrapped = std::sync::Arc::new(l);
    //
    //        {
    //            let wrapped = wrapped.clone();
    //            std::thread::spawn(move || {
    //                wrapped.recv().unwrap();
    //            });
    //        }
    //
    //        wrapped.recv().unwrap();
    //    }
    //

    // fn test_fut_scenario()
}
