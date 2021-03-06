use esp_idf_sys::wl_handle_t;
use std::ffi::CString;

pub const BASE_DIR: &str = "/mnt";

static mut WL_HANDLE: wl_handle_t = 0;

pub struct StorageHandle {
    base_path: CString,
}

impl StorageHandle {
    pub fn new() -> Self {
        let base_path = CString::new(BASE_DIR).expect("Invalid CString.");
        let partition_label = CString::new("storage").expect("Invalid CString.");

        let fat_cfg = esp_idf_sys::esp_vfs_fat_mount_config_t {
            max_files: 4,
            format_if_mount_failed: false,
            ..Default::default()
        };

        unsafe {
            esp_idf_sys::esp_vfs_fat_spiflash_mount(
                base_path.as_ptr(),
                partition_label.as_ptr(),
                &fat_cfg as *const _,
                &mut WL_HANDLE as *mut _,
            );
        }

        Self {
            base_path,
        }
    }
}

impl Default for StorageHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StorageHandle {
    fn drop(&mut self) {
        unsafe {
            esp_idf_sys::esp_vfs_fat_spiflash_unmount(self.base_path.as_ptr(), WL_HANDLE);
        }
    }
}
