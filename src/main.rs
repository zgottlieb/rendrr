#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;
extern crate gfx_hal as hal;

extern crate winit;
extern crate env_logger;

pub mod css;
pub mod dom;
pub mod html;
pub mod style;

// use std::env;
use std::fs::File;
use std::io::prelude::*;

use hal::{buffer, command, format as f, image as i, memory as m, pass, pso, pool, window::Extent2D};
use hal::{Device, Instance, PhysicalDevice, Surface, Swapchain};
use hal::{DescriptorPool, FrameSync, Primitive, Backbuffer, SwapchainConfig};
use hal::pso::{PipelineStage, ShaderStageFlags, Specialization};
use hal::format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle};
// use hal::pass::Subpass;
// use hal::queue::Submission;

// All those constants
const DIMS: Extent2D = Extent2D { width: 1024, height: 768 };

#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
struct Vertex {
    a_Pos: [f32; 2],
    a_Uv: [f32; 2],
}

const QUAD: [Vertex; 6] = [
    Vertex { a_Pos: [ -0.5, 0.33 ], a_Uv: [0.0, 1.0] },
    Vertex { a_Pos: [  0.5, 0.33 ], a_Uv: [1.0, 1.0] },
    Vertex { a_Pos: [  0.5,-0.33 ], a_Uv: [1.0, 0.0] },

    Vertex { a_Pos: [ -0.5, 0.33 ], a_Uv: [0.0, 1.0] },
    Vertex { a_Pos: [  0.5,-0.33 ], a_Uv: [1.0, 0.0] },
    Vertex { a_Pos: [ -0.5,-0.33 ], a_Uv: [0.0, 0.0] },
];

fn parse_css() -> css::Stylesheet {
    let mut source = File::open("test.css").unwrap();
    let mut css = String::new();
    source.read_to_string(&mut css).unwrap();
    css::parse(css)
}

fn parse_html() -> dom::Node {
    let mut html_file = File::open("test.html").unwrap();
    let mut contents = String::new();
    html_file.read_to_string(&mut contents).unwrap();
    html::parse(contents)
}


fn setup_rendering() {
    let mut events_loop = winit::EventsLoop::new();

    let wb = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::from_physical(winit::dpi::PhysicalSize {
            width: DIMS.width as _,
            height: DIMS.height as _,
        }, 1.0)).with_title("I owe you a name".to_string());

    // instantiate backend
    #[cfg(not(feature = "gl"))]
    let (window, _instance, mut adapters, mut surface) = {
        let window = wb.build(&events_loop).unwrap();
        let instance = back::Instance::create("gfx-rs quad", 1);
        let surface = instance.create_surface(&window);
        let adapters = instance.enumerate_adapters();
        (window, instance, adapters, surface)
    };
    #[cfg(feature = "gl")]
    let (mut adapters, mut surface) = {
        let window = {
            let builder =
                back::config_context(back::glutin::ContextBuilder::new(), ColorFormat::SELF, None)
                    .with_vsync(true);
            back::glutin::GlWindow::new(wb, builder, &events_loop).unwrap()
        };

        let surface = back::Surface::from_window(window);
        let adapters = surface.enumerate_adapters();
        (adapters, surface)
    };

    for adapter in &adapters {
        println!("ADAPTER: {:?}", adapter.info);
    }

    let mut adapter = adapters.remove(0);
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let limits = adapter.physical_device.limits();

    // Build a new device and associated command queues
    let (mut device, mut queue_group) = adapter
        .open_with::<_, hal::Graphics>(1, |family| surface.supports_queue_family(family))
        .unwrap();

    let mut command_pool =
        device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty(), 16);

    // Setup renderpass and pipeline
    let set_layout = device.create_descriptor_set_layout(
        &[
            pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::SampledImage,
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            },
            pso::DescriptorSetLayoutBinding {
                binding: 1,
                ty: pso::DescriptorType::Sampler,
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            },
        ],
        &[],
    );

    // Descriptors
    let mut desc_pool = device.create_descriptor_pool(
        1, // sets
        &[
            pso::DescriptorRangeDesc {
                ty: pso::DescriptorType::SampledImage,
                count: 1,
            },
            pso::DescriptorRangeDesc {
                ty: pso::DescriptorType::Sampler,
                count: 1,
            },
        ],
    );
    let desc_set = desc_pool.allocate_set(&set_layout).unwrap();

    // Buffer allocations
    println!("Memory types: {:?}", memory_types);

    let buffer_stride = std::mem::size_of::<Vertex>() as u64;
    let buffer_len = QUAD.len() as u64 * buffer_stride;

    let buffer_unbound = device
        .create_buffer(buffer_len, buffer::Usage::VERTEX)
        .unwrap();
    let buffer_req = device.get_buffer_requirements(&buffer_unbound);

    let upload_type = memory_types
        .iter()
        .enumerate()
        .position(|(id, mem_type)| {
            buffer_req.type_mask & (1 << id) != 0
                && mem_type.properties.contains(m::Properties::CPU_VISIBLE)
        })
        .unwrap()
        .into();

    let buffer_memory = device
        .allocate_memory(upload_type, buffer_req.size)
        .unwrap();
    let vertex_buffer = device
        .bind_buffer_memory(&buffer_memory, 0, buffer_unbound)
        .unwrap();

    // TODO: check transitions: read/write mapping and vertex buffer read
    {
        let mut vertices = device
            .acquire_mapping_writer::<Vertex>(&buffer_memory, 0..buffer_len)
            .unwrap();
        vertices.copy_from_slice(&QUAD);
        device.release_mapping_writer(vertices);
    }
}

// Version of main called if no features were provided.
#[cfg(not(any(feature = "vulkan", feature = "dx12", feature = "metal", feature = "gl")))]
fn main() {
    println!("You need to enable the native API feature (vulkan/metal) in order to test the LL");
}

// There's at least one feature enabled, so we can actually render
// something to the screen
#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal", feature = "gl"))]
fn main() {
    // TODO: implement flags to allow user to pass in file names
    // let args: Vec<String> = env::args().collect();

    env_logger::init();

    setup_rendering();

    let html = parse_html();
    let stylesheet = parse_css();

    let style_tree = style::build_style_tree(&html, &stylesheet);

    println!("{:#?}", html);
    println!("{:#?}", stylesheet);
    println!("{:#?}", style_tree);
}
