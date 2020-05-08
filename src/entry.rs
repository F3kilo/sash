use crate::util::Shared;
use ash::prelude::VkResult;
use ash::version::EntryV1_0;
use ash::vk;

#[derive(Clone)]
pub struct Entry {
    handle: Shared<ash::Entry>,
}

impl Entry {
    pub fn new() -> Result<Self, ash::LoadingError> {
        Ok(Self {
            handle: Shared::new(ash::Entry::new()?),
        })
    }

    pub fn try_enumerate_instance_version(&self) -> VkResult<Option<u32>> {
        self.handle.try_enumerate_instance_version()
    }

    pub fn enumerate_instance_layer_properties(&self) -> VkResult<Vec<vk::LayerProperties>> {
        self.handle.enumerate_instance_layer_properties()
    }

    pub fn enumerate_instance_extension_properties(
        &self,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        self.handle.enumerate_instance_extension_properties()
    }

    pub fn get_raw(&self) -> &ash::Entry {
        &self.handle
    }
}
