// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Delta Compression Engine API.
use ctypes::__int64;
use shared::basetsd::SIZE_T;
use shared::minwindef::{BOOL, DWORD, FILETIME, LPCVOID, LPVOID, UCHAR};
use um::wincrypt::ALG_ID;
use um::winnt::{LPCSTR, LPCWSTR};
/// Default size limit for source and target files and buffers.
const DELTA_FILE_SIZE_LIMIT: SIZE_T = 32 * 1024 * 1024;
/// Default size limit for options files and buffers.
const DELTA_OPTIONS_SIZE_LIMIT: SIZE_T = 128 * 1024 * 1024;
UNION!{union DELTA_INPUT_U {
    [usize; 1],
    /// Start of memory block, if it is not Editable.
    lpcStart lpcStart_mut: LPCVOID,
    /// Start of memory block, if it is Editable.
    lpStart lpStart_mut: LPVOID,
}}
/// Type for input memory blocks.
STRUCT!{struct DELTA_INPUT {
    u: DELTA_INPUT_U,
    /// Size of memory block in bytes.
    uSize: SIZE_T,
    /// TRUE if caller allows msdelta to edit this memory block, FALSE otherwise.
    Editable: bool,
}}
pub type LPDELTA_INPUT = *mut DELTA_INPUT;
pub type LPCDELTA_INPUT = *const DELTA_INPUT;
/// Type for output memory blocks.
STRUCT!{struct DELTA_OUTPUT {
    /// Start of memory block.
    lpStart: LPVOID,
    /// Size of memory block in bytes.
    uSize: SIZE_T,
}}
pub type LPDELTA_OUTPUT = *mut DELTA_OUTPUT;
pub type LPCDELTA_OUTPUT = *const DELTA_OUTPUT;
pub type DELTA_FILE_TYPE = __int64;
/// Raw file type.
const DELTA_FILE_TYPE_RAW: DELTA_FILE_TYPE = 0x00000001;
/// File type for I386 Portable Executable files.
const DELTA_FILE_TYPE_I386: DELTA_FILE_TYPE = 0x00000002;
/// File type for for IA64 Portable Executable files.
const DELTA_FILE_TYPE_IA64: DELTA_FILE_TYPE = 0x00000004;
/// File type for AMD64 Portable Executable files.
const DELTA_FILE_TYPE_AMD64: DELTA_FILE_TYPE = 0x00000008;
/// File type for I386 Portable Executable files with CLI4 transform.
const DELTA_FILE_TYPE_CLI4_I386: DELTA_FILE_TYPE = 0x00000010;
/// File type for AMD64 Portable Executable files with CLI4 transform.
const DELTA_FILE_TYPE_CLI4_AMD64: DELTA_FILE_TYPE = 0x00000020;
/// File type for ARM Portable Executable files with CLI4 transform.
const DELTA_FILE_TYPE_CLI4_ARM: DELTA_FILE_TYPE = 0x00000040;
/// File type for ARM64 Portable Executable files with CLI4 transform.
const DELTA_FILE_TYPE_CLI4_ARM64: DELTA_FILE_TYPE = 0x00000080;
/// File type set that treats any file as raw.
const DELTA_FILE_TYPE_SET_RAW_ONLY: DELTA_FILE_TYPE = DELTA_FILE_TYPE_RAW;
/// File type set that distinguishes I386, IA64 and AMD64 Portable Executable file and treats others as raw.
const DELTA_FILE_TYPE_SET_EXECUTABLES_1: DELTA_FILE_TYPE = DELTA_FILE_TYPE_RAW
                                                         | DELTA_FILE_TYPE_I386
                                                         | DELTA_FILE_TYPE_IA64
                                                         | DELTA_FILE_TYPE_AMD64;
