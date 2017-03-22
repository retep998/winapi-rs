#Guidelines

* Never get definitions from MinGW headers or MSDN. Always stick to the Windows SDK headers, in particular the latest Windows 10 SDK.

##Line length

* The maximum line length for `winapi-rs` is 99, and is strictly enforced.
* Avoid line breaks when possible, but if you cannot make it fit, add line breaks as late as possible.
* When breaking on binary operators, put the operator at the beginning of the new line.

##Functions

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

##Constants

* Convert macro constants to Rust constants.
* The type of the constant should depend on where the constant is used. MSDN may help for this.

```C
#define CLSCTX_INPROC           (CLSCTX_INPROC_SERVER|CLSCTX_INPROC_HANDLER)
```
```Rust
pub const CLSCTX_INPROC: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER;
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


##Organization of code

* All definitions go into the source file that directly maps to the header the definition is from.
** Stuff in `src/winrt` is special and has its own namespaced organization.
* Definitions are defined in the same order as they are in the original header.
* The `lib` folder is legacy from 0.2 and will eventually disappear once all definitions have been moved to their correct locations.
