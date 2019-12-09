use crate::setup::*;
use crate::vertex::*;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder,AutoCommandBuffer};
use vulkano::swapchain;
use vulkano::swapchain::{AcquireError, SwapchainCreationError};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use winit::{Event, WindowEvent};
use crate::text::{DrawText, DrawTextTrait};
use crate::{FPS,HEIGHT,WIDTH};
use std::time::{Duration, Instant};
use vulkano::image::{ImmutableImage, Dimensions};
use vulkano::sampler::{Sampler, SamplerAddressMode, Filter, MipmapMode};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::format::Format;
#[derive(Copy, Clone, PartialEq)]
pub struct CanvasGlob {
    pub size: (u16, u16),
    pub stroke: bool,
    pub color: [f32; 4],
    pub stroke_weight: u8,
    pub fill: bool,
    pub fill_color: [f32; 4],
    pub background_color: [f32; 4],
    pub fps: f32,
    pub resizeable: bool,
    pub text_size: f32,
}
impl CanvasGlob{
    pub fn show<F>(self, mut draw_fn:F)
    where F :FnMut()+ 'static,
    {
                let set;
        let mut previous_frame_end;
        let (mut env, mut events_loop) = init(self.size.0, self.size.1);
        unsafe{
        draw_fn();
        match &TEXTURE{
            Some((vec1,dim1))=>{
                let vec_tex = vec1.to_vec();
                let dimensions = *dim1;
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
        }
        }
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
                _ => (),
            });
            if done {
                return;
            }
            unsafe {
            env.dynamic_state.line_width = Some(CANVAS.stroke_weight as f32);
                match &STROKE_VERTECIES {
                    Some(vec1) => STROKE_VERTECIES = Some(vec1.to_vec()),
                    None => {
                        let vec2 = vec![];
                        STROKE_VERTECIES = Some(vec2);
                    }
                };
                match &FILL_VERTECIES {
                    Some(vec1) => FILL_VERTECIES = Some(vec1.to_vec()),
                    None => {
                        let vec2 = vec![];
                        FILL_VERTECIES = Some(vec2);
                    }
                };
                let stroke_vertex_buffer = CpuAccessibleBuffer::from_iter(
                    env.device.clone(),
                    BufferUsage::all(),
                    STROKE_VERTECIES.clone().unwrap().iter().cloned(),
                )
                .unwrap();
                let fill_vertex_buffer = CpuAccessibleBuffer::from_iter(
                    env.device.clone(),
                    BufferUsage::all(),
                    FILL_VERTECIES.clone().unwrap().iter().cloned(),
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
                match &TEXT_VEC {
                    Some(vec1) => {if vec1.len()>0{
                        for txt in vec1{
                            text.queue_text(txt.position[0],txt.position[0], CANVAS.text_size, txt.color,txt.text);
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
                }
                        }
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

                None =>{
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
                        }
                    }
                    },
                    None => {
                        let vec2 = vec![];
                        TEXT_VEC = Some(vec2);
                        command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
                    env.device.clone(),
                    env.queue.family(),
                )
                .unwrap()
                .build()
                .unwrap();
                    }
                };
                //let prev =/* env.*/ (&mut *previous_frame_end).take();
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
            if (end-start)>Duration::new(1,0){
                CANVAS.fps = counter1 as f32/(end-start).as_secs() as f32;
                FPS = CANVAS.fps;
            }
            HEIGHT = CANVAS.size.1;
            WIDTH = CANVAS.size.0;
            }
            zero_out();
            draw_fn();
            counter1+=1;
        }
        //});
    }
}
pub fn zero_out() {
    unsafe {
        match &STROKE_VERTECIES {
            None => {}
            Some(_vec1) => {
                let vec2 = vec![];
                STROKE_VERTECIES = Some(vec2);
            }
        };
        match &FILL_VERTECIES {
            None => {}
            Some(_vec1) => {
                let vec2 = vec![];
                FILL_VERTECIES = Some(vec2);
            }
        };
        match &TEXT_VEC {
            None => {}
            Some(_vec1) => {
                let vec2 = vec![];
                TEXT_VEC = Some(vec2);
            }
        };
    }
}
pub static mut CANVAS: CanvasGlob = CanvasGlob {
    size: (0, 0),
    stroke: true,
    color: [0.0, 0.0, 0.0, 1.0],
    stroke_weight: 8,
    fill: false,
    fill_color: [1.0, 1.0, 1.0, 1.0],
    background_color: [1.0, 1.0, 1.0, 1.0],
    fps: 30.0,
    resizeable: false,
    text_size: 18.0,
};
pub static mut FILL_VERTECIES: Option<Vec<Vertex>> = None;
pub static mut STROKE_VERTECIES: Option<Vec<Vertex>> = None;
pub static mut TEXT_VEC: Option<Vec<Stext>> = None;
pub static mut TEXTURE:Option<(Vec<u8>,Dimensions)> = None;
