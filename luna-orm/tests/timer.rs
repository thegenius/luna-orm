use luna_orm::prelude::Timer;
mod common;
use common::setup_logger;

#[test]
pub fn test_timer() {
    setup_logger();
    let timer = Timer::new("hello");
}

#[test]
pub fn test_timer_from_string() {
    setup_logger();
    let timer = Timer::new("hello".to_string());
}
