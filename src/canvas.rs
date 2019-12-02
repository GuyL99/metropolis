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
use winit::{Event, WindowEvent};
use crate::text::{DrawText, DrawTextTrait};
use crate::{FPS,HEIGHT,WIDTH};
use std::time::{Duration, Instant};
#[derive(Copy, Clone, PartialEq)]
pub struct Canvas {
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
    }
}
pub static mut CANVAS: Canvas = Canvas {
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
pub static mut TEXT_VEC:Option<Vec<Stext>> = None;
impl Canvas {
    pub fn show<F>(self, mut draw_fn: F)
    where
        F: FnMut() + 'static,
    {
        let (mut env, mut events_loop) = init(self.size.0, self.size.1);
        let mut text = DrawText::new(env.device.clone(), env.queue.clone(), env.swapchain.clone(), &env.images);
        let mut counter1 = 0;
        let start = Instant::now();
        let mut end;
        loop {
            let mut done = false;
            events_loop.poll_events(|ev| match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => done = true,
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => env.recreate_swapchain = true,
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
                env.previous_frame_end.as_mut().unwrap().cleanup_finished();
                if env.recreate_swapchain {
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
                    env.recreate_swapchain = false;
                }
                let (image_num, acquire_future) =
                    match swapchain::acquire_next_image(env.swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            env.recreate_swapchain = true;
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
                    }else{
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
                /*command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
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
                //.draw_text(&mut text, image_num)
                .build()
                .unwrap();*/
                let prev = env.previous_frame_end.take();
                let future = prev
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(env.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(env.queue.clone(), env.swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();
                match future {
                    Ok(future) => {
                        future.wait(None).unwrap();
                        env.previous_frame_end = Some(Box::new(future) as Box<_>);
                    }
                    Err(FlushError::OutOfDate) => {
                        env.recreate_swapchain = true;
                        env.previous_frame_end =
                            Some(Box::new(sync::now(env.device.clone())) as Box<_>);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        env.previous_frame_end =
                            Some(Box::new(sync::now(env.device.clone())) as Box<_>);
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
