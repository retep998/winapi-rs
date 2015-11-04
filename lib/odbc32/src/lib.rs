// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to odbc32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn SQLAllocHandle(
        handleType: SQLSMALLINT, inputHandle: SQLHANDLE, outputHandle: *mut SQLHANDLE,
    ) -> SQLRETURN;
    pub fn SQLConnectA(
        connectionHandle: SQLHDBC, serverName: *const SQLCHAR, nameLength1: SQLSMALLINT,
        userName: *const SQLCHAR, nameLength2: SQLSMALLINT, authentication: *const SQLCHAR,
        nameLength3: SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLConnectW(
        connectionHandle: SQLHDBC, serverName: *const SQLWCHAR, nameLength1: SQLSMALLINT,
        userName: *const SQLWCHAR, nameLength2: SQLSMALLINT, authentication: *const SQLWCHAR,
        nameLength3: SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLDescribeColA(
        statementHandle: SQLHSTMT, columnNumber: SQLUSMALLINT, columnName: *mut SQLCHAR,
        bufferLength: SQLSMALLINT, nameLength: *mut SQLSMALLINT, dataType: *mut SQLSMALLINT,
        columnSize: *mut SQLULEN, decimalDigits: *mut SQLSMALLINT, nullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLDescribeColW(
        statementHandle: SQLHSTMT, columnNumber: SQLUSMALLINT, columnName: *mut SQLWCHAR,
        bufferLength: SQLSMALLINT, nameLength: *mut SQLSMALLINT, dataType: *mut SQLSMALLINT,
        columnSize: *mut SQLULEN, decimalDigits: *mut SQLSMALLINT, nullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLDisconnect(connectionHandle: SQLHDBC) -> SQLRETURN;
    pub fn SQLExecDirectA(
        statementHandle: SQLHSTMT, statementText: *const SQLCHAR, textLength: SQLINTEGER,
    ) -> SQLRETURN;
    pub fn SQLExecDirectW(
        statementHandle: SQLHSTMT, statementText: *const SQLWCHAR, textLength: SQLINTEGER,
    ) -> SQLRETURN;
    pub fn SQLFetch(statementHandle: SQLHSTMT) -> SQLRETURN;
    pub fn SQLFreeHandle(handleType: SQLSMALLINT, handle: SQLHANDLE) -> SQLRETURN;
    pub fn SQLFreeStmt(statementHandle: SQLHSTMT, option: SQLUSMALLINT) -> SQLRETURN;
    pub fn SQLGetData(
        statementHandle: SQLHSTMT, columnNumber: SQLUSMALLINT, targetType: SQLSMALLINT,
        targetValue: SQLPOINTER, bufferLength: SQLLEN, strLen_or_IndPtr: *mut SQLLEN,
    ) -> SQLRETURN;
    pub fn SQLGetDiagRecA(
        handleType: SQLSMALLINT, handle: SQLHANDLE, recNumber: SQLSMALLINT, sqlstate: *mut SQLCHAR,
        nativeError: *mut SQLINTEGER, messageText: *mut SQLCHAR, bufferLength: SQLSMALLINT,
        textLength: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLGetDiagRecW(
        handleType: SQLSMALLINT, handle: SQLHANDLE, recNumber: SQLSMALLINT,
        sqlstate: *mut SQLWCHAR, nativeError: *mut SQLINTEGER, messageText: *mut SQLWCHAR,
        bufferLength: SQLSMALLINT, textLength: *mut SQLSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLNumResultCols(statementHandle: SQLHSTMT, columnCount: *mut SQLSMALLINT) -> SQLRETURN;
    pub fn SQLRowCount(statementHandle: SQLHSTMT, rowCount: *mut SQLLEN) -> SQLRETURN;
    pub fn SQLSetConnectAttr(
        connectionHandle: SQLHDBC, attribute: SQLINTEGER, value: SQLPOINTER,
        stringLength: SQLINTEGER,
    ) -> SQLRETURN;
    pub fn SQLSetEnvAttr(
        environmentHandle: SQLHENV, attribute: SQLINTEGER, value: SQLPOINTER,
        stringLength: SQLINTEGER,
    ) -> SQLRETURN;
    pub fn SQLDriverConnectA(
        hdbc: SQLHDBC, hwnd: SQLHWND, szConnStrIn: *const SQLCHAR, cchConnStrIn: SQLSMALLINT,
        szConnStrOut: *mut SQLCHAR, cchConnStrOutMax: SQLSMALLINT,
        pcchConnStrOut: *mut SQLSMALLINT, fDriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;
    pub fn SQLDriverConnectW(
        hdbc: SQLHDBC, hwnd: SQLHWND, szConnStrIn: *const SQLWCHAR, cchConnStrIn: SQLSMALLINT,
        szConnStrOut: *mut SQLWCHAR, cchConnStrOutMax: SQLSMALLINT,
        pcchConnStrOut: *mut SQLSMALLINT, fDriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;
}
