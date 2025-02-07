use nix::unistd;
use std::cell::RefCell;
use std::time::Duration;
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
#[derive(Debug)]
struct CustomTimer {
    created_at: Instant,
    finish_callback: fn() -> Result<(), TimerCallBackError>,
    call_at: Instant, // timer_uuid: &'timer_uuid str,
}

fn main() {
    set_timeout(4, || {
        println!("timeout");
        Ok(())
    });
    set_timeout(8, || {
        println!("timeout");
        Ok(())
    });
    set_timeout(1, || {
        println!("timeout");
        Ok(())
    });
    pr_queue();
}

fn set_timeout(seconds: u64, callback: TimerCallBackFn) {
    let created_at = Instant::now();
    insert_timer(CustomTimer {
        created_at,
        finish_callback: callback,
        call_at: created_at
            .checked_add(Duration::from_secs(seconds))
            .expect("can't overflow"),
    })
}

fn insert_timer(timer: CustomTimer) {
    let position = insert_position(&timer);
    TIMER_QUEUE.with_borrow_mut(|queue| queue.insert(position, timer));
    poll_alarm();
}

fn insert_position(timer: &CustomTimer) -> usize {
    TIMER_QUEUE.with_borrow(|queue| {
        match queue
            .iter()
            .position(|timer_queue| timer_queue.call_at > timer.call_at)
        {
            Some(index) => index,
            None => queue.len(),
        }
    })
}

fn pr_queue() {
    TIMER_QUEUE.with_borrow(|queue| {
        queue.iter().for_each(|timer| {
            println!("timer: {:?}", timer);
        })
    })
}

fn poll_alarm() {
    TIMER_QUEUE.with_borrow(|queue| {
        while !queue.is_empty() {
            let timer = queue.front().unwrap();
            let second = (timer.call_at - Instant::now()).as_secs_f32() as u32;
            unistd::alarm::set(second);
        }
    });
}
