// Copyright (c) William R. Fraser
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Bindings to the Extensible Storage Engine.
use ctypes::{
    __int64, c_char, c_double, c_int, c_long, c_uchar, c_ulong, c_ushort, c_void, wchar_t,
};
pub type JET_API_PTR = usize;
pub type JET_CBTYP = c_ulong;
pub type JET_COLTYP = c_ulong;
pub type JET_COLUMNID = c_ulong;
pub type JET_DATESERIAL = c_double;
pub type JET_DBID = c_ulong;
pub type JET_ERR = c_long;
pub type JET_ERRCAT = c_int; // actually a `typedef enum` in the C header.
pub type JET_GRBIT = c_ulong;
pub type JET_HANDLE = JET_API_PTR;
pub type JET_INSTANCE = JET_API_PTR;
pub type JET_LS = JET_API_PTR;
pub type JET_OBJTYP = c_ulong;
pub type JET_OSSNAPID = JET_API_PTR;
pub type JET_PCSTR = *const c_char;
pub type JET_PCWSTR = *const wchar_t;
pub type JET_PSTR = *mut c_char;
pub type JET_PWSTR = *mut wchar_t;
pub type JET_RELOP = c_int; // actually a `typedef enum` in the C header.
pub type JET_SESID = JET_API_PTR;
pub type JET_SNP = c_ulong;
pub type JET_SNT = c_ulong;
pub type JET_TABLEID = JET_API_PTR;
FN!{stdcall JET_CALLBACK(
    sesid: JET_SESID,
    dbid: JET_DBID,
    tableid: JET_TABLEID,
    cbtyp: JET_CBTYP,
    pvArg1: *mut c_void,
    pvArg2: *mut c_void,
    pvContext: *const c_void,
    ulUnused: JET_API_PTR,
) -> JET_ERR}
FN!{stdcall JET_PFNREALLOC(
    pvContext: *const c_void,
    pv: *const c_void,
    cb: c_ulong,
) -> JET_ERR}
FN!{stdcall JET_PFNSTATUS(
    sesid: JET_SESID,
    snp: JET_SNP,
    snt: JET_SNT,
    pv:  *mut c_void,
) -> JET_ERR}
STRUCT!{struct JET_COLUMNCREATE_W {
    cbStruct: c_ulong,
    szColumnName: *mut wchar_t,
    coltyp: JET_COLTYP,
    cbMax: c_ulong,
    grbit: JET_GRBIT,
    pvDefault: *mut c_void,
    cbDefault: c_ulong,
    cp: c_ulong,
    columnid: JET_COLUMNID,
    err: JET_ERR,
}}
STRUCT!{struct JET_COLUMNDEF {
    cbStruct: c_ulong,
    columnid: JET_COLUMNID,
    coltyp: JET_COLTYP,
    wCountry: c_ushort,
    langid: c_ushort,
    cp: c_ushort,
    wCollate: c_ushort,
    cbMax: c_ulong,
    grbit: JET_GRBIT,
}}
STRUCT!{struct JET_COMMIT_ID {
    signLog: JET_SIGNATURE,
    reserved: c_int,
    commitId: __int64,
}}
STRUCT!{struct JET_CONDITIONALCOLUMN_W {
    cbStruct: c_ulong,
    szColumnName: *mut wchar_t,
    grbit: JET_GRBIT,
}}
STRUCT!{struct JET_ENUMCOLUMN_u_s1 {
    cEnumColumnValue: c_ulong,
    rgEnumColumnValue: *mut JET_ENUMCOLUMNVALUE,
}}
STRUCT!{struct JET_ENUMCOLUMN_u_s2 {
    cbData: c_ulong,
    pvData: *mut c_void,
}}
UNION!{union JET_ENUMCOLUMN_u {
    [usize; 2],
    V1 V1_mut: JET_ENUMCOLUMN_u_s1,
    V2 V2_mut: JET_ENUMCOLUMN_u_s2,
}}
STRUCT!{struct JET_ENUMCOLUMN {
    columnid: JET_COLUMNID,
    err: JET_ERR,
    u: JET_ENUMCOLUMN_u,
}}
STRUCT!{struct JET_ENUMCOLUMNID {
    columnid: JET_COLUMNID,
    ctagSequence: c_ulong,
    rgtagSequence: *const c_ulong,
}}
STRUCT!{struct JET_ENUMCOLUMNVALUE {
    itagSequence: c_ulong,
    err: JET_ERR,
    cbData: c_ulong,
    pvData: *mut c_void,
}}
STRUCT!{struct JET_ERRINFOBASIC_W {
    cbStruct: c_ulong,
    errValue: JET_ERR,
    errcatMostSpecific: JET_ERRCAT,
    rgCategoricalHierarchy: [c_uchar; 8],
    lSourceLine: c_ulong,
    rgszSourceFile: [wchar_t; 64],
}}
STRUCT!{struct JET_INDEX_COLUMN {
    columnid: JET_COLUMNID,
    relop: JET_RELOP,
    pv: *mut c_void,
    cb: c_ulong,
    grbit: JET_GRBIT,
}}
#[cfg(target_pointer_width = "32")]
STRUCT!{struct JET_INDEXID {
    cbStruct: c_ulong,
    rgbIndexId: [c_char; 12], // sizeof(JET_API_PTR) + 2 * sizeof(unsigned long)
}}
#[cfg(target_pointer_width = "64")]
STRUCT!{struct JET_INDEXID {
    cbStruct: c_ulong,
    rgbIndexId: [c_char; 16], // sizeof(JET_API_PTR) + 2 * sizeof(unsigned long)
}}
STRUCT!{struct JET_INDEXCREATE_W {
    cbStruct: c_ulong,
    szIndexName: *mut wchar_t,
    szKey: *mut wchar_t,
    cbKey: c_ulong,
    grbit: JET_GRBIT,
    ulDensity: c_ulong,
    u1: JET_INDEXCREATE_W_u1,
    u2: JET_INDEXCREATE_W_u2,
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: c_ulong,
    err: JET_ERR,
    cbKeyMost: c_ulong,
}}
UNION!{union JET_INDEXCREATE_W_u1 {
    [usize; 1],
    lcid lcid_mut: c_ulong,
    pidxunicode pidxunicode_mut: *mut JET_UNICODEINDEX,
}}
UNION!{union JET_INDEXCREATE_W_u2 {
    [usize; 1],
    cbVarSegMac cbVarSegMac_mut: c_ulong,
    ptuplelimits ptuplelimits_mut: *mut JET_TUPLELIMITS,
}}
STRUCT!{struct JET_INDEXCREATE2_W {
    cbStruct: c_ulong,
    szIndexName: *mut wchar_t,
    szKey: *mut wchar_t,
    cbKey: c_ulong,
    grbit: JET_GRBIT,
    ulDensity: c_ulong,
    u1: JET_INDEXCREATE2_W_u1,
    u2: JET_INDEXCREATE2_W_u2,
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: c_ulong,
    err: JET_ERR,
    cbKeyMost: c_ulong,
    pSpacehints: *mut JET_SPACEHINTS,
}}
UNION!{union JET_INDEXCREATE2_W_u1 {
    [usize; 1],
    lcid lcid_mut: c_ulong,
    pidxunicode pidxunicode_mut: *mut JET_UNICODEINDEX,
}}
UNION!{union JET_INDEXCREATE2_W_u2 {
    [usize; 1],
    cbVarSegMac cbVarSegMac_mut: c_ulong,
    ptuplelimits ptuplelimits_mut: *mut JET_TUPLELIMITS,
}}
STRUCT!{struct JET_INDEXCREATE3_W {
    cbStruct: c_ulong,
    szIndexName: *mut wchar_t,
    szKey: *mut wchar_t,
    cbKey: c_ulong,
    grbit: JET_GRBIT,
    ulDensity: c_ulong,
    pidxunicode: *mut JET_UNICODEINDEX2,
    u: JET_INDEXCREATE3_W_u,
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: c_ulong,
    err: JET_ERR,
    cbKeyMost: c_ulong,
    pSpacehints: *mut JET_SPACEHINTS,
}}
UNION!{union JET_INDEXCREATE3_W_u {
    [usize; 1],
    cbVarSegMac cbVarSegMac_mut: c_ulong,
    ptuplelimits ptuplelimits_mut: *mut JET_TUPLELIMITS,
}}
STRUCT!{struct JET_INDEXRANGE {
    cbStruct: c_ulong,
    tableid: JET_TABLEID,
    grbit: JET_GRBIT,
}}
STRUCT!{struct JET_INSTANCE_INFO_W {
    hInstanceId: JET_INSTANCE,
    szInstanceName: *const wchar_t,
    cDatabases: JET_API_PTR,
    szDatabaseFileName: *const *const wchar_t,
    szDatabaseDisplayName: *const *const wchar_t,
    szDatabaseSLVFileName_Obsolete: *const *const wchar_t,
}}
STRUCT!{#[repr(packed)] struct JET_LGPOS {
    ib: c_ushort,
    isec: c_ushort,
    lGeneration: c_long,
}}
STRUCT!{struct JET_LOGINFO_W {
    cbSize: c_ulong,
    ulGenLow: c_ulong,
    ulGenHigh: c_ulong,
    szBaseName: [wchar_t; JET_BASE_NAME_LENGTH + 1],
}}
STRUCT!{#[repr(packed)] struct JET_LOGTIME {
    bSeconds: u8,
    bMinutes: u8,
    bHours: u8,
    bDay: u8,
    bMonth: u8,
    bYear: u8,
    bFiller1: u8,
    bFiller2: u8,
}}
BITFIELD!(JET_LOGTIME bFiller1: u8 [
    fTimeIsUTC set_fTimeIsUTC [0..1],
    bMillisecondsLow set_bMillisecondsLow [1..8],
]);
BITFIELD!(JET_LOGTIME bFiller2: u8 [
    fReserved set_fReserved [0..1],
    bMillisecondsHigh set_bMillisecondsHigh [1..4],
    fUnused set_fUnused [4..8],
]);
STRUCT!{struct JET_OPENTEMPORARYTABLE {
    cbStruct: c_ulong,
    prgcolumndef: *const JET_COLUMNDEF,
    ccolumn: c_ulong,
    pidxunicode: *mut JET_UNICODEINDEX,
    grbit: JET_GRBIT,
    prgcolumnid: *mut JET_COLUMNID,
    cbKeyMost: c_ulong,
    cbVarSegMac: c_ulong,
    tableid: JET_TABLEID,
}}
STRUCT!{struct JET_OPENTEMPORARYTABLE2 {
    cbStruct: c_ulong,
    prgcolumndef: *const JET_COLUMNDEF,
    ccolumn: c_ulong,
    pidxunicode: *mut JET_UNICODEINDEX2,
    grbit: JET_GRBIT,
    prgcolumnid: *mut JET_COLUMNID,
    cbKeyMost: c_ulong,
    cbVarSegMac: c_ulong,
    tableid: JET_TABLEID,
}}
STRUCT!{struct JET_RECORDLIST {
    cbStruct: c_ulong,
    tableid: JET_TABLEID,
    cRecord: c_ulong,
    columnidBookmark: JET_COLUMNID,
}}
STRUCT!{struct JET_RECPOS {
    cbStruct: c_ulong,
    centriesLT: c_ulong,
    centriesInRange: c_ulong,
    centriesTotal: c_ulong,
}}
STRUCT!{struct JET_RECSIZE {
    cbData: __int64,
    cbLongValueData: __int64,
    cbOverhead: __int64,
    cbLongValueOverhead: __int64,
    cNonTaggedColumns: __int64,
    cTaggedColumns: __int64,
    cLongValues: __int64,
    cMultiValues: __int64,
}}
STRUCT!{struct JET_RECSIZE2 {
    cbData: __int64,
    cbLongValueData: __int64,
    cbOverhead: __int64,
    cbLongValueOverhead: __int64,
    cNonTaggedColumns: __int64,
    cTaggedColumns: __int64,
    cLongValues: __int64,
    cMultiValues: __int64,
    cCompressedColumns: __int64,
    cbDataCompressed: __int64,
    cbLongValueDataCompressed: __int64,
}}
STRUCT!{struct JET_RETINFO {
    cbStruct: c_ulong,
    ibLongValue: c_ulong,
    itagSequence: c_ulong,
    columnidNextTagged: JET_COLUMNID,
}}
STRUCT!{struct JET_RETRIEVECOLUMN {
    columnid: JET_COLUMNID,
    pvData: *mut c_void,
    cbData: c_ulong,
    cbActual: c_ulong,
    grbit: JET_GRBIT,
    ibLongValue: c_ulong,
    itagSequence: c_ulong,
    columnidNextTagged: JET_COLUMNID,
    err: JET_ERR,
}}
STRUCT!{struct JET_RSTMAP_W {
    szDatabaseName: *mut wchar_t,
    szNewDatabaseName: *mut wchar_t,
}}
STRUCT!{struct JET_RSTINFO_W {
    cbStruct: c_ulong,
    rgrstmap: *mut JET_RSTMAP_W,
    crstmap: c_long,
    lgposStop: JET_LGPOS,
    logtimeStop: JET_LOGTIME,
    pfnStatus: JET_PFNSTATUS,
}}
STRUCT!{struct JET_SETCOLUMN {
    columnid: JET_COLUMNID,
    pvData: *mut c_void,
    cbData: c_ulong,
    grbit: JET_GRBIT,
    ibLongValue: c_ulong,
    itagSequence: c_ulong,
    err: JET_ERR,
}}
STRUCT!{struct JET_SETINFO {
    cbStruct: c_ulong,
    ibLongValue: c_ulong,
    itagSequence: c_ulong,
}}
STRUCT!{struct JET_SETSYSPARAM_W {
    paramid: c_ulong,
    lParam: JET_API_PTR,
    sz: *const wchar_t,
    err: JET_ERR,
}}
STRUCT!{struct JET_SIGNATURE {
    ulRandom: c_ulong,
    logtimeCreate: JET_LOGTIME,
    szComputerName: [u8; JET_MAX_COMPUTERNAME_LENGTH + 1],
}}
STRUCT!{struct JET_SPACEHINTS {
    cbStruct: c_ulong,
    ulInitialDensity: c_ulong,
    cbInitial: c_ulong,
    grbit: JET_GRBIT,
    ulMaintDensity: c_ulong,
    ulGrowth: c_ulong,
    cbMinExtent: c_ulong,
    cbMaxExtent: c_ulong,
}}
STRUCT!{struct JET_TABLECREATE_W {
    cbStruct: c_ulong,
    szTableName: *mut wchar_t,
    szTemplateTableName: *mut wchar_t,
    ulPages: c_ulong,
    ulDensity: c_ulong,
    rgcolumncreate: *const JET_COLUMNCREATE_W,
    cColumns: c_ulong,
    rgindexcreate: *const JET_INDEXCREATE_W,
    cIndexes: c_ulong,
    grbit: JET_GRBIT,
    tableid: JET_TABLEID,
    cCreated: c_ulong,
}}
STRUCT!{struct JET_TABLECREATE2_W {
    cbStruct: c_ulong,
    szTableName: *mut wchar_t,
    szTemplateTableName: *mut wchar_t,
    ulPages: c_ulong,
    ulDensity: c_ulong,
    rgcolumncreate: *const JET_COLUMNCREATE_W,
    cColumns: c_ulong,
    rgindexcreate: *const JET_INDEXCREATE_W,
    cIndexes: c_ulong,
    szCallback: *mut wchar_t,
    cbtyp: JET_CBTYP,
    grbit: JET_GRBIT,
    tableid: JET_TABLEID,
    cCreated: c_ulong,
}}
STRUCT!{struct JET_TABLECREATE3_W {
    cbStruct: c_ulong,
    szTableName: *mut wchar_t,
    szTemplateTableName: *mut wchar_t,
    ulPages: c_ulong,
    ulDensity: c_ulong,
    rgcolumncreate: *mut JET_COLUMNCREATE_W,
    cColumns: c_ulong,
    rgindexcreate: *mut JET_INDEXCREATE_W,
    cIndexes: c_ulong,
    szCallback: *mut wchar_t,
    cbtyp: JET_CBTYP,
    grbit: JET_GRBIT,
    pSeqSpacehints: *mut JET_SPACEHINTS,
    pLVSpacehints: *mut JET_SPACEHINTS,
    cbSeparateLV: c_ulong,
    tableid: JET_TABLEID,
    cCreated: c_ulong,
}}
STRUCT!{struct JET_TUPLELIMITS {
    chLengthMin: c_ulong,
    chLengthMax: c_ulong,
    chToIndexMax: c_ulong,
    cchIncrement: c_ulong,
    ichStart: c_ulong,
}}
STRUCT!{struct JET_UNICODEINDEX {
    lcid: c_ulong,
    dwMapFlags: c_ulong,
}}
STRUCT!{struct JET_UNICODEINDEX2 {
    szLocaleName: *mut wchar_t,
    dwMapFlags: c_ulong,
}}
pub const JET_MAX_COMPUTERNAME_LENGTH: usize = 15;
pub const JET_BASE_NAME_LENGTH: usize = 3;
pub const JET_ErrorInfoSpecificErr: c_ulong = 1;
pub const JET_relopEquals: JET_RELOP = 0;
pub const JET_relopPrefixEquals: JET_RELOP = 1;
pub const JET_relopNotEquals: JET_RELOP = 2;
pub const JET_relopLessThanOrEqual: JET_RELOP = 3;
pub const JET_relopLessThan: JET_RELOP = 4;
pub const JET_relopGreaterThanOrEqual: JET_RELOP = 5;
pub const JET_relopGreaterThan: JET_RELOP = 6;
pub const JET_relopBitmaskEqualsZero: JET_RELOP = 7;
pub const JET_relopBitmaskNotEqualsZero: JET_RELOP = 8;
pub const JET_errcatUnknown: JET_ERRCAT = 0;
pub const JET_errcatError: JET_ERRCAT = 1;
pub const JET_errcatOperation: JET_ERRCAT = 2;
pub const JET_errcatFatal: JET_ERRCAT = 3;
pub const JET_errcatIO: JET_ERRCAT = 4;
pub const JET_errcatResource: JET_ERRCAT = 5;
pub const JET_errcatMemory: JET_ERRCAT = 6;
pub const JET_errcatQuota: JET_ERRCAT = 7;
pub const JET_errcatDisk: JET_ERRCAT = 8;
pub const JET_errcatData: JET_ERRCAT = 9;
pub const JET_errcatCorruption: JET_ERRCAT = 10;
pub const JET_errcatInconsistent: JET_ERRCAT = 11;
pub const JET_errcatFragmentation: JET_ERRCAT = 12;
pub const JET_errcatApi: JET_ERRCAT = 13;
pub const JET_errcatUsage: JET_ERRCAT = 14;
pub const JET_errcatState: JET_ERRCAT = 15;
pub const JET_errcatObsolete: JET_ERRCAT = 16;
pub const JET_errcatMax: JET_ERRCAT = 17;
pub const JET_instanceNil: JET_INSTANCE = !0;
pub const JET_sesidNil: JET_SESID = !0;
pub const JET_tableidNil: JET_TABLEID = !0;
pub const JET_bitNil: JET_GRBIT = 0;
pub const JET_LSNil: JET_LS = !0;
pub const JET_dbidNil: JET_DBID = 0xFFFFFFFF;
pub const JET_cbtypNull: JET_CBTYP = 0x00000000;
pub const JET_cbtypFinalize: JET_CBTYP = 0x00000001;
pub const JET_cbtypBeforeInsert: JET_CBTYP = 0x00000002;
pub const JET_cbtypAfterInsert: JET_CBTYP = 0x00000004;
pub const JET_cbtypBeforeReplace: JET_CBTYP = 0x00000008;
pub const JET_cbtypAfterReplace: JET_CBTYP = 0x00000010;
pub const JET_cbtypBeforeDelete: JET_CBTYP = 0x00000020;
pub const JET_cbtypAfterDelete: JET_CBTYP = 0x00000040;
pub const JET_cbtypUserDefinedDefaultValue: JET_CBTYP = 0x00000080;
pub const JET_cbtypOnlineDefragCompleted: JET_CBTYP = 0x00000100;
pub const JET_cbtypFreeCursorLS: JET_CBTYP = 0x00000200;
pub const JET_cbtypFreeTableLS: JET_CBTYP = 0x00000400;
pub const JET_coltypNil: JET_COLTYP = 0;
pub const JET_coltypBit: JET_COLTYP = 1;
pub const JET_coltypUnsignedByte: JET_COLTYP = 2;
pub const JET_coltypShort: JET_COLTYP = 3;
pub const JET_coltypLong: JET_COLTYP = 4;
pub const JET_coltypCurrency: JET_COLTYP = 5;
pub const JET_coltypIEEESingle: JET_COLTYP = 6;
pub const JET_coltypIEEEDouble: JET_COLTYP = 7;
pub const JET_coltypDateTime: JET_COLTYP = 8;
pub const JET_coltypBinary: JET_COLTYP = 9;
pub const JET_coltypText: JET_COLTYP = 10;
pub const JET_coltypLongBinary: JET_COLTYP = 11;
pub const JET_coltypLongText: JET_COLTYP = 12;
pub const JET_coltypSLV: JET_COLTYP = 13;
pub const JET_coltypUnsignedLong: JET_COLTYP = 14;
pub const JET_coltypLongLong: JET_COLTYP = 15;
pub const JET_coltypGUID: JET_COLTYP = 16;
pub const JET_coltypUnsignedShort: JET_COLTYP = 17;
pub const JET_coltypUnsignedLongLong: JET_COLTYP = 18;
pub const JET_coltypMax: JET_COLTYP = 19;
pub const JET_objtypNil: JET_OBJTYP = 0;
pub const JET_objtypTable: JET_OBJTYP = 1;
pub const JET_snpRepair: JET_SNP = 2;
pub const JET_snpCompact: JET_SNP = 4;
pub const JET_snpRestore: JET_SNP = 8;
pub const JET_snpBackup: JET_SNP = 9;
pub const JET_snpUpgrade: JET_SNP = 10;
pub const JET_snpScrub: JET_SNP = 11;
pub const JET_snpUpgradeRecordFormat: JET_SNP = 12;
pub const JET_sntBegin: JET_SNT = 5;
pub const JET_sntRequirements: JET_SNT = 7;
pub const JET_sntProgress: JET_SNT = 0;
pub const JET_sntComplete: JET_SNT = 6;
pub const JET_sntFail: JET_SNT = 3;
pub const JET_ExceptionMsgBox: JET_API_PTR = 1;
pub const JET_ExceptionNone: JET_API_PTR = 2;
pub const JET_EventLoggingDisable: JET_API_PTR = 0;
pub const JET_EventLoggingLevelMax: JET_API_PTR = 100;
pub type ParamId = c_ulong;
pub const JET_paramSystemPath: ParamId = 0;
pub const JET_paramTempPath: ParamId = 1;
pub const JET_paramLogFilePath: ParamId = 2;
pub const JET_paramBaseName: ParamId = 3;
pub const JET_paramEventSource: ParamId = 4;
pub const JET_paramMaxSessions: ParamId = 5;
pub const JET_paramMaxOpenTables: ParamId = 6;
pub const JET_paramPreferredMaxOpenTables: ParamId = 7;
pub const JET_paramMaxCursors: ParamId = 8;
pub const JET_paramMaxVerPages: ParamId = 9;
pub const JET_paramMaxTemporaryTables: ParamId = 10;
pub const JET_paramLogFileSize: ParamId = 11;
pub const JET_paramLogBuffers: ParamId = 12;
pub const JET_paramWaitLogFlush: ParamId = 13;
pub const JET_paramLogCheckpointPeriod: ParamId = 14;
pub const JET_paramLogWaitingUserMax: ParamId = 15;
pub const JET_paramCommitDefault: ParamId = 16;
pub const JET_paramCircularLog: ParamId = 17;
pub const JET_paramDbExtensionSize: ParamId = 18;
pub const JET_paramPageTempDBMin: ParamId = 19;
pub const JET_paramPageFragment: ParamId = 20;
// 21: unused
pub const JET_paramBatchIOBufferMax: ParamId = 22;
pub const JET_paramCacheSizeMax: ParamId = 23;
pub const JET_paramCheckpointDepthMax: ParamId = 24;
pub const JET_paramLRUKCorrInterval: ParamId = 25;
pub const JET_paramLRUKHistoryMax: ParamId = 26;
pub const JET_paramLRUKPolicy: ParamId = 27;
pub const JET_paramLRUKTimeout: ParamId = 28;
pub const JET_paramLRUKTrxCorrInterval: ParamId = 29;
pub const JET_paramOutstandingIOMax: ParamId = 30;
pub const JET_paramStartFlushThreshold: ParamId = 31;
pub const JET_paramStopFlushThreshold: ParamId = 32;
// 33: unused
pub const JET_paramRecovery: ParamId = 34;
pub const JET_paramEnableOnlineDefrag: ParamId = 35;
// 36-40: unused
pub const JET_paramCacheSize: ParamId = 41;
// 42-43: unused
pub const JET_paramCheckFormatWhenOpenFail: ParamId = 44;
pub const JET_paramEnableIndexChecking: ParamId = 45;
pub const JET_paramEnableTempTableVersioning: ParamId = 46;
pub const JET_paramIgnoreLogVersion: ParamId = 47;
pub const JET_paramDeleteOldLogs: ParamId = 48;
pub const JET_paramEventSourceKey: ParamId = 49;
pub const JET_paramNoInformationEvent: ParamId = 50;
pub const JET_paramEventLoggingLevel: ParamId = 51;
pub const JET_paramDeleteOutOfRangeLogs: ParamId = 52;
pub const JET_paramAccessDeniedRetryPeriod: ParamId = 53;
pub const JET_paramEnableIndexCleanup: ParamId = 54;
// 55-59: unused
pub const JET_paramCacheSizeMin: ParamId = 60;
// 61-62: unused
pub const JET_paramPreferredVerPages: ParamId = 63;
pub const JET_paramDatabasePageSize: ParamId = 64;
pub const JET_paramDisableCallbacks: ParamId = 65;
// 66-68: unused
pub const JET_paramLogFileCreateAsynch: ParamId = 69;
pub const JET_paramErrorToString: ParamId = 70;
pub const JET_paramZeroDatabaseDuringBackup: ParamId = 71;
pub const JET_paramUnicodeIndexDefault: ParamId = 72;
pub const JET_paramRuntimeCallback: ParamId = 73;
// 74-76: unused
pub const JET_paramCleanupMismatchedLogFiles: ParamId = 77;
pub const JET_paramRecordUpgradeDirtyLevel: ParamId = 78;
// 79-80: unused
pub const JET_paramGlobalMinVerPages: ParamId = 81;
pub const JET_paramOSSnapshotTimeout: ParamId = 82;
// 83-97: unused
pub const JET_paramExceptionAction: ParamId = 98;
pub const JET_paramEventLogCache: ParamId = 99;
pub const JET_paramCreatePathIfNotExist: ParamId = 100;
pub const JET_paramPageHintCacheSize: ParamId = 101;
pub const JET_paramOneDatabasePerSession: ParamId = 102;
// 103: unused
pub const JET_paramMaxInstances: ParamId = 104;
pub const JET_paramVersionStoreTaskQueueMax: ParamId = 105;
// 106: unused
pub const JET_paramDisablePerfmon: ParamId = 107;
// 108-109: unused
pub const JET_paramIndexTuplesLengthMin: ParamId = 110;
pub const JET_paramIndexTuplesLengthMax: ParamId = 111;
pub const JET_paramIndexTuplesToIndexMax: ParamId = 112;
pub const JET_paramAlternateDatabaseRecoveryPath: ParamId = 113;
// 114-124: unused
pub const JET_paramCachedClosedTables: ParamId = 125;
pub const JET_paramEnableFileCache: ParamId = 126;
pub const JET_paramEnableViewCache: ParamId = 127;
pub const JET_paramVerPageSize: ParamId = 128;
pub const JET_paramConfiguration: ParamId = 129;
pub const JET_paramEnableAdvanced: ParamId = 130;
pub const JET_paramMaxColtyp: ParamId = 131;
pub const JET_paramIndexTupleIncrement: ParamId = 132;
pub const JET_paramIndexTupleStart: ParamId = 133;
pub const JET_paramKeyMost: ParamId = 134;
pub const JET_paramCheckpointIOMax: ParamId = 135;
pub const JET_paramLegacyFileNames: ParamId = 136;
// 137-151: deprecated JET_paramTableClass{n}Name
pub const JET_paramIOPriority: ParamId = 152;
pub const JET_paramWaypointLatency: ParamId = 153;
// 154-155: unused
pub const JET_paramEnablePersistedCallbacks: ParamId = 156;
// 157-159: unused
pub const JET_paramDefragmentSequentialBTrees: ParamId = 160;
pub const JET_paramDefragmentSequentialBTreesDensityCheckFrequency: ParamId = 161;
pub const JET_paramIOThrottlingTimeQuanta: ParamId = 162;
pub const JET_paramLVChunkSizeMost: ParamId = 163;
pub const JET_paramMaxCoalesceReadSize: ParamId = 164;
pub const JET_paramMaxCoalesceWriteSize: ParamId = 165;
pub const JET_paramMaxCoalesceReadGapSize: ParamId = 166;
pub const JET_paramMaxCoalesceWriteGapSize: ParamId = 167;
// the following aren't documented on MSDN, but are in the public header:
pub const JET_paramEnableDBScanInRecovery: ParamId = 169;
pub const JET_paramDbScanThrottle: ParamId = 170;
pub const JET_paramDbScanIntervalMinSec: ParamId = 171;
pub const JET_paramDbScanIntervalMaxSec: ParamId = 172;
// 173-176: unused
pub const JET_paramCachePriority: ParamId = 177;
pub const JET_paramMaxTransactionSize: ParamId = 178;
pub const JET_paramPrereadIOMax: ParamId = 179;
pub const JET_paramEnableDBScanSerialization: ParamId = 180;
pub const JET_paramHungIOThreshold: ParamId = 181;
pub const JET_paramHungIOActions: ParamId = 182;
pub const JET_paramMinDataForXpress: ParamId = 183;
pub const JET_paramEnableShinkDatabase: ParamId = 184;
// 185: reserved
pub const JET_paramProcessFriendlyName: ParamId = 186;
pub const JET_paramDurableCommitCallback: ParamId = 187;
pub const JET_paramEnableSqm: ParamId = 188;
pub const JET_paramConfigStoreSpec: ParamId = 189;
// End of param flags
// Flags for JET_paramLegacyFileNames
pub const JET_bitESE98FileNames: JET_GRBIT = 0x00000001;
pub const JET_bitEightDotThreeSoftCompat: JET_GRBIT = 0x00000002;
// Flags for JET_paramHungIOActions
pub const JET_bitHungIOEvent: JET_GRBIT = 0x00000001;
// Values for JET_paramEnableShrinkDatabase.
pub const JET_bitShrinkDatabaseOff: JET_GRBIT = 0x0;
pub const JET_bitShrinkDatabaseOn: JET_GRBIT = 0x1;
pub const JET_bitShrinkDatabaseRealtime: JET_GRBIT = 0x2;
pub const JET_bitShrinkDatabaseTrim: JET_GRBIT = 0x1;
// Flags for JetInit2, JetInit3
pub const JET_bitReplayIgnoreMissingDB: JET_GRBIT = 0x00000004;
pub const JET_bitRecoveryWithoutUndo: JET_GRBIT = 0x00000008;
pub const JET_bitTruncateLogsAfterRecovery: JET_GRBIT = 0x00000010;
pub const JET_bitReplayMissingMapEntryDB: JET_GRBIT = 0x00000020;
pub const JET_bitLogStreamMustExist: JET_GRBIT = 0x00000040;
pub const JET_bitReplayIgnoreLostLogs: JET_GRBIT = 0x00000080;
pub const JET_bitKeepDbAttachedAtEndOfRecovery: JET_GRBIT = 0x00001000;
// Flags for JetTerm2
pub const JET_bitTermComplete: JET_GRBIT = 0x00000001;
pub const JET_bitTermAbrupt: JET_GRBIT = 0x00000002;
pub const JET_bitTermStopBackup: JET_GRBIT = 0x00000004;
pub const JET_bitTermDirty: JET_GRBIT = 0x00000008;
// Flags for JetIdle
pub const JET_bitIdleFlushBuffers: JET_GRBIT = 0x00000001;
pub const JET_bitIdleCompact: JET_GRBIT = 0x00000002;
pub const JET_bitIdleStatus: JET_GRBIT = 0x00000004;
// Flags for JetEndSession
// Flags for JetAttachDatabase/JetOpenDatabase
pub const JET_bitDbReadOnly: JET_GRBIT = 0x00000001;
pub const JET_bitDbExclusive: JET_GRBIT = 0x00000002;
pub const JET_bitDbDeleteCorruptIndexes: JET_GRBIT = 0x00000010;
pub const JET_bitDbDeleteUnicodeIndexes: JET_GRBIT = 0x00000400;
pub const JET_bitDbUpgrade: JET_GRBIT = 0x00000200;
pub const JET_bitDbEnableBackgroundMaintenance: JET_GRBIT = 0x00000800;
pub const JET_bitDbPurgeCacheOnAttach: JET_GRBIT = 0x00001000;
// Flags for JetDetachDatabase2
pub const JET_bitForceDetach: JET_GRBIT = 0x00000001;
pub const JET_bitForceCloseAndDetach: JET_GRBIT = (0x00000002 | JET_bitForceDetach);
// Flags for JetCreateDatabase
pub const JET_bitDbRecoveryOff: JET_GRBIT = 0x00000008;
pub const JET_bitDbShadowingOff: JET_GRBIT = 0x00000080;
pub const JET_bitDbOverwriteExisting: JET_GRBIT = 0x00000200;
// Flags for JetBackup, JetBeginExternalBackup, JetBeginExternalBackupInstance,
//  JetBeginSurrogateBackup
pub const JET_bitBackupIncremental: JET_GRBIT = 0x00000001;
pub const JET_bitBackupAtomic: JET_GRBIT = 0x00000004;
pub const JET_bitBackupSnapshot: JET_GRBIT = 0x00000010;
// Flags for JetEndExternalBackupInstance2, JetEndSurrogateBackup
pub const JET_bitBackupEndNormal: JET_GRBIT = 0x0001;
pub const JET_bitBackupEndAbort: JET_GRBIT = 0x0002;
pub const JET_bitBackupTruncateDone: JET_GRBIT = 0x0100;
// Flags for JetCreateTableColumnIndex
pub const JET_bitTableCreateFixedDDL: JET_GRBIT = 0x00000001;
pub const JET_bitTableCreateTemplateTable: JET_GRBIT = 0x00000002;
pub const JET_bitTableCreateNoFixedVarColumnsInDerivedTables: JET_GRBIT = 0x00000004;
pub const JET_bitTableCreateImmutableStructure: JET_GRBIT = 0x00000008;
// Flags for JetAddColumn, JetGetColumnInfo, JetOpenTempTable
pub const JET_bitColumnFixed: JET_GRBIT = 0x00000001;
pub const JET_bitColumnTagged: JET_GRBIT = 0x00000002;
pub const JET_bitColumnNotNULL: JET_GRBIT = 0x00000004;
pub const JET_bitColumnVersion: JET_GRBIT = 0x00000008;
pub const JET_bitColumnAutoincrement: JET_GRBIT = 0x00000010;
pub const JET_bitColumnUpdatable: JET_GRBIT = 0x00000020;
pub const JET_bitColumnTTKey: JET_GRBIT = 0x00000040;
pub const JET_bitColumnTTDescending: JET_GRBIT = 0x00000080;
pub const JET_bitColumnMultiValued: JET_GRBIT = 0x00000400;
pub const JET_bitColumnEscrowUpdate: JET_GRBIT = 0x00000800;
pub const JET_bitColumnUnversioned: JET_GRBIT = 0x00001000;
pub const JET_bitColumnMaybeNull: JET_GRBIT = 0x00002000;
pub const JET_bitColumnFinalize: JET_GRBIT = 0x00004000;
pub const JET_bitColumnUserDefinedDefault: JET_GRBIT = 0x00008000;
pub const JET_bitColumnDeleteOnZero: JET_GRBIT = 0x00020000;
pub const JET_bitColumnCompressed: JET_GRBIT = 0x00080000;
// flags for JetDeleteColumn
pub const JET_bitDeleteColumnIgnoreTemplateColumns: JET_GRBIT = 0x00000001;
// Flags for JetSetCurrentIndex
pub const JET_bitMoveFirst: JET_GRBIT = 0x00000000;
pub const JET_bitNoMove: JET_GRBIT = 0x00000002;
// Flags for JetMakeKey
pub const JET_bitNewKey: JET_GRBIT = 0x00000001;
pub const JET_bitStrLimit: JET_GRBIT = 0x00000002;
pub const JET_bitSubStrLimit: JET_GRBIT = 0x00000004;
pub const JET_bitNormalizedKey: JET_GRBIT = 0x00000008;
pub const JET_bitKeyDataZeroLength: JET_GRBIT = 0x00000010;
pub const JET_bitFullColumnStartLimit: JET_GRBIT = 0x00000100;
pub const JET_bitFullColumnEndLimit: JET_GRBIT = 0x00000200;
pub const JET_bitPartialColumnStartLimit: JET_GRBIT = 0x00000400;
pub const JET_bitPartialColumnEndLimit: JET_GRBIT = 0x00000800;
// Flags for JetSetIndexRange
pub const JET_bitRangeInclusive: JET_GRBIT = 0x00000001;
pub const JET_bitRangeUpperLimit: JET_GRBIT = 0x00000002;
pub const JET_bitRangeInstantDuration: JET_GRBIT = 0x00000004;
pub const JET_bitRangeRemove: JET_GRBIT = 0x00000008;
// Flags for JetGetLock
pub const JET_bitReadLock: JET_GRBIT = 0x00000001;
pub const JET_bitWriteLock: JET_GRBIT = 0x00000002;
// Constants for JetMove
pub const JET_MoveFirst: i32 = (-0x80000000);
pub const JET_MovePrevious: i32 = (-1);
pub const JET_MoveNext: i32 = (1);
pub const JET_MoveLast: i32 = (0x7fffffff);
// Flags for JetMove
pub const JET_bitMoveKeyNE: JET_GRBIT = 0x00000001;
// Flags for JetSeek
pub const JET_bitSeekEQ: JET_GRBIT = 0x00000001;
pub const JET_bitSeekLT: JET_GRBIT = 0x00000002;
pub const JET_bitSeekLE: JET_GRBIT = 0x00000004;
pub const JET_bitSeekGE: JET_GRBIT = 0x00000008;
pub const JET_bitSeekGT: JET_GRBIT = 0x00000010;
pub const JET_bitSetIndexRange: JET_GRBIT = 0x00000020;
pub const JET_bitCheckUniqueness: JET_GRBIT = 0x00000040;
// Flags for JetGotoSecondaryIndexBookmark
pub const JET_bitBookmarkPermitVirtualCurrency: JET_GRBIT = 0x00000001;
// Flags for JET_CONDITIONALCOLUMN
pub const JET_bitIndexColumnMustBeNull: JET_GRBIT = 0x00000001;
pub const JET_bitIndexColumnMustBeNonNull: JET_GRBIT = 0x00000002;
// Flags for JET_INDEXRANGE
pub const JET_bitRecordInIndex: JET_GRBIT = 0x00000001;
pub const JET_bitRecordNotInIndex: JET_GRBIT = 0x00000002;
// Flags for JetCreateIndex
pub const JET_bitIndexUnique: JET_GRBIT = 0x00000001;
pub const JET_bitIndexPrimary: JET_GRBIT = 0x00000002;
pub const JET_bitIndexDisallowNull: JET_GRBIT = 0x00000004;
pub const JET_bitIndexIgnoreNull: JET_GRBIT = 0x00000008;
pub const JET_bitIndexIgnoreAnyNull: JET_GRBIT = 0x00000020;
pub const JET_bitIndexIgnoreFirstNull: JET_GRBIT = 0x00000040;
pub const JET_bitIndexLazyFlush: JET_GRBIT = 0x00000080;
pub const JET_bitIndexEmpty: JET_GRBIT = 0x00000100;
pub const JET_bitIndexUnversioned: JET_GRBIT = 0x00000200;
pub const JET_bitIndexSortNullsHigh: JET_GRBIT = 0x00000400;
pub const JET_bitIndexUnicode: JET_GRBIT = 0x00000800;
pub const JET_bitIndexTuples: JET_GRBIT = 0x00001000;
pub const JET_bitIndexTupleLimits: JET_GRBIT = 0x00002000;
pub const JET_bitIndexCrossProduct: JET_GRBIT = 0x00004000;
pub const JET_bitIndexKeyMost: JET_GRBIT = 0x00008000;
pub const JET_bitIndexDisallowTruncation: JET_GRBIT = 0x00010000;
pub const JET_bitIndexNestedTable: JET_GRBIT = 0x00020000;
pub const JET_bitIndexDotNetGuid: JET_GRBIT = 0x00040000;
pub const JET_bitIndexImmutableStructure: JET_GRBIT = 0x00080000;
// Flags for index key definition
pub const JET_bitKeyAscending: JET_GRBIT = 0x00000000;
pub const JET_bitKeyDescending: JET_GRBIT = 0x00000001;
// Flags for JetOpenTable
pub const JET_bitTableDenyWrite: JET_GRBIT = 0x00000001;
pub const JET_bitTableDenyRead: JET_GRBIT = 0x00000002;
pub const JET_bitTableReadOnly: JET_GRBIT = 0x00000004;
pub const JET_bitTableUpdatable: JET_GRBIT = 0x00000008;
pub const JET_bitTablePermitDDL: JET_GRBIT = 0x00000010;
pub const JET_bitTableNoCache: JET_GRBIT = 0x00000020;
pub const JET_bitTablePreread: JET_GRBIT = 0x00000040;
pub const JET_bitTableOpportuneRead: JET_GRBIT = 0x00000080;
pub const JET_bitTableSequential: JET_GRBIT = 0x00008000;
pub const JET_bitTableClassMask: JET_GRBIT = 0x000F0000;
pub const JET_bitTableClassNone: JET_GRBIT = 0x00000000;
pub const JET_bitTableClass1: JET_GRBIT = 0x00010000;
pub const JET_bitTableClass2: JET_GRBIT = 0x00020000;
pub const JET_bitTableClass3: JET_GRBIT = 0x00030000;
pub const JET_bitTableClass4: JET_GRBIT = 0x00040000;
pub const JET_bitTableClass5: JET_GRBIT = 0x00050000;
pub const JET_bitTableClass6: JET_GRBIT = 0x00060000;
pub const JET_bitTableClass7: JET_GRBIT = 0x00070000;
pub const JET_bitTableClass8: JET_GRBIT = 0x00080000;
pub const JET_bitTableClass9: JET_GRBIT = 0x00090000;
pub const JET_bitTableClass10: JET_GRBIT = 0x000A0000;
pub const JET_bitTableClass11: JET_GRBIT = 0x000B0000;
pub const JET_bitTableClass12: JET_GRBIT = 0x000C0000;
pub const JET_bitTableClass13: JET_GRBIT = 0x000D0000;
pub const JET_bitTableClass14: JET_GRBIT = 0x000E0000;
pub const JET_bitTableClass15: JET_GRBIT = 0x000F0000;
pub const JET_bitLSReset: JET_GRBIT = 0x00000001;
pub const JET_bitLSCursor: JET_GRBIT = 0x00000002;
pub const JET_bitLSTable: JET_GRBIT = 0x00000004;
// Flags for JetSetTableSequential and JetPrereadIndexRanges
pub const JET_bitPrereadForward: JET_GRBIT = 0x00000001;
pub const JET_bitPrereadBackward: JET_GRBIT = 0x00000002;
pub const JET_bitPrereadFirstPage: JET_GRBIT = 0x00000004;
pub const JET_bitPrereadNormalizedKey: JET_GRBIT = 0x00000008;
// Flags for JetOpenTempTable
pub const JET_bitTTIndexed: JET_GRBIT = 0x00000001;
pub const JET_bitTTUnique: JET_GRBIT = 0x00000002;
pub const JET_bitTTUpdatable: JET_GRBIT = 0x00000004;
pub const JET_bitTTScrollable: JET_GRBIT = 0x00000008;
pub const JET_bitTTSortNullsHigh: JET_GRBIT = 0x00000010;
pub const JET_bitTTForceMaterialization: JET_GRBIT = 0x00000020;
pub const JET_bitTTErrorOnDuplicateInsertion: JET_GRBIT = JET_bitTTForceMaterialization;
pub const JET_bitTTForwardOnly: JET_GRBIT = 0x00000040;
pub const JET_bitTTIntrinsicLVsOnly: JET_GRBIT = 0x00000080;
pub const JET_bitTTDotNetGuid: JET_GRBIT = 0x00000100;
// Flags for JetSetColumn
pub const JET_bitSetAppendLV: JET_GRBIT = 0x00000001;
pub const JET_bitSetOverwriteLV: JET_GRBIT = 0x00000004;
pub const JET_bitSetSizeLV: JET_GRBIT = 0x00000008;
pub const JET_bitSetZeroLength: JET_GRBIT = 0x00000020;
pub const JET_bitSetSeparateLV: JET_GRBIT = 0x00000040;
pub const JET_bitSetUniqueMultiValues: JET_GRBIT = 0x00000080;
pub const JET_bitSetUniqueNormalizedMultiValues: JET_GRBIT = 0x00000100;
pub const JET_bitSetRevertToDefaultValue: JET_GRBIT = 0x00000200;
pub const JET_bitSetIntrinsicLV: JET_GRBIT = 0x00000400;
pub const JET_bitSetCompressed: JET_GRBIT = 0x00020000;
pub const JET_bitSetUncompressed: JET_GRBIT = 0x00010000;
// Space Hint Flags / JET_SPACEHINTS
// Generic
pub const JET_bitSpaceHintsUtilizeParentSpace: JET_GRBIT = 0x00000001;
// Create
pub const JET_bitCreateHintAppendSequential: JET_GRBIT = 0x00000002;
pub const JET_bitCreateHintHotpointSequential: JET_GRBIT = 0x00000004;
// Retrieve
pub const JET_bitRetrieveHintReserve1: JET_GRBIT = 0x00000008;
pub const JET_bitRetrieveHintTableScanForward: JET_GRBIT = 0x00000010;
pub const JET_bitRetrieveHintTableScanBackward: JET_GRBIT = 0x00000020;
pub const JET_bitRetrieveHintReserve2: JET_GRBIT = 0x00000040;
pub const JET_bitRetrieveHintReserve3: JET_GRBIT = 0x00000080;
// Delete / .grbitDelete
pub const JET_bitDeleteHintTableSequential: JET_GRBIT = 0x00000100;
// Options for JetPrepareUpdate
pub const JET_prepInsert: c_ulong = 0;
pub const JET_prepReplace: c_ulong = 2;
pub const JET_prepCancel: c_ulong = 3;
pub const JET_prepReplaceNoLock: c_ulong = 4;
pub const JET_prepInsertCopy: c_ulong = 5;
pub const JET_prepInsertCopyDeleteOriginal: c_ulong = 7;
pub const JET_prepInsertCopyReplaceOriginal: c_ulong = 9;
// Flags for JetUpdate
pub const JET_bitUpdateCheckESE97Compatibility: JET_GRBIT = 0x00000001;
// Flags for JetEscrowUpdate
pub const JET_bitEscrowNoRollback: JET_GRBIT = 0x0001;
// Flags for JetRetrieveColumn
pub const JET_bitRetrieveCopy: JET_GRBIT = 0x00000001;
pub const JET_bitRetrieveFromIndex: JET_GRBIT = 0x00000002;
pub const JET_bitRetrieveFromPrimaryBookmark: JET_GRBIT = 0x00000004;
pub const JET_bitRetrieveTag: JET_GRBIT = 0x00000008;
pub const JET_bitRetrieveNull: JET_GRBIT = 0x00000010;
pub const JET_bitRetrieveIgnoreDefault: JET_GRBIT = 0x00000020;
pub const JET_bitRetrieveLongId: JET_GRBIT = 0x00000040;
pub const JET_bitRetrieveLongValueRefCount: JET_GRBIT = 0x00000080;
pub const JET_bitRetrieveTuple: JET_GRBIT = 0x00000800;
// Flags for JET_INDEX_COLUMN
pub const JET_bitZeroLength: JET_GRBIT = 0x00000001;
// Flags for JetEnumerateColumns
pub const JET_bitEnumerateCopy: JET_GRBIT = JET_bitRetrieveCopy;
pub const JET_bitEnumerateIgnoreDefault: JET_GRBIT = JET_bitRetrieveIgnoreDefault;
pub const JET_bitEnumeratePresenceOnly: JET_GRBIT = 0x00020000;
pub const JET_bitEnumerateTaggedOnly: JET_GRBIT = 0x00040000;
pub const JET_bitEnumerateCompressOutput: JET_GRBIT = 0x00080000;
// Available on Server 2003 SP1
pub const JET_bitEnumerateIgnoreUserDefinedDefault: JET_GRBIT = 0x00100000;
pub const JET_bitEnumerateInRecordOnly: JET_GRBIT = 0x00200000;
// Flags for JetGetRecordSize
pub const JET_bitRecordSizeInCopyBuffer: JET_GRBIT = 0x00000001;
pub const JET_bitRecordSizeRunningTotal: JET_GRBIT = 0x00000002;
pub const JET_bitRecordSizeLocal: JET_GRBIT = 0x00000004;
// Flags for JetBeginTransaction2
pub const JET_bitTransactionReadOnly: JET_GRBIT = 0x00000001;
// Flags for JetCommitTransaction
pub const JET_bitCommitLazyFlush: JET_GRBIT = 0x00000001;
pub const JET_bitWaitLastLevel0Commit: JET_GRBIT = 0x00000002;
pub const JET_bitWaitAllLevel0Commit: JET_GRBIT = 0x00000008;
pub const JET_bitForceNewLog: JET_GRBIT = 0x00000010;
// Flags for JetRollback
pub const JET_bitRollbackAll: JET_GRBIT = 0x00000001;
// Flags for JetOSSnapshot APIs
// Flags for JetOSSnapshotPrepare
pub const JET_bitIncrementalSnapshot: JET_GRBIT = 0x00000001;
pub const JET_bitCopySnapshot: JET_GRBIT = 0x00000002;
pub const JET_bitContinueAfterThaw: JET_GRBIT = 0x00000004;
pub const JET_bitExplicitPrepare: JET_GRBIT = 0x00000008;
// Flags for JetOSSnapshotTruncateLog & JetOSSnapshotTruncateLogInstance
pub const JET_bitAllDatabasesSnapshot: JET_GRBIT = 0x00000001;
// Flags for JetOSSnapshotEnd
pub const JET_bitAbortSnapshot: JET_GRBIT = 0x00000001;
// grbits for JetResizeDatabase:
pub const JET_bitResizeDatabaseOnlyGrow: JET_GRBIT = 0x00000001;
pub const JET_bitResizeDatabaseOnlyShrink: JET_GRBIT = 0x00000002;
pub const JET_bitStopServiceAll: JET_GRBIT = 0x00000000;
pub const JET_bitStopServiceBackgroundUserTasks: JET_GRBIT = 0x00000002;
pub const JET_bitStopServiceQuiesceCaches: JET_GRBIT = 0x00000004;
pub const JET_bitStopServiceResume: JET_GRBIT = 0x80000000;
pub const JET_ColInfo: c_ulong = 0;
pub const JET_ColInfoList: c_ulong = 1;
pub const JET_ColInfoSysTabCursor: c_ulong = 3;
pub const JET_ColInfoBase: c_ulong = 4;
pub const JET_ColInfoListCompact: c_ulong = 5;
pub const JET_ColInfoByColid: c_ulong = 6;
pub const JET_ColInfoListSortColumnid: c_ulong = 7;
pub const JET_ColInfoBaseByColid: c_ulong = 8;
// Grbits for JET_GetColumnInfo and JetGetTableColumnInfo (OR together with the info level)
pub const JET_ColInfoGrbitNonDerivedColumnsOnly: c_ulong = 0x80000000;
pub const JET_ColInfoGrbitMinimalInfo: c_ulong = 0x40000000;
pub const JET_ColInfoGrbitSortByColumnid: c_ulong = 0x20000000;
// ERROR CODES:
// SUCCESS
pub const JET_errSuccess: JET_ERR = 0;
// ERRORS
pub const JET_wrnNyi: JET_ERR = -1;
// SYSTEM errors
pub const JET_errRfsFailure: JET_ERR = -100;
pub const JET_errRfsNotArmed: JET_ERR = -101;
pub const JET_errFileClose: JET_ERR = -102;
pub const JET_errOutOfThreads: JET_ERR = -103;
pub const JET_errTooManyIO: JET_ERR = -105;
pub const JET_errTaskDropped: JET_ERR = -106;
pub const JET_errInternalError: JET_ERR = -107;
pub const JET_errDisabledFunctionality: JET_ERR = -112;
pub const JET_errUnloadableOSFunctionality: JET_ERR = -113;
// BUFFER MANAGER errors
pub const JET_errDatabaseBufferDependenciesCorrupted: JET_ERR = -255;
// DIRECTORY MANAGER errors
pub const JET_wrnRemainingVersions: JET_ERR = 321;
pub const JET_errPreviousVersion: JET_ERR = -322;
pub const JET_errPageBoundary: JET_ERR = -323;
pub const JET_errKeyBoundary: JET_ERR = -324;
pub const JET_errBadPageLink: JET_ERR = -327;
pub const JET_errBadBookmark: JET_ERR = -328;
pub const JET_errNTSystemCallFailed: JET_ERR = -334;
pub const JET_errBadParentPageLink: JET_ERR = -338;
pub const JET_errSPAvailExtCacheOutOfSync: JET_ERR = -340;
pub const JET_errSPAvailExtCorrupted: JET_ERR = -341;
pub const JET_errSPAvailExtCacheOutOfMemory: JET_ERR = -342;
pub const JET_errSPOwnExtCorrupted: JET_ERR = -343;
pub const JET_errDbTimeCorrupted: JET_ERR = -344;
pub const JET_wrnUniqueKey: JET_ERR = 345;
pub const JET_errKeyTruncated: JET_ERR = -346;
pub const JET_errDatabaseLeakInSpace: JET_ERR = -348;
pub const JET_errBadEmptyPage: JET_ERR = -351;
// RECORD MANAGER errors
pub const JET_wrnSeparateLongValue: JET_ERR = 406;
pub const JET_wrnRecordFoundGreater: JET_ERR = JET_wrnSeekNotEqual;
pub const JET_wrnRecordFoundLess: JET_ERR = JET_wrnSeekNotEqual;
pub const JET_errColumnIllegalNull: JET_ERR = JET_errNullInvalid;
pub const JET_errKeyTooBig: JET_ERR = -408;
pub const JET_errCannotSeparateIntrinsicLV: JET_ERR = -416;
pub const JET_errSeparatedLongValue: JET_ERR = -421;
pub const JET_errMustBeSeparateLongValue: JET_ERR = -423;
pub const JET_errInvalidPreread: JET_ERR = -424;
// LOGGING/RECOVERY errors
pub const JET_errInvalidLoggedOperation: JET_ERR = -500;
pub const JET_errLogFileCorrupt: JET_ERR = -501;
pub const JET_errNoBackupDirectory: JET_ERR = -503;
pub const JET_errBackupDirectoryNotEmpty: JET_ERR = -504;
pub const JET_errBackupInProgress: JET_ERR = -505;
pub const JET_errRestoreInProgress: JET_ERR = -506;
pub const JET_errMissingPreviousLogFile: JET_ERR = -509;
pub const JET_errLogWriteFail: JET_ERR = -510;
pub const JET_errLogDisabledDueToRecoveryFailure: JET_ERR = -511;
pub const JET_errCannotLogDuringRecoveryRedo: JET_ERR = -512;
pub const JET_errLogGenerationMismatch: JET_ERR = -513;
pub const JET_errBadLogVersion: JET_ERR = -514;
pub const JET_errInvalidLogSequence: JET_ERR = -515;
pub const JET_errLoggingDisabled: JET_ERR = -516;
pub const JET_errLogBufferTooSmall: JET_ERR = -517;
pub const JET_errLogSequenceEnd: JET_ERR = -519;
pub const JET_errNoBackup: JET_ERR = -520;
pub const JET_errInvalidBackupSequence: JET_ERR = -521;
pub const JET_errBackupNotAllowedYet: JET_ERR = -523;
pub const JET_errDeleteBackupFileFail: JET_ERR = -524;
pub const JET_errMakeBackupDirectoryFail: JET_ERR = -525;
pub const JET_errInvalidBackup: JET_ERR = -526;
pub const JET_errRecoveredWithErrors: JET_ERR = -527;
pub const JET_errMissingLogFile: JET_ERR = -528;
pub const JET_errLogDiskFull: JET_ERR = -529;
pub const JET_errBadLogSignature: JET_ERR = -530;
pub const JET_errBadDbSignature: JET_ERR = -531;
pub const JET_errBadCheckpointSignature: JET_ERR = -532;
pub const JET_errCheckpointCorrupt: JET_ERR = -533;
pub const JET_errMissingPatchPage: JET_ERR = -534;
pub const JET_errBadPatchPage: JET_ERR = -535;
pub const JET_errRedoAbruptEnded: JET_ERR = -536;
pub const JET_errPatchFileMissing: JET_ERR = -538;
pub const JET_errDatabaseLogSetMismatch: JET_ERR = -539;
pub const JET_errDatabaseStreamingFileMismatch: JET_ERR = -540;
pub const JET_errLogFileSizeMismatch: JET_ERR = -541;
pub const JET_errCheckpointFileNotFound: JET_ERR = -542;
pub const JET_errRequiredLogFilesMissing: JET_ERR = -543;
pub const JET_errSoftRecoveryOnBackupDatabase: JET_ERR = -544;
pub const JET_errLogFileSizeMismatchDatabasesConsistent: JET_ERR = -545;
pub const JET_errLogSectorSizeMismatch: JET_ERR = -546;
pub const JET_errLogSectorSizeMismatchDatabasesConsistent: JET_ERR = -547;
pub const JET_errLogSequenceEndDatabasesConsistent: JET_ERR = -548;
pub const JET_errStreamingDataNotLogged: JET_ERR = -549;
pub const JET_errDatabaseDirtyShutdown: JET_ERR = -550;
pub const JET_errDatabaseInconsistent: JET_ERR = JET_errDatabaseDirtyShutdown;
pub const JET_errConsistentTimeMismatch: JET_ERR = -551;
pub const JET_errDatabasePatchFileMismatch: JET_ERR = -552;
pub const JET_errEndingRestoreLogTooLow: JET_ERR = -553;
pub const JET_errStartingRestoreLogTooHigh: JET_ERR = -554;
pub const JET_errGivenLogFileHasBadSignature: JET_ERR = -555;
pub const JET_errGivenLogFileIsNotContiguous: JET_ERR = -556;
pub const JET_errMissingRestoreLogFiles: JET_ERR = -557;
pub const JET_wrnExistingLogFileHasBadSignature: JET_ERR = 558;
pub const JET_wrnExistingLogFileIsNotContiguous: JET_ERR = 559;
pub const JET_errMissingFullBackup: JET_ERR = -560;
pub const JET_errBadBackupDatabaseSize: JET_ERR = -561;
pub const JET_errDatabaseAlreadyUpgraded: JET_ERR = -562;
pub const JET_errDatabaseIncompleteUpgrade: JET_ERR = -563;
pub const JET_wrnSkipThisRecord: JET_ERR = 564;
pub const JET_errMissingCurrentLogFiles: JET_ERR = -565;
pub const JET_errDbTimeTooOld: JET_ERR = -566;
pub const JET_errDbTimeTooNew: JET_ERR = -567;
pub const JET_errMissingFileToBackup: JET_ERR = -569;
pub const JET_errLogTornWriteDuringHardRestore: JET_ERR = -570;
pub const JET_errLogTornWriteDuringHardRecovery: JET_ERR = -571;
pub const JET_errLogCorruptDuringHardRestore: JET_ERR = -573;
pub const JET_errLogCorruptDuringHardRecovery: JET_ERR = -574;
pub const JET_errMustDisableLoggingForDbUpgrade: JET_ERR = -575;
pub const JET_errBadRestoreTargetInstance: JET_ERR = -577;
pub const JET_wrnTargetInstanceRunning: JET_ERR = 578;
pub const JET_errRecoveredWithoutUndo: JET_ERR = -579;
pub const JET_errDatabasesNotFromSameSnapshot: JET_ERR = -580;
pub const JET_errSoftRecoveryOnSnapshot: JET_ERR = -581;
pub const JET_errCommittedLogFilesMissing: JET_ERR = -582;
pub const JET_errSectorSizeNotSupported: JET_ERR = -583;
pub const JET_errRecoveredWithoutUndoDatabasesConsistent: JET_ERR = -584;
pub const JET_wrnCommittedLogFilesLost: JET_ERR = 585;
pub const JET_errCommittedLogFileCorrupt: JET_ERR = -586;
pub const JET_wrnCommittedLogFilesRemoved: JET_ERR = 587;
pub const JET_wrnFinishWithUndo: JET_ERR = 588;
pub const JET_wrnDatabaseRepaired: JET_ERR = 595;
pub const JET_errUnicodeTranslationBufferTooSmall: JET_ERR = -601;
pub const JET_errUnicodeTranslationFail: JET_ERR = -602;
pub const JET_errUnicodeNormalizationNotSupported: JET_ERR = -603;
pub const JET_errUnicodeLanguageValidationFailure: JET_ERR = -604;
pub const JET_errExistingLogFileHasBadSignature: JET_ERR = -610;
pub const JET_errExistingLogFileIsNotContiguous: JET_ERR = -611;
pub const JET_errLogReadVerifyFailure: JET_ERR = -612;
pub const JET_errCheckpointDepthTooDeep: JET_ERR = -614;
pub const JET_errRestoreOfNonBackupDatabase: JET_ERR = -615;
pub const JET_errLogFileNotCopied: JET_ERR = -616;
pub const JET_errTransactionTooLong: JET_ERR = -618;
pub const JET_errEngineFormatVersionNoLongerSupportedTooLow: JET_ERR = -619;
pub const JET_errEngineFormatVersionNotYetImplementedTooHigh: JET_ERR = -620;
pub const JET_errEngineFormatVersionParamTooLowForRequestedFeature: JET_ERR = -621;
pub const JET_errEngineFormatVersionSpecifiedTooLowForLogVersion: JET_ERR = -622;
pub const JET_errEngineFormatVersionSpecifiedTooLowForDatabaseVersion: JET_ERR = -623;
pub const JET_errBackupAbortByServer: JET_ERR = -801;
pub const JET_errInvalidGrbit: JET_ERR = -900;
pub const JET_errTermInProgress: JET_ERR = -1000;
pub const JET_errFeatureNotAvailable: JET_ERR = -1001;
pub const JET_errInvalidName: JET_ERR = -1002;
pub const JET_errInvalidParameter: JET_ERR = -1003;
pub const JET_wrnColumnNull: JET_ERR = 1004;
pub const JET_wrnBufferTruncated: JET_ERR = 1006;
pub const JET_wrnDatabaseAttached: JET_ERR = 1007;
pub const JET_errDatabaseFileReadOnly: JET_ERR = -1008;
pub const JET_wrnSortOverflow: JET_ERR = 1009;
pub const JET_errInvalidDatabaseId: JET_ERR = -1010;
pub const JET_errOutOfMemory: JET_ERR = -1011;
pub const JET_errOutOfDatabaseSpace: JET_ERR = -1012;
pub const JET_errOutOfCursors: JET_ERR = -1013;
pub const JET_errOutOfBuffers: JET_ERR = -1014;
pub const JET_errTooManyIndexes: JET_ERR = -1015;
pub const JET_errTooManyKeys: JET_ERR = -1016;
pub const JET_errRecordDeleted: JET_ERR = -1017;
pub const JET_errReadVerifyFailure: JET_ERR = -1018;
pub const JET_errPageNotInitialized: JET_ERR = -1019;
pub const JET_errOutOfFileHandles: JET_ERR = -1020;
pub const JET_errDiskReadVerificationFailure: JET_ERR = -1021;
pub const JET_errDiskIO: JET_ERR = -1022;
pub const JET_errInvalidPath: JET_ERR = -1023;
pub const JET_errInvalidSystemPath: JET_ERR = -1024;
pub const JET_errInvalidLogDirectory: JET_ERR = -1025;
pub const JET_errRecordTooBig: JET_ERR = -1026;
pub const JET_errTooManyOpenDatabases: JET_ERR = -1027;
pub const JET_errInvalidDatabase: JET_ERR = -1028;
pub const JET_errNotInitialized: JET_ERR = -1029;
pub const JET_errAlreadyInitialized: JET_ERR = -1030;
pub const JET_errInitInProgress: JET_ERR = -1031;
pub const JET_errFileAccessDenied: JET_ERR = -1032;
pub const JET_errBufferTooSmall: JET_ERR = -1038;
pub const JET_wrnSeekNotEqual: JET_ERR = 1039;
pub const JET_errTooManyColumns: JET_ERR = -1040;
pub const JET_errContainerNotEmpty: JET_ERR = -1043;
pub const JET_errInvalidFilename: JET_ERR = -1044;
pub const JET_errInvalidBookmark: JET_ERR = -1045;
pub const JET_errColumnInUse: JET_ERR = -1046;
pub const JET_errInvalidBufferSize: JET_ERR = -1047;
pub const JET_errColumnNotUpdatable: JET_ERR = -1048;
pub const JET_errIndexInUse: JET_ERR = -1051;
pub const JET_errLinkNotSupported: JET_ERR = -1052;
pub const JET_errNullKeyDisallowed: JET_ERR = -1053;
pub const JET_errNotInTransaction: JET_ERR = -1054;
pub const JET_wrnNoErrorInfo: JET_ERR = 1055;
pub const JET_errMustRollback: JET_ERR = -1057;
pub const JET_wrnNoIdleActivity: JET_ERR = 1058;
pub const JET_errTooManyActiveUsers: JET_ERR = -1059;
pub const JET_errInvalidCountry: JET_ERR = -1061;
pub const JET_errInvalidLanguageId: JET_ERR = -1062;
pub const JET_errInvalidCodePage: JET_ERR = -1063;
pub const JET_errInvalidLCMapStringFlags: JET_ERR = -1064;
pub const JET_errVersionStoreEntryTooBig: JET_ERR = -1065;
pub const JET_errVersionStoreOutOfMemoryAndCleanupTimedOut: JET_ERR = -1066;
pub const JET_wrnNoWriteLock: JET_ERR = 1067;
pub const JET_wrnColumnSetNull: JET_ERR = 1068;
pub const JET_errVersionStoreOutOfMemory: JET_ERR = -1069;
pub const JET_errCannotIndex: JET_ERR = -1071;
pub const JET_errRecordNotDeleted: JET_ERR = -1072;
pub const JET_errTooManyMempoolEntries: JET_ERR = -1073;
pub const JET_errOutOfObjectIDs: JET_ERR = -1074;
pub const JET_errOutOfLongValueIDs: JET_ERR = -1075;
pub const JET_errOutOfAutoincrementValues: JET_ERR = -1076;
pub const JET_errOutOfDbtimeValues: JET_ERR = -1077;
pub const JET_errOutOfSequentialIndexValues: JET_ERR = -1078;
pub const JET_errRunningInOneInstanceMode: JET_ERR = -1080;
pub const JET_errRunningInMultiInstanceMode: JET_ERR = -1081;
pub const JET_errSystemParamsAlreadySet: JET_ERR = -1082;
pub const JET_errSystemPathInUse: JET_ERR = -1083;
pub const JET_errLogFilePathInUse: JET_ERR = -1084;
pub const JET_errTempPathInUse: JET_ERR = -1085;
pub const JET_errInstanceNameInUse: JET_ERR = -1086;
pub const JET_errSystemParameterConflict: JET_ERR = -1087;
pub const JET_errInstanceUnavailable: JET_ERR = -1090;
pub const JET_errDatabaseUnavailable: JET_ERR = -1091;
pub const JET_errInstanceUnavailableDueToFatalLogDiskFull: JET_ERR = -1092;
pub const JET_errInvalidSesparamId: JET_ERR = -1093;
pub const JET_errOutOfSessions: JET_ERR = -1101;
pub const JET_errWriteConflict: JET_ERR = -1102;
pub const JET_errTransTooDeep: JET_ERR = -1103;
pub const JET_errInvalidSesid: JET_ERR = -1104;
pub const JET_errWriteConflictPrimaryIndex: JET_ERR = -1105;
pub const JET_errInTransaction: JET_ERR = -1108;
pub const JET_errRollbackRequired: JET_ERR = -1109;
pub const JET_errTransReadOnly: JET_ERR = -1110;
pub const JET_errSessionWriteConflict: JET_ERR = -1111;
pub const JET_errRecordTooBigForBackwardCompatibility: JET_ERR = -1112;
pub const JET_errCannotMaterializeForwardOnlySort: JET_ERR = -1113;
pub const JET_errSesidTableIdMismatch: JET_ERR = -1114;
pub const JET_errInvalidInstance: JET_ERR = -1115;
pub const JET_errDirtyShutdown: JET_ERR = -1116;
// unused -1117
pub const JET_errReadPgnoVerifyFailure: JET_ERR = -1118;
pub const JET_errReadLostFlushVerifyFailure: JET_ERR = -1119;
pub const JET_errFileSystemCorruption: JET_ERR = -1121;
pub const JET_wrnShrinkNotPossible: JET_ERR = 1122;
pub const JET_errRecoveryVerifyFailure: JET_ERR = -1123;
pub const JET_errFilteredMoveNotSupported: JET_ERR = -1124;
pub const JET_errDatabaseDuplicate: JET_ERR = -1201;
pub const JET_errDatabaseInUse: JET_ERR = -1202;
pub const JET_errDatabaseNotFound: JET_ERR = -1203;
pub const JET_errDatabaseInvalidName: JET_ERR = -1204;
pub const JET_errDatabaseInvalidPages: JET_ERR = -1205;
pub const JET_errDatabaseCorrupted: JET_ERR = -1206;
pub const JET_errDatabaseLocked: JET_ERR = -1207;
pub const JET_errCannotDisableVersioning: JET_ERR = -1208;
pub const JET_errInvalidDatabaseVersion: JET_ERR = -1209;
pub const JET_errDatabase200Format: JET_ERR = -1210;
pub const JET_errDatabase400Format: JET_ERR = -1211;
pub const JET_errDatabase500Format: JET_ERR = -1212;
pub const JET_errPageSizeMismatch: JET_ERR = -1213;
pub const JET_errTooManyInstances: JET_ERR = -1214;
pub const JET_errDatabaseSharingViolation: JET_ERR = -1215;
pub const JET_errAttachedDatabaseMismatch: JET_ERR = -1216;
pub const JET_errDatabaseInvalidPath: JET_ERR = -1217;
pub const JET_errDatabaseIdInUse: JET_ERR = -1218;
pub const JET_errForceDetachNotAllowed: JET_ERR = -1219;
pub const JET_errCatalogCorrupted: JET_ERR = -1220;
pub const JET_errPartiallyAttachedDB: JET_ERR = -1221;
pub const JET_errDatabaseSignInUse: JET_ERR = -1222;
pub const JET_errDatabaseCorruptedNoRepair: JET_ERR = -1224;
pub const JET_errInvalidCreateDbVersion: JET_ERR = -1225;
pub const JET_errDatabaseNotReady: JET_ERR = -1230;
pub const JET_errDatabaseAttachedForRecovery: JET_ERR = -1231;
pub const JET_errTransactionsNotReadyDuringRecovery: JET_ERR = -1232;
pub const JET_wrnTableEmpty: JET_ERR = 1301;
pub const JET_errTableLocked: JET_ERR = -1302;
pub const JET_errTableDuplicate: JET_ERR = -1303;
pub const JET_errTableInUse: JET_ERR = -1304;
pub const JET_errObjectNotFound: JET_ERR = -1305;
pub const JET_errDensityInvalid: JET_ERR = -1307;
pub const JET_errTableNotEmpty: JET_ERR = -1308;
pub const JET_errInvalidTableId: JET_ERR = -1310;
pub const JET_errTooManyOpenTables: JET_ERR = -1311;
pub const JET_errIllegalOperation: JET_ERR = -1312;
pub const JET_errTooManyOpenTablesAndCleanupTimedOut: JET_ERR = -1313;
pub const JET_errObjectDuplicate: JET_ERR = -1314;
pub const JET_errInvalidObject: JET_ERR = -1316;
pub const JET_errCannotDeleteTempTable: JET_ERR = -1317;
pub const JET_errCannotDeleteSystemTable: JET_ERR = -1318;
pub const JET_errCannotDeleteTemplateTable: JET_ERR = -1319;
pub const JET_errExclusiveTableLockRequired: JET_ERR = -1322;
pub const JET_errFixedDDL: JET_ERR = -1323;
pub const JET_errFixedInheritedDDL: JET_ERR = -1324;
pub const JET_errCannotNestDDL: JET_ERR = -1325;
pub const JET_errDDLNotInheritable: JET_ERR = -1326;
pub const JET_wrnTableInUseBySystem: JET_ERR = 1327;
pub const JET_errInvalidSettings: JET_ERR = -1328;
pub const JET_errClientRequestToStopJetService: JET_ERR = -1329;
pub const JET_errCannotAddFixedVarColumnToDerivedTable: JET_ERR = -1330;
pub const JET_errIndexCantBuild: JET_ERR = -1401;
pub const JET_errIndexHasPrimary: JET_ERR = -1402;
pub const JET_errIndexDuplicate: JET_ERR = -1403;
pub const JET_errIndexNotFound: JET_ERR = -1404;
pub const JET_errIndexMustStay: JET_ERR = -1405;
pub const JET_errIndexInvalidDef: JET_ERR = -1406;
pub const JET_errInvalidCreateIndex: JET_ERR = -1409;
pub const JET_errTooManyOpenIndexes: JET_ERR = -1410;
pub const JET_errMultiValuedIndexViolation: JET_ERR = -1411;
pub const JET_errIndexBuildCorrupted: JET_ERR = -1412;
pub const JET_errPrimaryIndexCorrupted: JET_ERR = -1413;
pub const JET_errSecondaryIndexCorrupted: JET_ERR = -1414;
pub const JET_wrnCorruptIndexDeleted: JET_ERR = 1415;
pub const JET_errInvalidIndexId: JET_ERR = -1416;
pub const JET_wrnPrimaryIndexOutOfDate: JET_ERR = 1417;
pub const JET_wrnSecondaryIndexOutOfDate: JET_ERR = 1418;
pub const JET_errIndexTuplesSecondaryIndexOnly: JET_ERR = -1430;
pub const JET_errIndexTuplesTooManyColumns: JET_ERR = -1431;
pub const JET_errIndexTuplesOneColumnOnly: JET_ERR = JET_errIndexTuplesTooManyColumns;
pub const JET_errIndexTuplesNonUniqueOnly: JET_ERR = -1432;
pub const JET_errIndexTuplesTextBinaryColumnsOnly: JET_ERR = -1433;
pub const JET_errIndexTuplesTextColumnsOnly: JET_ERR = JET_errIndexTuplesTextBinaryColumnsOnly;
pub const JET_errIndexTuplesVarSegMacNotAllowed: JET_ERR = -1434;
pub const JET_errIndexTuplesInvalidLimits: JET_ERR = -1435;
pub const JET_errIndexTuplesCannotRetrieveFromIndex: JET_ERR = -1436;
pub const JET_errIndexTuplesKeyTooSmall: JET_ERR = -1437;
pub const JET_errInvalidLVChunkSize: JET_ERR = -1438;
pub const JET_errColumnCannotBeEncrypted: JET_ERR = -1439;
pub const JET_errCannotIndexOnEncryptedColumn: JET_ERR = -1440;
pub const JET_errColumnLong: JET_ERR = -1501;
pub const JET_errColumnNoChunk: JET_ERR = -1502;
pub const JET_errColumnDoesNotFit: JET_ERR = -1503;
pub const JET_errNullInvalid: JET_ERR = -1504;
pub const JET_errColumnIndexed: JET_ERR = -1505;
pub const JET_errColumnTooBig: JET_ERR = -1506;
pub const JET_errColumnNotFound: JET_ERR = -1507;
pub const JET_errColumnDuplicate: JET_ERR = -1508;
pub const JET_errMultiValuedColumnMustBeTagged: JET_ERR = -1509;
pub const JET_errColumnRedundant: JET_ERR = -1510;
pub const JET_errInvalidColumnType: JET_ERR = -1511;
pub const JET_wrnColumnMaxTruncated: JET_ERR = 1512;
pub const JET_errTaggedNotNULL: JET_ERR = -1514;
pub const JET_errNoCurrentIndex: JET_ERR = -1515;
pub const JET_errKeyIsMade: JET_ERR = -1516;
pub const JET_errBadColumnId: JET_ERR = -1517;
pub const JET_errBadItagSequence: JET_ERR = -1518;
pub const JET_errColumnInRelationship: JET_ERR = -1519;
pub const JET_wrnCopyLongValue: JET_ERR = 1520;
pub const JET_errCannotBeTagged: JET_ERR = -1521;
pub const JET_errDefaultValueTooBig: JET_ERR = -1524;
pub const JET_errMultiValuedDuplicate: JET_ERR = -1525;
pub const JET_errLVCorrupted: JET_ERR = -1526;
pub const JET_errMultiValuedDuplicateAfterTruncation: JET_ERR = -1528;
pub const JET_errDerivedColumnCorruption: JET_ERR = -1529;
pub const JET_errInvalidPlaceholderColumn: JET_ERR = -1530;
pub const JET_wrnColumnSkipped: JET_ERR = 1531;
pub const JET_wrnColumnNotLocal: JET_ERR = 1532;
pub const JET_wrnColumnMoreTags: JET_ERR = 1533;
pub const JET_wrnColumnTruncated: JET_ERR = 1534;
pub const JET_wrnColumnPresent: JET_ERR = 1535;
pub const JET_wrnColumnSingleValue: JET_ERR = 1536;
pub const JET_wrnColumnDefault: JET_ERR = 1537;
pub const JET_errColumnCannotBeCompressed: JET_ERR = -1538;
pub const JET_wrnColumnNotInRecord: JET_ERR = 1539;
pub const JET_errColumnNoEncryptionKey: JET_ERR = -1540;
pub const JET_errRecordNotFound: JET_ERR = -1601;
pub const JET_errRecordNoCopy: JET_ERR = -1602;
pub const JET_errNoCurrentRecord: JET_ERR = -1603;
pub const JET_errRecordPrimaryChanged: JET_ERR = -1604;
pub const JET_errKeyDuplicate: JET_ERR = -1605;
pub const JET_errAlreadyPrepared: JET_ERR = -1607;
pub const JET_errKeyNotMade: JET_ERR = -1608;
pub const JET_errUpdateNotPrepared: JET_ERR = -1609;
pub const JET_wrnDataHasChanged: JET_ERR = 1610;
pub const JET_errDataHasChanged: JET_ERR = -1611;
pub const JET_wrnKeyChanged: JET_ERR = 1618;
pub const JET_errLanguageNotSupported: JET_ERR = -1619;
pub const JET_errDecompressionFailed: JET_ERR = -1620;
pub const JET_errUpdateMustVersion: JET_ERR = -1621;
pub const JET_errDecryptionFailed: JET_ERR = -1622;
// Sort Table errors
pub const JET_errTooManySorts: JET_ERR = -1701;
pub const JET_errInvalidOnSort: JET_ERR = -1702;
// Other errors
pub const JET_errTempFileOpenError: JET_ERR = -1803;
pub const JET_errTooManyAttachedDatabases: JET_ERR = -1805;
pub const JET_errDiskFull: JET_ERR = -1808;
pub const JET_errPermissionDenied: JET_ERR = -1809;
pub const JET_errFileNotFound: JET_ERR = -1811;
pub const JET_errFileInvalidType: JET_ERR = -1812;
pub const JET_wrnFileOpenReadOnly: JET_ERR = 1813;
pub const JET_errAfterInitialization: JET_ERR = -1850;
pub const JET_errLogCorrupted: JET_ERR = -1852;
pub const JET_errInvalidOperation: JET_ERR = -1906;
pub const JET_errAccessDenied: JET_ERR = -1907;
pub const JET_wrnIdleFull: JET_ERR = 1908;
pub const JET_errTooManySplits: JET_ERR = -1909;
pub const JET_errSessionSharingViolation: JET_ERR = -1910;
pub const JET_errEntryPointNotFound: JET_ERR = -1911;
pub const JET_errSessionContextAlreadySet: JET_ERR = -1912;
pub const JET_errSessionContextNotSetByThisThread: JET_ERR = -1913;
pub const JET_errSessionInUse: JET_ERR = -1914;
pub const JET_errRecordFormatConversionFailed: JET_ERR = -1915;
pub const JET_errOneDatabasePerSession: JET_ERR = -1916;
pub const JET_errRollbackError: JET_ERR = -1917;
pub const JET_errFlushMapVersionUnsupported: JET_ERR = -1918;
pub const JET_errFlushMapDatabaseMismatch: JET_ERR = -1919;
pub const JET_errFlushMapUnrecoverable: JET_ERR = -1920;
pub const JET_wrnDefragAlreadyRunning: JET_ERR = 2000;
pub const JET_wrnDefragNotRunning: JET_ERR = 2001;
pub const JET_errDatabaseAlreadyRunningMaintenance: JET_ERR = -2004;
pub const JET_wrnCallbackNotRegistered: JET_ERR = 2100;
pub const JET_errCallbackFailed: JET_ERR = -2101;
pub const JET_errCallbackNotResolved: JET_ERR = -2102;
pub const JET_errSpaceHintsInvalid: JET_ERR = -2103;
pub const JET_errOSSnapshotInvalidSequence: JET_ERR = -2401;
pub const JET_errOSSnapshotTimeOut: JET_ERR = -2402;
pub const JET_errOSSnapshotNotAllowed: JET_ERR = -2403;
pub const JET_errOSSnapshotInvalidSnapId: JET_ERR = -2404;
pub const JET_errLSCallbackNotSpecified: JET_ERR = -3000;
pub const JET_errLSAlreadySet: JET_ERR = -3001;
pub const JET_errLSNotSet: JET_ERR = -3002;
// FILE and DISK ERRORS
pub const JET_errFileIOSparse: JET_ERR = -4000;
pub const JET_errFileIOBeyondEOF: JET_ERR = -4001;
pub const JET_errFileIOAbort: JET_ERR = -4002;
pub const JET_errFileIORetry: JET_ERR = -4003;
pub const JET_errFileIOFail: JET_ERR = -4004;
pub const JET_errFileCompressed: JET_ERR = -4005;
extern "system" {
    pub fn JetAddColumnW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szColumnName: JET_PCWSTR,
        pcolumndef: *const JET_COLUMNDEF,
        pvDefault: *const c_void,
        cbDefault: c_ulong,
        pcolumnid: *mut JET_COLUMNID,
    ) -> JET_ERR;
    pub fn JetAttachDatabaseW(
        sesid: JET_SESID,
        szFilename: JET_PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetAttachDatabase2W(
        sesid: JET_SESID,
        szFilename: JET_PCWSTR,
        cpgDatabaseSizeMax: c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetBackupW(
        szBackupPath: JET_PCWSTR,
        grbt: JET_GRBIT,
        pfnStatus: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetBackupInstanceW(
        instance: JET_INSTANCE,
        szBackupPath: JET_PCWSTR,
        grbit: JET_GRBIT,
        pfnStatus: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetBeginExternalBackup(
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetBeginExternalBackupInstance(
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetBeginSessionW(
        instance: JET_INSTANCE,
        psesid: *mut JET_SESID,
        szUserName: JET_PCWSTR,
        szPassword: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetBeginTransaction(
        sesid: JET_SESID,
    ) -> JET_ERR;
    pub fn JetBeginTransaction2(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetBeginTransaction3(
        sesid: JET_SESID,
        trxid: i64,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCloseDatabase(
        sesid: JET_SESID,
        dbid: JET_DBID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCloseFile(
        hfFile: JET_HANDLE,
    ) -> JET_ERR;
    pub fn JetCloseFileInstance(
        instance: JET_INSTANCE,
        hfFile: JET_HANDLE,
    ) -> JET_ERR;
    pub fn JetCloseTable(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetCommitTransaction(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCommitTransaction2(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
        cmsecDurableCommit: c_ulong,
        pCommitId: JET_COMMIT_ID,
    ) -> JET_ERR;
    pub fn JetCompactW(
        sesid: JET_SESID,
        szDatabaseSrc: JET_PCWSTR,
        szDatabaseDest: JET_PCWSTR,
        pfnStatus: JET_PFNSTATUS,
        pconvert: *mut c_void, // actually *JET_CONVERT (but not supported since WinXP)
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetComputeStats(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetCreateDatabaseW(
        sesid: JET_SESID,
        szFilename: JET_PCWSTR,
        szConnect: JET_PCWSTR,
        pdbid: *mut JET_DBID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCreateDatabase2W(
        sesid: JET_SESID,
        szFilename: JET_PCWSTR,
        cpgDatabaseSizeMax: c_ulong,
        pdbid: *mut JET_DBID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCreateIndexW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: JET_PCWSTR,
        grbit: JET_GRBIT,
        szKey: JET_PCWSTR,
        cbKey: c_ulong,
        lDensity: c_ulong,
    ) -> JET_ERR;
    pub fn JetCreateIndex2W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pindexcreate: *const JET_INDEXCREATE_W,
        cIndexCreate: c_ulong,
    ) -> JET_ERR;
    pub fn JetCreateIndex3W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pindexcreate: *const JET_INDEXCREATE2_W,
        cIndexCreate: c_ulong,
    ) -> JET_ERR;
    pub fn JetCreateIndex4W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pindexcreate: *const JET_INDEXCREATE3_W,
        cIndexCreate: c_ulong,
    ) -> JET_ERR;
    pub fn JetCreateInstanceW(
        pinstance: *mut JET_INSTANCE,
        szInstanceName: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetCreateInstance2W(
        pinstance: *mut JET_INSTANCE,
        szInstanceName: JET_PCWSTR,
        szDisplayName: JET_PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCreateTableW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
        lPages: c_ulong,
        lDensity: c_ulong,
        ptableid: *mut JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetCreateTableColumnIndexW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        ptablecreate: *mut JET_TABLECREATE_W,
    ) -> JET_ERR;
    pub fn JetCreateTableColumnIndex2W(
        sesid: JET_SESID,
        dbid: JET_DBID,
        ptablecreate: *mut JET_TABLECREATE2_W,
    ) -> JET_ERR;
    pub fn JetCreateTableColumnIndex3W(
        sesid: JET_SESID,
        dbid: JET_DBID,
        ptablecreate: *mut JET_TABLECREATE3_W,
    ) -> JET_ERR;
    pub fn JetCreateTableColumnIndex4W(
        sesid: JET_SESID,
        dbid: JET_DBID,
        ptablecreate: *mut JET_TABLECREATE3_W,
    ) -> JET_ERR;
    pub fn JetDefragmentW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
        pcPasses: *mut c_ulong,
        pcSeconds: *mut c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetDefragment2W(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
        pcPasses: *mut c_ulong,
        pcSeconds: *mut c_ulong,
        callback: JET_CALLBACK,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetDefragment3W(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
        pcPasses: *mut c_ulong,
        pcSeconds: *mut c_ulong,
        callback: JET_CALLBACK,
        pvContext: *const c_void,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetDelete(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetDeleteColumnW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szColumnName: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetDeleteColumn2W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szColumnName: JET_PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetDeleteIndex(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetDeleteTableW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetDetachDatabaseW(
        sesid: JET_SESID,
        szFilename: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetDetachDatabase2W(
        sesid: JET_SESID,
        szFilename: JET_PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetDupCursor(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        ptableid: *mut JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetDupSession(
        sesid: JET_SESID,
        psesid: *mut JET_SESID,
    ) -> JET_ERR;
    pub fn JetEnableMultiInstanceW(
        psetsysparam: *const JET_SETSYSPARAM_W,
        csetsysparam: c_ulong,
        pcsetsucceed: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetEndExternalBackup() -> JET_ERR;
    pub fn JetEndExternalBackupInstance(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetEndExternalBackupInstance2(
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetEndSession(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetEnumerateColumns(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        cEnumColumnId: c_ulong,
        rgEnumColumnId: *const JET_ENUMCOLUMNID,
        pcEnumColumn: *mut c_ulong,
        prgEnumColumn: *mut *const JET_ENUMCOLUMN,
        pfnRealloc: JET_PFNREALLOC,
        pvReallocContext: *const c_void,
        cbDataMost: c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetEscrowUpdate(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        columnid: JET_COLUMNID,
        pv: *const c_void,
        cbMax: c_ulong,
        pvOld: *mut c_void,
        cbOldMax: c_ulong,
        pcbOldActual: *mut c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetExternalRestoreW(
        szCheckpointFilePath: JET_PCWSTR,
        szLogPath: JET_PCWSTR,
        rgrstmap: *const JET_RSTMAP_W,
        crstfilemap: c_long,
        szBackupLogPath: JET_PCWSTR,
        genLow: c_long,
        genHigh: c_long,
        pfn: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetExternalRestore2W(
        szCheckpointFilePath: JET_PCWSTR,
        szLogPath: JET_PCWSTR,
        rgrstmap: *const JET_RSTMAP_W,
        crstfilemap: c_long,
        szBackupLogPath: JET_PCWSTR,
        pLogInfo: *mut JET_LOGINFO_W,
        szTargetInstanceName: JET_PCWSTR,
        szTargetInstanceLogPath: JET_PCWSTR,
        szTargetInstanceCheckpointPath: JET_PCWSTR,
        pfn: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetFreeBuffer(
        pbBuf: *mut u8,
    ) -> JET_ERR;
    pub fn JetGetAttachInfoW(
        wzzDatabases: JET_PWSTR,
        cbMax: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetGetAttachInfoInstanceW(
        instance: JET_INSTANCE,
        szzDatabases: JET_PWSTR,
        cbMax: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetGetBookmark(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvBookmark: *mut c_void,
        cbMax: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetGetColumnInfoW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
        pwColumnNameOrId: JET_PCWSTR,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetCurrentIndexW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: JET_PWSTR,
        cbIndexName: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetCursorInfo(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetDatabaseFileInfoW(
        szDatabaseName: JET_PCWSTR,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetDetabaseInfoW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetErrorInfoW(
        pvContext: *const c_void,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetGetIndexInfoW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
        szIndexName: JET_PCWSTR,
        pvResult: *mut c_void,
        cbResult: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetInstanceInfoW(
        pcInstanceInfo: *mut c_ulong,
        paInstanceInfo: *mut *const JET_INSTANCE_INFO_W,
    ) -> JET_ERR;
    pub fn JetGetInstanceMiscInfo(
        instance: JET_INSTANCE,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetLock(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetGetLogInfoW(
        szzLogs: JET_PWSTR,
        cbMax: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetGetLogInfoInstanceW(
        instance: JET_INSTANCE,
        wszzLogs: JET_PWSTR,
        cbMax: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetGetLogInfoInstance2W(
        instance: JET_INSTANCE,
        wszzLogs: JET_PWSTR,
        cbMax: c_ulong,
        pcbActual: *mut c_ulong,
        pLogInfo: *mut JET_LOGINFO_W,
    ) -> JET_ERR;
    pub fn JetGetLS(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pls: *mut JET_LS,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetGetObjectInfoW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        objtyp: JET_OBJTYP,
        szContainerName: JET_PCWSTR,
        szObjectName: JET_PCWSTR,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetRecordPosition(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        precpos: *mut JET_RECPOS,
        cbRecpos: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetRecordSize(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        precsize: *mut JET_RECSIZE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetGetRecordSize2(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        precsize: *mut JET_RECSIZE2,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetGetSecondaryIndexBookmark(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvSecondaryKey: *mut c_void,
        cbSecondaryKeyMax: c_ulong,
        pcbSecondaryKeyActual: *mut c_ulong,
        pvPrimaryBookmark: *mut c_void,
        cbPrimaryBookmarkMax: c_ulong,
        pcbPrimaryBookmarkActual: *mut c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetGetSessionParameter(
        sesid: JET_SESID,
        sesparamid: c_ulong,
        pvParam: *mut c_void,
        cbParamMax: c_ulong,
        pcbParamActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetGetSystemParameterW(
        instance: JET_INSTANCE,
        sesid: JET_SESID,
        paramid: c_ulong,
        plParam: *mut JET_API_PTR,
        szParam: JET_PWSTR,
        cbMax: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetTableColumnInfoW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szColumnName: JET_PCWSTR,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetTableIndexInfoW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: JET_PCWSTR,
        pvResult: *mut c_void,
        cbResult: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetTableInfoW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvResult: *mut c_void,
        cbMax: c_ulong,
        InfoLevel: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetThreadStats(
        pvResult: *mut c_void,
        cbMax: c_ulong,
    ) -> JET_ERR;
    pub fn JetGetTruncateLogInfoInstanceW(
        instance: JET_INSTANCE,
        wszzLogs: JET_PWSTR,
        cbMax: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetGetVersion(
        sesid: JET_SESID,
        pwVersion: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetGotoBookmark(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvBookmark: *const c_void,
        cbBookmark: c_ulong,
    ) -> JET_ERR;
    pub fn JetGotoPosition(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        precpos: *const JET_RECPOS,
    ) -> JET_ERR;
    pub fn JetGrowDatabase(
        sesid: JET_SESID,
        dbid: JET_DBID,
        cpg: c_ulong,
        pcpgReal: *mut c_ulong, // this is erroneously annotated as `_In_` in the header
    ) -> JET_ERR;
    pub fn JetIdle(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetIndexRecordCount(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pcrec: *mut c_ulong,
        crecMax: c_ulong,
    ) -> JET_ERR;
    pub fn JetInit(
        pinstance: *mut JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetInit2(
        pinstance: *mut JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetInit3W(
        pinstance: *mut JET_INSTANCE,
        prstInfo: *const JET_RSTINFO_W,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetIntersectIndexes(
        sesid: JET_SESID,
        rgindexrange: *const JET_INDEXRANGE,
        cindexrange: c_ulong,
        precordlist: *mut JET_RECORDLIST,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetMakeKey(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvData: *const c_void,
        cbData: c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetMove(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        cRow: c_long,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOpenDatabaseW(
        sesid: JET_SESID,
        szFilename: JET_PCWSTR,
        szConnect: JET_PCWSTR,
        pdbid: *mut JET_DBID,
        grbit: JET_GRBIT
    ) -> JET_ERR;
    pub fn JetOpenFileW(
        szFileName: JET_PCWSTR,
        phfFile: *mut JET_HANDLE,
        pulFileSizeLow: *mut c_ulong,
        pulFileSizeHigh: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetOpenFileInstanceW(
        instance: JET_INSTANCE,
        szFileName: JET_PCWSTR,
        phfFile: *mut JET_HANDLE,
        pulFileSizeLow: *mut c_ulong,
        pulFileSizeHigh: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetOpenTableW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
        pvParameters: *const c_void,
        cbParameters: c_ulong,
        grbit: JET_GRBIT,
        ptableid: *mut JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetOpenTemporaryTable(
        sesid: JET_SESID,
        popentemporarytable: *const JET_OPENTEMPORARYTABLE,
    ) -> JET_ERR;
    pub fn JetOpenTempTable(
        sesid: JET_SESID,
        prgcolumndef: *const JET_COLUMNDEF,
        ccolumn: c_ulong,
        grbit: JET_GRBIT,
        ptableid: *mut JET_TABLEID,
        prgcolumnid: *mut JET_COLUMNID,
    ) -> JET_ERR;
    pub fn JetOpenTempTable2(
        sesid: JET_SESID,
        prgcolumndef: *const JET_COLUMNDEF,
        ccolumn: c_ulong,
        lcid: c_ulong,
        grbit: JET_GRBIT,
        ptableid: *mut JET_TABLEID,
        prgcolumnid: *mut JET_COLUMNID,
    ) -> JET_ERR;
    pub fn JetOpenTempTable3(
        sesid: JET_SESID,
        prgcolumndef: *const JET_COLUMNDEF,
        ccolumn: c_ulong,
        pidxunicode: *const JET_UNICODEINDEX,
        grbit: JET_GRBIT,
        ptableid: *mut JET_TABLEID,
        prgcolumnid: *mut JET_COLUMNID,
    ) -> JET_ERR;
    pub fn JetOSSnapshotAbort(
        snapId: JET_OSSNAPID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOSSnapshotEnd(
        snapId: JET_OSSNAPID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOSSnapshotFreezeW(
        snapId: JET_OSSNAPID,
        pcInstanceInfo: *mut c_ulong,
        paInstanceInfo: *mut *mut JET_INSTANCE_INFO_W,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOSSnapshotGetFreezeInfoW(
        snapId: JET_OSSNAPID,
        pcInstanceInfo: *mut c_ulong,
        paInstanceInfo: *mut *mut JET_INSTANCE_INFO_W,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOSSnapshotPrepare(
        psnapId: *mut JET_OSSNAPID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOSSnapshotPrepareInstance(
        snapId: JET_OSSNAPID,
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOSSnapshotThaw(
        snapId: JET_OSSNAPID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOSSnapshotTruncateLog(
        snapId: JET_OSSNAPID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOSSnapshotTruncateLogInstance(
        snapId: JET_OSSNAPID,
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetPrepareUpdate(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        prep: c_ulong,
    ) -> JET_ERR;
    pub fn JetPrereadIndexRanges(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        rgIndexRanges: *const JET_INDEXRANGE,
        cIndexRanges: c_ulong,
        pcRangesPreread: *mut c_ulong,
        rgcolumnidPreread: *const JET_COLUMNID,
        ccolumnidPreread: c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetPrereadKeys(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        rgpvKeys: *const *const c_void,
        rgcbKeys: *const c_ulong,
        ckeys: c_long,
        pckeysPreread: *mut c_long,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetReadFile(
        hfFile: JET_HANDLE,
        pv: *mut c_void,
        cb: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetReadFileInstance(
        instance: JET_INSTANCE,
        hfFile: JET_HANDLE,
        pv: *mut c_void,
        cb: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetRegisterCallback(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        cbtyp: JET_CBTYP,
        pCallback: JET_CALLBACK,
        pvContext: *mut c_void,
        phCallbackId: *const JET_HANDLE,
    ) -> JET_ERR;
    pub fn JetRenameColumnW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szName: JET_PCWSTR,
        szNameNew: JET_PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetRenameTableW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szName: JET_PCWSTR,
        szNameNew: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetResetSessionContext(
        sesid: JET_SESID,
    ) -> JET_ERR;
    pub fn JetResetTableSequential(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetResizeDatabase(
        sesid: JET_SESID,
        dbid: JET_DBID,
        cpgTarget: c_ulong,
        pcpgActual: *mut c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetRestoreW(
        szSource: JET_PCWSTR,
        pfn: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetRestore2W(
        sz: JET_PCWSTR,
        szDest: JET_PCWSTR,
        pfn: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetRestoreInstanceW(
        instance: JET_INSTANCE,
        sz: JET_PCWSTR,
        szDest: JET_PCWSTR,
        pfn: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetRetrieveColumn(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        columnid: JET_COLUMNID,
        pvData: *mut c_void,
        cbData: c_ulong,
        pcbActual: *mut c_ulong,
        grbit: JET_GRBIT,
        pretinfo: *mut JET_RETINFO,
    ) -> JET_ERR;
    pub fn JetRetrieveColumns(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pretrievecolumn: *mut JET_RETRIEVECOLUMN,
        cretrievecolumn: c_ulong,
    ) -> JET_ERR;
    pub fn JetRetrieveKey(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvData: *mut c_void,
        cbMax: c_ulong,
        pcbActual: *mut c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetRollback(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSeek(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetColumn(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        columnid: JET_COLUMNID,
        pvData: *const c_void,
        cbData: c_ulong,
        grbit: JET_GRBIT,
        psetinfo: *const JET_SETINFO,
    ) -> JET_ERR;
    pub fn JetSetColumnDefaultValueW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: JET_PCWSTR,
        szColumnName: JET_PCWSTR,
        pvData: *const c_void,
        cbData: c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetColumns(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        psetcolumn: *const JET_SETCOLUMN,
        csetcolumn: c_ulong,
    ) -> JET_ERR;
    pub fn JetSetCurrentIndexW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetSetCurrentIndex2W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: JET_PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetCurrentIndex3W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: JET_PCWSTR,
        grbit: JET_GRBIT,
        itagSequence: c_ulong,
    ) -> JET_ERR;
    pub fn JetSetCurrentIndex4W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: JET_PCWSTR,
        pindexid: *const JET_INDEXID,
        grbit: JET_GRBIT,
        itagSequence: c_ulong,
    ) -> JET_ERR;
    pub fn JetSetCursorFilter(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        rgFilters: *const JET_INDEX_COLUMN,
        cFilters: c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetDatabaseSizeW(
        sesid: JET_SESID,
        szDatabaseName: JET_PCWSTR,
        cpg: c_ulong,
        pcpgReal: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetSetIndexRange(
        sesid: JET_SESID,
        tableidSrc: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetLS(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        ls: JET_LS,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetSessionContext(
        sesid: JET_SESID,
        ulContext: JET_API_PTR,
    ) -> JET_ERR;
    pub fn JetSetSessionParameter(
        sesid: JET_SESID,
        sesparamid: c_ulong,
        pvParam: *const c_void,
        cbParam: c_ulong,
    ) -> JET_ERR;
    pub fn JetSetSystemParameterW(
        pinstance: *mut JET_INSTANCE,
        sesid: JET_SESID,
        paramid: c_ulong,
        lParam: JET_API_PTR,
        szParam: JET_PCWSTR,
    ) -> JET_ERR;
    pub fn JetSetTableSequential(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetStopBackup() -> JET_ERR;
    pub fn JetStopBackupInstance(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetStopService() -> JET_ERR;
    pub fn JetStopServiceInstance(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetStopServiceInstance2(
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetTerm(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetTerm2(
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetTruncateLog() -> JET_ERR;
    pub fn JetTruncateLogInstance(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetUnregisterCallback(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        cbtyp: JET_CBTYP,
        hCallbackId: JET_HANDLE,
    ) -> JET_ERR;
    pub fn JetUpdate(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvBookmark: *mut c_void,
        cbBookmark: c_ulong,
        pcbActual: *mut c_ulong,
    ) -> JET_ERR;
    pub fn JetUpdate2(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvBookmark: *mut c_void,
        cbBookmark: c_ulong,
        pcbActual: *mut c_ulong,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
}
