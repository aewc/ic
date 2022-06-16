use crate::{Memory, WASM_PAGE_SIZE};
use ic_cdk::api::stable::{stable64_grow, stable64_read, stable64_size, stable64_write};
use std::marker::PhantomData;

const GB: u64 = 1 << 30;
const STABLE_MEMORY_SIZE: u64 = 8 * GB;
const MAX_PAGES: u64 = STABLE_MEMORY_SIZE / WASM_PAGE_SIZE;

/// A `Memory` that is based on a vector.
pub struct StableStorage<T> {
    _marker: PhantomData<T>,
}

impl<T: candid::CandidType + serde::de::DeserializeOwned> Memory for StableStorage<T> {
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
