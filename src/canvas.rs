use crate::setup::*;
use crate::vertex::*;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain;
use vulkano::swapchain::{AcquireError, SwapchainCreationError};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
#[derive(Copy, Clone, PartialEq)]
pub struct Canvas {
    pub size: (u16, u16),
    pub stroke: bool,
    pub color: [f32; 4],
    pub stroke_weight: u8,
    pub fill: bool,
    pub fill_color: [f32; 4],
    pub background_color: [f32; 4],
    pub fps: u8,
    pub resizeable: bool,
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
    stroke_weight: 1,
    fill: false,
    fill_color: [1.0, 1.0, 1.0, 1.0],
    background_color: [1.0, 1.0, 1.0, 1.0],
    fps: 30,
    resizeable: false,
};
pub static mut FILL_VERTECIES: Option<Vec<Vertex>> = None;
pub static mut STROKE_VERTECIES: Option<Vec<Vertex>> = None;
impl Canvas {
    pub fn show<F>(self, mut draw_fn: F)
    where
        F: FnMut() + 'static,
    {
        let mut env = init(self.size.0, self.size.1);
        let events_loop = EventLoop::new();
        events_loop.run(move |ev, _, cf| {
            loop {
                *cf = ControlFlow::Poll;
                //let mut cf = &(ControlFlow::Poll);
                //let event = winit::event::WindowEvent;
                match ev {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => *cf = ControlFlow::Exit,
                    Event::WindowEvent {
                        event: WindowEvent::Resized(_),
                        ..
                    } => env.recreate_swapchain = true,
                    _ => {}
                }
                unsafe {
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
                                .inner_size()
                                .to_physical(window.hidpi_factor())
                                .into();
                            [dimensions.0, dimensions.1]
                        };
                        let (new_swapchain, new_images) =
                            match env.swapchain.recreate_with_dimension(dimensions) {
                                Ok(r) => r,
                                Err(SwapchainCreationError::UnsupportedDimensions) => return,
                                Err(err) => panic!("{:?}", err),
                            };
                        env.swapchain = new_swapchain;
                        env.framebuffers = window_size_dependent_setup(
                            &new_images,
                            env.render_pass.clone(),
                            &mut env.dynamic_state,
                        );
                        env.recreate_swapchain = false;
                    }
                    let (image_num, acquire_future) =
                        match swapchain::acquire_next_image(env.swapchain.clone(), None) {
                            Ok(r) => r,
                            Err(AcquireError::OutOfDate) => {
                                env.recreate_swapchain = true;
                                return;
                            }
                            Err(err) => panic!("{:?}", err),
                        };
                    let clear_values = vec![self.background_color.into()];
                    let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
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
                }
                zero_out();
                draw_fn();
            }
        });
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
