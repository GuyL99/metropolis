use crate::setup::*;
use crate::vertex::*;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState,AutoCommandBuffer};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain;
use vulkano::swapchain::{AcquireError, SwapchainCreationError};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use winit::Window;
use winit::{ModifiersState,KeyboardInput,ElementState,Event, WindowEvent,VirtualKeyCode};
use winit::dpi::LogicalPosition;
use winit::MouseScrollDelta;
use crate::text::{DrawText, DrawTextTrait};
use std::time::{Duration, Instant};
use vulkano::image::{ImmutableImage, Dimensions};
use vulkano::sampler::{Sampler, SamplerAddressMode, Filter, MipmapMode};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::format::Format;
use crate::color::Color;
use crate::mapping;
use crate::mapping::*;
use image::*;
use crate::math::{bezier_points, catmull_rom_chain};
pub use winit::VirtualKeyCode as keyCode;
pub use winit::MouseButton;
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}
///This struct is meant for loading and saving the image once and not every frame, it improves
///framerate in aout 12 fps.
#[derive(Clone)]
pub struct Image{
    pub image_data:Vec<u8>,
    pub dimensions:Dimensions,
}
///the public canvas struct(there is actually an inner one for the static functions). it is
///mutithreading safe but needs a slightly different way to use:
///```
///use metropolis::color::*;
///use metropolis::canvas::Canvas;
///fn main(){
///       let height = 600;
///   let width = 800;
///   //size(width, height);
///   let mut canv:Canvas= Canvas::new(width,height);
///   canv.background(grayscale(100));
///   let draw = |mut canvas:Canvas|->Canvas {
///       let curve_vec: Vec<[i64; 2]> = vec![
///           [0, 400],
///           [30, 370],
///           [50, 300],
///           [75, 257],
///           [80, 240],
///           [150, 150],
///          [250, 050],
///      ];
///       canvas.bezierCurve(curve_vec);
///        canvas
///   };
///   canv.show(draw);
///}
///```
///as you may see the draw loop is designed a bit different.
#[derive(Clone, PartialEq)]
pub struct Canvas {
    size: (u16, u16),
    stroke: bool,
    color: [f32; 4],
    stroke_weight: u8,
    fill: bool,
    fill_color: [f32; 4],
    background_color: [f32; 4],
    pub fps: f32,
    resizeable: bool,
    text_size: f32,
    fill_vec: Vec<Vertex>,     
    text_vec: Vec<Stext>,   
    stroke_vec: Vec<Vertex>,     
    texture:Option<(Vec<u8>,Dimensions)>,
    key:Key,
    cursor_pos:(u16,u16),
    mouse:Mouse,
    mouse_scroll:MouseScroll,
    //draw:FnMut() + 'static, 
}
#[derive(Copy,Clone,PartialEq)]
struct MouseScroll{
    pub delta:(i64,i64),
    pub moder:ModifiersState,
}
impl MouseScroll{
    pub fn new()->MouseScroll{
        let moder = ModifiersState{shift:false,ctrl:false,alt:false,logo:false};    
        let delta = (0,0);
        MouseScroll{delta,moder}
    }
    pub fn delta_x(self)->i64{
        self.delta.0
    }
    pub fn delta_y(self)->i64{
        self.delta.1//.PixelDelta.y as i64
    }
}
#[derive(Copy,Clone,PartialEq)]
struct Mouse{
    pub btn:Option<MouseButton>,
    pub moder:ModifiersState,
}
impl Mouse{
    pub fn new()->Mouse{
        let moder = ModifiersState{shift:false,ctrl:false,alt:false,logo:false};    
        let btn = None;
        Mouse{btn,moder}
    }
}
#[derive(Copy,Clone,PartialEq)]
struct Key{
    pub keycode:Option<VirtualKeyCode>,
    pub moder:ModifiersState,
}
impl Key{
    pub fn new()->Key{
        let moder = ModifiersState{shift:false,ctrl:false,alt:false,logo:false};    
        let keycode = None;
        Key{keycode,moder}
    }
    pub fn get_mod(self)->ModifiersState{
        self.moder
    }
}
impl Canvas {
    ///returns the current key that is pressed on the mouse.
    #[allow(non_snake_case)]
    pub fn mouseClick(&mut self)->MouseButton{
        match self.mouse.btn{
        Some(btn)=> {return btn;},
        None=> {return MouseButton::Other(99);}
        }
    }
    ///returns the current key that is pressed.
    #[allow(non_snake_case)]
    pub fn keyPressed(&mut self)->VirtualKeyCode{
        match self.key.keycode{
        Some(key)=> {return key;},
        None=> {return VirtualKeyCode::Power;}
        }
    }
    ///returns the x scroll delta of the mouse
    #[allow(non_snake_case)]
    pub fn mouseScrollX(&self)->i64{
        self.mouse_scroll.delta_x()
    }
    ///returns the y scroll delta of the mouse
    #[allow(non_snake_case)]
    pub fn mouseScrollY(&self)->i64{
        self.mouse_scroll.delta_y()
    }
    ///returns the x position of the mouse
    #[allow(non_snake_case)]
    pub fn mouseX(&self)->u16{
        self.cursor_pos.0
    }
    ///returns the y position of the mouse
    #[allow(non_snake_case)]
    pub fn mouseY(&self)->u16{
        self.cursor_pos.1
    }
    ///returns the current state of the modifiers
    pub fn get_modifiers(self)->ModifiersState{
        self.key.get_mod()
    }
    ///sets the background color(using the color struct).
    pub fn background(&mut self,color:Color){
        let r = color.get_r();
        let g = color.get_g();
        let b = color.get_b();
        let a = color.get_a();
        self.background_color = mapping::map_colors([r, g, b, a]);
    }
    ///sets the stroke weight(the width of lines and points
    #[allow(non_snake_case)]
    pub fn strokeWeight(&mut self,weight:u8){
        self.stroke_weight = weight;
    }
    ///enables stroke and receives the color of the stroke(the struct color) and sets the stroke color to be
    ///the color.
    pub fn stroke(&mut self,color:Color){
        let r = color.get_r();
        let g = color.get_g();
        let b = color.get_b();
        let a = color.get_a();
        self.stroke = true;
        self.color = mapping::map_colors([r, g, b, a]);
    }
    ///retruns the stroke state of the canvas
    pub fn get_stroke(self)->bool{
        self.stroke
    }
    ///retruns the fill state of the canvas
    pub fn get_fill(self)->bool{
        self.fill
    }
    ///only sets fill on, no need to send in color, last fill color will e used
    pub fn set_fill(&mut self,fill:bool){
        self.fill = fill;
    }
    ///only sets stroke on, no need to send in color, last stroke color will e used
    pub fn set_stroke(&mut self,stroke:bool){
        self.stroke = stroke;
    }
    ///disables stroke on the canvas.
    #[allow(non_snake_case)]
    pub fn noStroke(&mut self){
        self.stroke = false;
    }
    ///disables fill on the canvas.
    #[allow(non_snake_case)]
    pub fn noFill(&mut self) {
        self.fill = false;
    }
    ///enables fill and receives the color of the fill(the struct color) and sets the fill color to be
    ///the color.
    pub fn fill(&mut self, color:Color) {
        let r = color.get_r();
        let g = color.get_g();
        let b = color.get_b();
        let a = color.get_a();
        self.fill = true;
        self.fill_color = mapping::map_colors([r, g, b, a]);
    }
    ///recieves f32 ext size and sets the canvases text_size to that size
    #[allow(non_snake_case)]
    pub fn textSize(&mut self,sz:u8) {
        self.text_size = sz as f32;
    }
    ///creates the canvas with the width and height sent to this function
    pub fn size(&mut self,width: u16, height: u16) {
        self.size = (width, height);
    }
    ///returns the size of the canvas
    pub fn get_size(self)->(u16,u16) {
        self.size
    }
    ///returns the height of the canvas
    pub fn height(self)->u16{
        self.size.1 
    }
    ///returns the width of the canvas
    pub fn width(self)->u16{
        self.size.1 
    }
    ///creates a new canvas surface for rendering
    pub fn new(width:u16,height:u16)->Canvas{
        Canvas{
    size: (width, height),
    stroke: true,
    color: [0.0, 0.0, 0.0, 1.0],
    stroke_weight: 8,
    fill: false,
    fill_color: [1.0, 1.0, 1.0, 1.0],
    background_color: [1.0, 1.0, 1.0, 1.0],
    fps: 60.0,
    resizeable: false,
    text_size: 18.0,
    text_vec: vec![],
    fill_vec: vec![],
    stroke_vec: vec![],
    texture:None,
    key:Key::new(),
    cursor_pos:(0,0),
    mouse:Mouse::new(),
    mouse_scroll:MouseScroll::new(),
        }
    }
    ///this is the function used to run the animation
    pub fn show<F>(mut self, mut draw_fn: F)
    where
        F: FnMut(Self)->Canvas + 'static,
    {
        let set;
        let mut previous_frame_end;
        let (mut env, mut events_loop) = init(self.size.0, self.size.1);
        self = draw_fn(self);
        //draw_fn();
        match self.texture.clone(){
            Some((vec1,dim1))=>{
                let vec_tex = vec1.to_vec();
                let dimensions = dim1;
        let (texture, tex_future) = {
                ImmutableImage::from_iter(
                vec_tex.iter().cloned(),
                dimensions,
                Format::R8G8B8A8Srgb,
                env.queue.clone()
                ).unwrap()
        };
    let sampler = Sampler::new(env.device.clone(), Filter::Linear, Filter::Linear,
    MipmapMode::Nearest, SamplerAddressMode::Repeat, SamplerAddressMode::Repeat,
    SamplerAddressMode::Repeat, 0.0, 1.0, 0.0, 0.0).unwrap();
    set = Some(Arc::new(PersistentDescriptorSet::start(env.tex_pipeline.clone(),0)
    .add_sampled_image(texture.clone(), sampler.clone()).unwrap()
    .build().unwrap()));
        previous_frame_end = Box::new(tex_future) as Box<dyn GpuFuture>;
            },
            None =>{
                set =None;
                previous_frame_end =Box::new(env.previous_frame_end.unwrap());
            }
        };
        let mut text = DrawText::new(env.device.clone(), env.queue.clone(), env.swapchain.clone(), &env.images);
        let mut counter1 = 0;
        let start = Instant::now();
        let mut end;
        let mut recreate_swapchain = env.recreate_swapchain;

        loop {
            let mut done = false;
                previous_frame_end.cleanup_finished();
            events_loop.poll_events(|ev| match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => done = true,
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => recreate_swapchain = true,
                Event::WindowEvent{event,..}=>
                match event{
                    WindowEvent::KeyboardInput{
                        input:KeyboardInput{
                            state: ElementState::Pressed,
                            virtual_keycode: Some(key),
                            modifiers,
                            ..
                        },
                        ..
                } => {
                    if key == VirtualKeyCode::W && modifiers.ctrl{
                        done = true;
                    }
                    self.key = Key{keycode:Some(key),moder:modifiers};
                },
                    WindowEvent::CursorMoved{
                        position:LogicalPosition{x:posx,y:posy},
                        ..
                   }=>{self.cursor_pos = (posx as u16,posy as u16);},
                    WindowEvent::MouseInput{
                            state: ElementState::Pressed,
                            button: button1,
                            modifiers,
                            ..
                } => {
                    self.mouse = Mouse{btn:Some(button1),moder:modifiers};
                },
                    WindowEvent::MouseWheel{
                            delta: MouseScrollDelta::PixelDelta(pos),//{x:posx,y:posy},
                            modifiers,
                            ..
                } => {
                    self.mouse_scroll = MouseScroll{delta:(pos.x as i64,pos.y as i64),moder:modifiers};
                },
                _=>{},
            }
                _ => (),
            });
            if done {
                return;
            }
            env.dynamic_state.line_width = Some(self.stroke_weight as f32);
                let stroke_vertex_buffer = CpuAccessibleBuffer::from_iter(
                    env.device.clone(),
                    BufferUsage::all(),
                    self.stroke_vec.clone().iter().cloned(),
                )
                .unwrap();
                let fill_vertex_buffer = CpuAccessibleBuffer::from_iter(
                    env.device.clone(),
                    BufferUsage::all(),
                    self.fill_vec.clone().iter().cloned(),
                )
                .unwrap();
                let window = env.surface.window();
                if recreate_swapchain {
                    let dimensions = {
                        let dimensions: (u32, u32) = window
                            .get_inner_size()
                            .unwrap()
                            .to_physical(window.get_hidpi_factor())
                            .into();
                        [dimensions.0, dimensions.1]
                    };
                    let (new_swapchain, new_images) =
                        match env.swapchain.recreate_with_dimension(dimensions) {
                            Ok(r) => r,
                            Err(SwapchainCreationError::UnsupportedDimensions) => continue,
                            Err(err) => panic!("{:?}", err),
                        };
                    env.swapchain = new_swapchain;
                    env.framebuffers = window_size_dependent_setup(
                        &new_images,
                        env.render_pass.clone(),
                        &mut env.dynamic_state,
                    );
                    text = DrawText::new(env.device.clone(), env.queue.clone(), env.swapchain.clone(), &new_images);
                    recreate_swapchain = false;
                }
                let (image_num, acquire_future) =
                    match swapchain::acquire_next_image(env.swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            recreate_swapchain = true;
                            continue;
                        }
                        Err(err) => panic!("{:?}", err),
                    };
                let clear_values = vec![self.background_color.into()];
                let command_buffer:AutoCommandBuffer;
                if self.text_vec.len()>0{
                        for txt in self.text_vec{
                            text.queue_text(txt.position[0],txt.position[0], self.text_size, txt.color,txt.text);
                        }
                        match set.clone(){
                            Some(set)=>{
                command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
                    env.device.clone(),
                    env.queue.family(),
                )
                .unwrap()
                .begin_render_pass(env.framebuffers[image_num].clone(), false, clear_values)
                .unwrap()
                .draw(
                    env.tex_pipeline.clone(),
                    &env.dynamic_state,
                    vec![fill_vertex_buffer.clone()],
                    set.clone(),
                    (),
                )
                .unwrap()
                .draw(
                    env.stroke_pipeline.clone(),
                    &env.dynamic_state,
                    vec![stroke_vertex_buffer.clone()],
                    (),
                    (),
                )
                .unwrap()
                .end_render_pass()
                .unwrap()
                .draw_text(&mut text, image_num)
                .build()
                .unwrap();
                            },
                None=>{
                command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
                    env.device.clone(),
                    env.queue.family(),
                )
                .unwrap()
                .begin_render_pass(env.framebuffers[image_num].clone(), false, clear_values)
                .unwrap()
                .draw(
                    env.fill_pipeline.clone(),
                    &env.dynamic_state,
                    vec![fill_vertex_buffer.clone()],
                    (),
                    (),
                )
                .unwrap()
                .draw(
                    env.stroke_pipeline.clone(),
                    &env.dynamic_state,
                    vec![stroke_vertex_buffer.clone()],
                    (),
                    (),
                )
                .unwrap()
                .end_render_pass()
                .unwrap()
                .draw_text(&mut text, image_num)
                .build()
                .unwrap();
                },
                        };
                    }else{
                        match set.clone(){
                            Some(set)=>{
                        command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
                    env.device.clone(),
                    env.queue.family(),
                )
                .unwrap()
                .begin_render_pass(env.framebuffers[image_num].clone(), false, clear_values)
                .unwrap()
                .draw(
                    env.tex_pipeline.clone(),
                    &env.dynamic_state,
                    vec![fill_vertex_buffer.clone()],
                    set.clone(),
                    (),
                )
                .unwrap()
                .draw(
                    env.stroke_pipeline.clone(),
                    &env.dynamic_state,
                    vec![stroke_vertex_buffer.clone()],
                    (),
                    (),
                )
                .unwrap()
                .end_render_pass()
                .unwrap()
                .build()
                .unwrap();
                            },
                        None=>{
                    command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
                    env.device.clone(),
                    env.queue.family(),
                )
                .unwrap()
                .begin_render_pass(env.framebuffers[image_num].clone(), false, clear_values)
                .unwrap()
                .draw(
                    env.fill_pipeline.clone(),
                    &env.dynamic_state,
                    vec![fill_vertex_buffer.clone()],
                    (),
                    (),
                )
                .unwrap()
                .draw(
                    env.stroke_pipeline.clone(),
                    &env.dynamic_state,
                    vec![stroke_vertex_buffer.clone()],
                    (),
                    (),
                )
                .unwrap()
                .end_render_pass()
                .unwrap()
                .build()
                .unwrap();
                }
                        };
                    }
                let future = previous_frame_end
                    .join(acquire_future)
                    .then_execute(env.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(env.queue.clone(), env.swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();
                match future {
                    Ok(future) => {
                        future.wait(None).unwrap();
                        previous_frame_end = Box::new(future) as Box<_>;
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end =
                            Box::new(sync::now(env.device.clone())) as Box<_>;
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        previous_frame_end =
                            Box::new(sync::now(env.device.clone())) as Box<_>;
                    }
                }
            end = Instant::now();
            if end- start>Duration::new(1,0){
            self.fps =counter1 as f32/(end-start).as_secs() as f32;
            }
            self.text_vec = vec![];
            self.fill_vec = vec![];
            self.stroke_vec = vec![];
            //draw_fn(self.clone());
            self= draw_fn(self);
            self.key.keycode = Some(VirtualKeyCode::Power);
            self.mouse.btn = Some(MouseButton::Other(99));
            self.mouse_scroll.delta = (0,0);
            counter1+=1;
        }
        //});
        }
