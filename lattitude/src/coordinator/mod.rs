use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use actix::{Actor, AsyncContext, Context, Handler, Message, Recipient};
use num_traits::{FromPrimitive, ToPrimitive};
use effigy::color::Color;
use effigy::pixelfield::PixelField;
use liein::canvas::{Canvas, CanvasActor};
use liein::view::{DiscriminantPixelField, PubSub};
use crate::display::DisplayPixelField;


pub struct Coordinator<C: Color, D> {
    active_page: D,
    pages: HashMap<D, PixelField<C>>,
    displays: Vec<Recipient<DisplayPixelField<C>>>,
    _marker: PhantomData<(C, D)>,
}

impl<C: Color, D> Coordinator<C, D> {

    pub fn new(initial_page: D) -> Self {
        Self {
            active_page: initial_page,
            pages: HashMap::new(),
            displays: vec![],
            _marker: Default::default(),
        }
    }

    pub fn add_display(&mut self, display: Recipient<DisplayPixelField<C>>) {
        self.displays.push( display );
    }

}

impl<C, D> Coordinator<C, D>
    where
        C: Color,
        D: FromPrimitive + ToPrimitive + Unpin + 'static
{
    pub fn activate_page(&mut self, discriminant: D) {}

    pub fn update_content(&mut self, pixel_field: DiscriminantPixelField<C>) {}
}

impl<C, D> Actor for Coordinator<C, D>
    where
        C: Color,
        D: FromPrimitive + ToPrimitive + Unpin + 'static
{
    type Context = Context<Self>;
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct AddPage<C, D>
    where
        C: Color,
        D: FromPrimitive + ToPrimitive + Unpin + PartialEq + Eq + Hash + Send + Copy + 'static
{
    pub discriminant: D,
    pub page: Recipient<PubSub<C>>,
}

impl<C, D> Handler<AddPage<C, D>> for Coordinator<C, D>
    where
        C: Color,
        D: FromPrimitive + ToPrimitive + Unpin + PartialEq + Eq + Hash + Send + Copy + Debug + 'static

{
    type Result = ();

    fn handle(&mut self, msg: AddPage<C, D>, ctx: &mut Self::Context) -> Self::Result {
        self.pages.insert(msg.discriminant, PixelField::default());
        if let Some(discriminant) = msg.discriminant.to_u32() {
            msg.page.do_send(
                PubSub::Subscribe(ctx.address().recipient(), discriminant)
            )
        }
    }
}


/*
#[derive(Message)]
#[rtype(result="()")]
pub struct AddDisplay<C: Color> {
    display: Recipient<DisplayPixelField<C>>,
}

impl<C, D> Handler<AddDisplay<C>> for Coordinator<C, D>
    where
        C: Color,
        D: FromPrimitive + ToPrimitive + Unpin + PartialEq + Eq + Hash + Send + Copy + 'static

{
    type Result = ();

    fn handle(&mut self, msg: AddDisplay<C>, ctx: &mut Self::Context) -> Self::Result {
        self.displays.push( msg.display );
    }
}

 */



impl<C,D> Handler<DiscriminantPixelField<C>> for Coordinator<C, D>
    where
        C: Color,
        D: FromPrimitive + ToPrimitive + Unpin + PartialEq + Eq + Hash + Send + Copy + Debug + 'static
{
    type Result = ();

    fn handle(&mut self, msg: DiscriminantPixelField<C>, _ctx: &mut Self::Context) -> Self::Result {
        let DiscriminantPixelField {
            pixel_field,
            discriminant,
        } = msg;

        if let Some(discriminant) = D::from_u32(discriminant) {
            if discriminant == self.active_page {
                println!("active page {:?}", discriminant);
                for display in &self.displays {
                    println!("sending to display");
                    display.do_send( DisplayPixelField{
                        pixel_field: pixel_field.clone()
                    });
                }
            }
            let pixel_field = pixel_field.lock().unwrap();
            self.pages.insert(discriminant,pixel_field.clone());
        }
    }
}
