use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
};

struct SharedState {
    time_elapsed: bool,
    waker: Option<Waker>,
}

struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut data = self.shared_state.lock().unwrap();
        if data.time_elapsed {
            return Poll::Ready(());
        }
        data.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            time_elapsed: false,
            waker: Option::None,
        }));

        let shared_state_thread = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut state = shared_state_thread.lock().unwrap();
            state.time_elapsed = true;
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

