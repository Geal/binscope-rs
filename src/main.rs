#![feature(trace_macros)]

extern crate byteorder;
extern crate libc;
extern crate mmap;
#[macro_use] extern crate nom;

use std::fs;
use std::io::Cursor;

use byteorder::{ReadBytesExt, LittleEndian};
use mmap::{MemoryMap, MapOption};
use nom::{IResult, le_i32, le_u8, le_u16, le_u32, le_u64};


// --------------------------------------------------

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

#[cfg(unix)]
fn get_fd(file: &fs::File) -> libc::c_int {
  file.as_raw_fd()
}

#[cfg(windows)]
fn get_fd(file: &fs::File) -> libc::HANDLE {
  file.as_raw_handle()
}

// --------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct DosHeader {
  pub e_magic:    u16,
  pub e_cblp:     u16,
  pub e_cp:       u16,
  pub e_crlc:     u16,
  pub e_cparhdr:  u16,
  pub e_minalloc: u16,
  pub e_maxalloc: u16,
  pub e_ss:       u16,
  pub e_sp:       u16,
  pub e_csum:     u16,
  pub e_ip:       u16,
  pub e_cs:       u16,
  pub e_lfarlc:   u16,
  pub e_ovno:     u16,
  pub e_res:      [u16; 4],
  pub e_oemid:    u16,
  pub e_oeminfo:  u16,
  pub e_res2:     [u16; 10],
  pub e_lfanew:   i32,
}


pub fn dos_header(input: &[u8]) -> IResult<&[u8], DosHeader> {
  chain!(input,
    tag!("MZ")         ~
    e_cblp:     le_u16 ~
    e_cp:       le_u16 ~
    e_crlc:     le_u16 ~
    e_cparhdr:  le_u16 ~
    e_minalloc: le_u16 ~
    e_maxalloc: le_u16 ~
    e_ss:       le_u16 ~
    e_sp:       le_u16 ~
    e_csum:     le_u16 ~
    e_ip:       le_u16 ~
    e_cs:       le_u16 ~
    e_lfarlc:   le_u16 ~
    e_ovno:     le_u16 ~
    //e_res:      le_u16 ~  TODO
    take!(8)           ~
    e_oemid:    le_u16 ~
    e_oeminfo:  le_u16 ~
    //e_res2:     le_u16 ~  TODO
    take!(20)          ~
    e_lfanew:   le_i32 ,
    || {
      DosHeader {
        e_magic:    0x5A4D, // 'MZ',
        e_cblp:     e_cblp,
        e_cp:       e_cp,
        e_crlc:     e_crlc,
        e_cparhdr:  e_cparhdr,
        e_minalloc: e_minalloc,
        e_maxalloc: e_maxalloc,
        e_ss:       e_ss,
        e_sp:       e_sp,
        e_csum:     e_csum,
        e_ip:       e_ip,
        e_cs:       e_cs,
        e_lfarlc:   e_lfarlc,
        e_ovno:     e_ovno,
        //e_res:      le_u16 ~  TODO
        e_res: [0; 4],
        e_oemid:    e_oemid,
        e_oeminfo:  e_oeminfo,
        //e_res2:     le_u16 ~  TODO
        e_res2: [0; 10],
        e_lfanew:   e_lfanew,
      }
    }
   )
}


#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct NtHeaders {
  pub Signature:      u32,
  pub FileHeader:     FileHeader,
  pub OptionalHeader: OptionalHeader,
}


#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct FileHeader {
  pub Machine:              u16,
  pub NumberOfSections:     u16,
  pub TimeDateStamp:        u32,
  pub PointerToSymbolTable: u32,
  pub NumberOfSymbols:      u32,
  pub SizeOfOptionalHeader: u16,
  pub Characteristics:      u16,
}


const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;


#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct DataDirectory {
  pub VirtualAddress: u32,
  pub Size:           u32,
}


