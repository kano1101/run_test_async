use futures::future::FutureExt as _;

pub async fn run_test_async<F1, F2, F3, R>(setup_async: F1, test: F2, teardown_async: F3) -> R
where
    F1: std::future::Future,
    F2: std::future::Future<Output = R>,
    F3: std::future::Future,
{
    setup_async.await;
    let result = std::panic::AssertUnwindSafe(test).catch_unwind().await;
    teardown_async.await;

    match result {
        Err(err) => {
            std::panic::resume_unwind(err);
        }
        Ok(ok) => return ok,
    }
}
pub async fn bench<T>(
    message: impl Into<&str> + Send,
    target_fn: impl std::future::Future<Output = T> + Send,
) -> T {
    let now = std::time::Instant::now();
    let r = target_fn.await;
    let duration_time = format!("{:?}", now.elapsed());
    let message = message.into();
    tracing::info!(duration_time, message);
    r
}
