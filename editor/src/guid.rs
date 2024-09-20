use std::sync::atomic::{self, AtomicU32};

#[derive(Debug)]
pub struct GuidGenerator(AtomicU32);

pub type Guid = u32;

impl GuidGenerator {
    pub const fn new() -> Self {
        Self(AtomicU32::new(0))
    }

    pub fn next_guid(&self) -> Guid {
        self.0.fetch_add(1, atomic::Ordering::Relaxed) as Guid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_guid_gen() {
        let gen = GuidGenerator::new();

        for i in 0..100 {
            assert_eq!(i, gen.next_guid())
        }
    }
}
