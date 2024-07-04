use crate::localize::localize;
use crate::window::Window;

mod localize;
mod monitor;
mod window;

fn main() -> cosmic::iced::Result {
    localize();
    cosmic::applet::run::<Window>(false, ())
}