const DELTA_FILE_TYPE_SET_EXECUTABLES: DELTA_FILE_TYPE = DELTA_FILE_TYPE_SET_EXECUTABLES_1;
/// File type set that distinguishes I386, IA64, ARM, and AMD64 Portable Executable file and treats others as raw.
const DELTA_FILE_TYPE_SET_EXECUTABLES_2: DELTA_FILE_TYPE = DELTA_FILE_TYPE_RAW
                                                         | DELTA_FILE_TYPE_CLI4_I386
                                                         | DELTA_FILE_TYPE_IA64
                                                         | DELTA_FILE_TYPE_CLI4_AMD64
                                                         | DELTA_FILE_TYPE_CLI4_ARM;
/// File type set that distinguishes I386, IA64, ARM, ARM64, and AMD64 Portable Executable file and treats others as raw.
const DELTA_FILE_TYPE_SET_EXECUTABLES_3: DELTA_FILE_TYPE = DELTA_FILE_TYPE_RAW |
                                                         | DELTA_FILE_TYPE_CLI4_I386
                                                         | DELTA_FILE_TYPE_IA64
                                                         | DELTA_FILE_TYPE_CLI4_AMD64
                                                         | DELTA_FILE_TYPE_CLI4_ARM
                                                         | DELTA_FILE_TYPE_CLI4_ARM64;
const DELTA_FILE_TYPE_SET_EXECUTABLES_LATEST : DELTA_FILE_TYPE = DELTA_FILE_TYPE_SET_EXECUTABLES_3;
/// Type for msdelta flags.
pub type DELTA_FLAG_TYPE = __int64;
/// No flags.
const DELTA_FLAG_NONE: DELTA_FLAG_TYPE = 0x00000000;
/// Allow application of legacy PA19 deltas by mspatcha.dll.
const DELTA_APPLY_FLAG_ALLOW_PA19: DELTA_FLAG_TYPE = 0x00000001;
/// Transform E8 pieces (relative calls in x86) of target file .
const DELTA_FLAG_E8: DELTA_FLAG_TYPE = 0x00000001;
/// Mark non-executable parts of source PE.
const DELTA_FLAG_MARK: DELTA_FLAG_TYPE = 0x00000002;
/// Transform imports of source PE.
const DELTA_FLAG_IMPORTS: DELTA_FLAG_TYPE = 0x00000004;
/// Transform exports of source PE.
const DELTA_FLAG_EXPORTS: DELTA_FLAG_TYPE = 0x00000008;
/// Transform resources of source PE.
const DELTA_FLAG_RESOURCES: DELTA_FLAG_TYPE = 0x00000010;
/// Transform relocations of source PE.
const DELTA_FLAG_RELOCS: DELTA_FLAG_TYPE = 0x00000020;
/// Smash lock prefixes of source PE.
const DELTA_FLAG_I386_SMASHLOCK: DELTA_FLAG_TYPE = 0x00000040;
/// Transform relative jumps of source I386 (x86) PE.
const DELTA_FLAG_I386_JMPS: DELTA_FLAG_TYPE = 0x00000080;
/// Transform relative calls of source I386 (x86) PE.
const DELTA_FLAG_I386_CALLS: DELTA_FLAG_TYPE = 0x00000100;
/// Transform instructions of source AMD64 (x86-64) PE.
const DELTA_FLAG_AMD64_DISASM: DELTA_FLAG_TYPE = 0x00000200;
/// Transform pdata of source AMD64 (x86-64) PE.
const DELTA_FLAG_AMD64_PDATA: DELTA_FLAG_TYPE = 0x00000400;
/// Transform intstructions of source IA64 (Itanium) PE.
const DELTA_FLAG_IA64_DISASM: DELTA_FLAG_TYPE = 0x00000800;
/// Transform pdata of source IA64 (Itanium) PE.
const DELTA_FLAG_IA64_PDATA: DELTA_FLAG_TYPE = 0x00001000;
/// Unbind source PE.
const DELTA_FLAG_UNBIND: DELTA_FLAG_TYPE = 0x00002000;
/// Transform CLI instructions of source PE.
const DELTA_FLAG_CLI_DISASM: DELTA_FLAG_TYPE = 0x00004000;
/// Transform CLI Metadata of source PE.
const DELTA_FLAG_CLI_METADATA: DELTA_FLAG_TYPE = 0x00008000;
/// Transform headers of source PE.
const DELTA_FLAG_HEADERS: DELTA_FLAG_TYPE = 0x00010000;
/// Allow source or target file or buffer to exceed its default size limit.
const DELTA_FLAG_IGNORE_FILE_SIZE_LIMIT: DELTA_FLAG_TYPE = 0x00020000;
/// Allow options buffer or file to exceed its default size limit.
const DELTA_FLAG_IGNORE_OPTIONS_SIZE_LIMIT: DELTA_FLAG_TYPE = 0x00040000;
/// Transform instructions of source ARM PE.
const DELTA_FLAG_ARM_DISASM: DELTA_FLAG_TYPE = 0x00080000;
/// Transform pdata of source ARM PE.
const DELTA_FLAG_ARM_PDATA: DELTA_FLAG_TYPE = 0x00100000;
/// Transform CLI4 Metadata of source PE.
const DELTA_FLAG_CLI4_METADATA: DELTA_FLAG_TYPE = 0x00200000;
/// Transform CLI4 instructions of source PE.
const DELTA_FLAG_CLI4_DISASM: DELTA_FLAG_TYPE = 0x00400000;
/// Transform instructions of source ARM PE.
const DELTA_FLAG_ARM64_DISASM: DELTA_FLAG_TYPE = 0x00800000;
/// Transform pdata of source ARM PE.
const DELTA_FLAG_ARM64_PDATA: DELTA_FLAG_TYPE = 0x01000000;
/// List the default transform combination for individual ISA
const DELTA_DEFAULT_FLAGS_RAW: DELTA_FLAG_TYPE = DELTA_FLAG_NONE;
const DELTA_DEFAULT_FLAGS_I386: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                | DELTA_FLAG_IMPORTS
                                                | DELTA_FLAG_EXPORTS
                                                | DELTA_FLAG_RESOURCES
                                                | DELTA_FLAG_RELOCS
                                                | DELTA_FLAG_I386_SMASHLOCK
                                                | DELTA_FLAG_I386_JMPS
                                                | DELTA_FLAG_I386_CALLS
                                                | DELTA_FLAG_UNBIND
                                                | DELTA_FLAG_CLI_DISASM
                                                | DELTA_FLAG_CLI_METADATA;
