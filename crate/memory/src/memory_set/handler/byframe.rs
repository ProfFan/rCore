use super::*;

#[derive(Debug, Clone)]
pub struct ByFrame<T: FrameAllocator> {
    allocator: T,
}

impl<T: FrameAllocator> MemoryHandler for ByFrame<T> {
    fn box_clone(&self) -> Box<MemoryHandler> {
        Box::new(self.clone())
    }

    fn map(&self, pt: &mut PageTable, addr: VirtAddr, attr: &MemoryAttr) {
        let target = self.allocator.alloc().expect("failed to allocate frame");
        let entry = pt.map(addr, target);
        attr.apply(entry);
    }

    fn unmap(&self, pt: &mut PageTable, addr: VirtAddr) {
        let target = pt.get_entry(addr).expect("fail to get entry").target();
        self.allocator.dealloc(target);
        pt.unmap(addr);
    }

    fn handle_page_fault(&self, _pt: &mut PageTable, _addr: VirtAddr) -> bool {
        false
    }
}

impl<T: FrameAllocator> ByFrame<T> {
    pub fn new(allocator: T) -> Self {
        ByFrame { allocator }
    }
}
