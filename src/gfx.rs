use std::cell::RefCell;
use wgpu::util::DeviceExt;

pub trait Renderer {
    fn render<'a, 'b>(&'a self, data: &'a GfxRenderData, render_pass: &mut wgpu::RenderPass<'b>) where 'a: 'b;
}
pub struct GfxRenderData<'a> {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'a>,
    pub aspect_ratio_bind_group: wgpu::BindGroup,
    pub aspect_ratio_bind_group_layout: wgpu::BindGroupLayout,
}

pub struct Gfx<'a> {
    pub data: RefCell<GfxRenderData<'a>>,
    renderers: Vec<Box<dyn Renderer>>,
}
impl<'a> Gfx<'a> {
    pub async fn new(window: &'a winit::window::Window) -> Self {
        let size = window.inner_size();
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
        let aspect_ratio_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[size.width, size.height]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let aspect_ratio_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let aspect_ratio_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &aspect_ratio_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: aspect_ratio_buffer.as_entire_binding() },
            ],
        });
        let data = GfxRenderData {
            size,
            device,
            queue,
            surface,
            aspect_ratio_bind_group,
            aspect_ratio_bind_group_layout,
        };
        Self {
            data: RefCell::new(data),
            renderers: vec![],
        }
    }
    pub fn add_renderer(&mut self, renderer: Box<dyn Renderer>) -> usize {
        self.renderers.push(renderer);
        self.renderers.len() - 1
    }
    pub fn draw(&mut self) {
        let data = self.data.borrow_mut();
        let output = data.surface.get_current_texture().expect("Failed to get output texture");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = data
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

            for renderer in self.renderers.iter() {
                renderer.render(&data, &mut render_pass);
            }
        }

        let command_buffer = encoder.finish();
        data.queue.submit(std::iter::once(command_buffer));
        output.present();
    }
}
