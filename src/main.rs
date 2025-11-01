mod math;
mod scene;
mod renderer;
mod shaders;
mod utils;

use std::time::Instant;

use math::{Vec3, viewport};
use renderer::{Framebuffer, Uniforms, PlanetParams};
use renderer::pipeline::draw_mesh;
use scene::{load_obj, Camera, Input, Action};
use shaders::{make_shader, ShaderKind};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), String> {
    // Ventana 
    let width: u32 = 960;
    let height: u32 = 540;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Lab 04 - Static Shaders (Rust)")
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .map_err(|e| format!("Window: {e:?}"))?;

    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(width, height, surface_texture)
        .map_err(|e| format!("Pixels: {e}"))?;

    // Framebuffer
    let mut fb = Framebuffer::new(width as usize, height as usize);

    // Carga esfera
    let mesh = load_obj("assets/sphere.obj")?;
    println!("OK sphere: {} vértices, {} triángulos", mesh.vertices.len(), mesh.indices.len());
    
    // DEBUG: Verificar bounds del mesh
    if !mesh.vertices.is_empty() {
        let first = mesh.vertices[0].pos;
        println!("Primer vértice: ({}, {}, {})", first.x, first.y, first.z);
    }

    // Cámara 
    let mut cam = Camera::default();
    cam.eye = Vec3::new(0.0, 0.0, 4.0); 
    cam.center = Vec3::new(0.0, 0.0, 0.0);
    cam.set_aspect(width as f32 / height as f32);
    
    println!("Cámara inicial: eye={:?}, center={:?}", cam.eye, cam.center);

    // Input 
    let mut input = Input::new();

    // Uniforms base
    let mut uniforms = Uniforms {
        time: 0.0,
        light_dir: Vec3::new(0.5, 0.7, 0.2).normalize(),
        view: cam.view(),
        proj: cam.proj(),
        model: math::Mat4::identity(),
        camera_pos: cam.eye,
        planet: PlanetParams::default(),
    };

    // Estado app
    let mut shader_kind = ShaderKind::Rocky; // ← por defecto Rocky
    let mut show_rings = true;
    let mut show_moon = true;
    let mut show_all = true;
    let mut running = true;

    let mut last = Instant::now();
    let mut frame_count = 0;
    let mut saved_screenshot = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    running = false;
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input: KeyboardInput { state, virtual_keycode: Some(vk), .. }, .. } => {
                    let is_down = state == ElementState::Pressed;
                    let action_opt = match vk {
                        // Movimiento
                        VirtualKeyCode::W => Some(Action::MoveForward),
                        VirtualKeyCode::S => Some(Action::MoveBackward),
                        VirtualKeyCode::A => Some(Action::MoveLeft),
                        VirtualKeyCode::D => Some(Action::MoveRight),
                        VirtualKeyCode::Space => Some(Action::MoveUp),
                        VirtualKeyCode::LShift => Some(Action::MoveDown),

                        // Rotación cámara
                        VirtualKeyCode::Left  => Some(Action::YawLeft),
                        VirtualKeyCode::Right => Some(Action::YawRight),
                        VirtualKeyCode::Up    => Some(Action::PitchUp),
                        VirtualKeyCode::Down  => Some(Action::PitchDown),

                        // Shaders (1..5)
                        VirtualKeyCode::Key1 => Some(Action::Shader1),
                        VirtualKeyCode::Key2 => Some(Action::Shader2),
                        VirtualKeyCode::Key3 => Some(Action::Shader3),
                        VirtualKeyCode::Key4 => Some(Action::Shader4),
                        VirtualKeyCode::Key5 => Some(Action::Shader5),
                        VirtualKeyCode::Numpad1 => Some(Action::Shader1),
                        VirtualKeyCode::Numpad2 => Some(Action::Shader2),
                        VirtualKeyCode::Numpad3 => Some(Action::Shader3),
                        VirtualKeyCode::Numpad4 => Some(Action::Shader4),
                        VirtualKeyCode::Numpad5 => Some(Action::Shader5),

                        // Mostrar todos / toggles
                        VirtualKeyCode::Key0 => { if is_down { show_all = !show_all; } None },
                        VirtualKeyCode::Numpad0 => { if is_down { show_all = !show_all; } None },
                        VirtualKeyCode::R => Some(Action::ToggleRings),
                        VirtualKeyCode::M => Some(Action::ToggleMoon),

                        // Util
                        VirtualKeyCode::P => Some(Action::Screenshot),
                        VirtualKeyCode::Escape => Some(Action::Quit),
                        _ => None,
                    };
                    if let Some(a) = action_opt {
                        println!("Key event: {:?} down={} action={:?}", vk, is_down, a);
                        if is_down { input.action_down(a); } else { input.action_up(a); }
                    } else {
                        println!("Key event: {:?} down={} (no action)", vk, is_down);
                    }
                }
                WindowEvent::Resized(size) => {
                    let _ = pixels.resize_surface(size.width, size.height);
                    let _ = pixels.resize_buffer(size.width, size.height);
                    cam.set_aspect(size.width as f32 / size.height as f32);
                }
                _ => {}
            },

            // Lógica y pedir redraw
            Event::MainEventsCleared => {
                if !running { *control_flow = ControlFlow::Exit; return; }

                let now = Instant::now();
                let dt = (now - last).as_secs_f32();
                last = now;

                if input.is_pressed(Action::Quit) { *control_flow = ControlFlow::Exit; return; }

                // Mapear 1..5 → Rocky, Gas, Ice, Moon, Rings
                if input.is_pressed(Action::Shader1) { println!("Action pressed: Shader1"); shader_kind = ShaderKind::Rocky; show_all = false; }
                if input.is_pressed(Action::Shader2) { println!("Action pressed: Shader2"); shader_kind = ShaderKind::Gas;   show_all = false; }
                if input.is_pressed(Action::Shader3) { println!("Action pressed: Shader3"); shader_kind = ShaderKind::Ice;   show_all = false; }
                if input.is_pressed(Action::Shader4) { println!("Action pressed: Shader4"); shader_kind = ShaderKind::Moon;  show_all = false; }
                if input.is_pressed(Action::Shader5) { println!("Action pressed: Shader5"); shader_kind = ShaderKind::Rings; show_all = false; }

                if input.is_pressed(Action::ToggleRings) { show_rings = !show_rings; }
                if input.is_pressed(Action::ToggleMoon)  { show_moon  = !show_moon;  }

                update_camera(&mut cam, &input, dt);

                uniforms.time += dt;
                uniforms.view = cam.view();
                uniforms.proj = cam.proj();
                uniforms.camera_pos = cam.eye;

                window.request_redraw();
            }

            // Render 
            Event::RedrawRequested(_) => {
                frame_count += 1;

                let size = window.inner_size();
                let fw = size.width as usize;
                let fh = size.height as usize;

                if fb.width != fw || fb.height != fh {
                    fb = Framebuffer::new(fw, fh);
                    cam.set_aspect(fw as f32 / fh as f32);
                }

                let vp = viewport(0.0, 0.0, fw as f32, fh as f32, 1.0);

                // Render 
                fb.clear_color(renderer::buffers::Color::rgb(5, 8, 12));
                fb.clear_depth();

                // DEBUG
                if frame_count % 60 == 0 {
                    println!("Frame {}: cam.eye={:?}, triángulos={}", 
                             frame_count, cam.eye, mesh.indices.len());
                }

                if show_all {
                    // Rocky
                    let mut rocky = crate::shaders::planeta_rocosso::Rocky::default();
                    rocky.orbit_radius = 2.2; 
                    rocky.orbit_speed = 0.6;  
                    rocky.rot_speed = 0.6; 
                    rocky.orbit_world = true;
                    let mut u_rocky = uniforms;
                    u_rocky.model = math::mat::translate(Vec3::new(-3.0, 0.0, 0.0)) * math::mat::scale(Vec3::new(0.85,0.85,0.85));
                    draw_mesh(&mut fb, &mesh, &mut rocky, &u_rocky, vp);

                    // Gas beige
                    let mut gas = make_shader(ShaderKind::Gas);
                    let mut u_gas = uniforms;
                    u_gas.model = math::mat::translate(Vec3::new(-1.5, 0.0, 0.0)) * math::mat::scale(Vec3::new(0.95,0.95,0.95));
                    draw_mesh(&mut fb, &mesh, &mut *gas, &u_gas, vp);

                    // Ice (verde)
                    let mut ice = crate::shaders::planeta_verde::Ice::default();
                    ice.rot_speed = 0.45;
                    let mut u_ice = uniforms;
                    u_ice.model = math::mat::translate(Vec3::new(1.0, 0.0, 0.0)) * math::mat::scale(Vec3::new(0.9,0.9,0.9));
                    draw_mesh(&mut fb, &mesh, &mut ice, &u_ice, vp);

                    // ===== NUEVOS =====
                    // Gaseoso azul vibrante
                    let mut gas_blue = crate::shaders::planeta_gaseoso::PlanetaGaseoso::default();
                    let mut u_gas_blue = uniforms;
                    u_gas_blue.model = math::mat::translate(Vec3::new(2.6, 0.0, 0.0)) * math::mat::scale(Vec3::new(0.95,0.95,0.95));
                    draw_mesh(&mut fb, &mesh, &mut gas_blue, &u_gas_blue, vp);

                    // Planeta rojo
                    let mut red = crate::shaders::planeta_rojo::PlanetaRojo::default();
                    let mut u_red = uniforms;
                    u_red.model = math::mat::translate(Vec3::new(4.5, 0.0, 0.0)) * math::mat::scale(Vec3::new(0.9,0.9,0.9));
                    draw_mesh(&mut fb, &mesh, &mut red, &u_red, vp);
                    // ===================

                    // Overlays opcionales
                    if show_rings {
                        let mut rings = make_shader(ShaderKind::Rings);
                        draw_mesh(&mut fb, &mesh, &mut *rings, &u_gas, vp);
                    }
                    if show_moon {
                        let mut moon = make_shader(ShaderKind::Moon);
                        draw_mesh(&mut fb, &mesh, &mut *moon, &u_rocky, vp);
                    }
                } else {
                    let mut u_center = uniforms;
                    u_center.model = math::mat::translate(Vec3::new(0.0,0.0,0.0)) * math::mat::scale(Vec3::new(1.0,1.0,1.0));

                    match shader_kind {
                        ShaderKind::Rocky => {
                            let mut r = crate::shaders::planeta_rocosso::Rocky::default();
                            r.orbit_radius = 2.2;
                            r.orbit_speed = 0.6;
                            r.rot_speed = 0.6;
                            r.orbit_world = true;
                            draw_mesh(&mut fb, &mesh, &mut r, &u_center, vp);
                            if show_moon {
                                let mut moon = make_shader(ShaderKind::Moon);
                                draw_mesh(&mut fb, &mesh, &mut *moon, &u_center, vp);
                            }
                        }
                        ShaderKind::Gas => {
                            let mut p = make_shader(ShaderKind::Gas);
                            draw_mesh(&mut fb, &mesh, &mut *p, &u_center, vp);
                            if show_rings {
                                let mut rings = make_shader(ShaderKind::Rings);
                                draw_mesh(&mut fb, &mesh, &mut *rings, &u_center, vp);
                            }
                        }
                        ShaderKind::Ice => {
                            let mut p = make_shader(ShaderKind::Ice);
                            draw_mesh(&mut fb, &mesh, &mut *p, &u_center, vp);
                        }
                        ShaderKind::Moon => {
                            let mut p = make_shader(ShaderKind::Moon);
                            draw_mesh(&mut fb, &mesh, &mut *p, &u_center, vp);
                        }
                        ShaderKind::Rings => {
                            let mut p = make_shader(ShaderKind::Rings);
                            draw_mesh(&mut fb, &mesh, &mut *p, &u_center, vp);
                        }

                        // ===== NUEVOS (si tu enum ya los tiene) =====
                        #[allow(non_camel_case_types)]
                        ShaderKind::GasBlue => {
                            let mut p = crate::shaders::planeta_gaseoso::PlanetaGaseoso::default();
                            draw_mesh(&mut fb, &mesh, &mut p, &u_center, vp);
                        }
                        #[allow(non_camel_case_types)]
                        ShaderKind::Red => {
                            let mut p = crate::shaders::planeta_rojo::PlanetaRojo::default();
                            draw_mesh(&mut fb, &mesh, &mut p, &u_center, vp);
                        }
                        // ============================================
                    }
                }

                if input.is_pressed(Action::Screenshot) {
                    std::fs::create_dir_all("screenshots").ok();
                    let path = format!("screenshots/frame_{:.0}.png", uniforms.time*1000.0);
                    fb.save_png(&path).ok();
                    println!("Guardado: {path}");
                }

                if !saved_screenshot {
                    std::fs::create_dir_all("screenshots").ok();
                    let path = "screenshots/auto_first_frame.png".to_string();
                    match fb.save_png(&path) {
                        Ok(_) => println!("Auto-screenshot guardada: {}", path),
                        Err(e) => eprintln!("No pude guardar auto-screenshot: {}", e),
                    }
                    saved_screenshot = true;
                }

                let frame = pixels.frame_mut();
                let px_count = (frame.len() / 4).min(fb.color.len());
                for i in 0..px_count {
                    let c = fb.color[i];
                    let o = i * 4;
                    frame[o + 0] = c.r;
                    frame[o + 1] = c.g;
                    frame[o + 2] = c.b;
                    frame[o + 3] = c.a;
                }

                if let Err(e) = pixels.render() {
                    eprintln!("pixels.render: {e}");
                }
                input.begin_frame();
            }

            _ => {}
        }
    });
}

// Helpers 
fn update_camera(cam: &mut Camera, input: &Input, dt: f32) {
    let fwd   = (input.is_held(Action::MoveForward) as i32 - input.is_held(Action::MoveBackward) as i32) as f32;
    let right = (input.is_held(Action::MoveRight)   as i32 - input.is_held(Action::MoveLeft)    as i32) as f32;
    let up    = (input.is_held(Action::MoveUp)      as i32 - input.is_held(Action::MoveDown)    as i32) as f32;
    cam.move_free(fwd, right, up, dt);

    let yaw   = (input.is_held(Action::YawRight) as i32 - input.is_held(Action::YawLeft)  as i32) as f32;
    let pitch = (input.is_held(Action::PitchUp)  as i32 - input.is_held(Action::PitchDown)as i32) as f32;
    cam.rotate_free(yaw, pitch, dt);
}
