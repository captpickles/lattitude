use std::time::Duration;
use actix::Message;
use chrono::{DateTime, Utc};
use liein::controller::Controller;
use liein::controller::periodic::PeriodicController;

#[derive(Message, Copy, Clone, Debug)]
#[rtype( result = "()")]
pub struct CurrentDateTime(pub DateTime<Utc>);

impl CurrentDateTime {
    fn new() -> Self {
        Self( Utc::now() )
    }

}

pub struct Clock;


impl Controller for Clock {
    type Output = CurrentDateTime;
    type Configuration = ();
}

impl PeriodicController for Clock {
    fn cadence(&mut self) -> Duration {
        Duration::from_secs(1)
    }

    fn period_expired(&mut self) -> Option<Self::Output> {
        Some(CurrentDateTime::new())
    }
}