#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct OptionalHeader32 {
  pub Magic:                       u16,
  pub MajorLinkerVersion:          u8,
  pub MinorLinkerVersion:          u8,
  pub SizeOfCode:                  u32,
  pub SizeOfInitializedData:       u32,
  pub SizeOfUninitializedData:     u32,
  pub AddressOfEntryPoint:         u32,
  pub BaseOfCode:                  u32,
  pub BaseOfData:                  u32,
  pub ImageBase:                   u32,
  pub SectionAlignment:            u32,
  pub FileAlignment:               u32,
  pub MajorOperatingSystemVersion: u16,
  pub MinorOperatingSystemVersion: u16,
  pub MajorImageVersion:           u16,
  pub MinorImageVersion:           u16,
  pub MajorSubsystemVersion:       u16,
  pub MinorSubsystemVersion:       u16,
  pub Win32VersionValue:           u32,
  pub SizeOfImage:                 u32,
  pub SizeOfHeaders:               u32,
  pub CheckSum:                    u32,
  pub Subsystem:                   u16,
  pub DllCharacteristics:          u16,
  pub SizeOfStackReserve:          u32,
  pub SizeOfStackCommit:           u32,
  pub SizeOfHeapReserve:           u32,
  pub SizeOfHeapCommit:            u32,
  pub LoaderFlags:                 u32,
  pub NumberOfRvaAndSizes:         u32,
  pub DataDirectory:               [DataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct OptionalHeader64 {
  pub Magic:                       u16,
  pub MajorLinkerVersion:          u8,
  pub MinorLinkerVersion:          u8,
  pub SizeOfCode:                  u32,
  pub SizeOfInitializedData:       u32,
  pub SizeOfUninitializedData:     u32,
  pub AddressOfEntryPoint:         u32,
  pub BaseOfCode:                  u32,
  pub BaseOfData:                  u32,
  pub ImageBase:                   u64,
  pub SectionAlignment:            u32,
  pub FileAlignment:               u32,
  pub MajorOperatingSystemVersion: u16,
  pub MinorOperatingSystemVersion: u16,
  pub MajorImageVersion:           u16,
  pub MinorImageVersion:           u16,
  pub MajorSubsystemVersion:       u16,
  pub MinorSubsystemVersion:       u16,
  pub Win32VersionValue:           u32,
  pub SizeOfImage:                 u32,
  pub SizeOfHeaders:               u32,
  pub CheckSum:                    u32,
  pub Subsystem:                   u16,
  pub DllCharacteristics:          u16,
  pub SizeOfStackReserve:          u64,
  pub SizeOfStackCommit:           u64,
  pub SizeOfHeapReserve:           u64,
  pub SizeOfHeapCommit:            u64,
  pub LoaderFlags:                 u32,
  pub NumberOfRvaAndSizes:         u32,
  pub DataDirectory:               [DataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
}


#[derive(Debug, PartialEq, Eq)]
pub enum OptionalHeader {
  Headers32(OptionalHeader32),
  Headers64(OptionalHeader64),
  Unknown,
}


#[allow(non_snake_case)]
fn file_header(input: &[u8]) -> IResult<&[u8], FileHeader> {
  chain!(input,
    machine:              le_u16 ~
    numberOfSections:     le_u16 ~
    timeDateStamp:        le_u32 ~
    pointerToSymbolTable: le_u32 ~
    numberOfSymbols:      le_u32 ~
    sizeOfOptionalHeader: le_u16 ~
    characteristics:      le_u16 ,

    || {
      FileHeader {
        Machine:              machine,
        NumberOfSections:     numberOfSections,
        TimeDateStamp:        timeDateStamp,
        PointerToSymbolTable: pointerToSymbolTable,
        NumberOfSymbols:      numberOfSymbols,
        SizeOfOptionalHeader: sizeOfOptionalHeader,
        Characteristics:      characteristics,
      }
    }
  )
}


#[allow(non_snake_case)]
fn optional_header_32(input: &[u8]) -> IResult<&[u8], OptionalHeader32> {
  chain!(input,
    magic:                       le_u16 ~
    majorLinkerVersion:          le_u8  ~
    minorLinkerVersion:          le_u8  ~
    sizeOfCode:                  le_u32 ~
    sizeOfInitializedData:       le_u32 ~
    sizeOfUninitializedData:     le_u32 ~
    addressOfEntryPoint:         le_u32 ~
    baseOfCode:                  le_u32 ~
    baseOfData:                  le_u32 ~
    imageBase:                   le_u32 ~
    sectionAlignment:            le_u32 ~
    fileAlignment:               le_u32 ~
    majorOperatingSystemVersion: le_u16 ~
    minorOperatingSystemVersion: le_u16 ~
    majorImageVersion:           le_u16 ~
    minorImageVersion:           le_u16 ~
    majorSubsystemVersion:       le_u16 ~
    minorSubsystemVersion:       le_u16 ~
    win32VersionValue:           le_u32 ~
    sizeOfImage:                 le_u32 ~
    sizeOfHeaders:               le_u32 ~
    checkSum:                    le_u32 ~
    subsystem:                   le_u16 ~
    dllCharacteristics:          le_u16 ~
    sizeOfStackReserve:          le_u32 ~
    sizeOfStackCommit:           le_u32 ~
    sizeOfHeapReserve:           le_u32 ~
    sizeOfHeapCommit:            le_u32 ~
    loaderFlags:                 le_u32 ~
    numberOfRvaAndSizes:         le_u32 ,

    || {
      OptionalHeader32 {
        Magic:                       magic,
        MajorLinkerVersion:          majorLinkerVersion,
        MinorLinkerVersion:          minorLinkerVersion,
        SizeOfCode:                  sizeOfCode,
        SizeOfInitializedData:       sizeOfInitializedData,
        SizeOfUninitializedData:     sizeOfUninitializedData,
        AddressOfEntryPoint:         addressOfEntryPoint,
        BaseOfCode:                  baseOfCode,
        BaseOfData:                  baseOfData,
        ImageBase:                   imageBase,
        SectionAlignment:            sectionAlignment,
        FileAlignment:               fileAlignment,
        MajorOperatingSystemVersion: majorOperatingSystemVersion,
        MinorOperatingSystemVersion: minorOperatingSystemVersion,
        MajorImageVersion:           majorImageVersion,
        MinorImageVersion:           minorImageVersion,
        MajorSubsystemVersion:       majorSubsystemVersion,
        MinorSubsystemVersion:       minorSubsystemVersion,
        Win32VersionValue:           win32VersionValue,
        SizeOfImage:                 sizeOfImage,
        SizeOfHeaders:               sizeOfHeaders,
        CheckSum:                    checkSum,
        Subsystem:                   subsystem,
        DllCharacteristics:          dllCharacteristics,
        SizeOfStackReserve:          sizeOfStackReserve,
        SizeOfStackCommit:           sizeOfStackCommit,
        SizeOfHeapReserve:           sizeOfHeapReserve,
        SizeOfHeapCommit:            sizeOfHeapCommit,
        LoaderFlags:                 loaderFlags,
        NumberOfRvaAndSizes:         numberOfRvaAndSizes,

        // TODO
        DataDirectory:               [DataDirectory{VirtualAddress: 0, Size: 0}; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
      }
    }
  )
}


#[allow(non_snake_case)]
fn optional_header_64(input: &[u8]) -> IResult<&[u8], OptionalHeader64> {
  chain!(input,
    magic:                       le_u16 ~
    majorLinkerVersion:          le_u8  ~
    minorLinkerVersion:          le_u8  ~
    sizeOfCode:                  le_u32 ~
    sizeOfInitializedData:       le_u32 ~
    sizeOfUninitializedData:     le_u32 ~
    addressOfEntryPoint:         le_u32 ~
    baseOfCode:                  le_u32 ~
    baseOfData:                  le_u32 ~
    imageBase:                   le_u64 ~
    sectionAlignment:            le_u32 ~
    fileAlignment:               le_u32 ~
    majorOperatingSystemVersion: le_u16 ~
    minorOperatingSystemVersion: le_u16 ~
    majorImageVersion:           le_u16 ~
    minorImageVersion:           le_u16 ~
    majorSubsystemVersion:       le_u16 ~
    minorSubsystemVersion:       le_u16 ~
    win32VersionValue:           le_u32 ~
    sizeOfImage:                 le_u32 ~
    sizeOfHeaders:               le_u32 ~
    checkSum:                    le_u32 ~
    subsystem:                   le_u16 ~
    dllCharacteristics:          le_u16 ~
    sizeOfStackReserve:          le_u64 ~
    sizeOfStackCommit:           le_u64 ~
    sizeOfHeapReserve:           le_u64 ~
    sizeOfHeapCommit:            le_u64 ~
    loaderFlags:                 le_u32 ~
    numberOfRvaAndSizes:         le_u32 ,

    || {
      OptionalHeader64 {
        Magic:                       magic,
        MajorLinkerVersion:          majorLinkerVersion,
        MinorLinkerVersion:          minorLinkerVersion,
        SizeOfCode:                  sizeOfCode,
        SizeOfInitializedData:       sizeOfInitializedData,
        SizeOfUninitializedData:     sizeOfUninitializedData,
        AddressOfEntryPoint:         addressOfEntryPoint,
        BaseOfCode:                  baseOfCode,
        BaseOfData:                  baseOfData,
        ImageBase:                   imageBase,
        SectionAlignment:            sectionAlignment,
        FileAlignment:               fileAlignment,
        MajorOperatingSystemVersion: majorOperatingSystemVersion,
        MinorOperatingSystemVersion: minorOperatingSystemVersion,
        MajorImageVersion:           majorImageVersion,
        MinorImageVersion:           minorImageVersion,
        MajorSubsystemVersion:       majorSubsystemVersion,
        MinorSubsystemVersion:       minorSubsystemVersion,
        Win32VersionValue:           win32VersionValue,
        SizeOfImage:                 sizeOfImage,
        SizeOfHeaders:               sizeOfHeaders,
        CheckSum:                    checkSum,
        Subsystem:                   subsystem,
        DllCharacteristics:          dllCharacteristics,
        SizeOfStackReserve:          sizeOfStackReserve,
        SizeOfStackCommit:           sizeOfStackCommit,
        SizeOfHeapReserve:           sizeOfHeapReserve,
        SizeOfHeapCommit:            sizeOfHeapCommit,
        LoaderFlags:                 loaderFlags,
        NumberOfRvaAndSizes:         numberOfRvaAndSizes,

        // TODO
        DataDirectory:               [DataDirectory{VirtualAddress: 0, Size: 0}; IMAGE_NUMBEROF_DIRECTORY_ENTRIES],
      }
    }
  )
}


fn nt_headers(input: &[u8]) -> IResult<&[u8], NtHeaders> {
  chain!(input,
    signature:   tag!("PE\0\0")                                           ~
    file_header: file_header                                              ~
    optional_32: cond!(file_header.Machine == 0x014c, call!(optional_header_32)) ~
    optional_64: cond!(file_header.Machine == 0x8664, call!(optional_header_64)) ,

    || {
      // Parse the signature - we could use the hard-coded value, but this
      // doesn't duplicate code :-)
      let sig = {
        let mut cur = Cursor::new(signature);
        cur.read_u32::<LittleEndian>().unwrap()
      };

      // Pick and wrap the correct optional header
      let optional_header = match (optional_32, optional_64) {
        (Some(hdr), None) => OptionalHeader::Headers32(hdr),
        (None, Some(hdr)) => OptionalHeader::Headers64(hdr),
        (None, None)      => OptionalHeader::Unknown,

        (Some(_), Some(_)) => unreachable!(),
      };

      // All set!
      NtHeaders {
        Signature:      sig,
        FileHeader:     file_header,
        OptionalHeader: optional_header,
      }
    }
  )
}


fn check(buf: &[u8]) {
  let dos_header = match dos_header(buf) {
    IResult::Done(_, header) => {
      println!("Parsed DOS header");
      println!("{:?}", header);

      header
    }
    e => {
      println!("Could not parse DOS header: {:?}", e);
      return;
    }
  };

  // Validate the offset into the buffer.
  let offset = {
    if dos_header.e_lfanew < 0 {
      println!("e_lfanew is negative: {}", dos_header.e_lfanew);
      return;
    }

    let val = dos_header.e_lfanew as usize;
    if val > buf.len() {
      println!("Invalid e_lfanew: {}", val);
      return;
    }

    val
  };

  // Read the PE header.
  let nt_headers = match nt_headers(&buf[offset..]) {
    IResult::Done(_, header) => {
      println!("Parsed NT headers");
      println!("{:?}", header);

      header
    }
    e => {
      println!("Could not parse NT headers: {:?}", e);
      return;
    }
  };
}


fn main() {
  let input_path = "test_binaries/x64/CompileFlags-no-GS.exe";

  let file = fs::OpenOptions::new()
    .create(true)
    .read(true)
    .write(true)
    .open(input_path)
    .unwrap();

  let fd = get_fd(&file);

  let chunk = MemoryMap::new(1, &[
    MapOption::MapReadable,
    MapOption::MapWritable,
    MapOption::MapFd(fd),
  ]).unwrap();

  let file_data: &[u8] = unsafe { std::slice::from_raw_parts(chunk.data() as *const _, chunk.len()) };

  println!("Mapped the file");
  check(file_data);
}
