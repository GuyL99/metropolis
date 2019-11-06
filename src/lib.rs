pub mod shaders{
    pub mod vs {
        vulkano_shaders::shader!{
            ty: "vertex",
            src: "#version 310 es
precision highp float;
layout(location = 0) in vec2 position;
layout(location = 1) in vec4 color;
layout(location = 0) out vec4 _color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    _color = color;
}"
        }
    }

    pub mod fs {
        vulkano_shaders::shader!{
            ty: "fragment",
            src: "#version 310 es
precision highp float;
layout(location = 0) in vec4 _color;
layout(location = 0) out vec4 f_color;
void main() {
    f_color = _color;
}
"
        }
    }
}
/*#[derive(Default, Debug, Clone,Copy)]
pub struct Vertex { pub position: [f32; 2] , pub color: [f32;4]}
vulkano::impl_vertex!(Vertex, position,color);*/
mod vertex;
use vertex::Vertex;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, DeviceExtensions,Queue};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};
use vulkano::swapchain;
use vulkano::swapchain::*;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::sync;
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder};
use winit::event::{Event, WindowEvent};
use std::sync::Arc;
use std::sync::Mutex;

//use vulkano::framebuffer::RenderPassDesc;
use vulkano::pipeline::GraphicsPipelineAbstract;
//use vulkano::descriptor::pipeline_layout::PipelineLayoutAbstract;
//use vulkano::framebuffer::RenderPassSubpassInterface;
use shaders::vs;
use shaders::fs;
//pub struct 
pub struct Preper{
    pub device:Arc<Device>,
    pub queue:Arc<Queue>,
    pub surface:Arc<Surface<Window>>,
    pub swapchain:Arc<Swapchain<Window>>,
    pub images:Vec<Arc<SwapchainImage<Window>>>,
    pub render_pass:Arc<dyn RenderPassAbstract + Send + Sync>,
    pub fill_pipeline:Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    pub stroke_pipeline:Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    pub dynamic_state:DynamicState,
    pub framebuffers:Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    pub recreate_swapchain:bool,
    pub previous_frame_end:Option<Box<dyn GpuFuture>>
}
pub fn init()->Preper{
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).unwrap()
    };
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    println!("Using device: {} (type: {:?})", physical.name(), physical.ty());
    let events_loop = EventLoop::new();
    let surface = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();
    let window = surface.window();
    let queue_family = physical.queue_families().find(|&q| {
        q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
    }).unwrap();
    let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
    let (device, mut queues) = Device::new(physical, physical.supported_features(), &device_ext,
        [(queue_family, 0.5)].iter().cloned()).unwrap();
    let queue = queues.next().unwrap();
    let (swapchain, images) = {
        let caps = surface.capabilities(physical).unwrap();
        let usage = caps.supported_usage_flags;
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let initial_dimensions = {
            // convert to physical pixels
            let dimensions: (u32, u32) = window.inner_size().to_physical(window.hidpi_factor()).into();
            [dimensions.0, dimensions.1]
        };
        Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
            initial_dimensions, 1, usage, &queue, SurfaceTransform::Identity, alpha,
            PresentMode::Fifo, true, None).unwrap()

    };
    let render_pass = Arc::new(vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load:Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap());
    let vs = vs::Shader::load(device.clone()).unwrap();
    let fs = fs::Shader::load(device.clone()).unwrap();
    let fill_pipeline = Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer::<Vertex>()
        .vertex_shader(vs.main_entry_point(), ())
        .triangle_list()
        .viewports_dynamic_scissors_irrelevant(1)
        .fragment_shader(fs.main_entry_point(), ())
        .blend_alpha_blending()
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap());
    let stroke_pipeline = Arc::new(GraphicsPipeline::start()
        .vertex_input_single_buffer::<Vertex>()
        .vertex_shader(vs.main_entry_point(), ())
        .line_list()
        .viewports_dynamic_scissors_irrelevant(1)
        .fragment_shader(fs.main_entry_point(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap());
    let mut dynamic_state = DynamicState { line_width: None, viewports: None, scissors: None };
    let framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);
    let recreate_swapchain = false;
    let previous_frame_end = Some(Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>);
    Preper{device,queue,surface,swapchain,images,render_pass,fill_pipeline,stroke_pipeline,dynamic_state,framebuffers,recreate_swapchain,previous_frame_end}
}
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0 .. 1.0,
    };
    dynamic_state.viewports = Some(vec!(viewport));

    images.iter().map(|image| {
        Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(image.clone()).unwrap()
                .build().unwrap()
        ) as Arc<dyn FramebufferAbstract + Send + Sync>
    }).collect::<Vec<_>>()
}
use lazy_static::*;
lazy_static!{
    static ref STROKE_VERTECIES:Mutex<Vec<Vertex>>= Mutex::new(vec![]);
    static ref FILL_VERTECIES:Mutex<Vec<Vertex>> = Mutex::new(vec![]);
}
pub fn add_to_fill(pusher:Vertex){
    FILL_VERTECIES.lock().unwrap().push(pusher);
}
pub fn add_to_stroke(pusher:Vertex){
    STROKE_VERTECIES.lock().unwrap().push(pusher);
}
lazy_static!{
    pub static ref CANVAS:Canvas = Canvas{size:(0,0),stroke:true,color:[1.0,1.0,1.0,1.0],stroke_weight:1,fill:false,fill_color:[1.0,1.0,1.0,1.0],background_color:[1.0,1.0,1.0,1.0],fps:30,resizeable:false};
}
#[derive(Copy,Clone,PartialEq)]
pub struct Canvas{
    pub size:(u16,u16),
    stroke:bool,
    color:[f32;4],
    stroke_weight:u8,
    fill:bool,
    fill_color:[f32;4],
    background_color:[f32;4],
    fps:u8,
    resizeable:bool,
}
/*
pub fn size(width:u16,height:u16)->Canvas{
    Canvas{size:(width,height),stroke:true,color:[1.0,1.0,1.0,1.0],stroke_weight:1,fill:false,fill_color:[1.0,1.0,1.0,1.0],background_color:[1.0,1.0,1.0,1.0],fps:30,resizeable:false}
    }*/
