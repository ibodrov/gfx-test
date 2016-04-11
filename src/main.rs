extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_app;

pub use gfx_app::{ColorFormat, DepthFormat};

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
    out_color: gfx::RenderTarget<ColorFormat> = "Target0",
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

struct App<R: gfx::Resources>{
    bundle: pipe::Bundle<R>,
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(mut factory: F, init: gfx_app::Init<R>) -> Self {
        use gfx::traits::FactoryExt;

        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/triangle_150.glslv"),
            .. gfx_app::shade::Source::empty()
        };

        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/triangle_150.glslf"),
            .. gfx_app::shade::Source::empty()
        };

        let instance_cols = 800 / TILE_SIZE;
        let instance_rows = 520 / TILE_SIZE;
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

        let pso = factory.create_pipeline_simple(
            vs.select(init.backend).unwrap(),
            ps.select(init.backend).unwrap(),
            gfx::state::CullFace::Back,
            pipe::new()
        ).unwrap();

        let proj = cgmath::ortho(0.0, 800.0, 520.0, 0.0, -1.0, 1.0);

        let data = pipe::Data {
            vbuf: vertex_buffer,
            instance: quad_instances,
            transform: proj.into(),
            out_color: init.color,
        };

        App {
            bundle: pipe::bundle(slice, pso, data),
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        encoder.clear(&self.bundle.data.out_color, CLEAR_COLOR);
        self.bundle.encode(encoder);
    }
}

pub fn main() {
    use gfx_app::Application;
    App::launch_default("Tiles");
}
