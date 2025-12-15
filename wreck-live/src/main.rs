use vulkano::{
    VulkanLibrary,
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
};

fn main() {
    let library = VulkanLibrary::new().expect("Where's Vulkan?");
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )
    .expect("Instance should be created");
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");
}
