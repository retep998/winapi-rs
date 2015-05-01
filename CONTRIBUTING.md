## Guidelines ##
* Column limit is a strict 99. Avoid newlines it is possible to fit within 99, and when it isn't
  add newlines as late as possible.
* Multiline functions and structs should have commas after every field and parameter.
* When doing multiline functions, use this style:
```rust
    pub fn QueryInformationJobObject(
        hJob: HANDLE, JobObjectInformationClass: JOBOBJECTINFOCLASS,
        lpJobObjectInformation: LPVOID, cbJobObjectInformationLength: DWORD,
        lpReturnLength: LPDWORD,
    ) -> BOOL;
```
* Functions go into the respective `-sys` crate inside `lib`. All types and constants go into the
  main crate.
