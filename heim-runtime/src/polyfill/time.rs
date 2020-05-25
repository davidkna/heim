use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use futures_timer::Delay;
use futures_util::{future::Future, stream::Stream};

/// Naive interval stream implementation.
///
/// Resets `delay` each time when it fires.
#[derive(Debug)]
pub struct Interval {
    interval: Duration,
    delay: Delay,
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if Pin::new(&mut self.delay).poll(cx).is_pending() {
            return Poll::Pending;
        }
        let interval = self.interval;
        self.delay.reset(interval);
        Poll::Ready(Some(()))
    }
}

pub fn interval(duration: Duration) -> Interval {
    Interval {
        interval: duration,
        delay: Delay::new(duration),
    }
}