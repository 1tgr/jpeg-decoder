#[cfg(feature = "std")]
mod threaded;
mod immediate;

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32", not(feature = "std")))]
pub use self::immediate::ImmediateWorker as PlatformWorker;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32", not(feature = "std"))))]
pub use self::threaded::ThreadedWorker as PlatformWorker;

use alloc::sync::Arc;
use alloc::vec::Vec;
use error::Result;
use parser::Component;

pub struct RowData {
    pub index: usize,
    pub component: Component,
    pub quantization_table: Arc<[u16; 64]>,
}

pub trait Worker: Sized {
    fn new() -> Result<Self>;
    fn start(&mut self, row_data: RowData) -> Result<()>;
    fn append_row(&mut self, row: (usize, Vec<i16>)) -> Result<()>;
    fn get_result(&mut self, index: usize) -> Result<Vec<u8>>;
}