const DELTA_DEFAULT_FLAGS_IA64: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                | DELTA_FLAG_IMPORTS
                                                | DELTA_FLAG_EXPORTS
                                                | DELTA_FLAG_RESOURCES
                                                | DELTA_FLAG_RELOCS
                                                | DELTA_FLAG_IA64_DISASM
                                                | DELTA_FLAG_IA64_PDATA
                                                | DELTA_FLAG_UNBIND
                                                | DELTA_FLAG_CLI_DISASM
                                                | DELTA_FLAG_CLI_METADATA;
const DELTA_DEFAULT_FLAGS_AMD64: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                                 | DELTA_FLAG_IMPORTS
                                                 | DELTA_FLAG_EXPORTS
                                                 | DELTA_FLAG_RESOURCES
                                                 | DELTA_FLAG_RELOCS
                                                 | DELTA_FLAG_AMD64_DISASM
                                                 | DELTA_FLAG_AMD64_PDATA
                                                 | DELTA_FLAG_UNBIND
                                                 | DELTA_FLAG_CLI_DISASM
                                                 | DELTA_FLAG_CLI_METADATA;
const DELTA_CLI4_FLAGS_I386: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                             | DELTA_FLAG_IMPORTS
                                             | DELTA_FLAG_EXPORTS
                                             | DELTA_FLAG_RESOURCES
                                             | DELTA_FLAG_RELOCS
                                             | DELTA_FLAG_I386_SMASHLOCK
                                             | DELTA_FLAG_I386_JMPS
                                             | DELTA_FLAG_I386_CALLS
                                             | DELTA_FLAG_UNBIND
                                             | DELTA_FLAG_CLI4_DISASM
                                             | DELTA_FLAG_CLI4_METADATA;
