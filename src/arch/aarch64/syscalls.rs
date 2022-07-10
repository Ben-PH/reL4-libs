//!   To simplify the definition of the various seL4 syscalls/syscall-wrappers we define
//!   some helper assembly functions. These functions are designed to cover the different
//!   cases of sending/receiving data in registers to/from the kernel. The most 'complex'
//!   version is arm_sys_send_recv, and all other functions are limited versions that allow
//!   for registers to not be unnecessarily clobbered

// /*
//  * Copyright 2020, Data61, CSIRO (ABN 41 687 119 230)
//  *
//  * SPDX-License-Identifier: BSD-2-Clause
//  */
use crate::types::Word;
use core::arch::asm;

// #include <autoconf.h>
// #include <sel4/functions.h>
// #include <sel4/types.h>

// #ifdef CONFIG_KERNEL_MCS
// #define MCS_PARAM_DECL(r)    register seL4_Word reply_reg asm(r) = reply
// #define MCS_PARAM    , "r"(reply_reg)
// #else
// #define MCS_PARAM_DECL(r)
// #define MCS_PARAM
// #endif

///   Only fills metadata registers into the kernel (skips message registers). Expects nothing to be sent back by the kernel. Used by directional one way sends that do not contain data (e.g. seL4_Notify)
pub fn arm_sys_send(sys: Word, dest: Word, info: Word, mr0: Word, mr1: Word, mr2: Word, mr3: Word) {
    //
    unsafe {
        asm!(
            "mov x0, {dest_ptr}",
            "mov x1, {info}",
            "mov x2, {msg0}",
            "mov x3, {msg1}",
            "mov x4, {msg2}",
            "mov x5, {msg3}",
            "mov x7, {scno}",
            "svc #7",
            dest_ptr = in(reg) dest.0,
            info = in(reg) info.0,
            msg0 = in(reg) mr0.0,
            msg1 = in(reg) mr1.0,
            msg2 = in(reg) mr2.0,
            msg3 = in(reg) mr3.0,
            scno = in(reg) sys.0,
            options(nomem),

        )
    }
}

#[cfg(not(kernel_mcs))]
#[inline(always)]
/// Similar to arm_sys_send except it does not take a word for the destination register. Used for undirected one way sends that contain data (e.g. seL4_Reply)
pub fn arm_sys_reply(sys: Word, info: Word, mr0: Word, mr1: Word, mr2: Word, mr3: Word) {
    unsafe {
        asm!(
            "mov x1, {info}",
            "mov x2, {msg0}",
            "mov x3, {msg1}",
            "mov x4, {msg2}",
            "mov x5, {msg3}",
            "mov x7, {scno}",
            "svc #0",
            info = in(reg) info.0,
            msg0 = in(reg) mr0.0,
            msg1 = in(reg) mr1.0,
            msg2 = in(reg) mr2.0,
            msg3 = in(reg) mr3.0,
            scno = in(reg) sys.0,
            options(nomem),
        )
    }
}

#[inline(always)]
/// Only fills metadata registers into the kernel (skips message registers). Expects nothing to be sent back by the kernel. Used by directional one way sends that do not contain data (e.g. seL4_Notify)
pub fn arm_sys_send_null(sys: Word, src: Word, info_arg: Word) {
    unsafe {
        asm!(
            "mov x0, {dest_ptr}",
            "mov x1, {info}",
            "mov x7, {scno}",
            "svc #0",
            dest_ptr = in(reg) src.0,
            info = in(reg) info_arg.0,
            scno = in(reg) sys.0,
            options(nomem),
        )
    }
}

