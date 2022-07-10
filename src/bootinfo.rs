const MAX_BOOTINFO_UNTYPED_CAPS: usize = env!(BOOT_INFO_UT_CAPS_MAX);
pub enum InitialCap {
    // null cap
    CapNull,
    // initial thread's TCB cap
    CapInitThreadTCB,
    // initial thread's root CNode cap
    CapInitThreadCNode,
    // initial thread's VSpace cap
    CapInitThreadVSpace,
    // global IRQ controller cap
    CapIRQControl,
    // global ASID controller cap
    CapASIDControl,
    // initial thread's ASID pool cap
    CapInitThreadASIDPool,
    // global IO port control cap (null cap if not supported)
    CapIOPortControl,
    // global IO space cap (null cap if no IOMMU support)
    CapIOSpace,
    // bootinfo frame cap
    CapBootInfoFrame,
    // initial thread's IPC buffer frame cap
    CapInitThreadIPCBuffer,
    // global domain controller cap
    CapDomain,
    // global SMMU SID controller cap, null cap if not supported
    CapSMMUSIDControl,
    // global SMMU CB controller cap, null cap if not supported
    CapSMMUCBControl,
    #[cfg(kernel_mcs)]
    // initial thread's scheduling context cap
    CapInitThreadSC,
}

// /* Legacy code will have assumptions on the vspace root being a Page Directory
//  * type, so for now we define one to the other */
// #define seL4_CapInitThreadPD seL4_CapInitThreadVSpace

// /* types */
// typedef seL4_Word seL4_SlotPos;
struct SlotPos(Word);

struct SlotRegion {
    start: SlotPos, /* first CNode slot position OF region */
    end: SlotPos,   /* first CNode slot position AFTER region */
}

struct UntypedDesc {
    paddr: Word,   /* physical address of untyped cap  */
    size_bits: u8, /* size (2^n) bytes of each untyped */
    is_device: u8, /* whether the untyped is a device  */
    padding: [u8; sizeof(Word) - 2 * sizeof(u8)],
}

struct BootInfo {
    // length of any additional bootinfo information
    extra_len: Word,
    // ID [0..numNodes-1] of the seL4 node (0 if uniprocessor)
    node_id: NodeId,
    // number of seL4 nodes (1 if uniprocessor)
    numNodes: Word,
    // number of IOMMU PT levels (0 if no IOMMU support)
    numIOPTLevels: Word,
    // pointer to initial thread's IPC buffer
    // TODO: Determine the nature of the collection (static defined width? dynamic? configed?)
    ipc_buffer: [IPCBuffer; 0],
    // empty slots (null caps)
    empty: SlotRegion,
    // shared-frame caps (shared between seL4 nodes)
    shared_frames: SlotRegion,
    // userland-image frame caps
    user_image_frames: SlotRegion,
    // userland-image paging structure caps
    user_image_paging: SlotRegion,
    // IOSpace caps for ARM SMMU
    io_space_caps: SlotRegion,
    // caps for any pages used to back the additional bootinfo information
    extra_boot_info_pages: SlotRegion,
    // initial thread's root CNode size (2^n slots)
    init_thread_cnode_size_bits: SlotRegion,
    // Initial thread's domain ID
    init_thread_domain: Domain,
    #[cfg(kernel_mcs)]
    // Caps to sched_control for each node
    schedcontrol: SlotRegion,
    // untyped-object caps (untyped caps)
    untyped: SlotRegion,
    // information about each untyped
    /* the untypedList should be the last entry in this struct, in order
     * to make this struct easier to represent in other languages */
    untyped_list: [UntypedDesc; MAX_BOOTINFO_UNTYPED_CAPS],
}

// /* If extraLen > 0, then 4K after the start of bootinfo there is a region of the
//  * size extraLen that contains additional boot info data chunks. They are
//  * arch/platform specific and may or may not exist in any given execution. Each
//  * chunk has a header that contains an ID to describe the chunk. All IDs share a
//  * global namespace to ensure uniqueness.
//  */
// typedef enum {
//     SEL4_BOOTINFO_HEADER_PADDING            = 0,
//     SEL4_BOOTINFO_HEADER_X86_VBE            = 1,
//     SEL4_BOOTINFO_HEADER_X86_MBMMAP         = 2,
//     SEL4_BOOTINFO_HEADER_X86_ACPI_RSDP      = 3,
//     SEL4_BOOTINFO_HEADER_X86_FRAMEBUFFER    = 4,
//     SEL4_BOOTINFO_HEADER_X86_TSC_FREQ       = 5, /* frequency is in MHz */
//     SEL4_BOOTINFO_HEADER_FDT                = 6, /* device tree */
//     /* Add more IDs here, the two elements below must always be at the end. */
//     SEL4_BOOTINFO_HEADER_NUM,
//     SEL4_FORCE_LONG_ENUM(seL4_BootInfoID)
// } seL4_BootInfoID;

// /* Common header for all additional bootinfo chunks to describe the chunk. */
// typedef struct seL4_BootInfoHeader {
//     seL4_Word id;  /* identifier of the following blob */
//     seL4_Word len; /* length of the chunk, including this header */
// } seL4_BootInfoHeader;

// SEL4_COMPILE_ASSERT(
//     invalid_seL4_BootInfoHeader,
//     sizeof(seL4_BootInfoHeader) == 2 * sizeof(seL4_Word));
