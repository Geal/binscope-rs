#![allow(dead_code)]

pub const PAGE_SIZE: usize = 4096;

pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;

pub const IMAGE_FILE_MACHINE_UNKNOWN: u16 = 0;
pub const IMAGE_FILE_MACHINE_I386: u16 = 0x014c;
pub const IMAGE_FILE_MACHINE_R3000: u16 = 0x0162;
pub const IMAGE_FILE_MACHINE_R4000: u16 = 0x0166;
pub const IMAGE_FILE_MACHINE_R10000: u16 = 0x0168;
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: u16 = 0x0169;
pub const IMAGE_FILE_MACHINE_ALPHA: u16 = 0x0184;
pub const IMAGE_FILE_MACHINE_SH3: u16 = 0x01a2;
pub const IMAGE_FILE_MACHINE_SH3DSP: u16 = 0x01a3;
pub const IMAGE_FILE_MACHINE_SH3E: u16 = 0x01a4;
pub const IMAGE_FILE_MACHINE_SH4: u16 = 0x01a6;
pub const IMAGE_FILE_MACHINE_SH5: u16 = 0x01a8;
pub const IMAGE_FILE_MACHINE_ARM: u16 = 0x01c0;
pub const IMAGE_FILE_MACHINE_ARMV7: u16 = 0x01c4;
pub const IMAGE_FILE_MACHINE_ARMNT: u16 = 0x01c4;
pub const IMAGE_FILE_MACHINE_THUMB: u16 = 0x01c2;
pub const IMAGE_FILE_MACHINE_AM33: u16 = 0x01d3;
pub const IMAGE_FILE_MACHINE_POWERPC: u16 = 0x01F0;
pub const IMAGE_FILE_MACHINE_POWERPCFP: u16 = 0x01f1;
pub const IMAGE_FILE_MACHINE_IA64: u16 = 0x0200;
pub const IMAGE_FILE_MACHINE_MIPS16: u16 = 0x0266;
pub const IMAGE_FILE_MACHINE_ALPHA64: u16 = 0x0284;
pub const IMAGE_FILE_MACHINE_MIPSFPU: u16 = 0x0366;
pub const IMAGE_FILE_MACHINE_MIPSFPU16: u16 = 0x0466;
pub const IMAGE_FILE_MACHINE_AXP64: u16 = IMAGE_FILE_MACHINE_ALPHA64;
pub const IMAGE_FILE_MACHINE_TRICORE: u16 = 0x0520;
pub const IMAGE_FILE_MACHINE_CEF: u16 = 0x0CEF;
pub const IMAGE_FILE_MACHINE_EBC: u16 = 0x0EBC;
pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
pub const IMAGE_FILE_MACHINE_M32R: u16 = 0x9041;
pub const IMAGE_FILE_MACHINE_CEE: u16 = 0xc0ee;

pub const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 0x0004;
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 0x0008;
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: u16 = 0x0010;
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020;
pub const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 0x0080;
pub const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100;
pub const IMAGE_FILE_DEBUG_STRIPPED: u16 = 0x0200;
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400;
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 0x0800;
pub const IMAGE_FILE_SYSTEM: u16 = 0x1000;
pub const IMAGE_FILE_DLL: u16 = 0x2000;
pub const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 0x4000;
pub const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 0x8000;

pub const IMAGE_SUBSYSTEM_UNKNOWN: u16 = 0;
pub const IMAGE_SUBSYSTEM_NATIVE: u16 = 1;
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2;
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 3;
pub const IMAGE_SUBSYSTEM_OS2_CUI: u16 = 5;
pub const IMAGE_SUBSYSTEM_POSIX_CUI: u16 = 7;
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u16 = 8;
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16 = 9;
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16 = 10;
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16 = 11;
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16 = 12;
pub const IMAGE_SUBSYSTEM_EFI_ROM: u16 = 13;
pub const IMAGE_SUBSYSTEM_XBOX: u16 = 14;
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 16;

pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 0x0040;
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u16 = 0x0080;
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16 = 0x0100;
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u16 = 0x0200;
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16 = 0x0400;
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: u16 = 0x0800;
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: u16 = 0x1000;
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u16 = 0x2000;
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 0x8000;

pub const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0;
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2;
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: usize = 3;
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: usize = 4;
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5;
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: usize = 6;
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: usize = 7;
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: usize = 8;
pub const IMAGE_DIRECTORY_ENTRY_TLS: usize = 9;
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize = 10;
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: usize = 11;
pub const IMAGE_DIRECTORY_ENTRY_IAT: usize = 12;
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: usize = 13;
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: usize = 14;

pub const IMAGE_SCN_TYPE_NO_PAD: u32 = 0x00000008;

pub const IMAGE_SCN_CNT_CODE: u32 = 0x00000020;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x00000040;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x00000080;
pub const IMAGE_SCN_LNK_OTHER: u32 = 0x00000100;
pub const IMAGE_SCN_LNK_INFO: u32 = 0x00000200;
pub const IMAGE_SCN_LNK_REMOVE: u32 = 0x00000800;
pub const IMAGE_SCN_LNK_COMDAT: u32 = 0x00001000;
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: u32 = 0x00004000;
pub const IMAGE_SCN_GPREL: u32 = 0x00008000;
pub const IMAGE_SCN_MEM_FARDATA: u32 = 0x00008000;
pub const IMAGE_SCN_MEM_PURGEABLE: u32 = 0x00020000;
pub const IMAGE_SCN_MEM_16BIT: u32 = 0x00020000;
pub const IMAGE_SCN_MEM_LOCKED: u32 = 0x00040000;
pub const IMAGE_SCN_MEM_PRELOAD: u32 = 0x00080000;

pub const IMAGE_SCN_ALIGN_1BYTES: u32 = 0x00100000;
pub const IMAGE_SCN_ALIGN_2BYTES: u32 = 0x00200000;
pub const IMAGE_SCN_ALIGN_4BYTES: u32 = 0x00300000;
pub const IMAGE_SCN_ALIGN_8BYTES: u32 = 0x00400000;
pub const IMAGE_SCN_ALIGN_16BYTES: u32 = 0x00500000;
pub const IMAGE_SCN_ALIGN_32BYTES: u32 = 0x00600000;
pub const IMAGE_SCN_ALIGN_64BYTES: u32 = 0x00700000;
pub const IMAGE_SCN_ALIGN_128BYTES: u32 = 0x00800000;
pub const IMAGE_SCN_ALIGN_256BYTES: u32 = 0x00900000;
pub const IMAGE_SCN_ALIGN_512BYTES: u32 = 0x00A00000;
pub const IMAGE_SCN_ALIGN_1024BYTES: u32 = 0x00B00000;
pub const IMAGE_SCN_ALIGN_2048BYTES: u32 = 0x00C00000;
pub const IMAGE_SCN_ALIGN_4096BYTES: u32 = 0x00D00000;
pub const IMAGE_SCN_ALIGN_8192BYTES: u32 = 0x00E00000;

pub const IMAGE_SCN_ALIGN_MASK: u32 = 0x00F00000;

pub const IMAGE_SCN_LNK_NRELOC_OVFL: u32 = 0x01000000;
pub const IMAGE_SCN_MEM_DISCARDABLE: u32 = 0x02000000;
pub const IMAGE_SCN_MEM_NOT_CACHED: u32 = 0x04000000;
pub const IMAGE_SCN_MEM_NOT_PAGED: u32 = 0x08000000;
pub const IMAGE_SCN_MEM_SHARED: u32 = 0x10000000;
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;
pub const IMAGE_SCN_MEM_READ: u32 = 0x40000000;
pub const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000;

pub const IMAGE_SCN_SCALE_INDEX: u32 = 0x00000001;

pub const IMAGE_SIZEOF_SHORT_NAME: usize = 8;
