use std::vec;
use std::sync::Arc;

use ggez;
use ggez::{ContextBuilder, GameResult, Context, graphics};
use ggez::event;
use ggez::event::EventHandler;
use ggez::mint;
use rand::Rng;
use vulkano::image::{StorageImage, ImageDimensions};
use vulkano::image::view::ImageView;
use vulkano::instance::{Instance, InstanceCreateInfo};
// use ggez::graphics::AsStd140;

const ELEMENT_SIZE: f32 = 10.0;

// #[derive(AsStd140)]
// struct World {
//     scale: u32,
//     world_dim: mint::Vector2<u32>,
// }

#[derive(Copy, Clone)]
struct Element {
    color: ggez::graphics::Color,
}

struct MainState {
    world_width: i32,
    world_height: i32, 
    world: Vec<Vec<Element>>,
    rect: graphics::Rect,
    mesh: graphics::Mesh,
    image_buffer: Vec::<u8>,
    shader: graphics::Shader,
    // shader_params: graphics::ShaderParams<World>,
    // image: StorageImage,
    // view: ImageView<StorageImage>,
    vulkano: Arc<Instance>,
}

impl MainState {
    pub fn new(ctx: &mut Context, world_width: i32, world_height: i32) -> Self {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let rect = graphics::Rect::new(0.0, 0.0, ELEMENT_SIZE, ELEMENT_SIZE);
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, ggez::graphics::Color::WHITE).unwrap();
        let mut row: Vec<Element> = vec![];
        for _x in 0..world_width {
            row.push(Element{color: graphics::Color::WHITE})
        }
        let mut world_vec = vec![];
        for _y in 0..world_height {
            world_vec.push(row.clone());
        }
        let mut image_buffer = vec![255; (world_height * world_width * 4) as usize];
        // let mut image = graphics::Image::from_pixels(&ctx.gfx, &image_buffer, graphics::ImageFormat::Rgba8Uint, world_width as u32, world_height as u32);
        let shader = graphics::Shader::from_wgsl(ctx, include_str!("../resources/dimmer.wgsl"), "main");
        let contents = vec![0 as u32; (world_height * world_width) as usize];
        // let shader_params = World{
        //     world_dim: mint::Vector2{x: screen_w, y: screen_h},
        //     scale: 4 as u32

        // };

        let instance = Instance::new(InstanceCreateInfo::default()).expect("failed to create instance");


        // let image = StorageImage::new(
        //     device.clone(),
        //     ImageDimensions::Dim2d {
        //         width: 1024,
        //         height: 1024,
        //         array_layers: 1,
        //     },
        //     Format::R8G8B8A8_UNORM,
        //     Some(queue.family()),
        // )
        // .unwrap();

        // let view = ImageView::new_default(image.clone()).unwrap();


        MainState {
            world: world_vec,
            rect,
            mesh,
            world_width,
            world_height,
            image_buffer,
            shader,
            vulkano: instance,
            // image,
            // view
            // shader_params,
            
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = ctx.time.delta().as_secs_f32();
        let fps = ctx.time.fps();
        println!("{}", fps);

        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let draw_params = graphics::DrawParam::default();
        let draw_pos = mint::Vector2::<f32>{x: 100.0, y: 100.0};
        draw_params.dest(draw_pos);
        let mut rng = rand::thread_rng();
        for y in 0..self.world.len() {
            for x in 0..self.world[0].len() {
                let index = y * self.world_width as usize * 4 + x * 4;
                self.image_buffer[index] = rng.gen();
                self.image_buffer[index + 1] = rng.gen();
                self.image_buffer[index + 2] = rng.gen();
                self.image_buffer[index + 3] = rng.gen();
            }
        }

        // self.image = graphics::Image::from_rgba8(ctx, self.world_width as u16, self.world_height as u16, &self.image_buffer).unwrap();


        // self.image.draw(&mut canvas, draw_params);
        
        // let draw_params = draw_params.dest(self.player_1_pos);
        // graphics::draw(ctx, &racket_mesh, draw_params)?;

        // let draw_params = draw_params.dest(self.player_2_pos);
        // graphics::draw(ctx, &racket_mesh, draw_params)?;

        // let draw_params = draw_params.dest(self.ball_pos);
        // graphics::draw(ctx, &ball_mesh, draw_params)?;


        canvas.finish(ctx)?;
        Ok(())
    }
}


fn main() -> GameResult {
    let cb = ContextBuilder::new("falling_sand_rust", "Davidm");
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("falling_sand_rust");
    let mode = ggez::conf::WindowMode{
        width: 800.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: true,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: false,
        transparent: false,
        logical_size: Option::None
    };
    ctx.gfx.set_mode(mode)?;

    let state = MainState::new(&mut ctx, 300, 100);

    event::run(ctx, event_loop, state);
}


// //! A very simple shader example.

// use ggez::glam::Vec2;
// use ggez::graphics::{self, Color, DrawMode};
// use ggez::{event, graphics::AsStd140};
// use ggez::{Context, GameResult};
// use std::env;
// use std::path;

// #[derive(AsStd140)]
// struct Dim {
//     rate: f32,
// }

// struct MainState {
//     dim: Dim,
//     shader: graphics::Shader,
//     // params: graphics::ShaderParams<Dim>,
// }

// impl MainState {
//     fn new(ctx: &mut Context) -> GameResult<MainState> {
//         let dim = Dim { rate: 0.5 };
//         let shader =
//             graphics::Shader::from_wgsl(ctx, include_str!("../resources/dimmer.wgsl"), "main");
//         // let params = graphics::ShaderParams::new(ctx, &{}, &[], &[]);
//         Ok(MainState {
//             dim,
//             shader,
//             // params,
//         })
//     }
// }

// impl event::EventHandler<ggez::GameError> for MainState {
//     fn update(&mut self, ctx: &mut Context) -> GameResult {
//         self.dim.rate = 0.5 + (((ctx.time.ticks() as f32) / 100.0).cos() / 2.0);
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

//         let circle = graphics::Mesh::new_circle(
//             ctx,
//             DrawMode::fill(),
//             Vec2::new(100.0, 300.0),
//             100.0,
//             2.0,
//             Color::WHITE,
//         )?;
//         canvas.draw(&circle, Vec2::new(0.0, 0.0));

//         // self.params.set_uniforms(ctx, &self.dim);
//         canvas.set_shader(self.shader.clone());
//         // canvas.set_shader_params(self.params.clone());
//         let circle = graphics::Mesh::new_circle(
//             ctx,
//             DrawMode::fill(),
//             Vec2::new(400.0, 300.0),
//             100.0,
//             2.0,
//             Color::WHITE,
//         )?;
//         canvas.draw(&circle, Vec2::new(0.0, 0.0));

//         canvas.set_default_shader();
//         let circle = graphics::Mesh::new_circle(
//             ctx,
//             DrawMode::fill(),
//             Vec2::new(700.0, 300.0),
//             100.0,
//             2.0,
//             Color::WHITE,
//         )?;
//         canvas.draw(&circle, Vec2::new(0.0, 0.0));

//         canvas.finish(ctx)
//     }
// }

// pub fn main() -> GameResult {
//     let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
//         let mut path = path::PathBuf::from(manifest_dir);
//         path.push("resources");
//         path
//     } else {
//         path::PathBuf::from("./resources")
//     };

//     let cb = ggez::ContextBuilder::new("shader", "ggez").add_resource_path(resource_dir);
//     let (mut ctx, event_loop) = cb.build()?;

//     let state = MainState::new(&mut ctx)?;
//     event::run(ctx, event_loop, state)
// }
