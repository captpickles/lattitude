use crate::controller::Controller;
use actix::{Actor, AsyncContext, Context, Handler, Message, Recipient};
use std::time::Duration;

pub trait PeriodicController: Controller + Unpin {
    fn cadence(&mut self) -> Duration;
    fn period_expired(&mut self) -> Option<Self::Output>;
}

pub struct PeriodicActor<C>
where
    C: PeriodicController,
{
    controller: C,
    recipient: Recipient<C::Output>,
}

impl<C> PeriodicActor<C>
where
    C: PeriodicController,
{
    pub fn new(controller: C, recipient: Recipient<C::Output>) -> Self {
        Self {
            controller,
            recipient,
        }
    }
}

impl<C> Actor for PeriodicActor<C>
where
    C: PeriodicController + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let my_addr = ctx.address();

        let recipient = self.recipient.clone();
        my_addr.do_send(PeriodExpired {
            recipient: recipient.clone(),
        });

        ctx.run_interval(self.controller.cadence(), move |_, _ctx| {
            my_addr.do_send(PeriodExpired {
                recipient: recipient.clone(),
            })
        });
    }
}

impl<C> Handler<PeriodExpired<C::Output>> for PeriodicActor<C>
where
    C: PeriodicController + 'static,
    //<<C as Controller>::Output as Message>::Result: Send,
    //S: MessageResponse<PeriodicActor<C, S>, PeriodExpired<S>>,
{
    //type Result = S;
    type Result = ();

    fn handle(&mut self, msg: PeriodExpired<C::Output>, _ctx: &mut Self::Context) -> Self::Result {
        let result = self.controller.period_expired();
        if let Some(result) = result {
            msg.recipient.do_send(result);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PeriodExpired<S>
where
    S: Message + Send + 'static,
    S::Result: Send,
{
    pub recipient: Recipient<S>,
}