///recieves the x and y of the top spot and then the width and height of the rectangle you want
///built.
pub fn rect(&mut self,x: u16, y: u16, width: u16, height: u16) {
    
        let scale = [self.size.0, self.size.1];
        let t_l = map([x, y], scale);
        let b_r = map([x + width, y + height], scale);
        let t_r = map([x + width, y], scale);
        let b_l = map([x, y + height], scale);
        if self.fill {
            let color = self.fill_color;
            self.fill_vec.push(Vertex {
                position: b_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: t_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: t_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: t_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: b_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: b_r,
                color,
                tex_coords:[0f32,0f32],
            });
        }
        if self.stroke {
            let color = self.color;
            self.stroke_vec.push(Vertex {
                position: t_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: t_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: t_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: b_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: b_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: b_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: b_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: t_l,
                color,
                tex_coords:[0f32,0f32],
            });
        }
}
///recieves the x and y of the top spot and then the width of the sqaure you want built.
pub fn square(&mut self,x: u16, y: u16, width: u16) {
    
        let scale = [self.size.0, self.size.1];
        let t_l = map([x, y], scale);
        let b_r = map([x + width, y + width], scale);
        let t_r = map([x + width, y], scale);
        let b_l = map([x, y + width], scale);
        if self.fill {
            let color = self.fill_color;
            self.fill_vec.push(Vertex {
                position: b_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: t_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: t_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: t_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: b_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: b_r,
                color,
                tex_coords:[0f32,0f32],
            });
        }
        if self.stroke {
            let color = self.color;
            self.stroke_vec.push(Vertex {
                position: t_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: t_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: t_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: b_r,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: b_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: b_l,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: t_l,
                color,
                tex_coords:[0f32,0f32],
            });
        }
}
///recieves the x and y of the top point and then the x and the y of the bottom point and creates a
///line between them.
pub fn line(&mut self,x: u16, y: u16, x2: u16, y2: u16) {
    
        let scale = [self.size.0, self.size.1];
        let srt = map([x, y], scale);
        let fin = map([x2, y2], scale);
        let color = self.color;
        self.stroke_vec.push(Vertex {
            position: srt,
            color,
                tex_coords:[0f32,0f32],
        });
        self.stroke_vec.push(Vertex {
            position: fin,
            color,
                tex_coords:[0f32,0f32],
        });
}
///recieves the x and y of the 3 points of the triangle and creates it based on them
pub fn triangle(&mut self,x1: u16, y1: u16, x2: u16, y2: u16, x3: u16, y3: u16) {
    
        let scale = [self.size.0, self.size.1];
        let pt1 = map([x1, y1], scale);
        let pt2 = map([x2, y2], scale);
        let pt3 = map([x3, y3], scale);
        if self.fill {
            let color = self.fill_color;
            self.fill_vec.push(Vertex {
                position: pt1,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: pt2,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: pt3,
                color,
                tex_coords:[0f32,0f32],
            });
        }
        if self.stroke {
            let color = self.color;
            self.stroke_vec.push(Vertex {
                position: pt1,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt2,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt2,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt3,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt3,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt1,
                color,
                tex_coords:[0f32,0f32],
            });
        }
}
///recieves the x and y of the 4 points of the quad and creates it based on them
pub fn quad(&mut self,x1: u16, y1: u16, x2: u16, y2: u16, x3: u16, y3: u16, x4: u16, y4: u16) {
    
        let scale = [self.size.0, self.size.1];
        let pt1 = map([x1, y1], scale);
        let pt2 = map([x2, y2], scale);
        let pt3 = map([x3, y3], scale);
        let pt4 = map([x4, y4], scale);
        if self.fill {
            let color = self.fill_color;
            self.fill_vec.push(Vertex {
                position: pt1,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: pt2,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: pt3,
                color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: pt4,
                color,
                tex_coords:[0f32,0f32],
            });
        }
        if self.stroke {
            let color = self.color;
            self.stroke_vec.push(Vertex {
                position: pt1,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt2,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt2,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt3,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt3,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt4,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt4,
                color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: pt1,
                color,
                tex_coords:[0f32,0f32],
            });
        }
}
///recieves the x and the y of the center of the ellipse and the width and height of the ellipse
///and creates it accordingly
pub fn ellipse(&mut self,x: u16, y: u16, a: u16, b: u16) {
    
        let scale = [self.size.0, self.size.1];
        if self.stroke && !(self.fill && self.color == self.fill_color) {
            let mut pt_x = x as f32 + a as f32;
            let mut pt_y = y as f32;
            for an in (0..360).step_by(6) {
                let ptx = x as f32 + ((an as f32 / 360.0) * 6.28).cos() * a as f32;
                let pty = y as f32 + ((an as f32 / 360.0) * 6.28).sin() * b as f32;
                self.stroke_vec.push(Vertex {
                    position: map_circ([pt_x, pt_y], scale),
                    color: self.color,
                tex_coords:[0f32,0f32],
                });
                self.stroke_vec.push(Vertex {
                    position: map_circ([ptx, pty], scale),
                    color: self.color,
                tex_coords:[0f32,0f32],
                });
                pt_x = ptx;
                pt_y = pty;
            }
            self.stroke_vec.push(Vertex {
                position: map_circ([pt_x, pt_y], scale),
                color: self.color,
                tex_coords:[0f32,0f32],
            });
            pt_x = x as f32 + a as f32 + 0.5;
            pt_y = y as f32 + 0.5;
            self.stroke_vec.push(Vertex {
                position: map_circ([pt_x, pt_y], scale),
                color: self.color,
                tex_coords:[0f32,0f32],
            });
        }
        if self.fill {
            let mut pt_x = x as f32 + a as f32;
            let mut pt_y = y as f32;
            for an in (0..360).step_by(6) {
                let ptx = x as f32 + ((an as f32 / 360.0) * 6.28).cos() * a as f32;
                let pty = y as f32 + ((an as f32 / 360.0) * 6.28).sin() * b as f32;
                self.fill_vec.push(Vertex {
                    position: map_circ([pt_x, pt_y], scale),
                    color: self.fill_color,
                tex_coords:[0f32,0f32],
                });
                self.fill_vec.push(Vertex {
                    position: map_circ([ptx, pty], scale),
                    color: self.fill_color,
                tex_coords:[0f32,0f32],
                });
                self.fill_vec.push(Vertex {
                    position: map_circ([x as f32, y as f32], scale),
                    color: self.fill_color,
                tex_coords:[0f32,0f32],
                });
                pt_x = ptx;
                pt_y = pty;
            }
            self.fill_vec.push(Vertex {
                position: map_circ([pt_x, pt_y], scale),
                color: self.fill_color,
                tex_coords:[0f32,0f32],
            });
            pt_x = x as f32 + a as f32 + 0.5;
            pt_y = y as f32 + 0.5;
            self.fill_vec.push(Vertex {
                position: map_circ([pt_x, pt_y], scale),
                color: self.fill_color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: map_circ([x as f32, y as f32], scale),
                color: self.fill_color,
                tex_coords:[0f32,0f32],
            });
        }
}
///recieves the x and y of the center of the circle and the radius and builds it with them.
pub fn circle(&mut self,x: u16, y: u16, rad: u16) {
    
        let scale = [self.size.0, self.size.1];
        if self.stroke && !(self.fill && self.color == self.fill_color) {
            let mut pt_x = x as f32 + rad as f32;
            let mut pt_y = y as f32;
            for a in (0..360).step_by(6) {
                let ptx = x as f32 + ((a as f32 / 360.0) * 6.28).cos() * rad as f32;
                let pty = y as f32 + ((a as f32 / 360.0) * 6.28).sin() * rad as f32;
                self.stroke_vec.push(Vertex {
                    position: map_circ([pt_x, pt_y], scale),
                    color: self.color,
                    tex_coords:[0f32,0f32],
                });
                self.stroke_vec.push(Vertex {
                    position: map_circ([ptx, pty], scale),
                    color: self.color,
                    tex_coords:[0f32,0f32],
                });
                pt_x = ptx;
                pt_y = pty;
            }
            self.stroke_vec.push(Vertex {
                position: map_circ([pt_x, pt_y], scale),
                color: self.color,
                tex_coords:[0f32,0f32],
            });
            pt_x = x as f32 + rad as f32 + 0.5;
            pt_y = y as f32 + 0.5;
            self.stroke_vec.push(Vertex {
                position: map_circ([pt_x, pt_y], scale),
                color: self.color,
                tex_coords:[0f32,0f32],
            });
        }
        if self.fill {
            let mut pt_x = x as f32 + rad as f32;
            let mut pt_y = y as f32;
            for a in (0..360).step_by(6) {
                let ptx = x as f32 + ((a as f32 / 360.0) * 6.28).cos() * rad as f32;
                let pty = y as f32 + ((a as f32 / 360.0) * 6.28).sin() * rad as f32;
                self.fill_vec.push(Vertex {
                    position: map_circ([pt_x, pt_y], scale),
                    color: self.fill_color,
                    tex_coords:[0f32,0f32],
                });
                self.fill_vec.push(Vertex {
                    position: map_circ([ptx, pty], scale),
                    color: self.fill_color,
                    tex_coords:[0f32,0f32],
                });
                self.fill_vec.push(Vertex {
                    position: map_circ([x as f32, y as f32], scale),
                    color: self.fill_color,
                    tex_coords:[0f32,0f32],
                });
                pt_x = ptx;
                pt_y = pty;
            }
            self.fill_vec.push(Vertex {
                position: map_circ([pt_x, pt_y], scale),
                color: self.fill_color,
                tex_coords:[0f32,0f32],
            });
            pt_x = x as f32 + rad as f32 + 0.5;
            pt_y = y as f32 + 0.5;
            self.fill_vec.push(Vertex {
                position: map_circ([pt_x, pt_y], scale),
                color: self.fill_color,
                tex_coords:[0f32,0f32],
            });
            self.fill_vec.push(Vertex {
                position: map_circ([x as f32, y as f32], scale),
                color: self.fill_color,
                tex_coords:[0f32,0f32],
            });
        }
}
///recieves the x and the y and makes a small circle in the spot(size depends on strokeWeight).
pub fn point(&mut self,x: u16, y: u16) {
    
        let stro = self.stroke;
        let fil = self.fill;
        self.stroke = false;
        self.fill = true;
        self.circle(x, y, self.stroke_weight as u16);
        self.stroke = stro;
        self.fill = fil;
}
///create an arc from a circle, recieves the center of the circle and the radius and the degrees
///covered by the arc (360 degree arc is a full circle).
pub fn arc(&mut self,x: u16, y: u16, rad: u16, deg: u16) {
    
        let scale = [self.size.0, self.size.1];
        if self.stroke && !(self.fill && self.color == self.fill_color) {
            let mut pt_x = x as f32 + rad as f32;
            let mut pt_y = y as f32;
            for a in (0..deg + 6).step_by(6) {
                let ptx = x as f32 + ((a as f32 / 360.0) * 6.28).cos() * rad as f32;
                let pty = y as f32 + ((a as f32 / 360.0) * 6.28).sin() * rad as f32;
                self.stroke_vec.push(Vertex {
                    position: map_circ([pt_x, pt_y], scale),
                    color: self.color,
                    tex_coords:[0f32,0f32],
                });
                self.stroke_vec.push(Vertex {
                    position: map_circ([ptx, pty], scale),
                    color: self.color,
                    tex_coords:[0f32,0f32],
                });
                pt_x = ptx;
                pt_y = pty;
            }
        }
        if self.fill {
            let mut pt_x = x as f32 + rad as f32;
            let mut pt_y = y as f32;
            for a in (0..deg + 6).step_by(6) {
                let ptx = x as f32 + ((a as f32 / 360.0) * 6.28).cos() * rad as f32;
                let pty = y as f32 + ((a as f32 / 360.0) * 6.28).sin() * rad as f32;
                self.fill_vec.push(Vertex {
                    position: map_circ([pt_x, pt_y], scale),
                    color: self.fill_color,
                    tex_coords:[0f32,0f32],
                });
                self.fill_vec.push(Vertex {
                    position: map_circ([ptx, pty], scale),
                    color: self.fill_color,
                    tex_coords:[0f32,0f32],
                });
                self.fill_vec.push(Vertex {
                    position: map_circ([x as f32, y as f32], scale),
                    color: self.fill_color,
                    tex_coords:[0f32,0f32],
                });
                pt_x = ptx;
                pt_y = pty;
            }
        }
}
///loopes over the array and uses curveVertex to create a bezier curve
#[allow(non_snake_case)]
pub fn bezierCurve(&mut self,ptvec: Vec<[i64; 2]>) {
    for i in 0..(ptvec.len() - 3) {
        if (i + 1) % 4 == 0 || i == 0 {
            self.bezierCurveVertex(
                ptvec[i][0],
                ptvec[i][1],
                ptvec[i + 1][0],
                ptvec[i + 1][1],
                ptvec[i + 2][0],
                ptvec[i + 2][1],
                ptvec[i + 3][0],
                ptvec[i + 3][1],
            );
        }
    }
}
///loopes over the array and uses curveVertex to create a catmull rom chain curve
pub fn curve(&mut self,ptvec: Vec<[i64; 2]>) {
    for i in 0..(ptvec.len() - 3) {
        self.curveVertex(
            ptvec[i][0],
            ptvec[i][1],
            ptvec[i + 1][0],
            ptvec[i + 1][1],
            ptvec[i + 2][0],
            ptvec[i + 2][1],
            ptvec[i + 3][0],
            ptvec[i + 3][1],
        );
    }
}
///uses the catmull rom chain algorithm in order to create a curve
#[allow(non_snake_case)]
pub fn curveVertex(&mut self,x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64, x4: i64, y4: i64) {
    let c = catmull_rom_chain(x1, y1, x2, y2, x3, y3, x4, y4);
    
        let scale = [self.size.0, self.size.1];
        for pt in c.iter() {
            self.stroke_vec.push(Vertex {
                position: mapf(*pt, scale),
                color: self.color,
                tex_coords:[0f32,0f32],
            });
        }
}
///uses the cubic bezier curve algorithm in order to create a curve
#[allow(non_snake_case)]
pub fn bezierCurveVertex(&mut self,x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64, x4: i64, y4: i64) {
    let c = bezier_points(x1, y1, x2, y2, x3, y3, x4, y4);
    
        let scale = [self.size.0, self.size.1];
        let mut ptnxt = c[0];
        for pt in c.iter() {
            self.stroke_vec.push(Vertex {
                position: mapf(ptnxt, scale),
                color: self.color,
                tex_coords:[0f32,0f32],
            });
            self.stroke_vec.push(Vertex {
                position: mapf(*pt, scale),
                color: self.color,
                tex_coords:[0f32,0f32],
            });
            ptnxt = *pt;
        }
}
///drawes a text of a certain color and locaion on the canvas
pub fn text(&mut self,x:u16,y:u16,text:&'static str){
    
        self.text_vec.push(Stext{
            position: [x as f32,y as f32],
            color: self.color,
            text: text,
        });
}
///takes a path to the image and loads it into an Image struct
///should strictly be used outside the draw loop!
#[allow(non_snake_case)]
pub fn img(path:&str)->Image{
        let img = image::open(path).unwrap();
        img.resize(img.width() , img.height() ,image::imageops::FilterType::Nearest);
        let image_data = img.raw_pixels();
        let dimensions = Dimensions::Dim2d { width: img.width(), height: img.height() };
        Image{image_data,dimensions}
}
}