const DELTA_CLI4_FLAGS_AMD64: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                              | DELTA_FLAG_IMPORTS
                                              | DELTA_FLAG_EXPORTS
                                              | DELTA_FLAG_RESOURCES
                                              | DELTA_FLAG_RELOCS
                                              | DELTA_FLAG_AMD64_DISASM
                                              | DELTA_FLAG_AMD64_PDATA
                                              | DELTA_FLAG_UNBIND
                                              | DELTA_FLAG_CLI4_DISASM
                                              | DELTA_FLAG_CLI4_METADATA;
const DELTA_CLI4_FLAGS_ARM: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                            | DELTA_FLAG_IMPORTS
                                            | DELTA_FLAG_EXPORTS
                                            | DELTA_FLAG_RESOURCES
                                            | DELTA_FLAG_RELOCS
                                            | DELTA_FLAG_ARM_DISASM
                                            | DELTA_FLAG_ARM_PDATA
                                            | DELTA_FLAG_UNBIND
                                            | DELTA_FLAG_CLI4_DISASM
                                            | DELTA_FLAG_CLI4_METADATA;
const DELTA_CLI4_FLAGS_ARM64: DELTA_FLAG_TYPE = DELTA_FLAG_MARK
                                              | DELTA_FLAG_IMPORTS
                                              | DELTA_FLAG_EXPORTS
                                              | DELTA_FLAG_RESOURCES
                                              | DELTA_FLAG_RELOCS
                                              | DELTA_FLAG_ARM64_DISASM
                                              | DELTA_FLAG_ARM64_PDATA
                                              | DELTA_FLAG_UNBIND
                                              | DELTA_FLAG_CLI4_DISASM
                                              | DELTA_FLAG_CLI4_METADATA;
