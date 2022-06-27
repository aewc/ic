#[cfg(not(target_arch = "wasm32"))]
use crate::VectorMemory;
#[cfg(target_arch = "wasm32")]
use crate::{Memory, WASM_PAGE_SIZE};
#[cfg(target_arch = "wasm32")]
use ic_cdk::api::stable::{stable64_grow, stable64_read, stable64_size, stable64_write};

#[cfg(target_arch = "wasm32")]
const GB: u64 = 1 << 30;
#[cfg(target_arch = "wasm32")]
const STABLE_MEMORY_SIZE: u64 = 8 * GB;
#[cfg(target_arch = "wasm32")]
const MAX_PAGES: u64 = STABLE_MEMORY_SIZE / WASM_PAGE_SIZE;

/// A `Memory` that is based on canister stable storage.
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Default)]
pub struct StableStorage;

#[cfg(not(target_arch = "wasm32"))]
pub type StableStorage = VectorMemory;

#[cfg(target_arch = "wasm32")]
impl Memory for StableStorage {
    fn size(&self) -> u64 {
        stable64_size()
    }

    fn grow(&self, pages: u64) -> i64 {
        let size = self.size();
        match size.checked_add(pages) {
            Some(n) => {
                if n > MAX_PAGES {
                    return -1;
                }
                let result = stable64_grow(pages);
                if result.is_err() {
                    return -1;
                }
                size as i64
            }
            None => -1,
        }
    }

    fn read(&self, offset: u64, dst: &mut [u8]) {
        let n = offset
            .checked_add(dst.len() as u64)
            .expect("read: out of bounds");

        if n > self.size() * WASM_PAGE_SIZE {
            panic!("read: out of bounds");
        }
        stable64_read(offset, dst);
    }

    fn write(&self, offset: u64, src: &[u8]) {
        let n = offset
            .checked_add(src.len() as u64)
            .expect("write: out of bounds");

        if n > self.size() * WASM_PAGE_SIZE {
            panic!("write: out of bounds");
        }
        stable64_write(offset, src);
    }
}
