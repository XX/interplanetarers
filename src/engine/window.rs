use std::sync::mpsc::Receiver;
use std::mem;

use gl;
use glfw::{self, Glfw, Context, Key, Scancode, Action, Modifiers, Window as GlfwWindow, WindowEvent};

use lang::ObjectPar;
use engine::input::{MouseEvent, KeyEvent, InputEvent, InputControl};
use engine::timing::Timing;

type Events = Receiver<(f64, WindowEvent)>;

pub struct Window {
    pub controls: Vec<ObjectPar<InputControl>>,
    pub timing: Timing,
    glfw: Glfw,
    window: GlfwWindow,
    events: Option<Events>,
    last_mouse_pos: Option<(f32, f32)>,
}

impl Window {
    pub fn new(title: &str) -> Window {
        // ------------------------------
        // glfw: initialize and configure
        // ------------------------------
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // --------------------
        // glfw window creation
        // --------------------
        let (mut window, events) = glfw.create_window(800, 600, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // -------------------------------------
        // gl: load all OpenGL function pointers
        // -------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            controls: vec![],
            timing: Timing::default(),
            glfw,
            window,
            events: Some(events),
            last_mouse_pos: None,
        }
    }

    pub fn events_loop(&mut self) {
        let events = mem::replace(&mut self.events, None);

        while !self.window.should_close() {
            self.timing();

            // ## events
            if let Some(ref events) = events {
                self.process_events(events);
            }

            // ## render
            self.render();

            // ## glfw: poll IO events (keys pressed/released, mouse moved etc.)
            self.glfw.poll_events();
        }
    }

    /// per-frame time logic
    fn timing(&mut self) {
        let current_frame = self.glfw.get_time() as f32;
        self.timing.delta_time = current_frame - self.timing.last_frame;
        self.timing.last_frame = current_frame;
    }

    fn process_events(&mut self, events: &Events) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                },
                glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
                    let (x_pos, y_pos) = (x_pos as f32, y_pos as f32);

                    if self.last_mouse_pos.is_none() {
                        self.last_mouse_pos = Some((x_pos, y_pos));
                    }

                    let (x_last, y_last) = self.last_mouse_pos.unwrap();
                    let x_offset = x_pos - x_last;
                    let y_offset = y_last - y_pos; // reversed since y-coordinates go from bottom to top

                    self.last_mouse_pos = Some((x_pos, y_pos));

                    self.mouse_event(MouseEvent {
                        x_pos,
                        y_pos,
                        x_offset,
                        y_offset,
                        is_scroll: false,
                    });
                },
                glfw::WindowEvent::Scroll(x_offset, y_offset) => {
                    let (x_pos, y_pos) = if let Some(last_pos) = self.last_mouse_pos {
                        last_pos
                    } else {
                        (0.0, 0.0)
                    };
                    self.mouse_event(MouseEvent {
                        x_pos,
                        y_pos,
                        x_offset: x_offset as f32,
                        y_offset: y_offset as f32,
                        is_scroll: true,
                    });
                },
                WindowEvent::Key(key, code, action, modifiers) => {
                    self.keyboard_event(KeyEvent(key, code, action, modifiers))
                },
                _ => {}
            }
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.window.swap_buffers();
    }
}

impl InputEvent for Window {
    fn mouse_event(&mut self, event: MouseEvent) {
        for control in self.controls.iter() {
            if let Ok(mut control) = control.lock() {
                control.on_mouse(event.clone(), self.timing.delta_time);
            }
        }
    }

    fn keyboard_event(&mut self, event: KeyEvent) {
        match event {
            KeyEvent(Key::Escape, _, Action::Press, _) => self.window.set_should_close(true),
            _ => ()
        }

        for control in self.controls.iter() {
            if let Ok(mut control) = control.lock() {
                control.on_keyboard(event.clone(), self.timing.delta_time);
            }
        }
    }
}