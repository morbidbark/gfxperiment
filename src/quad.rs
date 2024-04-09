use crate::util::{Color, Vertex};
use crate::gfx::Render;

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

#[derive(Default)]
struct SolidQuad {
    /// Size of the quad given in physical pixels
    size: winit::dpi::PhysicalSize<f32>,
    /// Position of quad given in physical pixles
    position: winit::dpi::PhysicalPosition<f32>,
    color: Color,
}
impl SolidQuad {
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
    fn raw(&mut self, window_size: winit::dpi::PhysicalSize<u32>) -> QuadRaw {
        let position = [
            (self.position.x / window_size.width as f32 * 2.0) - 1.0,
            (self.position.y / window_size.height as f32 * -2.0) + 1.0,
        ];
        let scale = [
            self.size.width / window_size.width as f32,
            self.size.height / window_size.height as f32,
        ];
        let color = [self.color.r, self.color.g, self.color.b, self.color.a];
        QuadRaw {
            position,
            scale,
            color,
        }
    }
}

impl Render for SolidQuad {
    fn setup_pipeline() {
    }
    fn render() {}
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct QuadRaw {
    position: [f32; 2],
    scale: [f32; 2],
    color: [f32; 4],
}

