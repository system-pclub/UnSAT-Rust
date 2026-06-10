#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use core::arch::asm;

use windows_sys::Win32::{
    Foundation::HINSTANCE,
    System::{
        Diagnostics::Debug::{IMAGE_DATA_DIRECTORY, IMAGE_NT_HEADERS64},
        Kernel::LIST_ENTRY,
        SystemServices::{IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY},
        Threading::PEB,
        WindowsProgramming::LDR_DATA_TABLE_ENTRY,
    },
};

mod utils;

use utils::{c_strlen, cmp_utf16_ascii_caseinsensitive, slicecmp};

/// `fn get_proc_from_module(module_name: &str, func: ty) -> Option<ty>`
///
/// this macro takes a dll name of a loaded module and a `Type` whose
/// name matches one of the functions exported by the module, and returns
/// an `Option<T>` containing `Type` if a function of the name `Type` was found
/// within the module's exports
#[macro_export]
macro_rules! __get_proc_from_module_x64 {
    ($module_name:literal, $func:ty) => {
        match aragonite::win::x64::pebwalk::get_module_by_name($module_name) {
            Some(handle) => aragonite::win::x64::pebwalk::get_proc_addr!(handle, $func),
            None => None,
        }
    };
}
pub use crate::__get_proc_from_module_x64 as get_proc_from_module;

/// `fn get_proc_addr(module_name: HANDLE, func: ty) -> Option<ty>`
///
/// this macro takes a `HANDLE` to a loaded module and a `Type` whose name
/// matches one of the functions exported by the module in `HANDLE`, and
/// return an `Option<Type>` containing `Type` if a function of the name
/// `Type` was found within  the `HANDLE`'s exports
#[macro_export]
macro_rules! __get_proc_addr_x64 {
    ($handle:expr, $func:ty) => {
        aragonite::win::x64::pebwalk::get_proc_address::<$func>($handle, stringify!($func))
    };
}
pub use crate::__get_proc_addr_x64 as get_proc_addr;

/// read a qword from an offset at the gs register
///
/// # Safety
/// - ensure running on x64 arch
#[inline(always)]
pub unsafe fn __readgsqword(offset: u64) -> u64 {
    let ret: u64;
    asm!(
        "mov {}, gs:[{:e}]",
        lateout(reg) ret,
        in(reg) offset,
        options(nostack, readonly),
    );
    ret
}

/// walks the loaded modules list from the PEB to find a matching module, case insensitive
pub fn get_module_by_name(module_name: &str) -> Option<HINSTANCE> {
    let peb_offset: *const u64 = unsafe { __readgsqword(0x60) as *const u64 };
    let rf_peb: *const PEB = peb_offset as *const PEB;
    let peb = unsafe { *rf_peb };
    let ldr = unsafe { *peb.Ldr };

    let mut p_ldr_data_table_entry: *const LDR_DATA_TABLE_ENTRY =
        ldr.InMemoryOrderModuleList.Flink as *const LDR_DATA_TABLE_ENTRY;
    let mut p_list_entry = &ldr.InMemoryOrderModuleList as *const LIST_ENTRY;

    loop {
        let dll_name = unsafe {
            core::slice::from_raw_parts(
                (*p_ldr_data_table_entry).FullDllName.Buffer,
                (*p_ldr_data_table_entry).FullDllName.Length as usize / 2,
            )
        };
        if cmp_utf16_ascii_caseinsensitive(dll_name, module_name) {
            let module_base: HINSTANCE =
                unsafe { *p_ldr_data_table_entry }.Reserved2[0] as HINSTANCE;
            return Some(module_base);
        }

        if p_list_entry == ldr.InMemoryOrderModuleList.Blink {
            break;
        }
        p_list_entry = unsafe { *p_list_entry }.Flink;
        p_ldr_data_table_entry = unsafe { *p_list_entry }.Flink as *const LDR_DATA_TABLE_ENTRY;
    }
    None
}

/// parses the module exports to find the function with the exact match name
pub fn get_proc_address<T>(module_handle: HINSTANCE, func_name: &str) -> Option<T> {
    let dos_header = module_handle as *const IMAGE_DOS_HEADER;
    let module_handle = module_handle as u64;
    let nt_headers =
        (module_handle + unsafe { (*dos_header).e_lfanew as u64 }) as *const IMAGE_NT_HEADERS64;
    let data_directory =
        (&unsafe { (*nt_headers).OptionalHeader.DataDirectory[0] }) as *const IMAGE_DATA_DIRECTORY;
    let export_directory = (module_handle + unsafe { (*data_directory).VirtualAddress as u64 })
        as *const IMAGE_EXPORT_DIRECTORY;
    let address_array = module_handle + unsafe { (*export_directory).AddressOfFunctions as u64 };
    let mut name_array = module_handle + unsafe { (*export_directory).AddressOfNames as u64 };
    let mut name_ordinals =
        module_handle + unsafe { (*export_directory).AddressOfNameOrdinals as u64 };
    let num_funcs = unsafe { (*export_directory).NumberOfFunctions };
    for _ in 0..num_funcs {
        let name_offset = unsafe { *(name_array as *const u32) };
        let curr_func_name = (module_handle + name_offset as u64) as *const u8;
        let curr_func_name =
            unsafe { core::slice::from_raw_parts(curr_func_name, c_strlen(curr_func_name)) };
        if slicecmp(curr_func_name, func_name.as_bytes()) {
            let address_array = address_array
                + (unsafe { *(name_ordinals as *const u16) } as u64
                    * (core::mem::size_of::<u32>() as u64));
            let func_addr: T = unsafe {
                core::mem::transmute_copy(&(module_handle + *(address_array as *const u32) as u64))
            };
            return Some(func_addr);
        }

        name_array += core::mem::size_of::<u32>() as u64;
        name_ordinals += core::mem::size_of::<u16>() as u64;
    }
    None
}
