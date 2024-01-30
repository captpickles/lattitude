use actix::{Actor, Recipient};
use enum_primitive_derive::Primitive;
use liein::controller::periodic::{PeriodicActor, PeriodicController};
use liein::model::{ModelActor, PubSub};
use crate::integration::clock::{Clock, CurrentDateTime};

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
    fn periodic<C: PeriodicController + 'static>(controller: C) -> Recipient<PubSub<C::Output>> {
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
    use actix::ContextFutureSpawner;
    use chrono::Timelike;
    use effigy::color::Gray16;
    use liein::canvas::{Canvas, CanvasActor, HorizontalAlign, VerticalAlign};
    use liein::view::{View, ViewActor};
    use crate::coordinator::{AddPage, Coordinator};
    use crate::display::bmp::BmpDisplay;
    use crate::display::Display;
    use crate::font::typewriter;
    use crate::page::main::{MainPage, MainPageComponents};
    use crate::page::Page;
    use crate::view::StatusBar;
    use crate::view::text::Text;
    use super::*;

    #[actix::test]
    async fn whut() {
        let system = System::default();

        //let time = StatusBar::default().connect(system.clock);
        let time = Text::new(
            typewriter().unwrap(),
            200.0,
            |current_time: CurrentDateTime| {
                let h = current_time.0.hour();
                let m = current_time.0.minute();
                format!("{}:{:0>2}", h, m)
            }
        );

        let status_bar = time.connect(system.clock).start();

        let main_page = Page::<Gray16>::new(|page| {
            page.place(
                status_bar,
                (400, 400),
                HorizontalAlign::Left,
                VerticalAlign::Top,
            );
        }).start();

        let display: Box<dyn Display<Gray16>> = Box::new(BmpDisplay::new((1404, 1852)));
        let display = display.start();

        let mut coordinator = Coordinator::<Gray16, Pages>::new(Pages::Unbox);

        coordinator.add_display(display.recipient());

        let coordinator = coordinator.start();

        coordinator.do_send(AddPage {
            discriminant: Pages::Unbox,
            page: main_page.recipient(),
        });

        sleep(Duration::from_secs(1)).await;
    }
}

#[derive(Primitive, Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Pages {
    Unbox = 0,
    Splash = 1,
    Main = 2,
}