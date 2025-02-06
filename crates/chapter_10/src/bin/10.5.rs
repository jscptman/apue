use std::cell::RefCell;
use std::{collections::VecDeque, error::Error, fmt::Display, time::Instant};

#[derive(Debug)]
struct TimerCallBackError;
impl Display for TimerCallBackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for TimerCallBackError {}

type TimerCallBackFn = fn() -> Result<(), TimerCallBackError>;
thread_local! {
static TIMER_QUEUE: RefCell<VecDeque<CustomTimer>> = RefCell::new(VecDeque::new());
}
struct CustomTimer {
    created_at: Instant,
    finish_callback: fn() -> Result<(), TimerCallBackError>,
    // timer_uuid: &'timer_uuid str,
}

fn main() {
    let mut timer_queue: VecDeque<CustomTimer> = VecDeque::new();
}

fn set_timeout(seconds: usize, callback: TimerCallBackFn) {
    insert_timer(CustomTimer {
        created_at: Instant::now(),
        finish_callback: callback,
        // timer_uuid:
    })
}

fn insert_timer(timer: CustomTimer) {
    TIMER_QUEUE.with_borrow_mut(|queue| queue.push_back(timer));
}

fn insert_position() {}
