use actix::{Message, Recipient};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod periodic;

pub trait Controller {
    type Output: Message<Result = ()> + Send + Clone + Unpin + Debug + 'static;

    type Configuration: Serialize + for<'a> Deserialize<'a>;

    fn configure(&mut self, _configuration: Option<Self::Configuration>) {}
}

/*
impl<S> Message for PeriodExpired<S>
    where S: Message + Send + 'static,
          S::Result: Send
{
    type Result = S;
}

 */

pub trait RequestController: Controller {
    type Request: actix::Message;

    fn request(&mut self, _request: Self::Request, _sink: Recipient<<Self as Controller>::Output>)
    where
        <<Self as Controller>::Output as Message>::Result: Send,
    {
    }
}
