use crate::instance::Instance;
use crate::p_device::PhysicalDevice;
use crate::util::Shared;
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::vk;

struct RaiiDevice {
    instance: Instance,
    pdevice: PhysicalDevice,
    handle: ash::Device,
}

impl RaiiDevice {
    pub fn new(
        instance: Instance,
        pdevice: PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
    ) -> Result<Self, vk::Result> {
        let raw_pdevice = unsafe { pdevice.get_raw() };
        let raw_instance = unsafe { instance.get_raw() };
        let handle = unsafe { raw_instance.create_device(raw_pdevice, create_info, None)? };
        Ok(Self {
            instance,
            pdevice,
            handle,
        })
    }

    pub fn get_instance(&self) -> &Instance {
        &self.instance
    }

    pub fn get_physical_device(&self) -> &PhysicalDevice {
        &self.pdevice
    }

    /// Returns reference to ash::Device object.
    /// # Safety
    /// When no reference to Self left, returned device object will be destroyed and will not be valid.
    /// If returned device will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> &ash::Device {
        &self.handle
    }
}

impl Drop for RaiiDevice {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_device(None);
        }
    }
}

#[derive(Clone)]
pub struct Device {
    raii_device: Shared<RaiiDevice>,
}

impl Device {
    pub fn new(
        instance: Instance,
        pdevice: PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
    ) -> Result<Self, vk::Result> {
        let raii_device = Shared::new(RaiiDevice::new(instance, pdevice, create_info)?);
        Ok(Self { raii_device })
    }

    pub fn get_instance(&self) -> &Instance {
        &self.raii_device.get_instance()
    }

    pub fn get_physical_device(&self) -> &PhysicalDevice {
        &self.raii_device.get_physical_device()
    }

    /// Returns reference to ash::Device object.
    /// # Safety
    /// When no reference to Self left, returned device object will be destroyed and will not be valid.
    /// If returned device will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> &ash::Device {
        &self.raii_device.get_raw()
    }
}
