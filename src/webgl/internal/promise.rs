use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub enum Promise<T>
where
    T: Clone,
{
    Resolved(T),
    PendingNone,
    PendingOne(Box<dyn FnOnce(T)>),
    PendingMany(Vec<Box<dyn FnOnce(T)>>),
}

impl<T> Promise<T>
where
    T: Clone,
{
    pub fn new() -> Promise<T> {
        Promise::PendingNone
    }

    pub fn resolve(&mut self, value: T) {
        let prev = std::mem::replace(self, Promise::PendingNone);
        match prev {
            Promise::Resolved(value) => {
                *self = Promise::Resolved(value);
                return;
            }
            Promise::PendingNone => {}
            Promise::PendingOne(callback) => callback(value.clone()),
            Promise::PendingMany(callbacks) => {
                for callback in callbacks {
                    callback(value.clone());
                }
            }
        }

        *self = Promise::Resolved(value);
    }

    pub fn then(&mut self, callback: Box<dyn FnOnce(T)>) {
        let prev = std::mem::replace(self, Promise::PendingNone);
        match prev {
            Promise::Resolved(value) => {
                callback(value.clone());
                *self = Promise::Resolved(value);
            }
            Promise::PendingNone => *self = Promise::PendingOne(callback),
            Promise::PendingOne(prev_callback) => {
                *self = Promise::PendingMany(vec![prev_callback, callback])
            }
            Promise::PendingMany(mut callbacks) => {
                callbacks.push(callback);
                *self = Promise::PendingMany(callbacks);
            }
        }
    }
}

impl<T> Unpin for Promise<T> where T: Clone {}

impl<T> Future for Promise<T>
where
    T: Clone,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<T> {
        let this = self.get_mut();
        let prev = std::mem::replace(this, Promise::PendingNone);

        let waker = cx.waker().clone();
        match prev {
            Promise::Resolved(value) => {
                let result = value.clone();
                *this = Promise::Resolved(value);
                return Poll::Ready(result);
            }
            Promise::PendingNone => *this = Promise::PendingOne(Box::from(|_value| waker.wake())),
            Promise::PendingOne(prev_callback) => {
                *this = Promise::PendingMany(vec![prev_callback, Box::from(|_value| waker.wake())])
            }
            Promise::PendingMany(mut callbacks) => {
                callbacks.push(Box::from(|_value| waker.wake()));
                *this = Promise::PendingMany(callbacks);
            }
        }

        Poll::Pending
    }
}
