use crate::device::Device;
use crate::util::Shared;
use ash::prelude::VkResult;
use ash::version::DeviceV1_0;
use ash::vk;

struct RaiiCommandPool {
    handle: vk::CommandPool,
    device: Device,
}

impl RaiiCommandPool {
    pub fn new(device: Device, create_info: &vk::CommandPoolCreateInfo) -> VkResult<Self> {
        unsafe {
            let raw_device = device.get_raw();
            let handle = raw_device.create_command_pool(create_info, None)?;
            Ok(Self { handle, device })
        }
    }

    /// Returns Vulkan handle.
    /// # Safety
    /// When no reference to Self left, returned object will be destroyed and will not be valid.
    /// If returned object will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> vk::CommandPool {
        self.handle
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }
}

impl Drop for RaiiCommandPool {
    fn drop(&mut self) {
        unsafe {
            self.device
                .get_raw()
                .destroy_command_pool(self.handle, None);
        }
    }
}

pub struct CommandPool {
    raii_command_pool: Shared<RaiiCommandPool>,
}

impl CommandPool {
    pub fn new(device: Device, create_info: &vk::CommandPoolCreateInfo) -> VkResult<Self> {
        unsafe {
            let raii_command_pool = Shared::new(RaiiCommandPool::new(device, create_info)?);
            Ok(Self { raii_command_pool })
        }
    }

    /// Returns Vulkan handle.
    /// # Safety
    /// When no reference to Self left, returned object will be destroyed and will not be valid.
    /// If returned object will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> vk::CommandPool {
        self.raii_command_pool.get_raw()
    }

    pub fn get_device(&self) -> &Device {
        &self.raii_command_pool.get_device()
    }
}
