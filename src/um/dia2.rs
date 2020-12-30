use shared::guiddef::GUID;
use shared::ntdef::{HRESULT, LPCWSTR};
use shared::wtypes::BSTR;
use um::oaidl::VARIANT;
use um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
// Implements IDiaDataSource
RIDL!{#[uuid(0xe6756135, 0x1e65, 0x4d17, 0x85, 0x76, 0x61, 0x07, 0x61, 0x39, 0x8c, 0x3c)]
class DiaSource; }
RIDL!{#[uuid(0x79f1bb5f, 0xb66e, 0x48e5, 0xb6, 0xa9, 0x15, 0x45, 0xc3, 0x23, 0xca, 0x3d)]
interface IDiaDataSource(IDiaDataSourceVtbl): IUnknown(IUnknownVtbl) {
    fn get_lastError(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn loadDataFromPdb(
        pdbPath: LPCWSTR,
    ) -> HRESULT,
    fn loadAndValidateDataFromPdb(
        pdbPath: LPCWSTR,
        pcsig70: *const GUID,
        sig: u32,
        age: u32,
    ) -> HRESULT,
    fn loadDataForExe(
        executable: LPCWSTR,
        searchPath: LPCWSTR,
        pCallback: LPUNKNOWN,
    ) -> HRESULT,
    fn loadDataFromIStream(
        pIStream: *const IStream,
    ) -> HRESULT,
    fn openSession(
        ppSession: *mut *mut IDiaSession,
    ) -> HRESULT,
    fn loadDataFromCodeViewInfo(
        executable: LPCWSTR,
        searchPath: LPCWSTR,
        cbCvInfo: u32,
        pbCvInfo: *const u8,
        pCallback: LPUNKNOWN,
    ) -> HRESULT,
    fn loadDataFromMiscInfo(
        executable: LPCWSTR,
        searchPath: LPCWSTR,
        timeStampExe: u32,
        timeStampDbg: u32,
        sizeOfExe: u32,
        cbMiscInfo: u32,
        pbMiscInfo: *const u8,
        pCallback: LPUNKNOWN,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0000000c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IStream(IStreamVtbl): ISequentialStream(ISequentialStreamVtbl) {
    fn RemoteSeek(
        dlibMove: _LARGE_INTEGER,
        dwOrigin: u32,
        plibNewPosition: *mut _ULARGE_INTEGER,
    ) -> HRESULT,
    fn SetSize(
        libNewSize: _ULARGE_INTEGER,
    ) -> HRESULT,
    fn RemoteCopyTo(
        pstm: *const IStream,
        cb: _ULARGE_INTEGER,
        pcbRead: *mut _ULARGE_INTEGER,
        pcbWritten: *mut _ULARGE_INTEGER,
    ) -> HRESULT,
    fn Commit(
        grfCommitFlags: u32,
    ) -> HRESULT,
    fn Revert(
    ) -> HRESULT,
    fn LockRegion(
        libOffset: _ULARGE_INTEGER,
        cb: _ULARGE_INTEGER,
        dwLockType: u32,
    ) -> HRESULT,
    fn UnlockRegion(
        libOffset: _ULARGE_INTEGER,
        cb: _ULARGE_INTEGER,
        dwLockType: u32,
    ) -> HRESULT,
    fn Stat(
        pstatstg: *mut tagSTATSTG,
        grfStatFlag: u32,
    ) -> HRESULT,
    fn Clone(
        ppstm: *mut *mut IStream,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0c733a30, 0x2a1c, 0x11ce, 0xad, 0xe5, 0x00, 0xaa, 0x00, 0x44, 0x77, 0x3d)]
interface ISequentialStream(ISequentialStreamVtbl): IUnknown(IUnknownVtbl) {
    fn RemoteRead(
        pv: *mut u8,
        cb: u32,
        pcbRead: *mut u32,
    ) -> HRESULT,
    fn RemoteWrite(
        pv: *const u8,
        cb: u32,
        pcbWritten: *mut u32,
    ) -> HRESULT,
}}
STRUCT!{struct _LARGE_INTEGER {
    QuadPart: i64,
}}
STRUCT!{struct _ULARGE_INTEGER {
    QuadPart: u64,
}}
STRUCT!{struct tagSTATSTG {
    pwcsName: LPCWSTR,
    type_: u32,
    cbSize: _ULARGE_INTEGER,
    mtime: _FILETIME,
    ctime: _FILETIME,
    atime: _FILETIME,
    grfMode: u32,
    grfLocksSupported: u32,
    clsid: GUID,
    grfStateBits: u32,
    reserved: u32,
}}
STRUCT!{struct _FILETIME {
    dwLowDateTime: u32,
    dwHighDateTime: u32,
}}
RIDL!{#[uuid(0x2f609ee1, 0xd1c8, 0x4e24, 0x82, 0x88, 0x33, 0x26, 0xba, 0xdc, 0xd2, 0x11)]
interface IDiaSession(IDiaSessionVtbl): IUnknown(IUnknownVtbl) {
    fn get_loadAddress(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn put_loadAddress(
        pRetVal: u64,
    ) -> HRESULT,
    fn get_globalScope(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn getEnumTables(
        ppEnumTables: *mut *mut IDiaEnumTables,
    ) -> HRESULT,
    fn getSymbolsByAddr(
        ppEnumbyAddr: *mut *mut IDiaEnumSymbolsByAddr,
    ) -> HRESULT,
    fn findChildren(
        parent: *const IDiaSymbol,
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findChildrenEx(
        parent: *const IDiaSymbol,
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findChildrenExByAddr(
        parent: *const IDiaSymbol,
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        isect: u32,
        offset: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findChildrenExByVA(
        parent: *const IDiaSymbol,
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        va: u64,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findChildrenExByRVA(
        parent: *const IDiaSymbol,
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        rva: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findSymbolByAddr(
        isect: u32,
        offset: u32,
        symTag: SymTagEnum,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn findSymbolByRVA(
        rva: u32,
        symTag: SymTagEnum,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn findSymbolByVA(
        va: u64,
        symTag: SymTagEnum,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn findSymbolByToken(
        token: u32,
        symTag: SymTagEnum,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn symsAreEquiv(
        symbolA: *const IDiaSymbol,
        symbolB: *const IDiaSymbol,
    ) -> HRESULT,
    fn symbolById(
        id: u32,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn findSymbolByRVAEx(
        rva: u32,
        symTag: SymTagEnum,
        ppSymbol: *mut *mut IDiaSymbol,
        displacement: *mut i32,
    ) -> HRESULT,
    fn findSymbolByVAEx(
        va: u64,
        symTag: SymTagEnum,
        ppSymbol: *mut *mut IDiaSymbol,
        displacement: *mut i32,
    ) -> HRESULT,
    fn findFile(
        pCompiland: *const IDiaSymbol,
        name: LPCWSTR,
        compareFlags: u32,
        ppResult: *mut *mut IDiaEnumSourceFiles,
    ) -> HRESULT,
    fn findFileById(
        uniqueId: u32,
        ppResult: *mut *mut IDiaSourceFile,
    ) -> HRESULT,
    fn findLines(
        compiland: *const IDiaSymbol,
        file: *const IDiaSourceFile,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findLinesByAddr(
        seg: u32,
        offset: u32,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findLinesByRVA(
        rva: u32,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findLinesByVA(
        va: u64,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findLinesByLinenum(
        compiland: *const IDiaSymbol,
        file: *const IDiaSourceFile,
        linenum: u32,
        column: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInjectedSource(
        srcFile: LPCWSTR,
        ppResult: *mut *mut IDiaEnumInjectedSources,
    ) -> HRESULT,
    fn getEnumDebugStreams(
        ppEnumDebugStreams: *mut *mut IDiaEnumDebugStreams,
    ) -> HRESULT,
    fn findInlineFramesByAddr(
        parent: *const IDiaSymbol,
        isect: u32,
        offset: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findInlineFramesByRVA(
        parent: *const IDiaSymbol,
        rva: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findInlineFramesByVA(
        parent: *const IDiaSymbol,
        va: u64,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findInlineeLines(
        parent: *const IDiaSymbol,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInlineeLinesByAddr(
        parent: *const IDiaSymbol,
        isect: u32,
        offset: u32,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInlineeLinesByRVA(
        parent: *const IDiaSymbol,
        rva: u32,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInlineeLinesByVA(
        parent: *const IDiaSymbol,
        va: u64,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInlineeLinesByLinenum(
        compiland: *const IDiaSymbol,
        file: *const IDiaSourceFile,
        linenum: u32,
        column: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInlineesByName(
        name: LPCWSTR,
        option: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findAcceleratorInlineeLinesByLinenum(
        parent: *const IDiaSymbol,
        file: *const IDiaSourceFile,
        linenum: u32,
        column: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findSymbolsForAcceleratorPointerTag(
        parent: *const IDiaSymbol,
        tagValue: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findSymbolsByRVAForAcceleratorPointerTag(
        parent: *const IDiaSymbol,
        tagValue: u32,
        rva: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findAcceleratorInlineesByName(
        name: LPCWSTR,
        option: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn addressForVA(
        va: u64,
        pISect: *mut u32,
        pOffset: *mut u32,
    ) -> HRESULT,
    fn addressForRVA(
        rva: u32,
        pISect: *mut u32,
        pOffset: *mut u32,
    ) -> HRESULT,
    fn findILOffsetsByAddr(
        isect: u32,
        offset: u32,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findILOffsetsByRVA(
        rva: u32,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findILOffsetsByVA(
        va: u64,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInputAssemblyFiles(
        ppResult: *mut *mut IDiaEnumInputAssemblyFiles,
    ) -> HRESULT,
    fn findInputAssembly(
        index: u32,
        ppResult: *mut *mut IDiaInputAssemblyFile,
    ) -> HRESULT,
    fn findInputAssemblyById(
        uniqueId: u32,
        ppResult: *mut *mut IDiaInputAssemblyFile,
    ) -> HRESULT,
    fn getFuncMDTokenMapSize(
        pcb: *mut u32,
    ) -> HRESULT,
    fn getFuncMDTokenMap(
        cb: u32,
        pcb: *mut u32,
        pb: *mut u8,
    ) -> HRESULT,
    fn getTypeMDTokenMapSize(
        pcb: *mut u32,
    ) -> HRESULT,
    fn getTypeMDTokenMap(
        cb: u32,
        pcb: *mut u32,
        pb: *mut u8,
    ) -> HRESULT,
    fn getNumberOfFunctionFragments_VA(
        vaFunc: u64,
        cbFunc: u32,
        pNumFragments: *mut u32,
    ) -> HRESULT,
    fn getNumberOfFunctionFragments_RVA(
        rvaFunc: u32,
        cbFunc: u32,
        pNumFragments: *mut u32,
    ) -> HRESULT,
    fn getFunctionFragments_VA(
        vaFunc: u64,
        cbFunc: u32,
        cFragments: u32,
        pVaFragment: *mut u64,
        pLenFragment: *mut u32,
    ) -> HRESULT,
    fn getFunctionFragments_RVA(
        rvaFunc: u32,
        cbFunc: u32,
        cFragments: u32,
        pRvaFragment: *mut u32,
        pLenFragment: *mut u32,
    ) -> HRESULT,
    fn getExports(
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn getHeapAllocationSites(
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findInputAssemblyFile(
        pSymbol: *const IDiaSymbol,
        ppResult: *mut *mut IDiaInputAssemblyFile,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xcb787b2f, 0xbd6c, 0x4635, 0xba, 0x52, 0x93, 0x31, 0x26, 0xbd, 0x2d, 0xcd)]
interface IDiaSymbol(IDiaSymbolVtbl): IUnknown(IUnknownVtbl) {
    fn get_symIndexId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_symTag(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_lexicalParent(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_classParent(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_type(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_dataKind(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_locationType(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_addressSection(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_addressOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_relativeVirtualAddress(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_virtualAddress(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_registerId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_offset(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_length(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_slot(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_volatileType(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_constType(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_unalignedType(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_access(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_libraryName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_platform(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_language(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_editAndContinueEnabled(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_frontEndMajor(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_frontEndMinor(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_frontEndBuild(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_backEndMajor(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_backEndMinor(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_backEndBuild(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_sourceFileName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_unused(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_thunkOrdinal(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_thisAdjust(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_virtualBaseOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_virtual(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_intro(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_pure(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_callingConvention(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_value(
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_baseType(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_token(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_timeStamp(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_guid(
        pRetVal: *mut GUID,
    ) -> HRESULT,
    fn get_symbolsFileName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_reference(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_bitPosition(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_arrayIndexType(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_packed(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_constructor(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_overloadedOperator(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_nested(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasNestedTypes(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasAssignmentOperator(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasCastOperator(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_scoped(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_virtualBaseClass(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_indirectVirtualBaseClass(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_virtualBasePointerOffset(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_virtualTableShape(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_lexicalParentId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_classParentId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_typeId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_arrayIndexTypeId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_virtualTableShapeId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_code(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_function(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_managed(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_msil(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_virtualBaseDispIndex(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_undecoratedName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_age(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_signature(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_compilerGenerated(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_addressTaken(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_rank(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lowerBound(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_upperBound(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_lowerBoundId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_upperBoundId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_dataBytes(
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
    ) -> HRESULT,
    fn findChildren(
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findChildrenEx(
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findChildrenExByAddr(
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        isect: u32,
        offset: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findChildrenExByVA(
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        va: u64,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findChildrenExByRVA(
        symTag: SymTagEnum,
        name: LPCWSTR,
        compareFlags: u32,
        rva: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn get_targetSection(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_targetOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_targetRelativeVirtualAddress(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_targetVirtualAddress(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_machineType(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_oemId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_oemSymbolId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_types(
        cTypes: u32,
        pcTypes: *mut u32,
        pTypes: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_typeIds(
        cTypeIds: u32,
        pcTypeIds: *mut u32,
        pdwTypeIds: *mut u32,
    ) -> HRESULT,
    fn get_objectPointerType(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_udtKind(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_undecoratedNameEx(
        undecorateOptions: u32,
        name: *mut BSTR,
    ) -> HRESULT,
    fn get_noReturn(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_customCallingConvention(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_noInline(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_optimizedCodeDebugInfo(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_notReached(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_interruptReturn(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_farReturn(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isStatic(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasDebugInfo(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isLTCG(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isDataAligned(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasSecurityChecks(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_compilerName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_hasAlloca(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasSetJump(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasLongJump(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasInlAsm(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasEH(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasSEH(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasEHa(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isNaked(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isAggregated(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isSplitted(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_container(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_inlSpec(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_noStackOrdering(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_virtualBaseTableType(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_hasManagedCode(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isHotpatchable(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isCVTCIL(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isMSILNetmodule(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isCTypes(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isStripped(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_frontEndQFE(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_backEndQFE(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_wasInlined(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_strictGSCheck(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isCxxReturnUdt(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isConstructorVirtualBase(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_RValueReference(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_unmodifiedType(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_framePointerPresent(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isSafeBuffers(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_intrinsic(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_sealed(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hfaFloat(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hfaDouble(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_liveRangeStartAddressSection(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_liveRangeStartAddressOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_liveRangeStartRelativeVirtualAddress(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_countLiveRanges(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_liveRangeLength(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_offsetInUdt(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_paramBasePointerRegisterId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_localBasePointerRegisterId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_isLocationControlFlowDependent(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_stride(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_numberOfRows(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_numberOfColumns(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_isMatrixRowMajor(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_numericProperties(
        cnt: u32,
        pcnt: *mut u32,
        pProperties: *mut u32,
    ) -> HRESULT,
    fn get_modifierValues(
        cnt: u32,
        pcnt: *mut u32,
        pModifiers: *mut u16,
    ) -> HRESULT,
    fn get_isReturnValue(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isOptimizedAway(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_builtInKind(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_registerType(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_baseDataSlot(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_baseDataOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_textureSlot(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_samplerSlot(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_uavSlot(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_sizeInUdt(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_memorySpaceKind(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_unmodifiedTypeId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_subTypeId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_subType(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_numberOfModifiers(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_numberOfRegisterIndices(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_isHLSLData(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isPointerToDataMember(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isPointerToMemberFunction(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isSingleInheritance(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isMultipleInheritance(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isVirtualInheritance(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_restrictedType(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isPointerBasedOnSymbolValue(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_baseSymbol(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_baseSymbolId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_objectFileName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_isAcceleratorGroupSharedLocal(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isAcceleratorPointerTagLiveRange(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isAcceleratorStubFunction(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_numberOfAcceleratorPointerTags(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_isSdl(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isWinRTPointer(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isRefUdt(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isValueUdt(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isInterfaceUdt(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn findInlineFramesByAddr(
        isect: u32,
        offset: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findInlineFramesByRVA(
        rva: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findInlineFramesByVA(
        va: u64,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findInlineeLines(
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInlineeLinesByAddr(
        isect: u32,
        offset: u32,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInlineeLinesByRVA(
        rva: u32,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findInlineeLinesByVA(
        va: u64,
        length: u32,
        ppResult: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
    fn findSymbolsForAcceleratorPointerTag(
        tagValue: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn findSymbolsByRVAForAcceleratorPointerTag(
        tagValue: u32,
        rva: u32,
        ppResult: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn get_acceleratorPointerTags(
        cnt: u32,
        pcnt: *mut u32,
        pPointerTags: *mut u32,
    ) -> HRESULT,
    fn getSrcLineOnTypeDefn(
        ppResult: *mut *mut IDiaLineNumber,
    ) -> HRESULT,
    fn get_isPGO(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_hasValidPGOCounts(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isOptimizedForSpeed(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_PGOEntryCount(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_PGOEdgeCount(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_PGODynamicInstructionCount(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_staticSize(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_finalLiveStaticSize(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_phaseName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_hasControlFlowCheck(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_constantExport(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_dataExport(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_privateExport(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_noNameExport(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_exportHasExplicitlyAssignedOrdinal(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_exportIsForwarder(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_ordinal(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_frameSize(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_exceptionHandlerAddressSection(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_exceptionHandlerAddressOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_exceptionHandlerRelativeVirtualAddress(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_exceptionHandlerVirtualAddress(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn findInputAssemblyFile(
        ppResult: *mut *mut IDiaInputAssemblyFile,
    ) -> HRESULT,
    fn get_characteristics(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_coffGroup(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_bindID(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_bindSpace(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_bindSlot(
        pRetVal: *mut u32,
    ) -> HRESULT,
}}
ENUM!{enum SymTagEnum {
    SymTagNull = 0,
    SymTagExe = 1,
    SymTagCompiland = 2,
    SymTagCompilandDetails = 3,
    SymTagCompilandEnv = 4,
    SymTagFunction = 5,
    SymTagBlock = 6,
    SymTagData = 7,
    SymTagAnnotation = 8,
    SymTagLabel = 9,
    SymTagPublicSymbol = 10,
    SymTagUDT = 11,
    SymTagEnum = 12,
    SymTagFunctionType = 13,
    SymTagPointerType = 14,
    SymTagArrayType = 15,
    SymTagBaseType = 16,
    SymTagTypedef = 17,
    SymTagBaseClass = 18,
    SymTagFriend = 19,
    SymTagFunctionArgType = 20,
    SymTagFuncDebugStart = 21,
    SymTagFuncDebugEnd = 22,
    SymTagUsingNamespace = 23,
    SymTagVTableShape = 24,
    SymTagVTable = 25,
    SymTagCustom = 26,
    SymTagThunk = 27,
    SymTagCustomType = 28,
    SymTagManagedType = 29,
    SymTagDimension = 30,
    SymTagCallSite = 31,
    SymTagInlineSite = 32,
    SymTagBaseInterface = 33,
    SymTagVectorType = 34,
    SymTagMatrixType = 35,
    SymTagHLSLType = 36,
    SymTagCaller = 37,
    SymTagCallee = 38,
    SymTagExport = 39,
    SymTagHeapAllocationSite = 40,
    SymTagCoffGroup = 41,
    SymTagInlinee = 42,
    SymTagMax = 43,
}}
RIDL!{#[uuid(0xcab72c48, 0x443b, 0x48f5, 0x9b, 0x0b, 0x42, 0xf0, 0x82, 0x0a, 0xb2, 0x9a)]
interface IDiaEnumSymbols(IDiaEnumSymbolsVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: u32,
        symbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaSymbol,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfe30e878, 0x54ac, 0x44f1, 0x81, 0xba, 0x39, 0xde, 0x94, 0x0f, 0x60, 0x52)]
interface IDiaEnumLineNumbers(IDiaEnumLineNumbersVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: u32,
        lineNumber: *mut *mut IDiaLineNumber,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaLineNumber,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumLineNumbers,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb388eb14, 0xbe4d, 0x421d, 0xa8, 0xa1, 0x6c, 0xf7, 0xab, 0x05, 0x70, 0x86)]
interface IDiaLineNumber(IDiaLineNumberVtbl): IUnknown(IUnknownVtbl) {
    fn get_compiland(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_sourceFile(
        pRetVal: *mut *mut IDiaSourceFile,
    ) -> HRESULT,
    fn get_lineNumber(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lineNumberEnd(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_columnNumber(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_columnNumberEnd(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_addressSection(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_addressOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_relativeVirtualAddress(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_virtualAddress(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_length(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_sourceFileId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_statement(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_compilandId(
        pRetVal: *mut u32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa2ef5353, 0xf5a8, 0x4eb3, 0x90, 0xd2, 0xcb, 0x52, 0x6a, 0xcb, 0x3c, 0xdd)]
interface IDiaSourceFile(IDiaSourceFileVtbl): IUnknown(IUnknownVtbl) {
    fn get_uniqueId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_fileName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_checksumType(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_compilands(
        pRetVal: *mut *mut IDiaEnumSymbols,
    ) -> HRESULT,
    fn get_checksum(
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3bfe56b0, 0x390c, 0x4863, 0x94, 0x30, 0x1f, 0x3d, 0x08, 0x3b, 0x76, 0x84)]
interface IDiaInputAssemblyFile(IDiaInputAssemblyFileVtbl): IUnknown(IUnknownVtbl) {
    fn get_uniqueId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_index(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_timeStamp(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_pdbAvailableAtILMerge(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_fileName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_version(
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc65c2b0a, 0x1150, 0x4d7a, 0xaf, 0xcc, 0xe0, 0x5b, 0xf3, 0xde, 0xe8, 0x1e)]
interface IDiaEnumTables(IDiaEnumTablesVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: VARIANT,
        table: *mut *mut IDiaTable,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaTable,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumTables,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4a59fb77, 0xabac, 0x469b, 0xa3, 0x0b, 0x9e, 0xcc, 0x85, 0xbf, 0xef, 0x14)]
interface IDiaTable(IDiaTableVtbl): IEnumUnknown(IEnumUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: u32,
        element: *mut LPUNKNOWN,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000100, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumUnknown(IEnumUnknownVtbl): IUnknown(IUnknownVtbl) {
    fn RemoteNext(
        celt: u32,
        rgelt: *mut LPUNKNOWN,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumUnknown,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x624b7d9c, 0x24ea, 0x4421, 0x9d, 0x06, 0x3b, 0x57, 0x74, 0x71, 0xc1, 0xfa)]
interface IDiaEnumSymbolsByAddr(IDiaEnumSymbolsByAddrVtbl): IUnknown(IUnknownVtbl) {
    fn symbolByAddr(
        isect: u32,
        offset: u32,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn symbolByRVA(
        relativeVirtualAddress: u32,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn symbolByVA(
        virtualAddress: u64,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaSymbol,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Prev(
        celt: u32,
        rgelt: *mut *mut IDiaSymbol,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumSymbolsByAddr,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x10f3dbd9, 0x664f, 0x4469, 0xb8, 0x08, 0x94, 0x71, 0xc7, 0xa5, 0x05, 0x38)]
interface IDiaEnumSourceFiles(IDiaEnumSourceFilesVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: u32,
        sourceFile: *mut *mut IDiaSourceFile,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaSourceFile,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumSourceFiles,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd5612573, 0x6925, 0x4468, 0x88, 0x83, 0x98, 0xcd, 0xec, 0x8c, 0x38, 0x4a)]
interface IDiaEnumInjectedSources(IDiaEnumInjectedSourcesVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: u32,
        injectedSource: *mut *mut IDiaInjectedSource,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaInjectedSource,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumInjectedSources,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xae605cdc, 0x8105, 0x4a23, 0xb7, 0x10, 0x32, 0x59, 0xf1, 0xe2, 0x61, 0x12)]
interface IDiaInjectedSource(IDiaInjectedSourceVtbl): IUnknown(IUnknownVtbl) {
    fn get_crc(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_length(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_fileName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_objectFileName(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_virtualFilename(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_sourceCompression(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_source(
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x08cbb41e, 0x47a6, 0x4f87, 0x92, 0xf1, 0x1c, 0x9c, 0x87, 0xce, 0xd0, 0x44)]
interface IDiaEnumDebugStreams(IDiaEnumDebugStreamsVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: VARIANT,
        stream: *mut *mut IDiaEnumDebugStreamData,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaEnumDebugStreamData,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumDebugStreams,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x486943e8, 0xd187, 0x4a6b, 0xa3, 0xc4, 0x29, 0x12, 0x59, 0xff, 0xf6, 0x0d)]
interface IDiaEnumDebugStreamData(IDiaEnumDebugStreamDataVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn Item(
        index: u32,
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumDebugStreamData,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1c7ff653, 0x51f7, 0x457e, 0x84, 0x19, 0xb2, 0x0f, 0x57, 0xef, 0x7e, 0x4d)]
interface IDiaEnumInputAssemblyFiles(IDiaEnumInputAssemblyFilesVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: u32,
        file: *mut *mut IDiaInputAssemblyFile,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaInputAssemblyFile,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumInputAssemblyFiles,
    ) -> HRESULT,
}}
// Implements IDiaDataSource
RIDL!{#[uuid(0x91904831, 0x49ca, 0x4766, 0xb9, 0x5c, 0x25, 0x39, 0x7e, 0x2d, 0xd6, 0xdc)]
class DiaSourceAlt; }
// Implements IDiaStackWalker
RIDL!{#[uuid(0xce4a85db, 0x5768, 0x475b, 0xa4, 0xe1, 0xc0, 0xbc, 0xa2, 0x11, 0x2a, 0x6b)]
class DiaStackWalker; }
RIDL!{#[uuid(0x5485216b, 0xa54c, 0x469f, 0x96, 0x70, 0x52, 0xb2, 0x4d, 0x52, 0x29, 0xbb)]
interface IDiaStackWalker(IDiaStackWalkerVtbl): IUnknown(IUnknownVtbl) {
    fn getEnumFrames(
        pHelper: *const IDiaStackWalkHelper,
        ppenum: *mut *mut IDiaEnumStackFrames,
    ) -> HRESULT,
    fn getEnumFrames2(
        cpuid: CV_CPU_TYPE_e,
        pHelper: *const IDiaStackWalkHelper,
        ppenum: *mut *mut IDiaEnumStackFrames,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x21f81b1b, 0xc5bb, 0x42a3, 0xbc, 0x4f, 0xcc, 0xba, 0xa7, 0x5b, 0x9f, 0x19)]
interface IDiaStackWalkHelper(IDiaStackWalkHelperVtbl): IUnknown(IUnknownVtbl) {
    fn get_registerValue(
        index: u32,
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn put_registerValue(
        index: u32,
        pRetVal: u64,
    ) -> HRESULT,
    fn readMemory(
        type_: MemoryTypeEnum,
        va: u64,
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
    ) -> HRESULT,
    fn searchForReturnAddress(
        frame: *const IDiaFrameData,
        returnAddress: *mut u64,
    ) -> HRESULT,
    fn searchForReturnAddressStart(
        frame: *const IDiaFrameData,
        startAddress: u64,
        returnAddress: *mut u64,
    ) -> HRESULT,
    fn frameForVA(
        va: u64,
        ppFrame: *mut *mut IDiaFrameData,
    ) -> HRESULT,
    fn symbolForVA(
        va: u64,
        ppSymbol: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn pdataForVA(
        va: u64,
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
    ) -> HRESULT,
    fn imageForVA(
        vaContext: u64,
        pvaImageStart: *mut u64,
    ) -> HRESULT,
    fn addressForVA(
        va: u64,
        pISect: *mut u32,
        pOffset: *mut u32,
    ) -> HRESULT,
    fn numberOfFunctionFragmentsForVA(
        vaFunc: u64,
        cbFunc: u32,
        pNumFragments: *mut u32,
    ) -> HRESULT,
    fn functionFragmentsForVA(
        vaFunc: u64,
        cbFunc: u32,
        cFragments: u32,
        pVaFragment: *mut u64,
        pLenFragment: *mut u32,
    ) -> HRESULT,
}}
ENUM!{enum MemoryTypeEnum {
    MemTypeCode = 0,
    MemTypeData = 1,
    MemTypeStack = 2,
    MemTypeCodeOnHeap = 3,
    MemTypeAny = 0xffffffff,
}}
RIDL!{#[uuid(0xa39184b7, 0x6a36, 0x42de, 0x8e, 0xec, 0x7d, 0xf9, 0xf3, 0xf5, 0x9f, 0x33)]
interface IDiaFrameData(IDiaFrameDataVtbl): IUnknown(IUnknownVtbl) {
    fn get_addressSection(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_addressOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_relativeVirtualAddress(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_virtualAddress(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_lengthBlock(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lengthLocals(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lengthParams(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_maxStack(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lengthProlog(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lengthSavedRegisters(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_program(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_systemExceptionHandling(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_cplusplusExceptionHandling(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_functionStart(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_allocatesBasePointer(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_type(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_functionParent(
        pRetVal: *mut *mut IDiaFrameData,
    ) -> HRESULT,
    fn execute(
        frame: *mut IDiaStackWalkFrame,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x07c590c1, 0x438d, 0x4f47, 0xbd, 0xcd, 0x43, 0x97, 0xbc, 0x81, 0xad, 0x75)]
interface IDiaStackWalkFrame(IDiaStackWalkFrameVtbl): IUnknown(IUnknownVtbl) {
    fn get_registerValue(
        index: u32,
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn put_registerValue(
        index: u32,
        pRetVal: u64,
    ) -> HRESULT,
    fn readMemory(
        type_: MemoryTypeEnum,
        va: u64,
        cbData: u32,
        pcbData: *mut u32,
        pbData: *mut u8,
    ) -> HRESULT,
    fn searchForReturnAddress(
        frame: *const IDiaFrameData,
        returnAddress: *mut u64,
    ) -> HRESULT,
    fn searchForReturnAddressStart(
        frame: *const IDiaFrameData,
        startAddress: u64,
        returnAddress: *mut u64,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xec9d461d, 0xce74, 0x4711, 0xa0, 0x20, 0x7d, 0x8f, 0x9a, 0x1d, 0xd2, 0x55)]
interface IDiaEnumStackFrames(IDiaEnumStackFramesVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaStackFrame,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5edbc96d, 0xcdd6, 0x4792, 0xaf, 0xbe, 0xcc, 0x89, 0x00, 0x7d, 0x96, 0x10)]
interface IDiaStackFrame(IDiaStackFrameVtbl): IUnknown(IUnknownVtbl) {
    fn get_type(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_base(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_size(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_returnAddress(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_localsBase(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_lengthLocals(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lengthParams(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lengthProlog(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_lengthSavedRegisters(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_systemExceptionHandling(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_cplusplusExceptionHandling(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_functionStart(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_allocatesBasePointer(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_maxStack(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_registerValue(
        index: u32,
        pRetVal: *mut u64,
    ) -> HRESULT,
}}
ENUM!{enum CV_CPU_TYPE_e {
    CV_CFL_8080 = 0,
    CV_CFL_8086 = 1,
    CV_CFL_80286 = 2,
    CV_CFL_80386 = 3,
    CV_CFL_80486 = 4,
    CV_CFL_PENTIUM = 5,
    CV_CFL_PENTIUMII = 6,
    CV_CFL_PENTIUMPRO = 6,
    CV_CFL_PENTIUMIII = 7,
    CV_CFL_MIPS = 16,
    CV_CFL_MIPSR4000 = 16,
    CV_CFL_MIPS16 = 17,
    CV_CFL_MIPS32 = 18,
    CV_CFL_MIPS64 = 19,
    CV_CFL_MIPSI = 20,
    CV_CFL_MIPSII = 21,
    CV_CFL_MIPSIII = 22,
    CV_CFL_MIPSIV = 23,
    CV_CFL_MIPSV = 24,
    CV_CFL_M68000 = 32,
    CV_CFL_M68010 = 33,
    CV_CFL_M68020 = 34,
    CV_CFL_M68030 = 35,
    CV_CFL_M68040 = 36,
    CV_CFL_ALPHA = 48,
    CV_CFL_ALPHA_21064 = 48,
    CV_CFL_ALPHA_21164 = 49,
    CV_CFL_ALPHA_21164A = 50,
    CV_CFL_ALPHA_21264 = 51,
    CV_CFL_ALPHA_21364 = 52,
    CV_CFL_PPC601 = 64,
    CV_CFL_PPC603 = 65,
    CV_CFL_PPC604 = 66,
    CV_CFL_PPC620 = 67,
    CV_CFL_PPCFP = 68,
    CV_CFL_PPCBE = 69,
    CV_CFL_SH3 = 80,
    CV_CFL_SH3E = 81,
    CV_CFL_SH3DSP = 82,
    CV_CFL_SH4 = 83,
    CV_CFL_SHMEDIA = 84,
    CV_CFL_ARM3 = 96,
    CV_CFL_ARM4 = 97,
    CV_CFL_ARM4T = 98,
    CV_CFL_ARM5 = 99,
    CV_CFL_ARM5T = 100,
    CV_CFL_ARM6 = 101,
    CV_CFL_ARM_XMAC = 102,
    CV_CFL_ARM_WMMX = 103,
    CV_CFL_ARM7 = 104,
    CV_CFL_OMNI = 112,
    CV_CFL_IA64 = 128,
    CV_CFL_IA64_1 = 128,
    CV_CFL_IA64_2 = 129,
    CV_CFL_CEE = 144,
    CV_CFL_AM33 = 160,
    CV_CFL_M32R = 176,
    CV_CFL_TRICORE = 192,
    CV_CFL_X64 = 208,
    CV_CFL_AMD64 = 208,
    CV_CFL_EBC = 224,
    CV_CFL_THUMB = 240,
    CV_CFL_ARMNT = 244,
    CV_CFL_ARM64 = 246,
    CV_CFL_HYBRID_X86_ARM64 = 247,
    CV_CFL_ARM64EC = 248,
    CV_CFL_ARM64X = 249,
    CV_CFL_D3D11_SHADER = 256,
}}
RIDL!{#[uuid(0x0cf4b60e, 0x35b1, 0x4c6c, 0xbd, 0xd8, 0x85, 0x4b, 0x9c, 0x8e, 0x38, 0x57)]
interface IDiaSectionContrib(IDiaSectionContribVtbl): IUnknown(IUnknownVtbl) {
    fn get_compiland(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_addressSection(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_addressOffset(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_relativeVirtualAddress(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_virtualAddress(
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn get_length(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_notPaged(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_code(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_initializedData(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_uninitializedData(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_remove(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_comdat(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_discardable(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_notCached(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_share(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_execute(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_read(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_write(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_dataCrc(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_relocationsCrc(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_compilandId(
        pRetVal: *mut u32,
    ) -> HRESULT,
    fn get_code16bit(
        pRetVal: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1994deb2, 0x2c82, 0x4b1d, 0xa5, 0x7f, 0xaf, 0xf4, 0x24, 0xd5, 0x4a, 0x68)]
interface IDiaEnumSectionContribs(IDiaEnumSectionContribsVtbl): IUnknown(IUnknownVtbl) {
    fn get__NewEnum(
        pRetVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_count(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn Item(
        index: u32,
        section: *mut *mut IDiaSectionContrib,
    ) -> HRESULT,
    fn Next(
        celt: u32,
        rgelt: *mut *mut IDiaSectionContrib,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    fn Skip(
        celt: u32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IDiaEnumSectionContribs,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x611e86cd, 0xb7d1, 0x4546, 0x8a, 0x15, 0x07, 0x0e, 0x2b, 0x07, 0xa4, 0x27)]
interface IDiaSymbol2(IDiaSymbol2Vtbl): IDiaSymbol(IDiaSymbolVtbl) {
    fn get_isObjCClass(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isObjCCategory(
        pRetVal: *mut i32,
    ) -> HRESULT,
    fn get_isObjCProtocol(
        pRetVal: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x99b665f7, 0xc1b2, 0x49d3, 0x89, 0xb2, 0xa3, 0x84, 0x36, 0x1a, 0xca, 0xb5)]
interface IDiaSymbol3(IDiaSymbol3Vtbl): IDiaSymbol2(IDiaSymbol2Vtbl) {
    fn get_inlinee(
        pRetVal: *mut *mut IDiaSymbol,
    ) -> HRESULT,
    fn get_inlineeId(
        pRetVal: *mut u32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbf6c88a7, 0xe9d6, 0x4346, 0x99, 0xa1, 0xd0, 0x53, 0xde, 0x5a, 0x78, 0x08)]
interface IDiaSymbol4(IDiaSymbol4Vtbl): IDiaSymbol3(IDiaSymbol3Vtbl) {
    fn get_noexcept(
        pRetVal: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xabe2de00, 0xdc2d, 0x4793, 0xaf, 0x9a, 0xef, 0x1d, 0x90, 0x83, 0x26, 0x44)]
interface IDiaSymbol5(IDiaSymbol5Vtbl): IDiaSymbol4(IDiaSymbol4Vtbl) {
    fn get_hasAbsoluteAddress(
        pRetVal: *mut i32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8133dad3, 0x75fe, 0x4234, 0xac, 0x7e, 0xf8, 0xe7, 0xa1, 0xd3, 0xcb, 0xb3)]
interface IDiaSymbol6(IDiaSymbol6Vtbl): IDiaSymbol5(IDiaSymbol5Vtbl) {
    fn get_isStaticMemberFunc(
        pRetVal: *mut i32,
    ) -> HRESULT,
}}
