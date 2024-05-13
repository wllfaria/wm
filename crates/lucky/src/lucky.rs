use config::Config;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{mpsc::channel, Arc},
};
use xcb::x::{self, ChangeWindowAttributes};

use crate::{
    clients::Clients, cursor::Cursor, event::EventContext, handlers::Handlers, keyboard::Keyboard,
};

pub struct Lucky {
    conn: Arc<xcb::Connection>,
    keyboard: Keyboard,
    config: Rc<Config>,
    handlers: Handlers,
    clients: Rc<RefCell<Clients>>,
}

impl Lucky {
    pub fn new(config: Config) -> Self {
        let (conn, _) = xcb::Connection::connect(None).expect("failed to initialize self.conn to the X server. Check the DISPLAY environment variable");
        let conn = Arc::new(conn);
        let cursor = Cursor::new(conn.clone());

        let screen = conn
            .get_setup()
            .roots()
            .next()
            .expect("should have at least a single window");
        let root = screen.root();

        conn.check_request(conn.send_request_checked(&ChangeWindowAttributes {
            window: root,
            value_list: &[
                x::Cw::EventMask(
                    x::EventMask::SUBSTRUCTURE_REDIRECT | x::EventMask::SUBSTRUCTURE_NOTIFY,
                ),
                x::Cw::Cursor(cursor.cursor),
            ],
        }))
        .expect("failed to subscribe for substructure redirection");

        let config = Rc::new(config);
        Self {
            clients: Rc::new(RefCell::new(Clients::new(conn.clone()))),
            keyboard: Keyboard::new(conn.clone(), root, config.clone()),
            handlers: Handlers::default(),
            conn,
            config,
        }
    }

    pub fn run(mut self) {
        let (sender, receiver) = channel::<XEvent>();

        let conn = self.conn.clone();
        std::thread::spawn(move || loop {
            if let Ok(event) = conn.wait_for_event() {
                match event {
                    xcb::Event::X(xcb::x::Event::KeyPress(e)) => {
                        if sender.send(XEvent::KeyPress(e)).is_err() {
                            tracing::debug!("failed to send event through channel");
                            std::process::abort();
                        }
                    }
                    xcb::Event::X(xcb::x::Event::MapRequest(e)) => {
                        if sender.send(XEvent::MapRequest(e)).is_err() {
                            tracing::debug!("failed to send event through channel");
                            std::process::abort();
                        }
                    }
                    xcb::Event::X(xcb::x::Event::DestroyNotify(e)) => {
                        if sender.send(XEvent::DestroyNotify(e)).is_err() {
                            tracing::debug!("failed to send event through channel");
                            std::process::abort();
                        }
                    }
                    xcb::Event::X(xcb::x::Event::EnterNotify(e)) => {
                        // TODO: when entering the window we should focus it if `focus_on_hover` is
                        // enabled
                        if sender.send(XEvent::EnterNotify(e)).is_err() {
                            tracing::debug!("failed to send event through channel");
                            std::process::abort();
                        }
                    }
                    xcb::Event::X(xcb::x::Event::ConfigureRequest(_)) => todo!(),
                    xcb::Event::X(xcb::x::Event::PropertyNotify(_)) => {}
                    xcb::Event::X(xcb::x::Event::UnmapNotify(_)) => {}
                    _ => (),
                };
            };
            conn.flush().unwrap();
        });

        loop {
            if let Ok(event) = receiver.recv() {
                match event {
                    XEvent::KeyPress(event) => self.handlers.on_key_press(EventContext {
                        event,
                        conn: self.conn.clone(),
                        keyboard: &self.keyboard,
                        config: self.config.clone(),
                        clients: self.clients.clone(),
                    }),
                    XEvent::MapRequest(event) => self.handlers.on_map_request(EventContext {
                        event,
                        conn: self.conn.clone(),
                        keyboard: &self.keyboard,
                        config: self.config.clone(),
                        clients: self.clients.clone(),
                    }),
                    XEvent::DestroyNotify(event) => self.handlers.on_destroy_notify(EventContext {
                        event,
                        conn: self.conn.clone(),
                        keyboard: &self.keyboard,
                        config: self.config.clone(),
                        clients: self.clients.clone(),
                    }),
                    XEvent::EnterNotify(event) => self.handlers.on_enter_notify(EventContext {
                        event,
                        conn: self.conn.clone(),
                        keyboard: &self.keyboard,
                        config: self.config.clone(),
                        clients: self.clients.clone(),
                    }),
                    XEvent::UnmapNotify(_) => {}
                    XEvent::PropertyNotify(_) => {}
                    XEvent::ConfigureRequest(_) => todo!(),
                }
            }
        }
    }
}

pub enum XEvent {
    KeyPress(xcb::x::KeyPressEvent),
    MapRequest(xcb::x::MapRequestEvent),
    DestroyNotify(xcb::x::DestroyNotifyEvent),
    PropertyNotify(xcb::x::PropertyNotifyEvent),
    ConfigureRequest(xcb::x::ConfigureRequestEvent),
    EnterNotify(xcb::x::EnterNotifyEvent),
    UnmapNotify(xcb::x::UnmapNotifyEvent),
}
