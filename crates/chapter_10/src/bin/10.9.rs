use nix::{
    sys::signal::{
        self, SigSet,
        SigmaskHow::{SIG_BLOCK, SIG_UNBLOCK},
        Signal::{self, SIGALRM, SIGINT},
    },
    Result as NixResult,
};

fn main() {
    let mut sigset = SigSet::empty();
    sigset.add(SIGINT);
    sigset.add(SIGALRM);
    pr_mask("origin mask").unwrap_or_else(|errno| {
        panic!("line={}, pr_mask occurs an error: {}", line!(), errno);
    });
    signal::sigprocmask(SIG_BLOCK, Some(&sigset), None).unwrap_or_else(|errno| {
        panic!("line={}, sigprocmask occurs an error: {}", line!(), errno);
    });
    pr_mask("after block").unwrap_or_else(|errno| {
        panic!("line={}, pr_mask occurs an error: {}", line!(), errno);
    });
    signal::sigprocmask(SIG_UNBLOCK, Some(&sigset), None).unwrap_or_else(|errno| {
        panic!("line={}, sigprocmask occurs an error: {}", line!(), errno);
    });
    pr_mask("after unblock").unwrap_or_else(|errno| {
        panic!("line={}, pr_mask occurs an error: {}", line!(), errno);
    });
}

fn pr_mask(desc: &str) -> NixResult<()> {
    let mut old_set = SigSet::empty();
    signal::sigprocmask(SIG_UNBLOCK, None, Some(&mut old_set))?;
    println!("🚀 {}", desc);
    Signal::iterator().for_each(|signal| {
        if old_set.contains(signal) {
            println!("{}", signal.as_str());
        }
    });
    Ok(())
}
