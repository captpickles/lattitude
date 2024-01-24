use actix::{Actor, Recipient};
use liein::controller::periodic::{PeriodicActor, PeriodicController};
use liein::model::{ModelActor, PubSub};
use crate::controller::clock::{Clock, CurrentDateTime};

pub struct System {
    pub clock: Recipient<PubSub<CurrentDateTime>>,
}

impl Default for System {
    fn default() -> Self {
        Self {
            clock: Self::periodic(Clock),
        }
    }
}

impl System {

    fn periodic<C: PeriodicController + 'static>(controller: C) -> Recipient<PubSub<C::Output>>{
        let model = ModelActor::new().start();
        let _controller_addr = PeriodicActor::new(controller, model.clone().recipient()).start();
        model.recipient()
    }

}

#[cfg(test)]
mod test {
    use std::os::macos::raw::stat;
    use std::time::Duration;
    use actix::clock::sleep;
    use effigy::color::Gray16;
    use liein::canvas::{Canvas, CanvasActor, HorizontalAlign, VerticalAlign};
    use liein::view::{View, ViewActor};
    use crate::page::main::{MainPage, MainPageComponents};
    use crate::page::Page;
    use crate::view::StatusBar;
    use super::*;

    #[actix::test]
    async fn whut() {
        let system = System::new();

        let status_bar = StatusBar::new().connect(system.clock);
        let status_bar = status_bar.start();

        let main_page = Page::<Gray16>::new(|page| {
            page.place(
                status_bar,
                (0,0),
                HorizontalAlign::Left,
                VerticalAlign::Top
            );
        }).start();

        sleep(Duration::from_secs(3)).await;
    }

}