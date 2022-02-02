mod storage;

use anyhow::Result;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use crate::storage::StorageHandle;

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let _storage_handle = StorageHandle::default();

    use std::io::Read;
    let mut lorem = std::fs::File::open("/mnt/readme.txt")?;
    let mut buf = String::new();
    lorem.read_to_string(&mut buf)?;
    println!("{}", buf);

    Ok(())
}