pub fn size(width:u16,height:u16){
    CANVAS.set_size(width,height);
}
pub fn show(){
    CANVAS.show();
}
impl Canvas{
    pub fn set_size(mut self,width:u16,height:u16){
        self.size =(width,height);
    }
    pub fn show(self){
        let mut env = init();
        let events_loop = EventLoop::new();
        /*let color = [0.0,1.0,0.0,1.0];
        add_to_fill(Vertex { position: [1.00, 1.0], color});
        add_to_fill(Vertex { position: [1.00, 0.0] ,color});
        add_to_fill(Vertex { position: [0.00, 0.0] ,color});
        add_to_fill(Vertex { position: [0.00, 0.0] ,color});
        add_to_fill(Vertex { position: [0.00, 1.0] ,color});
        add_to_fill(Vertex { position: [1.00, 1.0] ,color});*/
        let stroke_vertex_buffer = CpuAccessibleBuffer::from_iter(env.device.clone(), BufferUsage::all(),STROKE_VERTECIES.lock().unwrap().iter().cloned()).unwrap();
        let fill_vertex_buffer = CpuAccessibleBuffer::from_iter(env.device.clone(), BufferUsage::all(),FILL_VERTECIES.lock().unwrap().iter().cloned()).unwrap();
        events_loop.run(move |ev, _, cf| {
            *cf = ControlFlow::Poll;
            match ev {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *cf = ControlFlow::Exit,
                Event::WindowEvent { event: WindowEvent::Resized(_), .. } => env.recreate_swapchain = true,
                _ => {},
            }
            let window = env.surface.window();
            env.previous_frame_end.as_mut().unwrap().cleanup_finished();
            if env.recreate_swapchain {
                let dimensions = {
                    let dimensions: (u32, u32) = window.inner_size().to_physical(window.hidpi_factor()).into();
                    [dimensions.0, dimensions.1]
                };
                let (new_swapchain, new_images) = match env.swapchain.recreate_with_dimension(dimensions) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => return,
                    Err(err) => panic!("{:?}", err)
                };
                env.swapchain = new_swapchain;
                env.framebuffers = window_size_dependent_setup(&new_images, env.render_pass.clone(), &mut env.dynamic_state);
                env.recreate_swapchain = false;
            }
            let (image_num, acquire_future) = match swapchain::acquire_next_image(env.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    env.recreate_swapchain = true;
                    return;
                },
                Err(err) => panic!("{:?}", err)
            };
            let clear_values = vec!(self.background_color.into());
            let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(env.device.clone(), env.queue.family()).unwrap()
                .begin_render_pass(env.framebuffers[image_num].clone(), false, clear_values)
                .unwrap()
                .draw(env.fill_pipeline.clone(), &env.dynamic_state, vec![fill_vertex_buffer.clone()], (), ())
                .unwrap()
                .draw(env.stroke_pipeline.clone(), &env.dynamic_state, vec![stroke_vertex_buffer.clone()], (), ())
                .unwrap()
                .end_render_pass()
                .unwrap()
                .build().unwrap();
        	let prev = env.previous_frame_end.take();
            let future = prev.unwrap().join(acquire_future)
                .then_execute(env.queue.clone(), command_buffer).unwrap()
                .then_swapchain_present(env.queue.clone(), env.swapchain.clone(), image_num)
                .then_signal_fence_and_flush();
            match future {
                Ok(future) => {
                    future.wait(None).unwrap();
                    env.previous_frame_end = Some(Box::new(future) as Box<_>);
                }
                Err(FlushError::OutOfDate) => {
                    env.recreate_swapchain = true;
                    env.previous_frame_end = Some(Box::new(sync::now(env.device.clone())) as Box<_>);
                }
                Err(e) => {
                    println!("{:?}", e);
                    env.previous_frame_end = Some(Box::new(sync::now(env.device.clone())) as Box<_>);
                }
            }
        });
    }
}
