use wgpu::util::DeviceExt;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

const WINDOW_SIZE: winit::dpi::PhysicalSize<f32> = winit::dpi::PhysicalSize {
    width: 600.0,
    height: 400.0,
};

// QUAD
const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
];
const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}
impl Color {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    fn with_alpha(&mut self, a: f32) -> &mut Self {
        self.a = a;
        self
    }

    const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

#[derive(Default)]
struct Quad {
    /// Size of the quad given in physical pixels
    size: winit::dpi::PhysicalSize<f32>,
    /// Position of quad given in physical pixles
    position: winit::dpi::PhysicalPosition<f32>,
    color: Color,
}
impl Quad {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.size = winit::dpi::PhysicalSize::new(width, height);
        self
    }
    pub fn with_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.position = winit::dpi::PhysicalPosition::new(x, y);
        self
    }
    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }
    fn raw(&mut self) -> QuadRaw {
        let position = [
            (self.position.x / WINDOW_SIZE.width * 2.0) - 1.0,
            (self.position.y / WINDOW_SIZE.height * -2.0) + 1.0,
        ];
        let scale = [
            self.size.width / WINDOW_SIZE.width,
            self.size.height / WINDOW_SIZE.height,
        ];
        let color = [self.color.r, self.color.g, self.color.b, self.color.a];
        QuadRaw {
            position,
            scale,
            color,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct QuadRaw {
    position: [f32; 2],
    scale: [f32; 2],
    color: [f32; 4],
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Graphics experiments")
        .with_resizable(false)
        .with_inner_size(WINDOW_SIZE)
        .build(&event_loop)
        .unwrap();
    let size = window.inner_size();

    event_loop.set_control_flow(ControlFlow::Poll);

    // WGPU inital setup
    let instance_desc = wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: wgpu::InstanceFlags::all(),
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    };
    let instance = wgpu::Instance::new(instance_desc);
    let surface = instance.create_surface(window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();
    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
        },
    );

    // Setup buffers, shaders, and render pipeline for rendering a triangle!!
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        usage: wgpu::BufferUsages::VERTEX,
        contents: bytemuck::cast_slice(&VERTICES),
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        usage: wgpu::BufferUsages::INDEX,
        contents: bytemuck::cast_slice(&INDICES),
    });
    let instances = [
        Quad::new()
            .with_size(100.0, 100.0)
            .with_position(100.0, 100.0)
            .with_color(Color::default())
            .raw(),
        Quad::new()
            .with_size(200.0, 300.0)
            .with_position(400.0, 400.0)
            .with_color(Color::RED)
            .raw(),
        Quad::new()
            .with_size(150.0, 80.0)
            .with_position(400.0, 100.0)
            .with_color(Color::BLUE)
            .raw(),
    ];
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        usage: wgpu::BufferUsages::VERTEX,
        contents: bytemuck::cast_slice(&instances),
    });

    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                // Vertex buffer stuff
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                },
                // Instance buffer stuff
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Quad>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 4,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 5,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                            shader_location: 6,
                        },
                    ],
                },
            ],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8Unorm,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });

    Ok(event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("The close button was pressed; stopping");
            elwt.exit();
        }
        Event::AboutToWait => {
            let output = surface.get_current_texture().expect("qweqwe");
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
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
                render_pass.set_pipeline(&render_pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..instances.len() as u32);
            }

            let command_buffer = encoder.finish();
            queue.submit(std::iter::once(command_buffer));
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
