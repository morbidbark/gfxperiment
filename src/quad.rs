use wgpu::util::DeviceExt;
use crate::color::Color;
use crate::gfx::{ Gfx, GfxRenderData, Renderer };
use crate::vertex::Vertex;

const VERTICES: [Vertex; 4] = [
    // Top-Left
    Vertex {
        pos: [-1.0, 1.0],
        uv: [0.0, 0.0],
    },
    // Top-Right
    Vertex {
        pos: [1.0, 1.0],
        uv: [1.0, 0.0],
    },
    // Bottom-Right
    Vertex {
        pos: [1.0, -1.0],
        uv: [1.0, 1.0],
    },
    // Bottom-Left
    Vertex {
        pos: [-1.0, -1.0],
        uv: [0.0, 1.0],
    },
];
const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

pub struct Quad {
    pub pos: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct QuadRaw {
    pos: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
}
impl From<&Quad> for QuadRaw {
    fn from(quad: &Quad) -> Self {
        Self {
            pos: quad.pos,
            size: [quad.width, quad.height],
            color: [quad.color.r, quad.color.g, quad.color.b, quad.color.a],
        }
    }
}

pub struct QuadRenderer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    quads: Vec<Quad>,
}
impl QuadRenderer {
    pub fn new(gfx: &mut Gfx) -> Self {
        let gfx = gfx.data.borrow_mut();
        let vertex_buffer = gfx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                usage: wgpu::BufferUsages::VERTEX,
                contents: bytemuck::cast_slice(&VERTICES),
            });
        let index_buffer = gfx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                usage: wgpu::BufferUsages::INDEX,
                contents: bytemuck::cast_slice(&INDICES),
            });
        let instance_buffer = gfx.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (std::mem::size_of::<QuadRaw>() * 128) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let shader = gfx
            .device
            .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let pipeline_layout = gfx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &gfx.aspect_ratio_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let pipeline = gfx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        // Vertex buffer stuff
                        Vertex::layout(),
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
        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            pipeline,
            quads: vec![],
        }
    }
    pub fn add(&mut self, quad: Quad) {
        self.quads.push(quad);
    }
}
impl Renderer for QuadRenderer {
    fn render<'a, 'b>(
        &'a self,
        data: &'a GfxRenderData,
        render_pass: &mut wgpu::RenderPass<'b>,
    )
    where
        'a: 'b
    {
        let instances = self.quads
            .iter()
            .map(|quad| quad.into())
            .collect::<Vec<QuadRaw>>();
        data.queue
            .write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_bind_group(0, &data.aspect_ratio_bind_group, &[]);
        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..instances.len() as u32);
    }
}
