mod controller;
mod keyboard;

use std::{sync::{Arc, Mutex}, thread::sleep, time::{Duration, Instant}};

use controller::*;
use enigo::{Enigo, Settings};
use log::{info, warn};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId, WindowAttributes};

use crate::keyboard::KeyboardPopup;

#[derive(Default)]
struct StadioApp {
    window: Option<Window>
}

impl ApplicationHandler for StadioApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_transparent(true)
            .with_decorations(false)
            .with_resizable(false);
        self.window = Some(event_loop.create_window(attrs).expect("Failed to create window"));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Close requested");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                sleep(Duration::from_millis(16));

                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            },
            _ => {}
        }
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut StadioApp::default()).expect("Failed to run StadioApp");
}

/*
fn main() {

    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default()).expect("Failed to init enigo")));
    let mut keyboard_popup = KeyboardPopup::new(enigo.clone());

    let mut controller = Controller::new();

    let mut last_frame = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now - last_frame;
        last_frame = now;

        if !controller.poll() {
            warn!("Failed to poll controller");
            sleep(Duration::from_millis(20));
            continue;
        }

        if let Err(e) = keyboard_popup.tick(dt, controller.l_stick) {
            warn!("Keyboard popup tick failed: {}", e);
        }

        // wait for at least 16ms
        sleep(Duration::from_millis(16).saturating_sub(now.elapsed()));
    }
}
*/
