use ddc_hi::{Ddc, Display};

const BRIGHTNESS_CODE: u8 = 0x10;

pub struct Monitor {
    pub display: Display,
    pub brightness: u16,
}

impl Monitor {
    pub fn new_vec() -> Vec<Self> {
        let mut monitors: Vec<Monitor> = vec![];
        for display in Display::enumerate() {
            monitors.push(Monitor {
                display,
                brightness: 0,
            });
        }
        monitors
    }

    pub fn set_screen_brightness(&mut self, brightness: u16) {
        self.brightness = brightness;
        let _ = self
            .display
            .handle
            .set_vcp_feature(BRIGHTNESS_CODE, brightness);
    }

    pub fn update_brightness(&mut self) {
        self.brightness = self
            .display
            .handle
            .get_vcp_feature(BRIGHTNESS_CODE)
            .map(|v| v.value())
            .unwrap_or_default();
    }
}
