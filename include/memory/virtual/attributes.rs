pub struct PageTableAttributes { }

impl PageTableAttributes {
    pub const PAGE_TABLE_ATTRIBUTES_SUPPORTED:  u64 = 0x00000001;
    pub const READ_WRITE_PERMISSION:            u64 = 0x00000002;
    pub const USED_SUPERVISION:                 u64 = 0x00000003;
    pub const WRITE_THROUGH_CACHING:            u64 = 0x00000004;
    pub const CACHE_DISABLE:                    u64 = 0x00000005;
    pub const ACCESSED:                         u64 = 0x00000006;
    pub const DIRTY:                            u64 = 0x00000007;
    pub const PAGE_ATTRIBUTES_TABLE:            u64 = 0x00000008;
    pub const GLOBAL:                           u64 = 0x00000009;

}