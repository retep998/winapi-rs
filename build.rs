// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::env::var;
// (header name, &[header dependencies], &[library dependencies])
const DATA: &'static [(&'static str, &'static [&'static str], &'static [&'static str])] = &[
    // km
    // mmos
    // shared
    ("basetsd", &[], &[]),
    ("bcrypt", &["minwindef", "ntdef"], &[]),
    ("bugcodes", &["ntdef"], &[]),
    ("cderr", &["minwindef"], &[]),
    ("cfg", &["minwindef"], &[]),
    ("d3d9", &["basetsd", "d3d9caps", "d3d9types", "guiddef", "minwindef", "unknwnbase", "windef", "wingdi", "winnt"], &[]),
    ("d3d9caps", &["d3d9types", "guiddef", "minwindef", "winnt"], &[]),
    ("d3d9types", &["basetsd", "guiddef", "minwindef", "windef", "winnt"], &[]),
    ("devguid", &[], &[]),
    ("devpropdef", &["guiddef", "minwindef", "winnt"], &[]),
    ("dxgi", &["basetsd", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("dxgi1_2", &["basetsd", "dxgi", "dxgiformat", "dxgitype", "guiddef", "minwinbase", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("dxgi1_3", &["dxgi", "dxgi1_2", "dxgiformat", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("dxgi1_4", &["basetsd", "dxgi1_2", "dxgi1_3", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("dxgiformat", &[], &[]),
    ("dxgitype", &["d3d9types", "dxgiformat", "minwindef"], &[]),
    ("guiddef", &[], &[]),
    ("hidclass", &["guiddef", "minwindef", "winioctl", "winnt"], &[]),
    ("hidpi", &["hidusage", "minwindef", "ntdef", "ntstatus", "winnt"], &[]),
    ("hidsdi", &["minwindef", "winnt"], &[]),
    ("hidusage", &["minwindef"], &[]),
    ("inaddr", &["minwindef"], &[]),
    ("ksmedia", &[], &[]),
    ("lmcons", &["minwindef", "winnt"], &[]),
    ("minwindef", &["basetsd", "ntdef"], &[]),
    ("mmreg", &["guiddef", "minwindef"], &[]),
    ("ntdef", &["basetsd"], &[]),
    ("ntstatus", &["ntdef"], &[]),
    ("rpcndr", &[], &[]),
    ("sspi", &["ntdef"], &[]),
    ("usb", &["minwindef", "winnt"], &[]),
    ("windef", &["minwindef", "winnt"], &[]),
    ("windowsx", &["minwindef"], &[]),
    ("winerror", &["minwindef"], &[]),
    ("wtypes", &["minwindef", "winnt", "wtypesbase"], &[]),
    ("wtypesbase", &["minwindef", "rpcndr", "winnt"], &[]),
    // ucrt
    // um
    ("gl-gl", &[], &[]),
    ("audioclient", &["audiosessiontypes", "basetsd", "guiddef", "minwindef", "mmreg", "strmif", "unknwnbase", "winerror", "winnt", "wtypesbase"], &[]),
    ("audiosessiontypes", &["minwindef"], &[]),
    ("avrt", &["guiddef", "minwindef", "winnt"], &["avrt"]),
    ("cfgmgr32", &["basetsd", "guiddef", "minwindef", "winnt"], &[]),
    ("cguid", &[], &[]),
    ("combaseapi", &["basetsd", "minwindef", "objidlbase", "winnt", "wtypesbase"], &["ole32"]),
    ("commctrl", &["basetsd", "guiddef", "minwinbase", "minwindef", "objidlbase", "vcruntime", "windef", "winnt", "winuser"], &[]),
    ("commdlg", &["basetsd", "minwindef", "prsht", "unknwnbase", "windef", "wingdi", "winnt", "winuser"], &[]),
    ("consoleapi", &["minwindef", "wincon", "winnt"], &["kernel32"]),
    ("corsym", &["basetsd", "objidlbase", "unknwnbase", "winnt"], &[]),
    ("d2d1", &["basetsd", "d2dbasetypes", "d3dcommon", "dcommon", "dwrite", "dxgi", "guiddef", "minwindef", "unknwnbase", "wincodec", "windef", "winnt"], &[]),
    ("d2dbasetypes", &["basetsd", "d3d9types", "minwindef", "windef"], &[]),
    ("d3d10", &["d3dcommon"], &[]),
    ("d3d10shader", &["d3d10", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d11", &["basetsd", "d3dcommon", "dxgiformat", "dxgitype", "guiddef", "minwindef", "unknwnbase", "windef", "winnt"], &[]),
    ("d3d11shader", &["basetsd", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d12", &["basetsd", "d3dcommon", "dxgiformat", "dxgitype", "guiddef", "minwinbase", "minwindef", "ntdef", "unknwnbase", "windef", "winnt"], &[]),
    ("d3d12sdklayers", &["basetsd", "d3d12", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3d12shader", &["basetsd", "d3dcommon", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3dcommon", &["basetsd", "minwindef", "unknwnbase", "winnt"], &[]),
    ("d3dcompiler", &["basetsd", "d3dcommon", "minwindef"], &[]),
    ("dbghelp", &["basetsd", "guiddef", "minwindef", "vcruntime", "winnt"], &[]),
    ("dcommon", &["dxgiformat"], &[]),
    ("docobj", &["guiddef", "minwindef", "oaidl", "unknwnbase", "winnt"], &[]),
    ("dpapi", &["minwindef", "windef", "winnt"], &[]),
    ("dsgetdc", &["guiddef", "minwindef", "winnt"], &[]),
    ("dsound", &["guiddef", "minwindef", "mmsystem", "unknwnbase", "windef", "winerror", "winnt"], &[]),
    ("dsrole", &["guiddef", "minwindef", "winnt"], &[]),
    ("dwmapi", &["minwindef"], &[]),
    ("dwrite", &["basetsd", "d2d1", "dcommon", "minwindef", "unknwnbase", "windef", "winerror", "wingdi", "winnt"], &[]),
    ("errhandlingapi", &["winnt"], &[]),
    ("fileapi", &["minwinbase", "winnt"], &[]),
    ("handleapi", &["minwindef", "winnt"], &["kernel32"]),
    ("heapapi", &["basetsd", "minwindef"], &[]),
    ("http", &["guiddef", "minwindef", "sspi", "winnt", "winsock2"], &[]),
    ("imm", &[], &[]),
    ("libloaderapi", &["basetsd", "minwindef", "winnt"], &[]),
    ("lmaccess", &["lmcons", "minwindef", "winnt"], &[]),
    ("lmdfs", &["guiddef", "minwindef", "winnt"], &[]),
    ("lmerrlog", &["minwindef", "winnt"], &[]),
    ("lmjoin", &["minwindef", "winnt"], &[]),
    ("lsalookup", &["guiddef", "minwindef", "winnt"], &[]),
    ("memoryapi", &["basetsd", "minwindef", "winnt"], &[]),
    ("minschannel", &["guiddef", "minwindef", "wincrypt", "winnt"], &[]),
    ("minwinbase", &["basetsd", "minwindef", "ntstatus", "winnt"], &[]),
    ("mmdeviceapi", &["guiddef", "minwindef", "propidl", "propsys", "unknwnbase", "winnt"], &[]),
    ("mmsystem", &["basetsd", "minwindef", "mmreg", "winnt"], &[]),
    ("mscat", &["guiddef", "minwindef", "mssip", "wincrypt", "winnt"], &[]),
    ("mssip", &["guiddef", "minwindef", "mscat", "wincrypt", "winnt"], &[]),
    ("ncrypt", &["basetsd", "sspi"], &["ncrypt"]),
    ("oaidl", &["basetsd", "guiddef", "minwindef", "unknwnbase", "winnt", "wtypes", "wtypesbase"], &[]),
    ("objidlbase", &["basetsd", "guiddef", "minwindef", "unknwnbase", "winnt", "wtypesbase"], &[]),
    ("pdh", &["minwindef", "winnt", "wtypesbase"], &[]),
    ("processthreadsapi", &["basetsd", "minwindef", "winnt"], &["kernel32"]),
    ("propidl", &["minwindef", "wtypes"], &[]),
    ("propsys", &["unknwnbase"], &[]),
    ("prsht", &["basetsd", "minwindef", "windef", "winnt", "winuser"], &[]),
    ("setupapi", &["basetsd", "commctrl", "devpropdef", "guiddef", "minwindef", "prsht", "spapidef", "windef", "winnt", "winreg"], &["setupapi"]),
    ("shellapi", &["basetsd", "guiddef", "minwinbase", "minwindef", "processthreadsapi", "windef", "winnt", "winuser"], &["shell32", "shlwapi"]),
    ("spapidef", &["minwindef", "winnt"], &[]),
    ("strmif", &["winnt"], &[]),
    ("unknwnbase", &["guiddef", "minwindef", "winnt"], &[]),
    ("vsserror", &["winnt"], &[]),
    ("winbase", &["basetsd", "cfgmgr32", "guiddef", "minwindef", "winnt"], &[]),
    ("wincodec", &["basetsd", "dcommon", "guiddef", "minwindef", "unknwnbase", "winnt"], &[]),
    ("wincon", &["minwinbase", "minwindef", "windef", "wingdi", "winnt"], &["kernel32"]),
    ("wincrypt", &["basetsd", "guiddef", "minwindef", "ncrypt", "vcruntime", "winnt"], &[]),
    ("winevt", &["minwindef", "winnt"], &[]),
    ("wingdi", &["basetsd", "minwindef", "windef", "winnt"], &[]),
    ("wininet", &["basetsd", "minwinbase", "minwindef", "ntdef", "windef", "winineti", "winnt"], &["wininet"]),
    ("winineti", &["minwindef"], &[]),
    ("winioctl", &["minwindef", "winnt"], &[]),
    ("winnt", &["basetsd", "excpt", "guiddef", "minwindef"], &[]),
    ("winreg", &["basetsd", "minwindef", "winnt"], &[]),
    ("winsock2", &["guiddef", "inaddr", "minwindef", "vcruntime", "winnt"], &[]),
    ("winuser", &["basetsd", "minwindef", "windef", "wingdi", "winnt"], &[]),
    ("xinput", &["minwindef", "winnt"], &[]),
    // vc
    ("excpt", &[], &[]),
    ("vcruntime", &[], &[]),
    // winrt
    ("activation", &["inspectable", "winnt"], &[]),
    ("hstring", &["winnt"], &[]),
    ("inspectable", &["guiddef", "hstring", "minwindef", "unknwnbase", "winnt"], &[]),
];
struct Header {
    required: bool,
    included: Cell<bool>,
    dependencies: &'static [&'static str],
    libraries: &'static [&'static str],
}
struct Graph(HashMap<&'static str, Header>);
impl Graph {
    fn generate() -> Graph {
        Graph(DATA.iter().map(|&(name, dependencies, libraries)| {
            let header = Header {
                required: false,
                included: Cell::new(false),
                dependencies: dependencies,
                libraries: libraries,
            };
            (name, header)
        }).collect())
    }
    fn identify_required(&mut self) {
        for (name, header) in &mut self.0 {
            if let Ok(_) = var(&format!("CARGO_FEATURE_{}", name)) {
                header.required = true;
                header.included.set(true);
            }
        }
    }
    fn check_everything(&self) {
        if let Ok(_) = var("CARGO_FEATURE_everything") {
            for (_, header) in &self.0 {
                header.included.set(true);
            }
        }
    }
    fn resolve_dependencies(&self) {
        let mut done = false;
        while !done {
            done = true;
            for (_, header) in &self.0 {
                if header.included.get() {
                    for dep in header.dependencies {
                        let dep = &self.0.get(dep).expect(dep);
                        if !dep.included.get() {
                            done = false;
                            dep.included.set(true);
                        }
                    }
                }
            }
        }
    }
    fn emit_features(&self) {
        for (name, header) in &self.0 {
            if header.included.get() && !header.required {
                println!("cargo:rustc-cfg=feature=\"{}\"", name);
            }
        }
    }
    fn emit_libraries(&self) {
        let libs = self.0.iter().filter(|&(_, header)| {
            header.included.get()
        }).flat_map(|(_, header)| {
            header.libraries.iter()
        }).collect::<HashSet<_>>();
        for lib in libs {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
    }
}
fn try_everything() {
    let mut graph = Graph::generate();
    graph.identify_required();
    graph.check_everything();
    graph.resolve_dependencies();
    graph.emit_features();
    graph.emit_libraries();
}
fn main() {
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        try_everything();
    }
}
