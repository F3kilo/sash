extern crate sash;

use ash::vk;
use sash::entry::Entry;
use sash::instance::Instance;
use sash::p_device::PhysicalDevice;

fn main() {
    println!("Hello world!");

    init_vulkan();
}

fn init_vulkan() {
    let entry = Entry::new().expect("Can't create entry.");
    let instance =
        Instance::new(entry, &vk::InstanceCreateInfo::default()).expect("Can't create instance");
    let pdevice = PhysicalDevice::enumerate(&instance)
        .expect("Can't enumerate physical device")
        .remove(0);
    println!("{:#?}", pdevice.get_properties());
    println!("{:#?}", pdevice.get_features());
}
