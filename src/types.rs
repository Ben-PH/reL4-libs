// TODO: make private, use conditional compilation
pub struct Word(pub usize);
pub struct CPtr(pub Word);

// #ifdef CONFIG_RETYPE_FAN_OUT_LIMIT
// #  define seL4_UntypedRetypeMaxObjects CONFIG_RETYPE_FAN_OUT_LIMIT
// #else
// #  define seL4_UntypedRetypeMaxObjects 256
// #endif

pub struct NodeId(pub Word);
pub struct PAddr(pub Word);
pub struct Domain(pub Word);

pub mod kern_obj_descriptors {
    //! These types are analagous to the FD (file descriptor) you get from a POSIX `open(1)` call:
    //! - You get something that is just a type-wrapped integer
    //! - You can combine that integer with a POSIX file-api system call
    //! - In this way, a POSIX file can be thought of a kernel-managed object+motheds item.
    //! - seL4 copabilities can be thought of as super-charged versions of POSIX FDs

    use super::CPtr;

    pub struct CNode(pub CPtr);
    pub struct IRQHandler(pub CPtr);
    pub struct IRQControl(pub CPtr);
    pub struct TCB(pub CPtr);
    pub struct Untyped(pub CPtr);
    pub struct DomainSet(pub CPtr);
    pub struct SchedContext(pub CPtr);
    pub struct SchedControl(pub CPtr);
}

pub struct Time(u64);
