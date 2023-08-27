use std::sync::Arc;
use vulkano::{
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags, Queue
    },
    image::{
        ImageUsage, SwapchainImage,
    },
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    swapchain::{
        Swapchain, SwapchainCreateInfo
    },
    VulkanLibrary,
};
use vulkano::device::physical::PhysicalDevice;
use vulkano::swapchain::Surface;
use vulkano_win::{VkSurfaceBuild};
use winit::{
    event_loop::{EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Engine {
    pub event_loop : EventLoop<()>,
    pub library : Arc<VulkanLibrary>,
    pub required_extensions: InstanceExtensions,
    pub instance : Arc<Instance>,
    pub surface : Arc<Surface>,
    pub device_extensions : DeviceExtensions,
    pub physical_device : Arc<PhysicalDevice>,
    pub queue_family_index : u32,
    pub device : Arc<Device>,
    pub queue : Arc<Queue>,
    pub swapchain : Arc<Swapchain>,
    pub images : Vec<Arc<SwapchainImage>>,

}

impl Engine {
    pub fn init_physical_device(instance : &Arc<Instance>, device_extensions : &DeviceExtensions ,surface: &Arc<Surface>) -> (Arc<PhysicalDevice>, u32){
        return instance.enumerate_physical_devices()
            .unwrap()
            .filter(|p| {
                p.supported_extensions().contains(device_extensions)
            })
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                    _ => 5,
                }
            })
            .expect("no suitable physical device found");
    }

    pub fn init_device_and_queue(physical_device : &Arc<PhysicalDevice>, device_extensions : &DeviceExtensions ,queue_family_index: u32) -> (Arc<Device>, Arc<Queue>) {
        let (device,mut queues) = Device::new(
            // Which physical device to connect to.
            physical_device.clone(),

            DeviceCreateInfo {
                // A list of optional features and extensions that our program needs to work correctly.
                // Some parts of the Vulkan specs are optional and must be enabled manually at device
                // creation. In this example the only thing we are going to need is the `khr_swapchain`
                // extension that allows us to draw to a window.
                enabled_extensions: device_extensions.clone(),

                // The list of queues that we are going to use. Here we only use one queue, from the
                // previously chosen queue family.
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],

                ..Default::default()
            },
        )
            .unwrap();

        let queue = queues.next().unwrap();
        return (device,queue);
    }

    pub fn init_swapchain(device: &Arc<Device>,surface: &Arc<Surface>) -> (Arc<Swapchain>,Vec<Arc<SwapchainImage>>){
        return {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();
            let image_format = Some(
                device
                    .physical_device()
                    .surface_formats(&surface, Default::default())
                    .unwrap()[0]
                    .0,
            );
            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();

            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,
                    image_format,
                    image_extent: window.inner_size().into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )
                .unwrap()
        };
    }

    pub fn new() -> Self {

        let event_loop = EventLoop::new();
        let library = VulkanLibrary::new().unwrap();
        let required_extensions = vulkano_win::required_extensions(&library);
        let instance = Instance::new(
            library.clone(),
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                enumerate_portability: true,
                ..Default::default()
            },
        )
            .unwrap();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let ( physical_device, queue_family_index) = Self::init_physical_device(&instance,&device_extensions,&surface);
        let (device, queue) = Self::init_device_and_queue(&physical_device,&device_extensions,queue_family_index);

        let (swapchain, images) = Self::init_swapchain(&device,&surface);

        Self {
            event_loop,
            library: library.clone(),
            required_extensions,
            instance,
            surface,
            device_extensions,
            physical_device,
            queue_family_index,
            device,
            queue,
            swapchain,
            images,
        }
    }
}