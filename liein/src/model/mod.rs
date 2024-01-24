use crate::view::StateUpdate;
use actix::{Actor, Context, Handler, Message, Recipient};
use std::fmt::Debug;

pub struct Model<S>
where
    S: Message + Send + 'static,
    S::Result: Send,
    S: Clone,
{
    state: Option<S>,
    subscribers: Vec<Recipient<StateUpdate<S>>>,
}

impl<S> Model<S>
where
    S: Message + Send + 'static,
    S::Result: Send,
    S: Clone,
{
    fn new() -> Self {
        Self {
            state: None,
            subscribers: vec![],
        }
    }

    fn subscribe(&mut self, recipient: Recipient<StateUpdate<S>>) {
        self.subscribers.push(recipient);
    }

    fn unsubscribe(&mut self, recipient: Recipient<StateUpdate<S>>) {
        self.subscribers = self
            .subscribers
            .iter()
            .filter(|e| **e != recipient)
            .cloned()
            .collect();
    }

    fn update(&mut self, state: S) {
        self.state.replace(state.clone());
        for subscriber in &self.subscribers {
            subscriber.do_send(StateUpdate {
                state: state.clone(),
            });
        }
    }
}

pub struct ModelActor<S>
where
    S: Message + Send + 'static,
    S::Result: Send,
    S: Clone,
{
    model: Model<S>,
}

impl<S> ModelActor<S>
where
    S: Message + Send + 'static,
    S::Result: Send,
    S: Clone,
{
    pub fn new() -> Self {
        Self {
            model: Model::new(),
        }
    }
}

impl<S> Actor for ModelActor<S>
where
    S: Message + Send + Unpin + 'static,
    S::Result: Send,
    S: Clone,
{
    type Context = Context<Self>;
}

impl<S> Handler<S> for ModelActor<S>
where
    S: Message<Result = ()> + Send + Unpin + Debug + 'static,
    S::Result: Send,
    S: Clone,
{
    type Result = ();

    fn handle(&mut self, msg: S, _ctx: &mut Self::Context) -> Self::Result {
        self.model.update(msg);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum PubSub<S>
where
    S: Message + Send + Unpin + 'static,
    S::Result: Send,
{
    Subscribe(Recipient<StateUpdate<S>>),
    Unsubscribe(Recipient<StateUpdate<S>>),
}

impl<S> Handler<PubSub<S>> for ModelActor<S>
where
    S: Message<Result = ()> + Send + Unpin + 'static,
    S::Result: Send,
    S: Clone,
{
    type Result = ();

    fn handle(&mut self, msg: PubSub<S>, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            PubSub::Subscribe(recipient) => {
                self.model.subscribe(recipient);
            }
            PubSub::Unsubscribe(recipient) => {
                self.model.unsubscribe(recipient);
            }
        }
    }
}
