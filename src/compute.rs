use crate::shaders::*;
use std::sync::Arc;
use vulkano::buffer::immutable::ImmutableBuffer;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::pipeline::ComputePipeline;
use vulkano::sync;
use vulkano::sync::GpuFuture;
use vulkano_shaders::*;
pub enum ops {
    Sub,
    Add,
    Mult,
    Div,
    FloatAdd,
    FloatSub,
    FloatDiv,
    FloatMult,
    AddVecs,
    SubVecs,
    FloatAddVecs,
    FloatSubVecs,
    FloatDivVecs,
    FloatMultVecs,
}
pub fn compute_ops<T: 'static>(arr1: [T; 100], scalar1: T, op: ops) -> [T; 100]
where
    T: Copy + Clone + Sync + Send,
{
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_compute())
        .expect("couldn't find a compute queue family");
    let mut a = 0;

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };
    let queue = queues.next().unwrap();
    match op {
        ops::Sub => {
            let shader =
                cs_sub::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), scalar1)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr2 = data_buffer.read().unwrap();
            return *arr2;
        }
        ops::Add => {
            let shader =
                cs_add::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), scalar1)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr2 = data_buffer.read().unwrap();
            return *arr2;
        }
        ops::Div => {
            let shader =
                cs_div::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), scalar1)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr2 = data_buffer.read().unwrap();
            return *arr2;
        }
        ops::Mult => {
            let shader =
                cs_mult::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), scalar1)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr2 = data_buffer.read().unwrap();
            return *arr2;
        }
        ops::FloatAdd => {
            let shader =
                cs_float_add::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), scalar1)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr2 = data_buffer.read().unwrap();
            return *arr2;
        }
        ops::FloatSub => {
            let shader =
                cs_float_sub::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), scalar1)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr2 = data_buffer.read().unwrap();
            return *arr2;
        }
        ops::FloatDiv => {
            let shader =
                cs_float_div::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), scalar1)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr2 = data_buffer.read().unwrap();
            return *arr2;
        }
        ops::FloatMult => {
            let shader = cs_float_mult::Shader::load(device.clone())
                .expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), scalar1)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr2 = data_buffer.read().unwrap();
            return *arr2;
        }
        _ => {return arr1;
        }
    }
}
pub fn compute_ops2<T: 'static>(arr1: [T; 100], arr2: [T; 100], op: ops) -> [T; 100]
where
    T: Copy + Clone + Sync + Send,
{
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_compute())
        .expect("couldn't find a compute queue family");

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };
    let queue = queues.next().unwrap();
    match op {
        ops::FloatSubVecs => {
            let shader = cs_float_sub_vec::Shader::load(device.clone())
                .expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr2)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr3 = data_buffer.read().unwrap();
            return *arr3;
        }
        ops::AddVecs => {
            let shader =
                cs_add_vec::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr2)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr3 = data_buffer.read().unwrap();
            return *arr3;
        }
        ops::SubVecs => {
            let shader =
                cs_sub_vec::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr2)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr3 = data_buffer.read().unwrap();
            return *arr3;
        }
        ops::FloatDivVecs => {
            let shader =
                cs_float_div_vec::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr2)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr3 = data_buffer.read().unwrap();
            return *arr3;
        }
        ops::FloatMultVecs => {
            let shader =
                cs_float_mult_vec::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let data_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr1)
                    .expect("failed to create buffer");
            let data_buffer2 =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), arr2)
                    .expect("failed to create buffer");
            let set = Arc::new(
                PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .add_buffer(data_buffer2.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let command_buffer =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap()
                    .dispatch([100, 1, 1], compute_pipeline.clone(), set.clone(), ())
                    .unwrap()
                    .build()
                    .unwrap();
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let arr3 = data_buffer.read().unwrap();
            return *arr3;
        }
        _=>{return arr1;},
    }
}
