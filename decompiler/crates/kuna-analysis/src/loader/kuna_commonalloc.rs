//! Size- and alignment-aware COMMON addresses, separate from extern slots.

pub(super) struct CommonAllocator {
    cursor: Option<u64>,
    limit: u64,
    first: Option<u64>,
}

impl CommonAllocator {
    pub(super) fn new(extern_base: u64, undefined_count: usize, is_64: bool) -> Self {
        let cursor = u64::try_from(undefined_count)
            .ok()
            .and_then(|count| count.checked_mul(16))
            .and_then(|size| extern_base.checked_add(size));
        Self {
            cursor,
            first: None,
            limit: if is_64 { u64::MAX } else { 1u64 << 32 },
        }
    }

    pub(super) fn range(&self) -> Option<(u64, u64)> {
        Some((self.first?, self.cursor?))
    }

    pub(super) fn allocate(&mut self, size: u64, alignment: u64) -> Option<u64> {
        let alignment = alignment.max(1);
        if !alignment.is_power_of_two() {
            return None;
        }
        let address = self.cursor?.checked_add(alignment - 1)? & !(alignment - 1);
        let end = address.checked_add(size.max(1))?;
        if end > self.limit {
            return None;
        }
        self.first.get_or_insert(address);
        self.cursor = Some(end);
        Some(address)
    }
}

#[cfg(test)]
mod tests {
    use super::CommonAllocator;

    #[test]
    fn full_sizes_and_alignment_keep_storage_disjoint() {
        let mut allocator = CommonAllocator::new(0x402000, 1, false);
        let first = allocator.allocate(64, 32).unwrap();
        let second = allocator.allocate(32, 64).unwrap();
        assert_eq!(first % 32, 0);
        assert_eq!(second % 64, 0);
        assert!(second >= first + 64);
        assert!(first >= 0x402010);
    }

    #[test]
    fn invalid_allocation_does_not_advance_or_wrap() {
        let mut allocator = CommonAllocator::new(0x402000, 0, true);
        assert_eq!(allocator.allocate(8, 3), None);
        assert_eq!(allocator.allocate(u64::MAX, 8), None);
        assert_eq!(allocator.allocate(8, 8), Some(0x402000));
        let mut narrow = CommonAllocator::new(0xfffffff0, 0, false);
        assert_eq!(narrow.allocate(32, 16), None);
        assert_eq!(narrow.allocate(16, 16), Some(0xfffffff0));
        let mut wide = CommonAllocator::new(u64::MAX - 3, 0, true);
        assert_eq!(wide.allocate(1, 8), None);
        assert_eq!(CommonAllocator::new(u64::MAX, 1, true).allocate(1, 1), None);
    }

    #[test]
    fn zero_size_symbols_still_have_distinct_addresses() {
        let mut allocator = CommonAllocator::new(0x402000, 0, false);
        assert_eq!(allocator.allocate(0, 0), Some(0x402000));
        assert_eq!(allocator.allocate(0, 0), Some(0x402001));
    }
}