/// Sends one register (destination) to the kernel and expects all registers to be returned by the kernel. Used for directed receives that return data (e.g. seL4_Recv)
// TODO: build-cfg with MCS stuff
pub fn arm_sys_recv(
    sys: Word,
    src: Word,
    out_badge: &mut Word,
    out_info: &mut Word,
    out_mr0: &mut Word,
    out_mr1: &mut Word,
    out_mr2: &mut Word,
    out_mr3: &mut Word,
    _reply: Word,
) {
    unsafe {
        asm!(
            "mov x0, {src_and_badge}",
            "mov x1, {info}",
            "mov x2, {msg0}",
            "mov x3, {msg1}",
            "mov x4, {msg2}",
            "mov x5, {msg3}",
            "mov x7, {scno}",
            "svc #0",
            src_and_badge = in(reg) src.0,
            info = out(reg) out_info.0,
            msg0 = out(reg) out_mr0.0,
            msg1 = out(reg) out_mr1.0,
            msg2 = out(reg) out_mr2.0,
            msg3 = out(reg) out_mr3.0,
            scno = in(reg) sys.0,
        )
    }
    out_badge.0 = src.0;
}

#[inline(always)]
/// Fills all registers into the kernel and expects all of them to be filled on return by the kernel. Used for directed send+receives where data flows both directions (e.g. seL4_Call, seL4_ReplyWait)
pub fn arm_sys_send_recv(
    sys: Word,
    dest: Word,
    out_badge: &mut Word,
    info_arg: Word,
    out_info: &mut Word,
    in_out_mr0: &mut Word,
    in_out_mr1: &mut Word,
    in_out_mr2: &mut Word,
    in_out_mr3: &mut Word,
    _reply: Word,
) {
    unsafe {
        asm!(
            "mov x0, {destptr}",
            "mov x1, {info}",
            "mov x2, {msg0}",
            "mov x3, {msg1}",
            "mov x4, {msg2}",
            "mov x5, {msg3}",
            "mov x7, {scno}",
            "svc #0",
            destptr = in(reg) dest.0,
            info = in(reg) info_arg.0,
            msg0 = inout(reg) in_out_mr0.0,
            msg1 = inout(reg) in_out_mr1.0,
            msg2 = inout(reg) in_out_mr2.0,
            msg3 = inout(reg) in_out_mr3.0,
            scno = in(reg) sys.0,
            options(nomem),
        )
    }
    out_badge.0 = dest.0;
    out_info.0 = info_arg.0;
}

#[cfg(kernel_mcs)]
#[inline(always)]
/// Fills all registers into the kernel and expects all of them to be filled on return by the kernel. Used for directed send+receives where data flows both directions on separate caps (e.g. seL4_NBSendRecv)
pub fn arm_sys_nbsend_recv(
    sys: Word,
    dest: Word,
    src: Word,
    out_badge: &mut Word,
    info_arg: Word,
    out_info: &mut Word,
    in_out_mr0: &mut Word,
    in_out_mr1: &mut Word,
    in_out_mr2: &mut Word,
    in_out_mr3: &mut Word,
    reply: Word,
) {
    unsafe {
        asm!(
            "mov x0, {src_and_badge}",
            "mov x1, {info}",
            "mov x2, {msg0}",
            "mov x3, {msg1}",
            "mov x4, {msg2}",
            "mov x5, {msg3}",
            "mov x6, {reply_reg}",
            "mov x8, {dest_reg}",
            "mov x7, {scno}",
            "svc #0",
            src_and_badge = in(reg) src.0,
            info = in(reg) info_arg.0,
            msg0 = inout(reg) in_out_mr0.0,
            msg1 = inout(reg) in_out_mr1.0,
            msg2 = inout(reg) in_out_mr2.0,
            msg3 = inout(reg) in_out_mr3.0,
            reply_reg = in(reg) reply.0,
            dest_reg = in(reg) dest,
            scno = in(reg) sys.0,
        )
    }
    out_badge.0 = src_and_badge.0;
    out_info.0 = info_arg.0;
}

/// Does not send any registers to the kernel or expect anything to be returned from the kernel. Used to trigger implicit kernel actions without any data (e.g. seL4_Yield)
#[inline(always)]
pub fn arm_sys_null(sys: Word) {
    unsafe { asm!("mov x7, {}", in(reg) sys.0, options(nomem)) }
}
