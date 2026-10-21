use windows_reactor::*;
use std::{error, result};
use tokio;

use tokio::time::{sleep, Duration};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub trait HasCancelMessage: Component {
    fn cancel_message() -> <Self as Component>::Message;
}

pub trait ComponentContextExt<C> where C: HasCancelMessage, <C as Component>::Message: Send + From<Box<dyn error::Error>> {
    fn spawn_future<F>(&self, f: F) -> ComponentTask
        where F: 'static + Send + Sync + Future<Output = <C as Component>::Message>;

    fn spawn_future_error<F>(&self, f: F) -> ComponentTask
        where F: 'static + Send + Sync + Future<Output = Result<<C as Component>::Message>>;
}

impl<C> ComponentContextExt<C> for ComponentContext<C> where C: HasCancelMessage, <C as Component>::Message: Send + From<Box<dyn error::Error>> {
    fn spawn_future<F>(&self, f: F) -> ComponentTask where F: 'static + Send + Sync + Future<Output = <C as Component>::Message> {
        let handle = crate::tokio_handle();

        self.spawn_background(move |cancel| {
            let is_cancelled = async move {
                while !cancel.is_cancelled() {
                    sleep(Duration::from_millis(100)).await;
                }
            };

            handle.block_on(async move {
                tokio::select! {
                    result = handle.spawn(f) => match result {
                        Ok(message) =>  message,
                        Err(err) => {
                            let err: Box<dyn error::Error> = err.into();
                            let err: C::Message = err.into();

                            err
                        },
                    },
                    _ = handle.spawn(is_cancelled) => {
                        C::cancel_message()
                    }
                }
            })
        })
    }

    fn spawn_future_error<F>(&self, f: F) -> ComponentTask where F: 'static + Send + Sync + Future<Output = Result<<C as Component>::Message>> {
        let handle = crate::tokio_handle();

        self.spawn_background(move |cancel| {
            let task = async move {
                match f.await {
                    Ok(message) => message,
                    Err(err) => err.into(),
                }
            };

            let is_cancelled = async move {
                while !cancel.is_cancelled() {
                    sleep(Duration::from_millis(100)).await;
                }
            };

            handle.block_on(async move {
                tokio::select! {
                    result = handle.spawn(task) => match result {
                        Ok(message) => message,
                        Err(err) => {
                            let err: Box<dyn error::Error> = err.into();
                            let err: C::Message = err.into();

                            err
                        },
                    },
                    _ = handle.spawn(is_cancelled) => {
                        C::cancel_message()
                    },
                }
            })
        })
    }
}

