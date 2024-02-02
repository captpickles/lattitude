use crate::view;
use crate::view::{DiscriminantPixelField, PubSub, Subscriber};
use actix::{Actor, AsyncContext, Context, Handler, Recipient};
use pixelfield::color::Color;
use pixelfield::pixelfield::PixelField;
use num_traits::{FromPrimitive, ToPrimitive};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, Debug)]
pub enum HorizontalAlign {
    Left,
    Right,
    Center,
}

#[derive(Copy, Clone, Debug)]
pub enum VerticalAlign {
    Top,
    Bottom,
    Middle,
}

pub trait Canvas<C: Color + Unpin>: Unpin + Send + 'static {
    type Discriminant: FromPrimitive + ToPrimitive + Unpin + 'static;

    fn draw(
        &mut self,
        pixel_field: Arc<Mutex<PixelField<C>>>,
        discriminant: Self::Discriminant,
    ) -> Option<Arc<Mutex<PixelField<C>>>>;

    fn into_actor(self) -> CanvasActor<Self, C>
    where
        Self: Sized,
    {
        CanvasActor::new(self)
    }
}

pub struct CanvasActor<CV: Canvas<C>, C: Color> {
    canvas: CV,
    components: Vec<(CV::Discriminant, Recipient<view::PubSub<C>>)>,
    subscribers: Vec<Subscriber<C>>,
    _marker: PhantomData<C>,
}

impl<CV: Canvas<C>, C: Color> CanvasActor<CV, C> {
    pub fn new(canvas: CV) -> Self {
        Self {
            canvas,
            components: vec![],
            subscribers: vec![],
            _marker: Default::default(),
        }
    }

    pub fn add<R: Into<Recipient<view::PubSub<C>>>>(
        &mut self,
        discriminant: CV::Discriminant,
        view: R,
    ) {
        self.components.push((discriminant, view.into()))
    }
}

impl<CV: Canvas<C>, C: Color> Actor for CanvasActor<CV, C> {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let my_addr = ctx.address();
        for (discriminant, component) in &self.components {
            if let Some(discriminant) = discriminant.to_u32() {
                component.do_send(view::PubSub::Subscribe(
                    my_addr.clone().recipient(),
                    discriminant,
                ));
            }
        }
    }
}

impl<CV: Canvas<C>, C: Color> Handler<DiscriminantPixelField<C>> for CanvasActor<CV, C> {
    type Result = ();

    fn handle(&mut self, msg: DiscriminantPixelField<C>, _ctx: &mut Self::Context) -> Self::Result {
        let DiscriminantPixelField {
            pixel_field,
            discriminant,
        } = msg;

        {
            let l = pixel_field.lock().unwrap();
            println!("canvas says {} for {}", l.len(), discriminant);
        }

        if let Some(discriminant) = CV::Discriminant::from_u32(discriminant) {
            if let Some(pixel_field) = self.canvas.draw(pixel_field, discriminant) {
                for subscriber in &self.subscribers {
                    subscriber.recipient.do_send(DiscriminantPixelField {
                        pixel_field: pixel_field.clone(),
                        discriminant: subscriber.discriminant,
                    })
                }
            }
        }
    }
}

impl<CV, C> Handler<PubSub<C>> for CanvasActor<CV, C>
where
    CV: Canvas<C>,
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
                    .retain(|e| (e.recipient != recipient) && (e.discriminant != discriminant))
            }
        }
    }
}
