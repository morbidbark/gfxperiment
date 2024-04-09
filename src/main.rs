use wgpu::{util::DeviceExt, TextureViewDimension};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WINDOW_SIZE: winit::dpi::PhysicalSize<f32> = winit::dpi::PhysicalSize {
    width: 600.0,
    height: 400.0,
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
    let size = window.inner_size();

    event_loop.set_control_flow(ControlFlow::Poll);

    let gfx = Gfx::new(window);

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

    // Load test texture and add to bind group
    let tex_bytes = include_bytes!("testtexture.png");
    let tex_image = image::load_from_memory(tex_bytes).unwrap();
    let tex_rgba = tex_image.to_rgba8();
    use image::GenericImageView;
    let tex_dims = tex_image.dimensions();
    let tex_size = wgpu::Extent3d {
        width: tex_dims.0,
        height: tex_dims.1,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: tex_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &tex_rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * tex_dims.0),
            rows_per_image: Some(tex_dims.1),
        },
        tex_size,
    );
    let tex_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let tex_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: None,
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&tex_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&tex_sampler),
            },
        ],
    });

    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
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
