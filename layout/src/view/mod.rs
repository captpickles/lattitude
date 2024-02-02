use crate::model;
use actix::{Actor, AsyncContext, Context, Handler, Message, Recipient};
use pixelfield::color::Color;
use pixelfield::pixelfield::PixelField;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

#[derive(Message)]
#[rtype(result = "()")]
pub enum PubSub<C>
where
    C: Color,
{
    Subscribe(Recipient<DiscriminantPixelField<C>>, u32),
    Unsubscribe(Recipient<DiscriminantPixelField<C>>, u32),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StateUpdate<S> {
    pub state: S,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DiscriminantPixelField<C: Color> {
    pub pixel_field: Arc<Mutex<PixelField<C>>>,
    pub discriminant: u32,
}

#[derive(Clone)]
pub struct Subscriber<C: Color> {
    pub recipient: Recipient<DiscriminantPixelField<C>>,
    pub discriminant: u32,
}

pub trait View<C: Color>: Unpin {
    type Input: Message<Result = ()> + Send + Clone + Unpin + Debug + 'static;
    fn update<I: Into<Self::Input>>(&mut self, state: I) -> Option<Arc<Mutex<PixelField<C>>>>;
    //fn repaint(&self) -> Option<PixelField<C>>;

    fn connect<R: Into<Recipient<model::PubSub<Self::Input>>>>(
        self,
        source: R,
    ) -> ViewActor<Self, C>
    where
        Self: Sized,
    {
        ViewActor::new(self, source.into())
    }
}

pub struct ViewActor<V, C>
where
    V: View<C>,
    C: Color,
{
    view: V,
    source: Recipient<model::PubSub<V::Input>>,
    subscribers: Vec<Subscriber<C>>,
    _marker: PhantomData<C>,
}

impl<V, C> ViewActor<V, C>
where
    V: View<C>,
    C: Color,
{
    pub fn new(view: V, source: Recipient<model::PubSub<V::Input>>) -> Self {
        Self {
            view,
            source,
            subscribers: vec![],
            _marker: PhantomData,
        }
    }
}

impl<V, C> Actor for ViewActor<V, C>
where
    V: View<C> + 'static,
    C: Color,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.source
            .do_send(model::PubSub::Subscribe(ctx.address().recipient()));
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.source
            .do_send(model::PubSub::Unsubscribe(ctx.address().recipient()));
    }
}

impl<V, C> Handler<StateUpdate<V::Input>> for ViewActor<V, C>
where
    V: View<C> + 'static,
    C: Color,
{
    type Result = ();

    fn handle(&mut self, msg: StateUpdate<V::Input>, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(pixel_field) = self.view.update(msg.state) {
            for subscriber in &self.subscribers {
                subscriber.recipient.do_send(DiscriminantPixelField {
                    pixel_field: pixel_field.clone(),
                    discriminant: subscriber.discriminant,
                })
            }
        }
    }
}

impl<V, C> Handler<PubSub<C>> for ViewActor<V, C>
where
    V: View<C> + 'static,
    C: Color,
{
    type Result = ();

    fn handle(&mut self, msg: PubSub<C>, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            PubSub::Subscribe(recipient, discriminant) => self.subscribers.push(Subscriber {
                recipient,
                discriminant,
            }),
            PubSub::Unsubscribe(recipient, discriminant) => {
                self
                    .subscribers
                    .retain(|e| (e.recipient != recipient) && (e.discriminant != discriminant));
            }
        }
    }
}

pub struct PrintlnView<S: Debug + Unpin + Clone + Send + Message<Result = ()> + 'static> {
    _marker: PhantomData<S>,
}

impl<S: Debug + Unpin + Clone + Send + Message<Result = ()> + 'static, C: Color> View<C>
    for PrintlnView<S>
{
    type Input = S;
    fn update<I: Into<Self::Input>>(&mut self, state: I) -> Option<Arc<Mutex<PixelField<C>>>>{
        let state = state.into();
        println!("VIEW {:#?}", state);
        None
    }

}
