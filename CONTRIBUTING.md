#Guidelines

* Never get definitions from MinGW headers or MSDN. Always stick to the Windows SDK headers, in particular the latest Windows 10 SDK.
* Definitions which depend on whether `UNICODE` is defined should not be included. It is the user's responsibility to explicitly choose between `W` and `A` functions (and they should always choose `W`).

##Line length

* The maximum line length for `winapi-rs` is 99, and is strictly enforced.
* Avoid line breaks when possible, but if you cannot make it fit, add line breaks as late as possible.
* When breaking on binary operators, put the operator at the beginning of the new line.

##Extern functions

* First parameter is the 32-bit calling convention.
* One parameter per line.

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

##Function pointers

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

##Constants

* Convert macro constants to Rust constants.
* The type of the constant should depend on where the constant is used. MSDN may help for this.

```C
#define CLSCTX_INPROC           (CLSCTX_INPROC_SERVER|CLSCTX_INPROC_HANDLER)
```
```Rust
pub const CLSCTX_INPROC: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER;
```

##GUIDs

```Rust
DEFINE_GUID!{GUID_DEVCLASS_SENSOR,
    0x5175d334, 0xc371, 0x4806, 0xb3, 0xba, 0x71, 0xfd, 0x53, 0xc9, 0x25, 0x8d}
```

##Structs

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

##COM interfaces

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

##Organization of code

* All definitions go into the source file that directly maps to the header the definition is from.
** Stuff in `src/winrt` is special and has its own namespaced organization.
* Definitions are defined in the same order as they are in the original header.
* The `lib` folder is legacy from 0.2 and will eventually disappear once all definitions have been moved to their correct locations.
