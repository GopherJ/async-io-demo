use mio::*;
use std::time::{Duration};
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let poll = Poll::new()?;
    let timeout = Duration::from_millis(10);
    let mut events = Events::with_capacity(1024);
    match poll.poll(&mut events, Some(timeout)) {
        Ok(num) => assert_eq!(0, num),
        Err(_) => unreachable!()
    }
    Ok(())
}