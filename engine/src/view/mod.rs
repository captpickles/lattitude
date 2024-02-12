use std::cell::RefCell;
use actix::dev::{MessageResponse, OneshotSender};
use actix::{Actor, Context, ContextFutureSpawner, Handler, Message, MessageResult, Recipient, ResponseActFuture, WrapFuture};
use pixelfield::pixelfield::{PixelField, Rectangle};
use std::future::{Future, IntoFuture};
use std::rc::Rc;
use tokio::sync::oneshot::Sender;

pub mod text;
pub mod canvas;
pub mod rotate;
pub mod scale;
pub mod trim;
pub mod pixels;

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

pub trait Renderable : Send + Sync {
    fn render(&self) -> Option<PixelField>;
}


