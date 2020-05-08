use crate::instance::Instance;
use ash::prelude::VkResult;
use ash::version::{InstanceV1_0, InstanceV1_1};
use ash::vk;
use ash::vk::PhysicalDeviceGroupProperties;

#[derive(Clone)]
pub struct PhysicalDevice {
    handle: vk::PhysicalDevice,
    instance: Instance,
}

impl PhysicalDevice {
    pub fn get_instance(&self) -> &Instance {
        &self.instance
    }

    pub fn enumerate(instance: &Instance) -> VkResult<Vec<Self>> {
        Ok(unsafe { instance.get_raw().enumerate_physical_devices()? }
            .iter()
            .map(|pdevice| Self {
                handle: *pdevice,
                instance: instance.clone(),
            })
            .collect())
    }

    pub fn enumerate_extension_properties(
        &self,
    ) -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        unsafe {
            self.instance
                .get_raw()
                .enumerate_device_extension_properties(self.handle)
        }
    }

    pub fn get_features(&self) -> vk::PhysicalDeviceFeatures {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_features(self.handle)
        }
    }

    pub fn get_format_properties(&self, format: vk::Format) -> vk::FormatProperties {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_format_properties(self.handle, format)
        }
    }

    pub fn get_image_format_properties(
        &self,
        format: vk::Format,
        typ: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
    ) -> VkResult<vk::ImageFormatProperties> {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_image_format_properties(
                    self.handle,
                    format,
                    typ,
                    tiling,
                    usage,
                    flags,
                )
        }
    }

    pub fn get_memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_memory_properties(self.handle)
        }
    }

    pub fn get_properties(&self) -> vk::PhysicalDeviceProperties {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_properties(self.handle)
        }
    }

    pub fn get_queue_family_properties(&self) -> Vec<vk::QueueFamilyProperties> {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_queue_family_properties(self.handle)
        }
    }

    pub fn enumerate_groups_len(instance: &Instance) -> usize {
        unsafe { instance.get_raw().enumerate_physical_device_groups_len() }
    }

    pub fn enumerate_groups(
        instance: &Instance,
        out: &mut [PhysicalDeviceGroupProperties],
    ) -> VkResult<()> {
        unsafe { instance.get_raw().enumerate_physical_device_groups(out) }
    }

    /// Returns Vulkan handle.
    /// # Safety
    /// When no reference to Self left, returned object will be destroyed and will not be valid.
    /// If returned object will be destroyed explicitly, than all clones of Self will become not valid.
    pub unsafe fn get_raw(&self) -> vk::PhysicalDevice {
        self.handle
    }

    pub fn get_external_buffer_properties(
        &self,
        external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo,
        out: &mut vk::ExternalBufferProperties,
    ) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_external_buffer_properties(
                    self.handle,
                    external_buffer_info,
                    out,
                )
        }
    }

    pub fn get_external_fence_properties(
        &self,
        external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
        out: &mut vk::ExternalFenceProperties,
    ) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_external_fence_properties(
                    self.handle,
                    external_fence_info,
                    out,
                )
        }
    }

    pub fn get_external_semaphore_properties(
        &self,
        external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
        out: &mut vk::ExternalSemaphoreProperties,
    ) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_external_semaphore_properties(
                    self.handle,
                    external_semaphore_info,
                    out,
                )
        }
    }

    pub fn get_features2(&self, features: &mut vk::PhysicalDeviceFeatures2) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_features2(self.handle, features)
        }
    }

    pub fn get_format_properties2(&self, format: vk::Format, out: &mut vk::FormatProperties2) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_format_properties2(self.handle, format, out)
        }
    }

    pub fn get_image_format_properties2(
        &self,
        format_info: &vk::PhysicalDeviceImageFormatInfo2,
        image_format_prop: &mut vk::ImageFormatProperties2,
    ) -> VkResult<()> {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_image_format_properties2(
                    self.handle,
                    format_info,
                    image_format_prop,
                )
        }
    }

    pub fn get_memory_properties2(&self, out: &mut vk::PhysicalDeviceMemoryProperties2) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_memory_properties2(self.handle, out)
        }
    }

    pub fn get_properties2(&self, prop: &mut vk::PhysicalDeviceProperties2) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_properties2(self.handle, prop)
        }
    }

    pub fn get_queue_family_properties2(
        &self,
        queue_family_props: &mut [vk::QueueFamilyProperties2],
    ) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_queue_family_properties2(self.handle, queue_family_props)
        }
    }

    pub fn get_queue_family_properties2_len(&self) -> usize {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_queue_family_properties2_len(self.handle)
        }
    }

    pub fn get_sparse_image_format_properties2(
        &self,
        format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
        out: &mut [vk::SparseImageFormatProperties2],
    ) {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_sparse_image_format_properties2(self.handle, format_info, out)
        }
    }

    pub fn get_sparse_image_format_properties2_len(
        &self,
        format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> usize {
        unsafe {
            self.instance
                .get_raw()
                .get_physical_device_sparse_image_format_properties2_len(self.handle, format_info)
        }
    }
}
