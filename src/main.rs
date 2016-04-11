#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;
extern crate time;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_vertex_struct!(Vertex {
    pos: [i32; 2] = "a_Pos",
    color: [f32; 4] = "a_Color",
});

gfx_vertex_struct!(Instance {
    translate: [i32; 2] = "a_Translate",
});

gfx_pipeline!(pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    instance: gfx::InstanceBuffer<Instance> = (),
    transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
    out: gfx::RenderTarget<ColorFormat> = "Target0",
});

const TILE_SIZE: i32 = 16;

const QUAD: [Vertex; 4] = [
    Vertex { pos: [ 0,         TILE_SIZE ], color: [1.0, 0.0, 0.0, 1.0] },
    Vertex { pos: [ TILE_SIZE, TILE_SIZE ], color: [0.0, 1.0, 0.0, 1.0] },
    Vertex { pos: [ 0,         0 ], color: [0.0, 0.0, 1.0, 1.0] },
    Vertex { pos: [ TILE_SIZE, 0 ], color: [0.0, 0.0, 0.0, 1.0] },
];

const QUAD_INDEX: &'static [u16] = &[0, 1, 2, 1, 3, 2];

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn main() {
    use gfx::traits::Factory;

    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768);

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        gfx::state::CullFace::Nothing,
        pipe::new()
    ).unwrap();

    let instance_cols = 1024 / TILE_SIZE;
    let instance_rows = 768 / TILE_SIZE;
    let instance_count = (instance_rows * instance_cols) as u32;

    let quad_instances = {
        let mut attributes = (0..instance_count).map(|_| Instance { translate: [0, 0] }).collect::<Vec<_>>();
        for i in 0..instance_rows {
            for j in 0..instance_cols {
                let idx = (i * instance_cols + j) as usize;
                let t = &mut attributes[idx].translate;
                t[0] = j * TILE_SIZE;
                t[1] = i * TILE_SIZE;
            }
        } 
        factory.create_buffer_const(&attributes, gfx::BufferRole::Vertex, gfx::Bind::empty()).unwrap()
    };

    let (vertex_buffer, mut slice) = factory.create_vertex_buffer_indexed(&QUAD, QUAD_INDEX);
    slice.instances = Some((instance_count, 0));
    let proj = cgmath::ortho(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        instance: quad_instances,
        transform: proj.into(),
        out: main_color
    };

    let mut t1 = 0.0;
    let mut frames = 0;

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

        frames += 1;

        let t2 = time::precise_time_s();
        if t2 - t1 > 1.0 {
            t1 = t2;
            window.set_title(&format!("FPS: {}", frames));
            frames = 0;
        }
    }
}
