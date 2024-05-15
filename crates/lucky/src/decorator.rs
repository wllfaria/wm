use config::Config;
use std::{rc::Rc, sync::Arc};

pub struct Decorator {
    config: Rc<Config>,
    conn: Arc<xcb::Connection>,
}

impl Decorator {
    pub fn new(conn: Arc<xcb::Connection>, config: Rc<Config>) -> Self {
        Decorator { conn, config }
    }

    pub fn maybe_decorate_client(&self, client: xcb::x::Window) -> anyhow::Result<xcb::x::Window> {
        let frame = self.create_frame()?;
        self.reparent_client(frame, client)?;
        Ok(frame)
    }

    fn reparent_client(&self, frame: xcb::x::Window, client: xcb::x::Window) -> anyhow::Result<()> {
        self.conn
            .check_request(self.conn.send_request_checked(&xcb::x::ReparentWindow {
                window: client,
                parent: frame,
                x: 0,
                y: 0,
            }))?;

        Ok(())
    }

    fn create_frame(&self) -> anyhow::Result<xcb::x::Window> {
        let frame = self.conn.generate_id();

        let root = self
            .conn
            .get_setup()
            .roots()
            .next()
            .expect("should have at least one screen to manage");

        self.conn
            .check_request(self.conn.send_request_checked(&xcb::x::CreateWindow {
                depth: xcb::x::COPY_FROM_PARENT as u8,
                wid: frame,
                parent: root.root(),
                x: 0,
                y: 0,
                width: 1,
                height: 1,
                border_width: self.config.border_width(),
                class: xcb::x::WindowClass::InputOutput,
                visual: root.root_visual(),
                value_list: &[
                    xcb::x::Cw::BackPixel(root.white_pixel()),
                    xcb::x::Cw::BorderPixel(self.config.border_color()),
                    xcb::x::Cw::EventMask(
                        xcb::x::EventMask::EXPOSURE
                            | xcb::x::EventMask::BUTTON_PRESS
                            | xcb::x::EventMask::BUTTON_RELEASE
                            | xcb::x::EventMask::POINTER_MOTION
                            | xcb::x::EventMask::ENTER_WINDOW
                            | xcb::x::EventMask::LEAVE_WINDOW,
                    ),
                ],
            }))?;

        Ok(frame)
    }
}