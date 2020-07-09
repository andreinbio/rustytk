use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
    SwapchainCreationError,
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};

use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use std::sync::Arc;

pub fn create_window() {
    let required_extensions = vulkano_win::required_extensions();

    // let instance = Instance::new(None, &required_extensions, None);//.unwrap();

    dbg!(&required_extensions);

    // let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    // dbg!(PhysicalDevice::enumerate(&instance));

    // Some little debug infos.
    // println!(
    //     "Using device: {} (type: {:?})",
    //     physical.name(),
    //     physical.ty()
    // );
}