use crate::entry::Entry;
use crate::util::Shared;
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk;

struct AutoDestroyInstance {
    entry: Entry,
    handle: ash::Instance,
}

impl AutoDestroyInstance {
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

    /// Returns reference to ash::Insance object.
    /// # Safety
    /// When no reference to Self left, returned instance object will be destroyed and will not be valid.
    /// If returned instance will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> &ash::Instance {
        &self.handle
    }
}

impl Drop for AutoDestroyInstance {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_instance(None);
        }
    }
}

#[derive(Clone)]
pub struct Instance {
    raii_instance: Shared<AutoDestroyInstance>,
}

impl Instance {
    pub fn new(
        entry: Entry,
        create_info: &vk::InstanceCreateInfo,
    ) -> Result<Self, ash::InstanceError> {
        let raii_instance = Shared::new(AutoDestroyInstance::new(entry, create_info)?);
        Ok(Self { raii_instance })
    }

    pub fn get_entry(&self) -> &Entry {
        &self.raii_instance.get_entry()
    }

    /// Returns reference to ash::Insance object.
    /// # Safety
    /// When no reference to Self left, returned instance object will be destroyed and will not be valid.
    /// If returned instance will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> &ash::Instance {
        self.raii_instance.get_raw()
    }
}