/// Maximal allowed size of hash in bytes.
pub const DELTA_MAX_HASH_SIZE: usize = 32;
/// Hash structure.
STRUCT!{struct DELTA_HASH {
    /// Size of hash in bytes. Does not exceed DELTA_MAX_HASH_SIZE.
    HashSize: DWORD,
    /// Hash value.
    HashValue: [UCHAR; DELTA_MAX_HASH_SIZE],
}}
pub type LPDELTA_HASH = *mut DELTA_HASH;
pub type LPCDELTA_HASH = *const DELTA_HASH;
/// Delta header information.
STRUCT!{struct DELTA_HEADER_INFO {
    /// Used file type set.
    FileTypeSet: DELTA_FILE_TYPE,
    /// Source file type.
    FileType: DELTA_FILE_TYPE,
    /// Delta flags.
    Flags: DELTA_FLAG_TYPE,
    /// Size of target file in bytes.
    TargetSize: SIZE_T,
    /// Time of target file.
    TargetFileTime: FILETIME,
    /// Algorithm used for hashing.
    TargetHashAlgId: ALG_ID,
    /// Target hash.
    TargetHash: DELTA_HASH,
}}
pub type LPDELTA_HEADER_INFO = *mut DELTA_HEADER_INFO;
pub type LPCDELTA_HEADER_INFO = *const DELTA_HEADER_INFO;
extern "system" {
    /// Applies a given delta to a given source file.
    /// The resultant target file is written to disk.
    /// All files accessed by ASCII file names.
    pub fn ApplyDeltaA(
        ApplyFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCSTR,
        lpDeltaName: LPCSTR,
        lpTargetName: LPCSTR,
    ) -> BOOL;
    /// Applies a given delta to a given source file.
    /// The resultant target file is put into allocated memory.
    pub fn ApplyDeltaB(
        ApplyFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Delta: DELTA_INPUT,
        lpTarget: LPDELTA_OUTPUT,
    ) -> BOOL;
    /// Applies a given delta to a given source file.
    /// The resultant target file is put into caller-provided memory.
    pub fn ApplyDeltaProvidedB(
        ApplyFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Delta: DELTA_INPUT,
        lpTarget: LPVOID,
        uTargetSize: SIZE_T,
    ) -> BOOL;
    /// Applies a given delta to a given source file.
    /// The resultant target file is written to disk.
    /// All files accessed by Unicode file names.
    pub fn ApplyDeltaW(
        ApplyFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCWSTR,
        lpDeltaName: LPCWSTR,
        lpTargetName: LPCWSTR,
    ) -> BOOL;
    /// Creates a delta from a given source file to a given target file.
    /// The resultant delta is written to disk.
    /// All files accessed by ASCII file names.
    pub fn CreateDeltaA(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCSTR,
        lpTargetName: LPCSTR,
        lpSourceOptionsName: LPCSTR,
        lpTargetOptionsName: LPCSTR,
        GlobalOptions: DELTA_INPUT,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDeltaName: LPCSTR,
    ) -> BOOL;
    /// Creates a delta from a given source file to a given target file in memory.
    /// The resultant delta is put into allocated memory.
    pub fn CreateDeltaB(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Target: DELTA_INPUT,
        SourceOptions: DELTA_INPUT,
        TargetOptions: DELTA_INPUT,
        GlobalOptions: DELTA_INPUT ,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDelta: LPDELTA_OUTPUT,
    ) -> BOOL;
    /// Creates a delta from a given source file to a given target file.
    /// The resultant delta is written to disk
    /// All files accessed by Unicode file names.
    pub fn CreateDeltaW(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCWSTR,
        lpTargetName: LPCWSTR,
        lpSourceOptionsName: LPCWSTR,
        lpTargetOptionsName: LPCWSTR,
        GlobalOptions: DELTA_INPUT,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDeltaName: LPCWSTR,
    ) -> BOOL;
    /// Frees memory block allocated by msdelta.
    pub fn DeltaFree(
        lpMemory: LPVOID,
    ) -> BOOL;
    /// Normalizes source buffer, normalization is  based on source file type, which is
    /// determined automatically according to the given file type set.
    pub fn DeltaNormalizeProvidedB(
        FileTypeSet: DELTA_FILE_TYPE,
        NormalizeFlags: DELTA_FLAG_TYPE,
        NormalizeOptions: DELTA_INPUT,
        lpSource: LPVOID,
        uSourceSize: SIZE_T,
    ) -> BOOL;
    /// Gets header information for a delta accessed by ASCII file name.
    pub fn GetDeltaInfoA(
        Delta: DELTA_INPUT,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    /// Gets header information for a delta in memory.
    pub fn GetDeltaInfoB(
        Delta: DELTA_INPUT,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    /// Gets header information for a delta accessed by Unicode file name.
    pub fn GetDeltaInfoW(
        lpDeltaName: LPCWSTR,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    /// Calculates a hash for normalized source file accessed by ASCII file name.
    /// Normalization is based on source file type,
    /// which is determined automatically according to the given file type set.
    pub fn GetDeltaSignatureA(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        lpSourceName: LPCSTR,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
    /// Calculates a hash for normalized source file in memory.
    /// Normalization is based on source file type,
    /// which is determined automatically according to the given file type set.
    pub fn GetDeltaSignatureB(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        Source: DELTA_INPUT,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
    /// Calculates a hash for normalized source file accessed by Unicode file name.
    /// Normalization is based on source file type,
    /// which is determined automatically according to the given file type set.
    pub fn GetDeltaSignatureW(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        lpSourceName: LPCWSTR,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
}
