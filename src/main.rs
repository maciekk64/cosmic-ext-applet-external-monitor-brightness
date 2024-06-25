use crate::window::Window;

mod monitor;
mod window;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Window>(false, ())
}
