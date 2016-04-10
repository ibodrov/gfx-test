#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_vertex_struct!(Vertex {
    pos: [i32; 2] = "a_Pos",
    color: [f32; 4] = "a_Color",
});

gfx_pipeline!(pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
    out: gfx::RenderTarget<ColorFormat> = "Target0",
});

const TILE_SIZE: i32 = 32;
const TILE_X: i32 = 10 * TILE_SIZE;
const TILE_Y: i32 = 10 * TILE_SIZE;

const QUAD: [Vertex; 4] = [
    Vertex { pos: [ TILE_X + -1 * TILE_SIZE, TILE_Y +  1 * TILE_SIZE ], color: [1.0, 0.0, 0.0, 1.0] },
    Vertex { pos: [ TILE_X +  1 * TILE_SIZE, TILE_Y +  1 * TILE_SIZE ], color: [0.0, 1.0, 0.0, 1.0] },
    Vertex { pos: [ TILE_X + -1 * TILE_SIZE, TILE_Y + -1 * TILE_SIZE ], color: [0.0, 0.0, 1.0, 1.0] },
    Vertex { pos: [ TILE_X +  1 * TILE_SIZE, TILE_Y + -1 * TILE_SIZE ], color: [1.0, 0.0, 0.0, 1.0] },
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        gfx::state::CullFace::Nothing,
        pipe::new()
    ).unwrap();

    let quad_index = vec![0, 1, 2, 1, 3, 2u16];

    let (vertex_buffer, slice) = factory.create_vertex_buffer_indexed(&QUAD, &quad_index[..]);
    let proj = cgmath::ortho(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        transform: proj.into(),
        out: main_color
    };

    'main: loop {
        // loop over events
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }
        // draw a frame
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
