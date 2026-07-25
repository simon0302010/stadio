mod controller;
mod keyboard;

use std::{
    num::NonZeroU32,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};

use controller::*;
use enigo::{Enigo, Mouse, Settings};
use log::{info, warn};
use softbuffer::{Context, Surface};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::keyboard::KeyboardPopup;

struct StadioApp {
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    enigo: Enigo,
    controller: Controller,
    keyboard_popup: KeyboardPopup,
    last_frame: Instant,
}

impl Default for StadioApp {
    fn default() -> Self {
        let enigo = Enigo::new(&Settings::default()).expect("Failed to init enigo");
        let mut keyboard_popup = KeyboardPopup::new();
        keyboard_popup.create_keys();

        Self {
            window: None,
            context: None,
            surface: None,
            enigo,
            controller: Controller::new(),
            keyboard_popup,
            last_frame: Instant::now(),
        }
    }
}

impl ApplicationHandler for StadioApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_transparent(true)
            .with_decorations(false)
            .with_resizable(false);
        let window = Rc::new(
            event_loop
                .create_window(attrs)
                .expect("Failed to create window"),
        );
        window.request_redraw();

        let context = Context::new(window.clone()).expect("Failed to create context");
        let mut surface = Surface::new(&context, window.clone()).expect("Failed to create surface");

        let size = window.inner_size();
        surface
            .resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            )
            .expect("Failed to resize surface");

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
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
            }
            WindowEvent::RedrawRequested => {
                let surface = self.surface.as_mut().unwrap();
                let window = self.window.as_ref().unwrap();

                let mut paint = Paint::default();

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut pixmap = Pixmap::new(width, height).expect("Failed to create pixmap");
                pixmap.fill(Color::from_rgba8(0, 0, 0, 0));

                let path = PathBuilder::from_circle((width / 2) as f32, (height / 2) as f32, 15.0)
                    .unwrap();
                paint.set_color_rgba8(255, 0, 0, 255);
                pixmap.fill_path(
                    &path,
                    &paint,
                    tiny_skia::FillRule::EvenOdd,
                    Transform::identity(),
                    None,
                );

                self.controller.poll();

                if self.controller.r_trigger {
                    self.enigo
                        .button(enigo::Button::Left, enigo::Direction::Click)
                        .expect("Failed to left click");
                }

                self.enigo.move_mouse(
                    (self.controller.r_stick.0 * 50.0) as i32,
                    (self.controller.r_stick.1 * -50.0) as i32,
                    enigo::Coordinate::Rel,
                );

                if let Err(e) = self.keyboard_popup.tick(
                    self.last_frame.elapsed(),
                    self.controller.l_stick,
                    width / 2,
                    height / 2,
                    &mut pixmap,
                ) {
                    warn!("Keyboard popup tick failed: {}", e);
                }

                let mut buffer = surface.buffer_mut().unwrap();

                for index in 0..(width * height) as usize {
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4] as u32) << 16;
                }

                buffer.present().unwrap();

                self.last_frame = Instant::now();

                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("Failed to initialize logger");

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
        .run_app(&mut StadioApp::default())
        .expect("Failed to run StadioApp");
}

/*
fn main() {

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
