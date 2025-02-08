use nix::sys::signal;
use nix::sys::signal::SigAction;
use nix::sys::signal::{SaFlags, SigHandler, SigSet, SIGALRM};
use nix::unistd;
use std::cell::RefCell;
use std::ffi::c_int;
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
static TIMER_QUEUE: RefCell<VecDeque<CustomTimer>> = const{RefCell::new(VecDeque::new())};
}
#[derive(Debug)]
struct CustomTimer {
    finish_callback: fn() -> Result<(), TimerCallBackError>,
    call_at: Instant,
}
fn main() {
    unsafe {
        signal::sigaction(
            SIGALRM,
            &SigAction::new(
                SigHandler::Handler(alarm_handler),
                SaFlags::empty(),
                SigSet::all(),
            ),
        )
        .unwrap()
    };
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
    poll_alarm();
}

fn set_timeout(seconds: u32, callback: TimerCallBackFn) {
    let created_at = Instant::now();
    insert_timer(CustomTimer {
        finish_callback: callback,
        call_at: created_at
            .checked_add(Duration::from_secs(seconds as u64))
            .expect("can't overflow"),
    })
}

fn insert_timer(timer: CustomTimer) {
    let position = insert_position(&timer);
    TIMER_QUEUE.with_borrow_mut(|queue| queue.insert(position, timer));
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
    TIMER_QUEUE.with_borrow_mut(|queue| {
        while !queue.is_empty() {
            let timer = queue.front().unwrap();
            let sleep_time = (timer.call_at - Instant::now()).as_secs_f32().round() as u32;
            println!("sleep_time: {}", sleep_time);
            unistd::alarm::set(sleep_time);
            let mut sigset = SigSet::empty();
            sigset.add(SIGALRM);
            sigset.wait().unwrap();
            (timer.finish_callback)().unwrap_or_else(|error| {
                eprintln!("timer finish callback returned error: {}", error)
            });
            queue.pop_front();
        }
    });
}

extern "C" fn alarm_handler(_: c_int) {}
