use crate::entry::Entry;
use crate::util::Shared;
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk;

struct RaiiInstance {
    entry: Entry,
    handle: ash::Instance,
}

impl RaiiInstance {
    pub fn new(
        entry: Entry,
        create_info: &vk::InstanceCreateInfo,
    ) -> Result<Self, ash::InstanceError> {
        let handle = unsafe { entry.get_raw().create_instance(create_info, None)? };
        Ok(Self { entry, handle })
    }

    pub fn get_entry(&self) -> &Entry {
        &self.entry
    }

    /// Returns Vulkan handle.
    /// # Safety
    /// When no reference to Self left, returned object will be destroyed and will not be valid.
    /// If returned object will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> &ash::Instance {
        &self.handle
    }
}

impl Drop for RaiiInstance {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_instance(None);
        }
    }
}

#[derive(Clone)]
pub struct Instance {
    raii_instance: Shared<RaiiInstance>,
}

impl Instance {
    pub fn new(
        entry: Entry,
        create_info: &vk::InstanceCreateInfo,
    ) -> Result<Self, ash::InstanceError> {
        let raii_instance = Shared::new(RaiiInstance::new(entry, create_info)?);
        Ok(Self { raii_instance })
    }

    pub fn get_entry(&self) -> &Entry {
        &self.raii_instance.get_entry()
    }

    /// Returns Vulkan handle.
    /// # Safety
    /// When no reference to Self left, returned object will be destroyed and will not be valid.
    /// If returned object will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> &ash::Instance {
        self.raii_instance.get_raw()
    }
}
