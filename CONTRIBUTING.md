# Guidelines

* Never get definitions from MinGW headers or MSDN. Always stick to the Windows SDK headers, in particular the latest Windows 10 SDK.
* Definitions which depend on whether `UNICODE` is defined should not be included. It is the user's responsibility to explicitly choose between `W` and `A` functions (and they should always choose `W`).

## Newlines and indentation

* The maximum line length for `winapi-rs` is 99, and is strictly enforced.
* Avoid line breaks when possible, but if you cannot make it fit, add line breaks as late as possible.
* When breaking on binary operators, put the operator at the beginning of the new line.
* Do not use aligned indentation. Indentation should always be block indentation.
* Always use spaces for indentation.

## Imports

* Imports should be in asciibetical order.

```Rust
use shared::basetsd::UINT64;
use shared::minwindef::{BOOL, BYTE, INT, LPVOID, UINT};
use um::d3dcommon::{
    D3D_CBUFFER_TYPE, D3D_FEATURE_LEVEL, D3D_INTERPOLATION_MODE, D3D_MIN_PRECISION, D3D_NAME,
    D3D_PARAMETER_FLAGS, D3D_PRIMITIVE, D3D_PRIMITIVE_TOPOLOGY, D3D_REGISTER_COMPONENT_TYPE,
    D3D_RESOURCE_RETURN_TYPE, D3D_SHADER_INPUT_TYPE, D3D_SHADER_VARIABLE_CLASS,
    D3D_SHADER_VARIABLE_TYPE, D3D_SRV_DIMENSION, D3D_TESSELLATOR_DOMAIN,
    D3D_TESSELLATOR_OUTPUT_PRIMITIVE, D3D_TESSELLATOR_PARTITIONING, ID3DBlob,
};
```

## Extern functions

* The calling convention specified should be the one for 32bit. Specify `system` for stdcall and `C` for cdecl (and `fastcall` for those couple of fastcall functions out there).

```Rust
extern "system" {
    pub fn GetProcessTimes(
        hProcess: HANDLE,
        lpCreationTime: LPFILETIME,
        lpExitTime: LPFILETIME,
        lpKernelTime: LPFILETIME,
        lpUserTime: LPFILETIME,
    ) -> BOOL;
    pub fn GetCurrentProcess() -> HANDLE;
}
```

## Function pointers

```Rust
FN!{stdcall DRAWSTATEPROC(
    hdc: HDC,
    lData: LPARAM,
    wData: WPARAM,
    cx: c_int,
    cy: c_int
) -> BOOL}
FN!{stdcall NAMEENUMPROCA(
    LPSTR,
    LPARAM
) -> BOOL}
```

## Constants

* Convert macro constants to Rust constants.
* The type of the constant should depend on where the constant is used. MSDN may help for this.

```C
#define CLSCTX_INPROC           (CLSCTX_INPROC_SERVER|CLSCTX_INPROC_HANDLER)
```
```Rust
pub const CLSCTX_INPROC: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER;
```

## GUIDs

```Rust
DEFINE_GUID!{GUID_DEVCLASS_SENSOR,
    0x5175d334, 0xc371, 0x4806, 0xb3, 0xba, 0x71, 0xfd, 0x53, 0xc9, 0x25, 0x8d}
```

## Structs

* One field per line.

```C
typedef struct _GROUP_AFFINITY {
    KAFFINITY Mask;
    WORD   Group;
    WORD   Reserved[3];
} GROUP_AFFINITY, *PGROUP_AFFINITY;
```
```Rust
STRUCT!{struct GROUP_AFFINITY {
    Mask: KAFFINITY,
    Group: WORD,
    Reserved: [WORD; 3],
}}
pub type PGROUP_AFFINITY = *mut GROUP_AFFINITY;
```

## COM interfaces

```Rust
RIDL!{#[uuid(0x6d4865fe, 0x0ab8, 0x4d91, 0x8f, 0x62, 0x5d, 0xd6, 0xbe, 0x34, 0xa3, 0xe0)]
interface IDWriteFontFileStream(IDWriteFontFileStreamVtbl): IUnknown(IUnknownVtbl) {
    fn ReadFileFragment(
        fragmentStart: *mut *const c_void,
        fileOffset: UINT64,
        fragmentSize: UINT64,
        fragmentContext: *mut *mut c_void
    ) -> HRESULT,
    fn ReleaseFileFragment(
        fragmentContext: *mut c_void
    ) -> (),
    fn GetFileSize(
        fileSize: *mut UINT64
    ) -> HRESULT,
    fn GetLastWriteTime(
        lastWriteTime: *mut UINT64
    ) -> HRESULT
}}
```

## Organization of code

* All definitions go into the source file that directly maps to the header the definition is from.
    * Stuff in `src/winrt` is special and has its own namespaced organization.
* Definitions are defined in the same order as they are in the original header.
* The `lib` folder is legacy from 0.2 and will eventually disappear once all definitions have been moved to their correct locations.

## Dealing with duplicates

* Sometimes two headers will define the same thing.
    * If the duplicated thing is a simple typedef or function definition or constant, then duplicate the definition.
    * If the duplicated thing is a struct or COM interface or union, then choose one header to be the canonical source of truth for that definition and publicly re-export the thing from the other header.
* Sometimes a COM interface will have two methods with identical names. If the two methods are both named `Foo`, then name them `Foo1` and `Foo2`.
