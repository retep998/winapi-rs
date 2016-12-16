// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to setupapi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    //pub fn CMP_GetServerSideDeviceInstallFlags();
    pub fn CMP_WaitNoPendingInstallEvents(dwTimeout: DWORD) -> DWORD;
    pub fn CM_Add_Empty_Log_Conf(
        plcLogConf: PLOG_CONF, dnDevInst: DEVINST, Priority: PRIORITY, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Add_Empty_Log_Conf_Ex(
        plcLogConf: PLOG_CONF, dnDevInst: DEVINST, Priority: PRIORITY, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Add_IDA(dnDevInst: DEVINST, pszID: PSTR, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Add_IDW(dnDevInst: DEVINST, pszID: PWSTR, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Add_ID_ExA(
        dnDevInst: DEVINST, pszID: PSTR, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Add_ID_ExW(
        dnDevInst: DEVINST, pszID: PWSTR, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Add_Range(
        ullStartValue: DWORDLONG, ullEndValue: DWORDLONG, rlh: RANGE_LIST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Add_Res_Des(
        prdResDes: PRES_DES, lcLogConf: LOG_CONF, ResourceID: RESOURCEID, ResourceData: PCVOID,
        ResourceLen: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Add_Res_Des_Ex(
        prdResDes: PRES_DES, lcLogConf: LOG_CONF, ResourceID: RESOURCEID, ResourceData: PCVOID,
        ResourceLen: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Connect_MachineA(UNCServerName: PCSTR, phMachine: PHMACHINE) -> CONFIGRET;
    pub fn CM_Connect_MachineW(UNCServerName: PCWSTR, phMachine: PHMACHINE) -> CONFIGRET;
    pub fn CM_Create_DevNodeA(
        pdnDevInst: PDEVINST, pDeviceID: DEVINSTID_A, dnParent: DEVINST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Create_DevNodeW(
        pdnDevInst: PDEVINST, pDeviceID: DEVINSTID_W, dnParent: DEVINST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Create_DevNode_ExA(
        pdnDevInst: PDEVINST, pDeviceID: DEVINSTID_A, dnParent: DEVINST, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Create_DevNode_ExW(
        pdnDevInst: PDEVINST, pDeviceID: DEVINSTID_W, dnParent: DEVINST, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Create_Range_List(prlh: PRANGE_LIST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Delete_Class_Key(ClassGuid: LPGUID, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Delete_Class_Key_Ex(
        ClassGuid: LPGUID, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Delete_DevNode_Key(
        dnDevNode: DEVNODE, ulHardwareProfile: ULONG, ulFlags: ULONG
    ) -> CONFIGRET;
    pub fn CM_Delete_DevNode_Key_Ex(
        dnDevNode: DEVNODE, ulHardwareProfile: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Delete_Device_Interface_KeyA(
        pszDeviceInterface: LPCSTR, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Delete_Device_Interface_KeyW(
        pszDeviceInterface: LPCWSTR, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Delete_Device_Interface_Key_ExA(
        pszDeviceInterface: LPCSTR, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Delete_Device_Interface_Key_ExW(
        pszDeviceInterface: LPCWSTR, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Delete_Range(
        ullStartValue: DWORDLONG, ullEndValue: DWORDLONG, rlh: RANGE_LIST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Detect_Resource_Conflict(
        dnDevInst: DEVINST, ResourceID: RESOURCEID, ResourceData: PCVOID, ResourceLen: ULONG,
        pbConflictDetected: PBOOL, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Detect_Resource_Conflict_Ex(
        dnDevInst: DEVINST, ResourceID: RESOURCEID, ResourceData: PCVOID, ResourceLen: ULONG,
        pbConflictDetected: PBOOL, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Disable_DevNode(dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Disable_DevNode_Ex(
        dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Disconnect_Machine(hMachine: HMACHINE) -> CONFIGRET;
    pub fn CM_Dup_Range_List(rlhOld: RANGE_LIST, rlhNew: RANGE_LIST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Enable_DevNode(dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Enable_DevNode_Ex(
        dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_Classes(
        ulClassIndex: ULONG, ClassGuid: LPGUID, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_Classes_Ex(
        ulClassIndex: ULONG, ClassGuid: LPGUID, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_EnumeratorsA(
        ulEnumIndex: ULONG, Buffer: PSTR, pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_EnumeratorsW(
        ulEnumIndex: ULONG, Buffer: PWSTR, pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_Enumerators_ExA(
        ulEnumIndex: ULONG, Buffer: PSTR, pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_Enumerators_ExW(
        ulEnumIndex: ULONG, Buffer: PWSTR, pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Find_Range(
        pullStart: PDWORDLONG, ullStart: DWORDLONG, ulLength: ULONG, ullAlignment: DWORDLONG,
        ullEnd: DWORDLONG, rlh: RANGE_LIST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_First_Range(
        rlh: RANGE_LIST, pullStart: PDWORDLONG, pullEnd: PDWORDLONG, preElement: PRANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Free_Log_Conf(lcLogConfToBeFreed: LOG_CONF, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Free_Log_Conf_Ex(
        lcLogConfToBeFreed: LOG_CONF, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Free_Log_Conf_Handle(lcLogConf: LOG_CONF) -> CONFIGRET;
    pub fn CM_Free_Range_List(rlh: RANGE_LIST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Free_Res_Des(prdResDes: PRES_DES, rdResDes: RES_DES, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Free_Res_Des_Ex(
        prdResDes: PRES_DES, rdResDes: RES_DES, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Free_Res_Des_Handle(rdResDes: RES_DES) -> CONFIGRET;
    pub fn CM_Free_Resource_Conflict_Handle(clConflictList: CONFLICT_LIST) -> CONFIGRET;
    pub fn CM_Get_Child(pdnDevInst: PDEVINST, dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Get_Child_Ex(
        pdnDevInst: PDEVINST, dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Key_NameA(
        ClassGuid: LPGUID, pszKeyName: LPSTR, pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Key_NameW(
        ClassGuid: LPGUID, pszKeyName: LPWSTR, pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Key_Name_ExA(
        ClassGuid: LPGUID, pszKeyName: LPSTR, pulLength: PULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Key_Name_ExW(
        ClassGuid: LPGUID, pszKeyName: LPWSTR, pulLength: PULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_NameA(
        ClassGuid: LPGUID, Buffer: PSTR, pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_NameW(
        ClassGuid: LPGUID, Buffer: PWSTR, pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Name_ExA(
        ClassGuid: LPGUID, Buffer: PSTR, pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Name_ExW(
        ClassGuid: LPGUID, Buffer: PWSTR, pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Registry_PropertyA(
        ClassGuid: LPGUID, ulProperty: ULONG, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Registry_PropertyW(
        ClassGuid: LPGUID, ulProperty: ULONG, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Depth(pulDepth: PULONG, dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Get_Depth_Ex(
        pulDepth: PULONG, dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Custom_PropertyA(
        dnDevInst: DEVINST, pszCustomPropertyName: PCSTR, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Custom_PropertyW(
        dnDevInst: DEVINST, pszCustomPropertyName: PCWSTR, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Custom_Property_ExA(
        dnDevInst: DEVINST, pszCustomPropertyName: PCSTR, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Custom_Property_ExW(
        dnDevInst: DEVINST, pszCustomPropertyName: PCWSTR, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Registry_PropertyA(
        dnDevInst: DEVINST, ulProperty: ULONG, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Registry_PropertyW(
        dnDevInst: DEVINST, ulProperty: ULONG, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Registry_Property_ExA(
        dnDevInst: DEVINST, ulProperty: ULONG, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Registry_Property_ExW(
        dnDevInst: DEVINST, ulProperty: ULONG, pulRegDataType: PULONG, Buffer: PVOID,
        pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Status(
        pulStatus: PULONG, pulProblemNumber: PULONG, dnDevInst: DEVINST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Status_Ex(
        pulStatus: PULONG, pulProblemNumber: PULONG, dnDevInst: DEVINST, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_IDA(
        dnDevInst: DEVINST, Buffer: PSTR, BufferLen: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_IDW(
        dnDevInst: DEVINST, Buffer: PWSTR, BufferLen: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_ExA(
        dnDevInst: DEVINST, Buffer: PSTR, BufferLen: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_ExW(
        dnDevInst: DEVINST, Buffer: PWSTR, BufferLen: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_ListA(
        pszFilter: PCSTR, Buffer: PCHAR, BufferLen: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_ListW(
        pszFilter: PCWSTR, Buffer: PWCHAR, BufferLen: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_ExA(
        pszFilter: PCSTR, Buffer: PCHAR, BufferLen: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_ExW(
        pszFilter: PCWSTR, Buffer: PWCHAR, BufferLen: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_SizeA(
        pulLen: PULONG, pszFilter: PCSTR, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_SizeW(
        pulLen: PULONG, pszFilter: PCWSTR, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_Size_ExA(
        pulLen: PULONG, pszFilter: PCSTR, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_Size_ExW(
        pulLen: PULONG, pszFilter: PCWSTR, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_Size(pulLen: PULONG, dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Get_Device_ID_Size_Ex(
        pulLen: PULONG, dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_AliasA(
        pszDeviceInterface: LPCSTR, AliasInterfaceGuid: LPGUID, pszAliasDeviceInterface: LPSTR,
        pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_AliasW(
        pszDeviceInterface: LPCWSTR, AliasInterfaceGuid: LPGUID, pszAliasDeviceInterface: LPWSTR,
        pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_Alias_ExA(
        pszDeviceInterface: LPCSTR, AliasInterfaceGuid: LPGUID, pszAliasDeviceInterface: LPSTR,
        pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_Alias_ExW(
        pszDeviceInterface: LPCWSTR, AliasInterfaceGuid: LPGUID, pszAliasDeviceInterface: LPWSTR,
        pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_ListA(
        InterfaceClassGuid: LPGUID, pDeviceID: DEVINSTID_A, Buffer: PCHAR, BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_ListW(
        InterfaceClassGuid: LPGUID, pDeviceID: DEVINSTID_W, Buffer: PWCHAR, BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_ExA(
        InterfaceClassGuid: LPGUID, pDeviceID: DEVINSTID_A, Buffer: PCHAR, BufferLen: ULONG,
        ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_ExW(
        InterfaceClassGuid: LPGUID, pDeviceID: DEVINSTID_W, Buffer: PWCHAR, BufferLen: ULONG,
        ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_SizeA(
        pulLen: PULONG, InterfaceClassGuid: LPGUID, pDeviceID: DEVINSTID_A, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_SizeW(
        pulLen: PULONG, InterfaceClassGuid: LPGUID, pDeviceID: DEVINSTID_W, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_Size_ExA(
        pulLen: PULONG, InterfaceClassGuid: LPGUID, pDeviceID: DEVINSTID_A, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_Size_ExW(
        pulLen: PULONG, InterfaceClassGuid: LPGUID, pDeviceID: DEVINSTID_W, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_First_Log_Conf(
        plcLogConf: PLOG_CONF, dnDevInst: DEVINST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_First_Log_Conf_Ex(
        plcLogConf: PLOG_CONF, dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Global_State(pulState: PULONG, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Get_Global_State_Ex(
        pulState: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_HW_Prof_FlagsA(
        pDeviceID: DEVINSTID_A, ulHardwareProfile: ULONG, pulValue: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_HW_Prof_FlagsW(
        pDeviceID: DEVINSTID_W, ulHardwareProfile: ULONG, pulValue: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_HW_Prof_Flags_ExA(
        pDeviceID: DEVINSTID_A, ulHardwareProfile: ULONG, pulValue: PULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_HW_Prof_Flags_ExW(
        pDeviceID: DEVINSTID_W, ulHardwareProfile: ULONG, pulValue: PULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Hardware_Profile_InfoA(
        ulIndex: ULONG, pHWProfileInfo: PHWPROFILEINFO_A, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Hardware_Profile_InfoW(
        ulIndex: ULONG, pHWProfileInfo: PHWPROFILEINFO_W, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Hardware_Profile_Info_ExA(
        ulIndex: ULONG, pHWProfileInfo: PHWPROFILEINFO_A, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Hardware_Profile_Info_ExW(
        ulIndex: ULONG, pHWProfileInfo: PHWPROFILEINFO_W, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Log_Conf_Priority(
        lcLogConf: LOG_CONF, pPriority: PRIORITY, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Log_Conf_Priority_Ex(
        lcLogConf: LOG_CONF, pPriority: PRIORITY, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Next_Log_Conf(
        plcLogConf: PLOG_CONF, lcLogConf: LOG_CONF, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Next_Log_Conf_Ex(
        plcLogConf: PLOG_CONF, lcLogConf: LOG_CONF, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Next_Res_Des(
        prdResDes: PRES_DES, rdResDes: RES_DES, ForResource: RESOURCEID, pResourceID: PRESOURCEID,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Next_Res_Des_Ex(
        prdResDes: PRES_DES, rdResDes: RES_DES, ForResource: RESOURCEID, pResourceID: PRESOURCEID,
        ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Parent(pdnDevInst: PDEVINST, dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Get_Parent_Ex(
        pdnDevInst: PDEVINST, dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Res_Des_Data(
        rdResDes: RES_DES, Buffer: PVOID, BufferLen: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Res_Des_Data_Ex(
        rdResDes: RES_DES, Buffer: PVOID, BufferLen: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Res_Des_Data_Size(
        pulSize: PULONG, rdResDes: RES_DES, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Res_Des_Data_Size_Ex(
        pulSize: PULONG, rdResDes: RES_DES, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Resource_Conflict_Count(
        clConflictList: CONFLICT_LIST, pulCount: PULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Resource_Conflict_DetailsA(
        clConflictList: CONFLICT_LIST, ulIndex: ULONG, pConflictDetails: PCONFLICT_DETAILS_A,
    ) -> CONFIGRET;
    pub fn CM_Get_Resource_Conflict_DetailsW(
        clConflictList: CONFLICT_LIST, ulIndex: ULONG, pConflictDetails: PCONFLICT_DETAILS_W,
    ) -> CONFIGRET;
    pub fn CM_Get_Sibling(pdnDevInst: PDEVINST, dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Get_Sibling_Ex(
        pdnDevInst: PDEVINST, dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Version() -> WORD;
    pub fn CM_Get_Version_Ex(hMachine: HMACHINE) -> WORD;
    pub fn CM_Intersect_Range_List(
        rlhOld1: RANGE_LIST, rlhOld2: RANGE_LIST, rlhNew: RANGE_LIST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Invert_Range_List(
        rlhOld: RANGE_LIST, rlhNew: RANGE_LIST, ullMaxValue: DWORDLONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Is_Dock_Station_Present(pbPresent: PBOOL) -> CONFIGRET;
    pub fn CM_Is_Dock_Station_Present_Ex(pbPresent: PBOOL, hMachine: HMACHINE) -> CONFIGRET;
    pub fn CM_Is_Version_Available(wVersion: WORD) -> BOOL;
    pub fn CM_Is_Version_Available_Ex(wVersion: WORD, hMachine: HMACHINE) -> BOOL;
    pub fn CM_Locate_DevNodeA(
        pdnDevInst: PDEVINST, pDeviceID: DEVINSTID_A, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Locate_DevNodeW(
        pdnDevInst: PDEVINST, pDeviceID: DEVINSTID_W, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Locate_DevNode_ExA(
        pdnDevInst: PDEVINST, pDeviceID: DEVINSTID_A, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Locate_DevNode_ExW(
        pdnDevInst: PDEVINST, pDeviceID: DEVINSTID_W, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Merge_Range_List(
        rlhOld1: RANGE_LIST, rlhOld2: RANGE_LIST, rlhNew: RANGE_LIST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Modify_Res_Des(
        prdResDes: PRES_DES, rdResDes: RES_DES, ResourceID: RESOURCEID, ResourceData: PCVOID,
        ResourceLen: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Modify_Res_Des_Ex(
        prdResDes: PRES_DES, rdResDes: RES_DES, ResourceID: RESOURCEID, ResourceData: PCVOID,
        ResourceLen: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Move_DevNode(
        dnFromDevInst: DEVINST, dnToDevInst: DEVINST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Move_DevNode_Ex(
        dnFromDevInst: DEVINST, dnToDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Next_Range(
        preElement: PRANGE_LIST, pullStart: PDWORDLONG, pullEnd: PDWORDLONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Class_KeyA(
        ClassGuid: LPGUID, pszClassName: LPCSTR, samDesired: REGSAM, Disposition: REGDISPOSITION,
        phkClass: PHKEY, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Class_KeyW(
        ClassGuid: LPGUID, pszClassName: LPCWSTR, samDesired: REGSAM, Disposition: REGDISPOSITION,
        phkClass: PHKEY, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Class_Key_ExA(
        ClassGuid: LPGUID, pszClassName: LPCSTR, samDesired: REGSAM, Disposition: REGDISPOSITION,
        phkClass: PHKEY, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_Class_Key_ExW(
        ClassGuid: LPGUID, pszClassName: LPCWSTR, samDesired: REGSAM, Disposition: REGDISPOSITION,
        phkClass: PHKEY, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_DevNode_Key(
        dnDevNode: DEVINST, samDesired: REGSAM, ulHardwareProfile: ULONG,
        Disposition: REGDISPOSITION, phkDevice: PHKEY, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_DevNode_Key_Ex(
        dnDevNode: DEVINST, samDesired: REGSAM, ulHardwareProfile: ULONG,
        Disposition: REGDISPOSITION, phkDevice: PHKEY, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_Device_Interface_KeyA(
        pszDeviceInterface: LPCSTR, samDesired: REGSAM, Disposition: REGDISPOSITION,
        phkDeviceInterface: PHKEY, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Device_Interface_KeyW(
        pszDeviceInterface: LPCWSTR, samDesired: REGSAM, Disposition: REGDISPOSITION,
        phkDeviceInterface: PHKEY, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Device_Interface_Key_ExA(
        pszDeviceInterface: LPCSTR, samDesired: REGSAM, Disposition: REGDISPOSITION,
        phkDeviceInterface: PHKEY, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_Device_Interface_Key_ExW(
        pszDeviceInterface: LPCWSTR, samDesired: REGSAM, Disposition: REGDISPOSITION,
        phkDeviceInterface: PHKEY, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_And_Remove_SubTreeA(
        dnAncestor: DEVINST, pVetoType: PPNP_VETO_TYPE, pszVetoName: LPSTR, ulNameLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_And_Remove_SubTreeW(
        dnAncestor: DEVINST, pVetoType: PPNP_VETO_TYPE, pszVetoName: LPWSTR, ulNameLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_And_Remove_SubTree_ExA(
        dnAncestor: DEVINST, pVetoType: PPNP_VETO_TYPE, pszVetoName: LPSTR, ulNameLength: ULONG,
        ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_And_Remove_SubTree_ExW(
        dnAncestor: DEVINST, pVetoType: PPNP_VETO_TYPE, pszVetoName: LPWSTR, ulNameLength: ULONG,
        ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_Arbitrator_Free_Data(
        pData: PVOID, DataLen: ULONG, dnDevInst: DEVINST, ResourceID: RESOURCEID, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_Arbitrator_Free_Data_Ex(
        pData: PVOID, DataLen: ULONG, dnDevInst: DEVINST, ResourceID: RESOURCEID, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_Arbitrator_Free_Size(
        pulSize: PULONG, dnDevInst: DEVINST, ResourceID: RESOURCEID, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_Arbitrator_Free_Size_Ex(
        pulSize: PULONG, dnDevInst: DEVINST, ResourceID: RESOURCEID, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_Remove_SubTree(dnAncestor: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Query_Remove_SubTree_Ex(
        dnAncestor: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_Resource_Conflict_List(
        pclConflictList: PCONFLICT_LIST, dnDevInst: DEVINST, ResourceID: RESOURCEID,
        ResourceData: PCVOID, ResourceLen: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Reenumerate_DevNode(dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Reenumerate_DevNode_Ex(
        dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_Driver(dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Register_Device_Driver_Ex(
        dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_InterfaceA(
        dnDevInst: DEVINST, InterfaceClassGuid: LPGUID, pszReference: LPCSTR,
        pszDeviceInterface: LPSTR, pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_InterfaceW(
        dnDevInst: DEVINST, InterfaceClassGuid: LPGUID, pszReference: LPCWSTR,
        pszDeviceInterface: LPWSTR, pulLength: PULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_Interface_ExA(
        dnDevInst: DEVINST, InterfaceClassGuid: LPGUID, pszReference: LPCSTR,
        pszDeviceInterface: LPSTR, pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_Interface_ExW(
        dnDevInst: DEVINST, InterfaceClassGuid: LPGUID, pszReference: LPCWSTR,
        pszDeviceInterface: LPWSTR, pulLength: PULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Remove_SubTree(dnAncestor: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Remove_SubTree_Ex(
        dnAncestor: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Request_Device_EjectA(
        dnDevInst: DEVINST, pVetoType: PPNP_VETO_TYPE, pszVetoName: LPSTR, ulNameLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Request_Device_EjectW(
        dnDevInst: DEVINST, pVetoType: PPNP_VETO_TYPE, pszVetoName: LPWSTR, ulNameLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Request_Device_Eject_ExA(
        dnDevInst: DEVINST, pVetoType: PPNP_VETO_TYPE, pszVetoName: LPSTR, ulNameLength: ULONG,
        ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Request_Device_Eject_ExW(
        dnDevInst: DEVINST, pVetoType: PPNP_VETO_TYPE, pszVetoName: LPWSTR, ulNameLength: ULONG,
        ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Request_Eject_PC() -> CONFIGRET;
    pub fn CM_Request_Eject_PC_Ex(hMachine: HMACHINE) -> CONFIGRET;
    pub fn CM_Run_Detection(ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Run_Detection_Ex(ulFlags: ULONG, hMachine: HMACHINE) -> CONFIGRET;
    pub fn CM_Set_Class_Registry_PropertyA(
        ClassGuid: LPGUID, ulProperty: ULONG, Buffer: PCVOID, ulLength: ULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_Class_Registry_PropertyW(
        ClassGuid: LPGUID, ulProperty: ULONG, Buffer: PCVOID, ulLength: ULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Problem(
        dnDevInst: DEVINST, ulProblem: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Problem_Ex(
        dnDevInst: DEVINST, ulProblem: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Registry_PropertyA(
        dnDevInst: DEVINST, ulProperty: ULONG, Buffer: PCVOID, ulLength: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Registry_PropertyW(
        dnDevInst: DEVINST, ulProperty: ULONG, Buffer: PCVOID, ulLength: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Registry_Property_ExA(
        dnDevInst: DEVINST, ulProperty: ULONG, Buffer: PCVOID, ulLength: ULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Registry_Property_ExW(
        dnDevInst: DEVINST, ulProperty: ULONG, Buffer: PCVOID, ulLength: ULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof(ulHardwareProfile: ULONG, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_Ex(
        ulHardwareProfile: ULONG, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_FlagsA(
        pDeviceID: DEVINSTID_A, ulConfig: ULONG, ulValue: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_FlagsW(
        pDeviceID: DEVINSTID_W, ulConfig: ULONG, ulValue: ULONG, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_Flags_ExA(
        pDeviceID: DEVINSTID_A, ulConfig: ULONG, ulValue: ULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_Flags_ExW(
        pDeviceID: DEVINSTID_A, ulConfig: ULONG, ulValue: ULONG, ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Setup_DevNode(dnDevInst: DEVINST, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Setup_DevNode_Ex(
        dnDevInst: DEVINST, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Test_Range_Available(
        ullStartValue: DWORDLONG, ullEndValue: DWORDLONG, rlh: RANGE_LIST, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Uninstall_DevNode(dnDevInst: DEVNODE, ulFlags: ULONG) -> CONFIGRET;
    pub fn CM_Uninstall_DevNode_Ex(
        dnDevInst: DEVNODE, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Unregister_Device_InterfaceA(
        pszDeviceInterface: LPCSTR, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Unregister_Device_InterfaceW(
        pszDeviceInterface: LPCWSTR, ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Unregister_Device_Interface_ExA(
        pszDeviceInterface: LPCSTR, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Unregister_Device_Interface_ExW(
        pszDeviceInterface: LPCWSTR, ulFlags: ULONG, hMachine: HMACHINE,
    ) -> CONFIGRET;
    // pub fn ExtensionPropSheetPageProc();
    // pub fn SetupDiMoveDuplicateDevice();
    // pub fn SetupDiReportAdditionalSoftwareRequested();
    // pub fn SetupDiReportDeviceInstallError();
    // pub fn SetupDiReportDriverNotFoundError();
    // pub fn SetupDiReportDriverPackageImportationError();
    // pub fn SetupDiReportGenericDriverInstalled();
    // pub fn SetupDiReportPnPDeviceProblem();
    // pub fn SetupGetInfSections();
    // pub fn pSetupDiCrimsonLogDeviceInstall();
}
