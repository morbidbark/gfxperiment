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
    let mut quad_renderer = Box::new(QuadRenderer::new(&mut gfx));

    let quads = vec![
        Quad { pos: [100., 100.], width: 30., height: 30., color: Color::WHITE },
        Quad { pos: [200., 200.], width: 40., height: 60., color: Color::RED },
        Quad { pos: [500., 300.], width: 80., height: 40., color: Color::BLUE },
    ];
    for quad in quads {
        quad_renderer.add(quad);
    }
    gfx.add_renderer(quad_renderer);

    Ok(event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("The close button was pressed; stopping");
            elwt.exit();
        }
        Event::AboutToWait => {
            gfx.draw();
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
