use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use gfxperiment::color::Color;
use gfxperiment::gfx::Gfx;
use gfxperiment::quad::{ Quad, QuadRenderer };

const WINDOW_SIZE: winit::dpi::PhysicalSize<u32> = winit::dpi::PhysicalSize {
    width: 600,
    height: 400,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Graphics experiments")
        .with_resizable(false)
        .with_inner_size(WINDOW_SIZE)
        .build(&event_loop)
        .unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut gfx = Gfx::new(&window).await;
    let mut quad_renderer = QuadRenderer::new(&mut gfx);

    let quads = vec![
        Quad { pos: [100., 100.], width: 30., height: 30., color: Color::WHITE },
        Quad { pos: [200., 200.], width: 40., height: 60., color: Color::RED },
        Quad { pos: [500., 300.], width: 80., height: 40., color: Color::BLUE },
    ];

    Ok(event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("The close button was pressed; stopping");
            elwt.exit();
        }
        Event::AboutToWait => {
            let output = gfx.surface.get_current_texture().expect("Failed to get output texture");
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = gfx
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                quad_renderer.render(&mut gfx, &mut render_pass, &quads);
            }

            let command_buffer = encoder.finish();
            gfx.queue.submit(std::iter::once(command_buffer));
            output.present();
        }
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            println!("{:?}", position);
        }
        _ => (),
    })?)
}
