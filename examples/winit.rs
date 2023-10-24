// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() {
    use window_shadows::set_shadow;
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_decorations(false)
        .build(&event_loop)
        .unwrap();

    set_shadow(&window, true).expect("Unsupported platform!");

    window.set_title("A fantastic window!");

    event_loop.run(move |event, event_loop| {
        event_loop.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => event_loop.exit(),
            _ => (),
        }
    });
}
