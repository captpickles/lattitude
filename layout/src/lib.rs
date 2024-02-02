#![allow(async_fn_in_trait)]

pub mod canvas;
pub mod controller;
pub mod model;
pub mod view;

#[cfg(test)]
mod test {

    #[cfg(test)]
    mod test {
        use crate::controller::periodic::{PeriodExpired, PeriodicActor, PeriodicController};
        use crate::controller::Controller;
        use actix::{Actor, Addr, Context, Handler, Message, MessageResponse, Recipient};
        use chrono::{DateTime, Utc};
        use std::marker::PhantomData;
        use std::time::Duration;
        use tokio::time;

        pub struct ClockController;

        #[derive(Copy, Clone, Debug, Message, MessageResponse)]
        #[rtype(result = "()")]
        pub struct CurrentTime(DateTime<Utc>);

        impl Controller for ClockController {
            //type Output where <<Self as Controller>::Output as Message>::Result: Send = CurrentTime;
            type Output = CurrentTime;
            type Configuration = ();
        }

        impl PeriodicController for ClockController {
            fn cadence(&mut self) -> Duration {
                Duration::from_secs(1)
            }
            //const CADENCE: Duration = Duration::from_secs(1);

            fn period_expired(&mut self) -> Option<CurrentTime> {
                Some(CurrentTime(Utc::now()))
            }
        }

        pub struct View<S>
        where
            S: Message,
        {
            _marker: PhantomData<S>,
        }

        impl<S> View<S>
        where
            S: Message + Unpin + 'static,
        {
            pub fn new() -> Self {
                Self {
                    _marker: Default::default(),
                }
            }
        }

        impl<S> Actor for View<S>
        where
            S: Message + Unpin + 'static,
        {
            type Context = Context<Self>;
        }

        impl Handler<CurrentTime> for View<CurrentTime> {
            type Result = ();

            fn handle(&mut self, msg: CurrentTime, ctx: &mut Self::Context) -> Self::Result {
                println!("received ---> {:?}", msg.0);
            }
        }

        /*
        #[actix::test]
        async fn whut() {
            let clock = PeriodicActor::new(ClockController).start();
            let clock_view: Addr<View<CurrentTime>> = View::new().start();

            let recipient = clock_view.recipient();

            for _ in 0..4 {
                let result = clock.send(
                    PeriodExpired {
                        recipient: recipient.clone()
                    }
                ).await;

                println!("outer {:?}", result);

                time::sleep(Duration::from_secs(1)).await;
            }
            time::sleep(Duration::from_secs(2)).await;
        }

         */
    }
}
