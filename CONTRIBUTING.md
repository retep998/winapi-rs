#Guidelines

##Style

###Line length

* The maximum line length for `winapi-rs` is 99. Any lines longer than this **must** be modified before they will be accepted.
* Avoid line breaks when possible, but if you cannot make it fit, add line breaks as late as possible (for example, between 80 and 95).

###Multiline functions

* Multiline functions and structs **should** have commas after every field and parameter.
* When implementing multiline functions, please use this style:  
```rust
    pub fn QueryInformationJobObject(
        hJob: HANDLE, JobObjectInformationClass: JOBOBJECTINFOCLASS,
        lpJobObjectInformation: LPVOID, cbJobObjectInformationLength: DWORD,
        lpReturnLength: LPDWORD,
    ) -> BOOL;
```

##Organization of code

* Functions go into the respective `-sys` crate inside `lib`. All types and constants go into the
  main crate.
* If you have types, enums, or constants defined in a .h file not represented in `winapi-rs`, please create a new file and reference it in lib.rs.
