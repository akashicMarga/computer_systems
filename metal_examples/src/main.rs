#![feature(allocator_api)]
use core::slice;
use std::mem;

pub mod matrix;
use metal::{ComputePipelineDescriptor, Device, DeviceRef, MTLResourceUsage, NSRange, MTLResourceOptions};


use crate::alloc::PageAlignedAllocator;

pub mod alloc;
pub mod utils;
use crate::matrix::Matrix;


const LIB_DATA_MATHCAL: &[u8] = include_bytes!("math_cal.metallib");

const LIB_DATA_DOTPROD: &[u8] = include_bytes!("dotprod.metallib");

struct MetalState<'a> {
    pub device: &'a metal::DeviceRef,
    pub queue: metal::CommandQueue,
    pub pipeline: metal::ComputePipelineState,
}

const LIB_DATA_MATRIXPROD: &[u8] = include_bytes!("matrixprod.metallib");


pub fn dotprod(v: &[u32], w: &[u32]) -> Vec<u32> {
    // will return a raw pointer to the result
    // the system will assign a GPU to use.
    let device: &DeviceRef = &Device::system_default().expect("No device found");

    // represents the library which contains the kernel.
    let lib = device.new_library_with_data(LIB_DATA_MATHCAL).unwrap();
    // create function pipeline.
    // this compiles the function, so a pipline can't be created in performance sensitive code.
    let function = lib.get_function("dot_product", None).unwrap();
    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .unwrap();

    let length = v.len() as u64;
    let size = length * core::mem::size_of::<u32>() as u64;
    assert_eq!(v.len(), w.len());

    let buffer_a = device.new_buffer_with_data(
        unsafe { mem::transmute(v.as_ptr()) },
        size,
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_b = device.new_buffer_with_data(
        unsafe { mem::transmute(w.as_ptr()) },
        size,
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_result = device.new_buffer(
        size, // the operation will return an array with the same size.
        MTLResourceOptions::StorageModeShared,
    );

    // a command queue for sending instructions to the device.
    let command_queue = device.new_command_queue();
    // for sending commands, a command buffer is needed.
    let command_buffer = command_queue.new_command_buffer();
    // to write commands into a buffer an encoder is needed, in our case a compute encoder.
    let compute_encoder = command_buffer.new_compute_command_encoder();
    compute_encoder.set_compute_pipeline_state(&pipeline);
    compute_encoder.set_buffers(
        0,
        &[Some(&buffer_a), Some(&buffer_b), Some(&buffer_result)],
        &[0; 3],
    );

    // specify thread count and organization
    let grid_size = metal::MTLSize::new(length, 1, 1);
    let threadgroup_size = metal::MTLSize::new(length, 1, 1);
    compute_encoder.dispatch_threads(grid_size, threadgroup_size);

    // end encoding and execute commands
    compute_encoder.end_encoding();
    command_buffer.commit();

    command_buffer.wait_until_completed();

    let ptr = buffer_result.contents() as *const u32;
    let len = buffer_result.length() as usize / mem::size_of::<u32>();
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    slice.to_vec()
}

fn prod<T: Copy>(ma: &Matrix<T>, mb: &Matrix<T>, state: MetalState) -> *mut std::ffi::c_void {
    assert!(ma.is_square());
    assert!(mb.is_square());
    assert_eq!(ma.rows, mb.rows);
    let size = ma.sizeof_entries();

    let buffer_a = state.device.new_buffer_with_data(
        utils::void_ptr(&ma.entries),
        size,
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_b = state.device.new_buffer_with_data(
        utils::void_ptr(&mb.entries),
        size,
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_result = state.device.new_buffer(
        size, // the result will be another suqare matrix of the same size
        MTLResourceOptions::StorageModeShared,
    );

    let command_buffer = state.queue.new_command_buffer();
    let compute_encoder = command_buffer.new_compute_command_encoder();
    compute_encoder.set_compute_pipeline_state(&state.pipeline);
    compute_encoder.set_buffers(
        0,
        &[Some(&buffer_a), Some(&buffer_b), Some(&buffer_result)],
        &[0; 3],
    );

    let n = ma.rows as u64;
    let w = state.pipeline.thread_execution_width();
    let h = state.pipeline.max_total_threads_per_threadgroup() / w;
    let grid_size = metal::MTLSize::new(n, n, 1);
    let threadgroup_size = metal::MTLSize::new(w, h, 1);
    compute_encoder.dispatch_threads(grid_size, threadgroup_size);

    // end encoding and execute commands
    compute_encoder.end_encoding();
    command_buffer.commit();

    command_buffer.wait_until_completed();

    buffer_result.contents()
}

fn main() {
    let result = dotprod(&[3, 4, 1, 7, 10, 20], &[2, 5, 6, 9, 5, 10]);
    println!("Dot product of two vectors: {:?}", result);

    //Matrix Product

    let device: &metal::DeviceRef = &Device::system_default().expect("No device found");
    let queue = device.new_command_queue();

    let lib = device.new_library_with_data(LIB_DATA_MATHCAL).unwrap();

    let function = lib.get_function("mul_matrices", None).unwrap();
    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .unwrap();

    let state = MetalState {
        device,
        queue,
        pipeline,
    };

    let matrix_a = Matrix::new(4, 4, &[1.0; 16]);
    let matrix_b = Matrix::new(4, 4, &[2.0; 16]);

    let result = prod(&matrix_a, &matrix_b, state) as *const [f32; 16];

    unsafe {
        println!("Matrix product result: {:?}", *result);
    };


    //Memory ops

    let device: &DeviceRef = &Device::system_default().expect("No device found");
    let lib = device.new_library_with_data(LIB_DATA_MATHCAL).unwrap();
    let kernel = lib.get_function("assign", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    let pipeline = device
        .new_compute_pipeline_state_with_function(
            pipeline_state_descriptor.compute_function().unwrap(),
        )
        .unwrap();

    // The shared vec size must be 1024 or 2048 for this to work
    let data = &mut vec![0_u32; 1024].to_vec_in(PageAlignedAllocator);
    let data_size = data.capacity() * core::mem::size_of::<u32>();

    let buffer = device.new_buffer_with_data(
        unsafe { unsafe { mem::transmute(data.as_ptr()) } },
        data_size.try_into().unwrap(),
        metal::MTLResourceOptions::StorageModeManaged,
    );

    let command_queue = device.new_command_queue();

    let command_buffer = command_queue.new_command_buffer();
    let compute_encoder = command_buffer.new_compute_command_encoder();
    compute_encoder.set_compute_pipeline_state(&pipeline);

    compute_encoder.set_buffer(0, Some(&buffer), 0);
    compute_encoder.use_resource(&buffer, MTLResourceUsage::Write);
    let grid_size = metal::MTLSize::new(data.len() as u64, 1, 1);
    let threadgroup_size = metal::MTLSize::new(data.len() as u64, 1, 1);

    compute_encoder.dispatch_threads(grid_size, threadgroup_size);
    compute_encoder.end_encoding();

    let a = command_queue.new_command_buffer();
    let blit_encoder = a.new_blit_command_encoder();
    blit_encoder.synchronize_resource(&buffer);
    blit_encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    unsafe {
        println!(
            "via contents(): {:?}",
            *(buffer.contents() as *mut [u32; 10])
        );
    }
    println!("rust vector: {:?}", data[0..10].to_vec());
}

