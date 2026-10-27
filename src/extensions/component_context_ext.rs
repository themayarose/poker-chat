use windows_reactor::*;
use tokio_util::sync::CancellationToken;
use std::{error, result};
use tokio;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub trait HasCancelMessage: Component {
    fn cancel_message() -> <Self as Component>::Message;
}

pub trait ComponentContextExt<C> where C: HasCancelMessage, <C as Component>::Message: Send + From<Box<dyn error::Error>> {
    fn spawn_future<F>(&self, f: F) -> CancellationToken
        where F: 'static + Send + Sync + Future<Output = <C as Component>::Message>;

    fn spawn_future_error<F>(&self, f: F) -> CancellationToken
        where F: 'static + Send + Sync + Future<Output = Result<<C as Component>::Message>>;
}

impl<C> ComponentContextExt<C> for ComponentContext<C> where C: HasCancelMessage, <C as Component>::Message: Send + From<Box<dyn error::Error>> {
    fn spawn_future<F>(&self, f: F) -> CancellationToken where F: 'static + Send + Sync + Future<Output = <C as Component>::Message> {
        let handle = crate::tokio_handle();
        let token = CancellationToken::new();
        let token_clone = token.clone();

        self.spawn_background(move |_| {
            handle.block_on(async move {
                let task = handle.spawn(f);
                let abort_handle = task.abort_handle();

                tokio::select! {
                    result = task => match result {
                        Ok(message) =>  message,
                        Err(err) => {
                            let err: Box<dyn error::Error> = err.into();
                            let err: C::Message = err.into();

                            err
                        },
                    },
                    _ = token_clone.cancelled() => {
                        abort_handle.abort();
                        C::cancel_message()
                    }
                }
            })
        });

        token
    }

    fn spawn_future_error<F>(&self, f: F) -> CancellationToken where F: 'static + Send + Sync + Future<Output = Result<<C as Component>::Message>> {
        let handle = crate::tokio_handle();
        let token = CancellationToken::new();
        let token_clone = token.clone();

        self.spawn_background(move |_| {
            let task = async move {
                match f.await {
                    Ok(message) => message,
                    Err(err) => err.into(),
                }
            };

            handle.block_on(async move {
                let task = handle.spawn(task);
                let abort_handle = task.abort_handle();

                tokio::select! {
                    result = task => match result {
                        Ok(message) => message,
                        Err(err) => {
                            let err: Box<dyn error::Error> = err.into();
                            let err: C::Message = err.into();

                            err
                        },
                    },
                    _ = token_clone.cancelled() => {
                        abort_handle.abort();
                        C::cancel_message()
                    },
                }
            })
        });

        token
    }
}

