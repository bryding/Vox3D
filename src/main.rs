use winit::{
    event::*,
    event_loop::{ ControlFlow, EventLoop },
    window::{ WindowBuilder, CursorGrabMode },
};
use std::time::Instant;

mod game;
mod renderer;
mod terrain;

use game::GameState;
use renderer::RenderState;

// Add this to track if we're in game mode
struct InputState {
    game_active: bool,
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Vox3D").build(&event_loop).unwrap();

    let mut game_state = GameState::new();
    let mut render_state = RenderState::new(&window, &game_state).await;

    // Track if we're in game mode
    let mut input_state = InputState {
        game_active: false,
    };

    let mut last_update_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                if input_state.game_active {
                    game_state.handle_mouse_motion(delta.0, delta.1);
                }
            }
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if !input_state.game_active {
                            input_state.game_active = true;
                            let _ = window
                                .set_cursor_grab(CursorGrabMode::Locked)
                                .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked));
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { state, virtual_keycode: Some(keycode), .. },
                        ..
                    } => {
                        match keycode {
                            VirtualKeyCode::Escape => {
                                if input_state.game_active {
                                    input_state.game_active = false;
                                    window.set_cursor_visible(true);
                                    let _ = window.set_cursor_grab(CursorGrabMode::None);
                                } else {
                                    *control_flow = ControlFlow::Exit;
                                }
                            }
                            _ => {
                                if input_state.game_active {
                                    game_state.handle_keyboard(*keycode, *state);
                                }
                            }
                        }
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        render_state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = Instant::now();
                let dt = now - last_update_time;
                last_update_time = now;

                if input_state.game_active {
                    game_state.update(dt);
                }

                render_state.update(&game_state);

                match render_state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
