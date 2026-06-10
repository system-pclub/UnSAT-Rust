#![allow(dead_code)]
use core::arch::asm;

/// Performs a syscall with one argument
///
/// # Safety
/// - ensure running on x64 arch linux
/// - ensure all arguments match the underlying syscall ABI
#[inline(always)]
pub unsafe fn syscall1(syscall: usize, arg1: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

/// Performs a syscall with two arguments
///
/// # Safety
/// - ensure running on x64 arch linux
/// - ensure all arguments match the underlying syscall ABI
#[inline(always)]
pub unsafe fn syscall2(syscall: usize, arg1: usize, arg2: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

/// Performs a syscall with three arguments
///
/// # Safety
/// - ensure running on x64 arch linux
/// - ensure all arguments match the underlying syscall ABI
#[inline(always)]
pub unsafe fn syscall3(syscall: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

/// Performs a syscall with four arguments
///
/// # Safety
/// - ensure running on x64 arch linux
/// - ensure all arguments match the underlying syscall ABI
#[inline(always)]
pub unsafe fn syscall4(
    syscall: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

/// Performs a syscall with five arguments
///
/// # Safety
/// - ensure running on x64 arch linux
/// - ensure all arguments match the underlying syscall ABI
#[inline(always)]
pub unsafe fn syscall5(
    syscall: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}

/// Performs a syscall with six arguments
///
/// # Safety
/// - ensure running on x64 arch linux
/// - ensure all arguments match the underlying syscall ABI
#[inline(always)]
pub unsafe fn syscall6(
    syscall: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") syscall,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}
