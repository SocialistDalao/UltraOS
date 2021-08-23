
use virtio_drivers::{VirtIOBlk, VirtIOHeader};
use crate::mm::{
    PhysAddr,
    VirtAddr,
    frame_alloc,
    frame_dealloc,
    PhysPageNum,
    FrameTracker,
    StepByOne,
    PageTable,
    //kernel_token,
    KERNEL_TOKEN,
};
use super::BlockDevice;
use spin::Mutex;
use alloc::vec::Vec;
use lazy_static::*;

#[allow(unused)]
const VIRTIO0: usize = 0x10001000;

pub struct VirtIOBlock(Mutex<VirtIOBlk<'static>>);

lazy_static! {
    static ref QUEUE_FRAMES: Mutex<Vec<FrameTracker>> = Mutex::new(Vec::new());
}

impl BlockDevice for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        // println!("{}",block_id);
        self.0.lock().read_block(block_id, buf).expect("Error when reading VirtIOBlk");
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0.lock().write_block(block_id, buf).expect("Error when writing VirtIOBlk");
    }
}


use crate::timer::get_time;
impl VirtIOBlock {
    #[allow(unused)]
    pub fn new() -> Self {
        let vb = Self(Mutex::new(VirtIOBlk::new(
            unsafe { &mut *(VIRTIO0 as *mut VirtIOHeader) }
        ).unwrap()));
        //vb.wtest();
        vb
    }

    pub fn wtest(&self){
        let mut buf = [0u8;512];
        self.read_block(0, &mut buf);
        let start = get_time();
        for i in 1..1000 {
            //println!("wtest");
            self.write_block(0, &buf);
        }
        let end = get_time();
        println!("[vblk writing test]: {}", end - start);
    }
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    let mut ppn_base = PhysPageNum(0);
    for i in 0..pages {
        let frame = frame_alloc().unwrap();
        if i == 0 { ppn_base = frame.ppn; }
        assert_eq!(frame.ppn.0, ppn_base.0 + i);
        QUEUE_FRAMES.lock().push(frame);
    }
    ppn_base.into()
}

#[no_mangle]
pub extern "C" fn virtio_dma_dealloc(pa: PhysAddr, pages: usize) -> i32 {
    let mut ppn_base: PhysPageNum = pa.into();
    for _ in 0..pages {
        frame_dealloc(ppn_base);
        ppn_base.step();
    }
    0
}

#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr(paddr.0)
}

#[no_mangle]
pub extern "C" fn virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    PageTable::from_token(KERNEL_TOKEN.token()).translate_va(vaddr).unwrap()
}
