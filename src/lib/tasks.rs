use std::future::Future;

pub fn run<T>(future: T) where
    T: Future + Send + 'static,
    <T as Future>::Output: Send
{
    tokio::task::spawn(future);
}
