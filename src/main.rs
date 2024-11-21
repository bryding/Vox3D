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

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Vox3D").build(&event_loop).unwrap();

    let mut game_state = GameState::new();
    let mut render_state = RenderState::new(&window, &game_state).await;

    window
        .set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
        .unwrap();
    window.set_cursor_visible(false);

    let mut last_update_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                game_state.handle_mouse_motion(delta.0, delta.1);
            }
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { state, virtual_keycode: Some(keycode), .. },
                        ..
                    } => {
                        if *keycode == VirtualKeyCode::Escape {
                            *control_flow = ControlFlow::Exit;
                        } else {
                            game_state.handle_keyboard(*keycode, *state);
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

                // Update game state
                game_state.update(dt);

                // Update render state with new camera data
                render_state.update_camera(
                    game_state.camera_position(),
                    game_state.camera_direction(),
                    game_state.camera_up()
                );

                // Render frame
                match render_state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => { render_state.resize(render_state.size) }
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

fn main() {
    pollster::block_on(run());
}
