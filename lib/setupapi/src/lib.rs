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
    pub fn InstallHinfSection(
        Window: HWND, ModuleHandle: HINSTANCE, CommandLine: PCSTR, ShowCommand: INT,
    );
    pub fn InstallHinfSectionA(
        Window: HWND, ModuleHandle: HINSTANCE, CommandLine: PCSTR, ShowCommand: INT,
    );
    pub fn InstallHinfSectionW(
        Window: HWND, ModuleHandle: HINSTANCE, CommandLine: PCWSTR, ShowCommand: INT,
    );
    pub fn SetupAddInstallSectionToDiskSpaceListA(
        DiskSpace: HDSKSPC, InfHandle: HINF, LayoutInfHandle: HINF, SectionName: PCSTR,
        Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddInstallSectionToDiskSpaceListW(
        DiskSpace: HDSKSPC, InfHandle: HINF, LayoutInfHandle: HINF, SectionName: PCWSTR,
        Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddSectionToDiskSpaceListA(
        DiskSpace: HDSKSPC, InfHandle: HINF, ListInfHandle: HINF, SectionName: PCSTR,
        Operation: UINT, Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddSectionToDiskSpaceListW(
        DiskSpace: HDSKSPC, InfHandle: HINF, ListInfHandle: HINF, SectionName: PCWSTR,
        Operation: UINT, Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddToDiskSpaceListA(
        DiskSpace: HDSKSPC, TargetFilespec: PCSTR, FileSize: LONGLONG, Operation: UINT,
        Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddToDiskSpaceListW(
        DiskSpace: HDSKSPC, TargetFilespec: PCWSTR, FileSize: LONGLONG, Operation: UINT,
        Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAddToSourceListA(Flags: DWORD, Source: PCSTR) -> BOOL;
    pub fn SetupAddToSourceListW(Flags: DWORD, Source: PCWSTR) -> BOOL;
    pub fn SetupAdjustDiskSpaceListA(
        DiskSpace: HDSKSPC, DriveRoot: LPCSTR, Amount: LONGLONG, Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupAdjustDiskSpaceListW(
        DiskSpace: HDSKSPC, DriveRoot: LPCWSTR, Amount: LONGLONG, Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupBackupErrorA(
        hwndParent: HWND, DialogTitle: PCSTR, SourceFile: PCSTR, TargetFile: PCSTR,
        Win32ErrorCode: UINT, Style: DWORD,
    ) -> UINT;
    pub fn SetupBackupErrorW(
        hwndParent: HWND, DialogTitle: PCWSTR, SourceFile: PCWSTR, TargetFile: PCWSTR,
        Win32ErrorCode: UINT, Style: DWORD,
    ) -> UINT;
    pub fn SetupCancelTemporarySourceList() -> BOOL;
    pub fn SetupCloseFileQueue(QueueHandle: HSPFILEQ) -> BOOL;
    pub fn SetupCloseInfFile(InfHandle: HINF);
    pub fn SetupCloseLog();
    pub fn SetupCommitFileQueue(
        Owner: HWND, QueueHandle: HSPFILEQ, MsgHandler: PSP_FILE_CALLBACK_A, Context: PVOID,
    ) -> BOOL;
    pub fn SetupCommitFileQueueA(
        Owner: HWND, QueueHandle: HSPFILEQ, MsgHandler: PSP_FILE_CALLBACK_A, Context: PVOID,
    ) -> BOOL;
    pub fn SetupCommitFileQueueW(
        Owner: HWND, QueueHandle: HSPFILEQ, MsgHandler: PSP_FILE_CALLBACK_W, Context: PVOID,
    ) -> BOOL;
    pub fn SetupConfigureWmiFromInfSectionA(
        InfHandle: HINF, SectionName: PCSTR, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupConfigureWmiFromInfSectionW(
        InfHandle: HINF, SectionName: PCWSTR, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupCopyErrorA(
        hwndParent: HWND, DialogTitle: PCSTR, DiskName: PCSTR, PathToSource: PCSTR,
        SourceFile: PCSTR, TargetPathFile: PCSTR, Win32ErrorCode: UINT, Style: DWORD,
        PathBuffer: PSTR, PathBufferSize: DWORD, PathRequiredSize: PDWORD,
    ) -> UINT;
    pub fn SetupCopyErrorW(
        hwndParent: HWND, DialogTitle: PCWSTR, DiskName: PCWSTR, PathToSource: PCWSTR,
        SourceFile: PCWSTR, TargetPathFile: PCWSTR, Win32ErrorCode: UINT, Style: DWORD,
        PathBuffer: PWSTR, PathBufferSize: DWORD, PathRequiredSize: PDWORD,
    ) -> UINT;
    pub fn SetupCopyOEMInfA(
        SourceInfFileName: PCSTR, OEMSourceMediaLocation: PCSTR, OEMSourceMediaType: DWORD,
        CopyStyle: DWORD, DestinationInfFileName: PSTR, DestinationInfFileNameSize: DWORD,
        RequiredSize: PDWORD, DestinationInfFileNameComponent: *mut PSTR,
    ) -> BOOL;
    pub fn SetupCopyOEMInfW(
        SourceInfFileName: PCWSTR, OEMSourceMediaLocation: PCWSTR, OEMSourceMediaType: DWORD,
        CopyStyle: DWORD, DestinationInfFileName: PWSTR, DestinationInfFileNameSize: DWORD,
        RequiredSize: PDWORD, DestinationInfFileNameComponent: *mut PWSTR,
    ) -> BOOL;
    pub fn SetupCreateDiskSpaceListA(Reserved1: PVOID, Reserved2: DWORD, Flags: UINT) -> HDSKSPC;
    pub fn SetupCreateDiskSpaceListW(Reserved1: PVOID, Reserved2: DWORD, Flags: UINT) -> HDSKSPC;
    pub fn SetupDecompressOrCopyFileA(
        SourceFileName: PCSTR, TargetFileName: PCSTR, CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupDecompressOrCopyFileW(
        SourceFileName: PCWSTR, TargetFileName: PCWSTR, CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupDefaultQueueCallback(
        Context: PVOID, Notification: UINT, Param1: UINT_PTR, Param2: UINT_PTR,
    ) -> UINT;
    pub fn SetupDefaultQueueCallbackA(
        Context: PVOID, Notification: UINT, Param1: UINT_PTR, Param2: UINT_PTR,
    ) -> UINT;
    pub fn SetupDefaultQueueCallbackW(
        Context: PVOID, Notification: UINT, Param1: UINT_PTR, Param2: UINT_PTR,
    ) -> UINT;
    pub fn SetupDeleteErrorA(
        hwndParent: HWND, DialogTitle: PCSTR, File: PCSTR, Win32ErrorCode: UINT, Style: DWORD,
    ) -> UINT;
    pub fn SetupDeleteErrorW(
        hwndParent: HWND, DialogTitle: PCWSTR, File: PCWSTR, Win32ErrorCode: UINT, Style: DWORD,
    ) -> UINT;
    pub fn SetupDestroyDiskSpaceList(DiskSpace: HDSKSPC) -> BOOL;
    pub fn SetupDiAskForOEMDisk(DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA) -> BOOL;
    pub fn SetupDiBuildClassInfoList(
        Flags: DWORD, ClassGuidList: LPGUID, ClassGuidListSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiBuildClassInfoListExA(
        Flags: DWORD, ClassGuidList: LPGUID, ClassGuidListSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiBuildClassInfoListExW(
        Flags: DWORD, ClassGuidList: LPGUID, ClassGuidListSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiBuildDriverInfoList(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, DriverType: DWORD,
    ) -> BOOL;
    pub fn SetupDiCallClassInstaller(
        InstallFunction: DI_FUNCTION, DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiCancelDriverInfoSearch(DeviceInfoSet: HDEVINFO) -> BOOL;
    pub fn SetupDiChangeState(DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA) -> BOOL;
    pub fn SetupDiClassGuidsFromNameA(
        ClassName: PCSTR, ClassGuidList: LPGUID, ClassGuidListSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiClassGuidsFromNameExA(
        ClassName: PCSTR, ClassGuidList: LPGUID, ClassGuidListSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiClassGuidsFromNameExW(
        ClassName: PCWSTR, ClassGuidList: LPGUID, ClassGuidListSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiClassGuidsFromNameW(
        ClassName: PCWSTR, ClassGuidList: LPGUID, ClassGuidListSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiClassNameFromGuidA(
        ClassGuid: *const GUID, ClassName: PSTR, ClassNameSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiClassNameFromGuidExA(
        ClassGuid: *const GUID, ClassName: PSTR, ClassNameSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiClassNameFromGuidExW(
        ClassGuid: *const GUID, ClassName: PWSTR, ClassNameSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiClassNameFromGuidW(
        ClassGuid: *const GUID, ClassName: PWSTR, ClassNameSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiCreateDevRegKeyA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Scope: DWORD, HwProfile: DWORD,
        KeyType: DWORD, InfHandle: HINF, InfSectionName: PCSTR,
    ) -> HKEY;
    pub fn SetupDiCreateDevRegKeyW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Scope: DWORD, HwProfile: DWORD,
        KeyType: DWORD, InfHandle: HINF, InfSectionName: PCWSTR,
    ) -> HKEY;
    pub fn SetupDiCreateDeviceInfoA(
        DeviceInfoSet: HDEVINFO, DeviceName: PCSTR, ClassGuid: *const GUID,
        DeviceDescription: PCSTR, hwndParent: HWND, CreationFlags: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiCreateDeviceInfoList(ClassGuid: *const GUID, hwndParent: HWND) -> HDEVINFO;
    pub fn SetupDiCreateDeviceInfoListExA(
        ClassGuid: *const GUID, hwndParent: HWND, MachineName: PCSTR, Reserved: PVOID,
    ) -> HDEVINFO;
    pub fn SetupDiCreateDeviceInfoListExW(
        ClassGuid: *const GUID, hwndParent: HWND, MachineName: PCWSTR, Reserved: PVOID,
    ) -> HDEVINFO;
    pub fn SetupDiCreateDeviceInfoW(
        DeviceInfoSet: HDEVINFO, DeviceName: PCWSTR, ClassGuid: *const GUID,
        DeviceDescription: PCWSTR, hwndParent: HWND, CreationFlags: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiCreateDeviceInterfaceA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, InterfaceClassGuid: *const GUID,
        ReferenceString: PCSTR, CreationFlags: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiCreateDeviceInterfaceRegKeyA(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA, Reserved: DWORD,
        samDesired: REGSAM, InfHandle: HINF, InfSectionName: PCSTR,
    ) -> HKEY;
    pub fn SetupDiCreateDeviceInterfaceRegKeyW(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA, Reserved: DWORD,
        samDesired: REGSAM, InfHandle: HINF, InfSectionName: PCWSTR,
    ) -> HKEY;
    pub fn SetupDiCreateDeviceInterfaceW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, InterfaceClassGuid: *const GUID,
        ReferenceString: PCWSTR, CreationFlags: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiDeleteDevRegKey(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Scope: DWORD, HwProfile: DWORD,
        KeyType: DWORD,
    ) -> BOOL;
    pub fn SetupDiDeleteDeviceInfo(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiDeleteDeviceInterfaceData(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiDeleteDeviceInterfaceRegKey(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA, Reserved: DWORD,
    ) -> BOOL;
    pub fn SetupDiDestroyClassImageList(ClassImageListData: PSP_CLASSIMAGELIST_DATA) -> BOOL;
    pub fn SetupDiDestroyDeviceInfoList(DeviceInfoSet: HDEVINFO) -> BOOL;
    pub fn SetupDiDestroyDriverInfoList(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, DriverType: DWORD,
    ) -> BOOL;
    pub fn SetupDiDrawMiniIcon(
        hdc: HDC, rc: RECT, MiniIconIndex: INT, Flags: DWORD,
    ) -> INT;
    pub fn SetupDiEnumDeviceInfo(
        DeviceInfoSet: HDEVINFO, MemberIndex: DWORD, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiEnumDeviceInterfaces(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, InterfaceClassGuid: *const GUID,
        MemberIndex: DWORD, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiEnumDriverInfoA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, DriverType: DWORD,
        MemberIndex: DWORD, DriverInfoData: PSP_DRVINFO_DATA_A,
    ) -> BOOL;
    pub fn SetupDiEnumDriverInfoW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, DriverType: DWORD,
        MemberIndex: DWORD, DriverInfoData: PSP_DRVINFO_DATA_W,
    ) -> BOOL;
    pub fn SetupDiGetActualModelsSectionA(
        Context: PINFCONTEXT, AlternatePlatformInfo: PSP_ALTPLATFORM_INFO, InfSectionWithExt: PSTR,
        InfSectionWithExtSize: DWORD, RequiredSize: PDWORD, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetActualModelsSectionW(
        Context: PINFCONTEXT, AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSectionWithExt: PWSTR, InfSectionWithExtSize: DWORD, RequiredSize: PDWORD,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetActualSectionToInstallA(
        InfHandle: HINF, InfSectionName: PCSTR, InfSectionWithExt: PSTR,
        InfSectionWithExtSize: DWORD, RequiredSize: PDWORD, Extension: *mut PSTR,
    ) -> BOOL;
    pub fn SetupDiGetActualSectionToInstallExA(
        InfHandle: HINF, InfSectionName: PCSTR, InfSectionWithExt: PSTR,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO, InfSectionWithExtSize: DWORD,
        RequiredSize: PDWORD, Extension: *mut PSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetActualSectionToInstallExW(
        InfHandle: HINF, InfSectionName: PCWSTR, InfSectionWithExt: PWSTR,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO, InfSectionWithExtSize: DWORD,
        RequiredSize: PDWORD, Extension: *mut PWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetActualSectionToInstallW(
        InfHandle: HINF, InfSectionName: PCWSTR, InfSectionWithExt: PWSTR,
        InfSectionWithExtSize: DWORD, RequiredSize: PDWORD, Extension: *mut PWSTR,
    ) -> BOOL;
    pub fn SetupDiGetClassBitmapIndex(ClassGuid: *const GUID, MiniIconIndex: PINT) -> BOOL;
    pub fn SetupDiGetClassDescriptionA(
        ClassGuid: *const GUID, ClassDescription: PSTR, ClassDescriptionSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassDescriptionExA(
        ClassGuid: *const GUID, ClassDescription: PSTR, ClassDescriptionSize: DWORD,
        RequiredSize: PDWORD, MachineName: PCSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassDescriptionExW(
        ClassGuid: *const GUID, ClassDescription: PWSTR, ClassDescriptionSize: DWORD,
        RequiredSize: PDWORD, MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassDescriptionW(
        ClassGuid: *const GUID, ClassDescription: PWSTR, ClassDescriptionSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassDevPropertySheetsA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        PropertySheetHeader: LPPROPSHEETHEADERA, PropertySheetHeaderPageListSize: DWORD,
        RequiredSize: PDWORD, PropertySheetType: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassDevPropertySheetsW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        PropertySheetHeader: LPPROPSHEETHEADERW, PropertySheetHeaderPageListSize: DWORD,
        RequiredSize: PDWORD, PropertySheetType: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassDevsA(
        ClassGuid: *const GUID, Enumerator: PCSTR, hwndParent: HWND, Flags: DWORD,
    ) -> HDEVINFO;
    pub fn SetupDiGetClassDevsExA(
        ClassGuid: *const GUID, Enumerator: PCSTR, hwndParent: HWND, Flags: DWORD,
        DeviceInfoSet: HDEVINFO, MachineName: PCSTR, Reserved: PVOID,
    ) -> HDEVINFO;
    pub fn SetupDiGetClassDevsExW(
        ClassGuid: *const GUID, Enumerator: PCWSTR, hwndParent: HWND, Flags: DWORD,
        DeviceInfoSet: HDEVINFO, MachineName: PCWSTR, Reserved: PVOID,
    ) -> HDEVINFO;
    pub fn SetupDiGetClassDevsW(
        ClassGuid: *const GUID, Enumerator: PCWSTR, hwndParent: HWND, Flags: DWORD,
    ) -> HDEVINFO;
    pub fn SetupDiGetClassImageIndex(
        ClassImageListData: PSP_CLASSIMAGELIST_DATA, ClassGuid: *const GUID, ImageIndex: PINT,
    ) -> BOOL;
    pub fn SetupDiGetClassImageList(ClassImageListData: PSP_CLASSIMAGELIST_DATA) -> BOOL;
    pub fn SetupDiGetClassImageListExA(
        ClassImageListData: PSP_CLASSIMAGELIST_DATA, MachineName: PCSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassImageListExW(
        ClassImageListData: PSP_CLASSIMAGELIST_DATA, MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassInstallParamsA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        ClassInstallParams: PSP_CLASSINSTALL_HEADER, ClassInstallParamsSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassInstallParamsW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        ClassInstallParams: PSP_CLASSINSTALL_HEADER, ClassInstallParamsSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassPropertyExW(
        ClassGuid: *const GUID, PropertyKey: *const DEVPROPKEY, PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE, PropertyBufferSize: DWORD, RequiredSize: PDWORD, Flags: DWORD,
        MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassPropertyKeys(
        ClassGuid: *const GUID, PropertyKeyArray: *mut DEVPROPKEY, PropertyKeyCount: DWORD,
        RequiredPropertyKeyCount: PDWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassPropertyKeysExW(
        ClassGuid: *const GUID, PropertyKeyArray: *mut DEVPROPKEY, PropertyKeyCount: DWORD,
        RequiredPropertyKeyCount: PDWORD, Flags: DWORD, MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassPropertyW(
        ClassGuid: *const GUID, PropertyKey: *const DEVPROPKEY, PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE, PropertyBufferSize: DWORD, RequiredSize: PDWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetClassRegistryPropertyA(
        ClassGuid: *const GUID, Property: DWORD, PropertyRegDataType: PDWORD,
        PropertyBuffer: PBYTE, PropertyBufferSize: DWORD, RequiredSize: PDWORD, MachineName: PCSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetClassRegistryPropertyW(
        ClassGuid: *const GUID, Property: DWORD, PropertyRegDataType: PDWORD,
        PropertyBuffer: PBYTE, PropertyBufferSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetCustomDevicePropertyA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, CustomPropertyName: PCSTR,
        Flags: DWORD, PropertyRegDataType: PDWORD, PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetCustomDevicePropertyW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, CustomPropertyName: PCWSTR,
        Flags: DWORD, PropertyRegDataType: PDWORD, PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInfoListClass(DeviceInfoSet: HDEVINFO, ClassGuid: LPGUID) -> BOOL;
    pub fn SetupDiGetDeviceInfoListDetailA(
        DeviceInfoSet: HDEVINFO, DeviceInfoSetDetailData: PSP_DEVINFO_LIST_DETAIL_DATA_A,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInfoListDetailW(
        DeviceInfoSet: HDEVINFO, DeviceInfoSetDetailData: PSP_DEVINFO_LIST_DETAIL_DATA_W,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInstallParamsA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstallParams: PSP_DEVINSTALL_PARAMS_A,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInstallParamsW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstallParams: PSP_DEVINSTALL_PARAMS_W,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInstanceIdA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, DeviceInstanceId: PSTR,
        DeviceInstanceIdSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInstanceIdW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, DeviceInstanceId: PWSTR,
        DeviceInstanceIdSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfaceAlias(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        AliasInterfaceClassGuid: *const GUID, AliasDeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfaceDetailA(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        DeviceInterfaceDetailData: PSP_DEVICE_INTERFACE_DETAIL_DATA_A,
        DeviceInterfaceDetailDataSize: DWORD, RequiredSize: PDWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfaceDetailW(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        DeviceInterfaceDetailData: PSP_DEVICE_INTERFACE_DETAIL_DATA_W,
        DeviceInterfaceDetailDataSize: DWORD, RequiredSize: PDWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfacePropertyKeys(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        PropertyKeyArray: *mut DEVPROPKEY, PropertyKeyCount: DWORD,
        RequiredPropertyKeyCount: PDWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceInterfacePropertyW(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        PropertyKey: *const DEVPROPKEY, PropertyType: *mut DEVPROPTYPE, PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD, RequiredSize: PDWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetDevicePropertyKeys(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        PropertyKeyArray: *mut DEVPROPKEY, PropertyKeyCount: DWORD,
        RequiredPropertyKeyCount: PDWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetDevicePropertyW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE, PropertyBuffer: PBYTE, PropertyBufferSize: DWORD,
        RequiredSize: PDWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceRegistryPropertyA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Property: DWORD,
        PropertyRegDataType: PDWORD, PropertyBuffer: PBYTE, PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDeviceRegistryPropertyW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Property: DWORD,
        PropertyRegDataType: PDWORD, PropertyBuffer: PBYTE, PropertyBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDriverInfoDetailA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A, DriverInfoDetailData: PSP_DRVINFO_DETAIL_DATA_A,
        DriverInfoDetailDataSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDriverInfoDetailW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W, DriverInfoDetailData: PSP_DRVINFO_DETAIL_DATA_W,
        DriverInfoDetailDataSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetDriverInstallParamsA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A, DriverInstallParams: PSP_DRVINSTALL_PARAMS,
    ) -> BOOL;
    pub fn SetupDiGetDriverInstallParamsW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W, DriverInstallParams: PSP_DRVINSTALL_PARAMS,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileFriendlyNameA(
        HwProfile: DWORD, FriendlyName: PSTR, FriendlyNameSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileFriendlyNameExA(
        HwProfile: DWORD, FriendlyName: PSTR, FriendlyNameSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileFriendlyNameExW(
        HwProfile: DWORD, FriendlyName: PWSTR, FriendlyNameSize: DWORD, RequiredSize: PDWORD,
        MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileFriendlyNameW(
        HwProfile: DWORD, FriendlyName: PWSTR, FriendlyNameSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileList(
        HwProfileList: PDWORD, HwProfileListSize: DWORD, RequiredSize: PDWORD,
        CurrentlyActiveIndex: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileListExA(
        HwProfileList: PDWORD, HwProfileListSize: DWORD, RequiredSize: PDWORD,
        CurrentlyActiveIndex: PDWORD, MachineName: PCSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetHwProfileListExW(
        HwProfileList: PDWORD, HwProfileListSize: DWORD, RequiredSize: PDWORD,
        CurrentlyActiveIndex: PDWORD, MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiGetINFClassA(
        InfName: PCSTR, ClassGuid: LPGUID, ClassName: PSTR, ClassNameSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetINFClassW(
        InfName: PCWSTR, ClassGuid: LPGUID, ClassName: PWSTR, ClassNameSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupDiGetSelectedDevice(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiGetSelectedDriverA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A,
    ) -> BOOL;
    pub fn SetupDiGetSelectedDriverW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W,
    ) -> BOOL;
    pub fn SetupDiGetWizardPage(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        InstallWizardData: PSP_INSTALLWIZARD_DATA, PageType: DWORD, Flags: DWORD,
    ) -> HPROPSHEETPAGE;
    pub fn SetupDiInstallClassA(
        hwndParent: HWND, InfFileName: PCSTR, Flags: DWORD, FileQueue: HSPFILEQ,
    ) -> BOOL;
    pub fn SetupDiInstallClassExA(
        hwndParent: HWND, InfFileName: PCSTR, Flags: DWORD, FileQueue: HSPFILEQ,
        InterfaceClassGuid: *const GUID, Reserved1: PVOID, Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupDiInstallClassExW(
        hwndParent: HWND, InfFileName: PCWSTR, Flags: DWORD, FileQueue: HSPFILEQ,
        InterfaceClassGuid: *const GUID, Reserved1: PVOID, Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupDiInstallClassW(
        hwndParent: HWND, InfFileName: PCWSTR, Flags: DWORD, FileQueue: HSPFILEQ,
    ) -> BOOL;
    pub fn SetupDiInstallDevice(DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA) -> BOOL;
    pub fn SetupDiInstallDeviceInterfaces(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiInstallDriverFiles(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiLoadClassIcon(
        ClassGuid: *const GUID, LargeIcon: *mut HICON, MiniIconIndex: PINT,
    ) -> BOOL;
    pub fn SetupDiLoadDeviceIcon(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, cxIcon: UINT, cyIcon: UINT,
        Flags: DWORD, hIcon: *mut HICON,
    ) -> BOOL;
    // pub fn SetupDiMoveDuplicateDevice();
    pub fn SetupDiOpenClassRegKey(ClassGuid: *const GUID, samDesired: REGSAM) -> HKEY;
    pub fn SetupDiOpenClassRegKeyExA(
        ClassGuid: *const GUID, samDesired: REGSAM, Flags: DWORD, MachineName: PCSTR,
        Reserved: PVOID,
    ) -> HKEY;
    pub fn SetupDiOpenClassRegKeyExW(
        ClassGuid: *const GUID, samDesired: REGSAM, Flags: DWORD, MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> HKEY;
    pub fn SetupDiOpenDevRegKey(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Scope: DWORD, HwProfile: DWORD,
        KeyType: DWORD, samDesired: REGSAM,
    ) -> HKEY;
    pub fn SetupDiOpenDeviceInfoA(
        DeviceInfoSet: HDEVINFO, DeviceInstanceId: PCSTR, hwndParent: HWND, OpenFlags: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiOpenDeviceInfoW(
        DeviceInfoSet: HDEVINFO, DeviceInstanceId: PCWSTR, hwndParent: HWND, OpenFlags: DWORD,
        DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiOpenDeviceInterfaceA(
        DeviceInfoSet: HDEVINFO, DevicePath: PCSTR, OpenFlags: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiOpenDeviceInterfaceRegKey(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA, Reserved: DWORD,
        samDesired: REGSAM,
    ) -> HKEY;
    pub fn SetupDiOpenDeviceInterfaceW(
        DeviceInfoSet: HDEVINFO, DevicePath: PCWSTR, OpenFlags: DWORD,
        DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    pub fn SetupDiRegisterCoDeviceInstallers(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiRegisterDeviceInfo(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Flags: DWORD,
        CompareProc: PSP_DETSIG_CMPPROC, CompareContext: PVOID,
        DupDeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiRemoveDevice(DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA) -> BOOL;
    pub fn SetupDiRemoveDeviceInterface(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
    ) -> BOOL;
    // pub fn SetupDiReportAdditionalSoftwareRequested();
    // pub fn SetupDiReportDeviceInstallError();
    // pub fn SetupDiReportDriverNotFoundError();
    // pub fn SetupDiReportDriverPackageImportationError();
    // pub fn SetupDiReportGenericDriverInstalled();
    // pub fn SetupDiReportPnPDeviceProblem();
    pub fn SetupDiRestartDevices(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSelectBestCompatDrv(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSelectDevice(DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA) -> BOOL;
    pub fn SetupDiSelectOEMDrv(
        hwndParent: HWND, DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSetClassInstallParamsA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        ClassInstallParams: PSP_CLASSINSTALL_HEADER, ClassInstallParamsSize: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetClassInstallParamsW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        ClassInstallParams: PSP_CLASSINSTALL_HEADER, ClassInstallParamsSize: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetClassPropertyExW(
        ClassGuid: *const GUID, PropertyKey: *const DEVPROPKEY, PropertyType: DEVPROPTYPE,
        PropertyBuffer: PBYTE, PropertyBufferSize: DWORD, Flags: DWORD, MachineName: PCWSTR,
        Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiSetClassPropertyW(
        ClassGuid: *const GUID, PropertyKey: *const DEVPROPKEY, PropertyType: DEVPROPTYPE,
        PropertyBuffer: PBYTE, PropertyBufferSize: DWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetClassRegistryPropertyA(
        ClassGuid: *const GUID, Property: DWORD, PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD, MachineName: PCSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiSetClassRegistryPropertyW(
        ClassGuid: *const GUID, Property: DWORD, PropertyBuffer: *const BYTE,
        PropertyBufferSize: DWORD, MachineName: PCWSTR, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiSetDeviceInstallParamsA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstallParams: PSP_DEVINSTALL_PARAMS_A,
    ) -> BOOL;
    pub fn SetupDiSetDeviceInstallParamsW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DeviceInstallParams: PSP_DEVINSTALL_PARAMS_W,
    ) -> BOOL;
    pub fn SetupDiSetDeviceInterfaceDefault(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Flags: DWORD, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupDiSetDeviceInterfacePropertyW(
        DeviceInfoSet: HDEVINFO, DeviceInterfaceData: PSP_DEVICE_INTERFACE_DATA,
        PropertyKey: *const DEVPROPKEY, PropertyType: DEVPROPTYPE, PropertyBuffer: PBYTE,
        PropertyBufferSize: DWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetDevicePropertyW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, PropertyKey: *const DEVPROPKEY,
        PropertyType: DEVPROPTYPE, PropertyBuffer: PBYTE, PropertyBufferSize: DWORD, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetDeviceRegistryPropertyA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Property: DWORD,
        PropertyBuffer: *const BYTE, PropertyBufferSize: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetDeviceRegistryPropertyW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA, Property: DWORD,
        PropertyBuffer: *const BYTE, PropertyBufferSize: DWORD,
    ) -> BOOL;
    pub fn SetupDiSetDriverInstallParamsA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A, DriverInstallParams: PSP_DRVINSTALL_PARAMS,
    ) -> BOOL;
    pub fn SetupDiSetDriverInstallParamsW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W, DriverInstallParams: PSP_DRVINSTALL_PARAMS,
    ) -> BOOL;
    pub fn SetupDiSetSelectedDevice(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDiSetSelectedDriverA(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_A,
    ) -> BOOL;
    pub fn SetupDiSetSelectedDriverW(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
        DriverInfoData: PSP_DRVINFO_DATA_W,
    ) -> BOOL;
    pub fn SetupDiUnremoveDevice(
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupDuplicateDiskSpaceListA(
        DiskSpace: HDSKSPC, Reserved1: PVOID, Reserved2: DWORD, Flags: UINT,
    ) -> HDSKSPC;
    pub fn SetupDuplicateDiskSpaceListW(
        DiskSpace: HDSKSPC, Reserved1: PVOID, Reserved2: DWORD, Flags: UINT,
    ) -> HDSKSPC;
    pub fn SetupEnumInfSectionsA(
        InfHandle: HINF, Index: UINT, Buffer: PSTR, Size: UINT, SizeNeeded: *mut UINT,
    ) -> BOOL;
    pub fn SetupEnumInfSectionsW(
        InfHandle: HINF, Index: UINT, Buffer: PWSTR, Size: UINT, SizeNeeded: *mut UINT,
    ) -> BOOL;
    pub fn SetupFindFirstLineA(
        InfHandle: HINF, Section: PCSTR, Key: PCSTR, Context: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupFindFirstLineW(
        InfHandle: HINF, Section: PCWSTR, Key: PCWSTR, Context: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupFindNextLine(ContextIn: PINFCONTEXT, ContextOut: PINFCONTEXT) -> BOOL;
    pub fn SetupFindNextMatchLineA(
        ContextIn: PINFCONTEXT, Key: PCSTR, ContextOut: PINFCONTEXT
    ) -> BOOL;
    pub fn SetupFindNextMatchLineW(
        ContextIn: PINFCONTEXT, Key: PCWSTR, ContextOut: PINFCONTEXT
    ) -> BOOL;
    pub fn SetupFreeSourceListA(List: *mut *mut PCSTR, Count: UINT) -> BOOL;
    pub fn SetupFreeSourceListW(List: *mut *mut PCWSTR, Count: UINT) -> BOOL;
    pub fn SetupGetBackupInformationA(
        QueueHandle: HSPFILEQ, BackupParams: PSP_BACKUP_QUEUE_PARAMS_A,
    ) -> BOOL;
    pub fn SetupGetBackupInformationW(
        QueueHandle: HSPFILEQ, BackupParams: PSP_BACKUP_QUEUE_PARAMS_W,
    ) -> BOOL;
    pub fn SetupGetBinaryField(
        Context: PINFCONTEXT, FieldIndex: DWORD, ReturnBuffer: PBYTE, ReturnBufferSize: DWORD,
        RequiredSize: LPDWORD,
    ) -> BOOL;
    pub fn SetupGetFieldCount(Context: PINFCONTEXT) -> DWORD;
    pub fn SetupGetFileCompressionInfoA(
        SourceFileName: PCSTR, ActualSourceFileName: *mut PSTR, SourceFileSize: PDWORD,
        TargetFileSize: PDWORD, CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupGetFileCompressionInfoExA(
        SourceFileName: PCSTR, ActualSourceFileNameBuffer: PSTR,
        ActualSourceFileNameBufferLen: DWORD, RequiredBufferLen: PDWORD, SourceFileSize: PDWORD,
        TargetFileSize: PDWORD, CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupGetFileCompressionInfoExW();
    pub fn SetupGetFileCompressionInfoW(
        SourceFileName: PCWSTR, ActualSourceFileName: *mut PWSTR, SourceFileSize: PDWORD,
        TargetFileSize: PDWORD, CompressionType: PUINT,
    ) -> DWORD;
    pub fn SetupGetFileQueueCount(
        FileQueue: HSPFILEQ, SubQueueFileOp: UINT, NumOperations: PUINT,
    ) -> BOOL;
    pub fn SetupGetFileQueueFlags(FileQueue: HSPFILEQ, Flags: PDWORD) -> BOOL;
    pub fn SetupGetInfDriverStoreLocationA(
        FileName: PCSTR, AlternatePlatformInfo: PSP_ALTPLATFORM_INFO, LocaleName: PCSTR,
        ReturnBuffer: PSTR, ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfDriverStoreLocationW(
        FileName: PCWSTR, AlternatePlatformInfo: PSP_ALTPLATFORM_INFO, LocaleName: PCWSTR,
        ReturnBuffer: PWSTR, ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfFileListA(
        DirectoryPath: PCSTR, InfStyle: DWORD, ReturnBuffer: PSTR, ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfFileListW(
        DirectoryPath: PCWSTR, InfStyle: DWORD, ReturnBuffer: PWSTR, ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfInformationA(
        InfSpec: LPCVOID, SearchControl: DWORD, ReturnBuffer: PSP_INF_INFORMATION,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfInformationW(
        InfSpec: LPCVOID, SearchControl: DWORD, ReturnBuffer: PSP_INF_INFORMATION,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfPublishedNameA(
        DriverStoreLocation: PCSTR, ReturnBuffer: PSTR, ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetInfPublishedNameW(
        DriverStoreLocation: PCWSTR, ReturnBuffer: PWSTR, ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    // pub fn SetupGetInfSections();
    pub fn SetupGetIntField(Context: PINFCONTEXT, FieldIndex: DWORD, IntegerValue: PINT) -> BOOL;
    pub fn SetupGetLineByIndexA(
        InfHandle: HINF, Section: PCSTR, Index: DWORD, Context: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupGetLineByIndexW(
        InfHandle: HINF, Section: PCWSTR, Index: DWORD, Context: PINFCONTEXT,
    ) -> BOOL;
    pub fn SetupGetLineCountA(InfHandle: HINF, Section: PCSTR) -> LONG;
    pub fn SetupGetLineCountW(InfHandle: HINF, Section: PCWSTR) -> LONG;
    pub fn SetupGetLineTextA(
        Context: PINFCONTEXT, InfHandle: HINF, Section: PCSTR, Key: PCSTR, ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD, ReturnBufferSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetLineTextW(
        Context: PINFCONTEXT, InfHandle: HINF, Section: PCWSTR, Key: PCWSTR, ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD, ReturnBufferSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetMultiSzFieldA(
        Context: PINFCONTEXT, FieldIndex: DWORD, ReturnBuffer: PSTR, ReturnBufferSize: DWORD,
        RequiredSize: LPDWORD,
    ) -> BOOL;
    pub fn SetupGetMultiSzFieldW(
        Context: PINFCONTEXT, FieldIndex: DWORD, ReturnBuffer: PWSTR, ReturnBufferSize: DWORD,
        RequiredSize: LPDWORD,
    ) -> BOOL;
    pub fn SetupGetNonInteractiveMode() -> BOOL;
    pub fn SetupGetSourceFileLocationA(
        InfHandle: HINF, InfContext: PINFCONTEXT, FileName: PCSTR, SourceId: PUINT,
        ReturnBuffer: PSTR, ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetSourceFileLocationW(
        InfHandle: HINF, InfContext: PINFCONTEXT, FileName: PCWSTR, SourceId: PUINT,
        ReturnBuffer: PWSTR, ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetSourceFileSizeA(
        InfHandle: HINF, InfContext: PINFCONTEXT, FileName: PCSTR, Section: PCSTR,
        FileSize: PDWORD, RoundingFactor: UINT,
    ) -> BOOL;
    pub fn SetupGetSourceFileSizeW(
        InfHandle: HINF, InfContext: PINFCONTEXT, FileName: PCWSTR, Section: PCWSTR,
        FileSize: PDWORD, RoundingFactor: UINT,
    ) -> BOOL;
    pub fn SetupGetSourceInfoA(
        InfHandle: HINF, SourceId: UINT, InfoDesired: UINT, ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetSourceInfoW(
        InfHandle: HINF, SourceId: UINT, InfoDesired: UINT, ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetStringFieldA(
        Context: PINFCONTEXT, FieldIndex: DWORD, ReturnBuffer: PSTR, ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetStringFieldW(
        Context: PINFCONTEXT, FieldIndex: DWORD, ReturnBuffer: PWSTR, ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetTargetPathA(
        InfHandle: HINF, InfContext: PINFCONTEXT, Section: PCSTR, ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetTargetPathW(
        InfHandle: HINF, InfContext: PINFCONTEXT, Section: PCWSTR, ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupGetThreadLogToken() -> SP_LOG_TOKEN;
    pub fn SetupInitDefaultQueueCallback(OwnerWindow: HWND) -> PVOID;
    pub fn SetupInitDefaultQueueCallbackEx(
        OwnerWindow: HWND, AlternateProgressWindow: HWND, ProgressMessage: UINT, Reserved1: DWORD,
        Reserved2: PVOID,
    ) -> PVOID;
    pub fn SetupInitializeFileLogA(LogFileName: PCSTR, Flags: DWORD) -> HSPFILELOG;
    pub fn SetupInitializeFileLogW(LogFileName: PCWSTR, Flags: DWORD) -> HSPFILELOG;
    pub fn SetupInstallFileA(
        InfHandle: HINF, InfContext: PINFCONTEXT, SourceFile: PCSTR, SourcePathRoot: PCSTR,
        DestinationName: PCSTR, CopyStyle: DWORD, CopyMsgHandler: PSP_FILE_CALLBACK_A,
        Context: PVOID,
    ) -> BOOL;
    pub fn SetupInstallFileExA(
        InfHandle: HINF, InfContext: PINFCONTEXT, SourceFile: PCSTR, SourcePathRoot: PCSTR,
        DestinationName: PCSTR, CopyStyle: DWORD, CopyMsgHandler: PSP_FILE_CALLBACK_A,
        Context: PVOID, FileWasInUse: PBOOL,
    ) -> BOOL;
    pub fn SetupInstallFileExW(
        InfHandle: HINF, InfContext: PINFCONTEXT, SourceFile: PCWSTR, SourcePathRoot: PCWSTR,
        DestinationName: PCWSTR, CopyStyle: DWORD, CopyMsgHandler: PSP_FILE_CALLBACK_W,
        Context: PVOID, FileWasInUse: PBOOL,
    ) -> BOOL;
    pub fn SetupInstallFileW(
        InfHandle: HINF, InfContext: PINFCONTEXT, SourceFile: PCWSTR, SourcePathRoot: PCWSTR,
        DestinationName: PCWSTR, CopyStyle: DWORD, CopyMsgHandler: PSP_FILE_CALLBACK_W,
        Context: PVOID,
    ) -> BOOL;
    pub fn SetupInstallFilesFromInfSectionA(
        InfHandle: HINF, LayoutInfHandle: HINF, FileQueue: HSPFILEQ, SectionName: PCSTR,
        SourceRootPath: PCSTR, CopyFlags: UINT,
    ) -> BOOL;
    pub fn SetupInstallFilesFromInfSectionW(
        InfHandle: HINF, LayoutInfHandle: HINF, FileQueue: HSPFILEQ, SectionName: PCWSTR,
        SourceRootPath: PCWSTR, CopyFlags: UINT,
    ) -> BOOL;
    pub fn SetupInstallFromInfSectionA(
        Owner: HWND, InfHandle: HINF, SectionName: PCSTR, Flags: UINT, RelativeKeyRoot: HKEY,
        SourceRootPath: PCSTR, CopyFlags: UINT, MsgHandler: PSP_FILE_CALLBACK_A, Context: PVOID,
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupInstallFromInfSectionW(
        Owner: HWND, InfHandle: HINF, SectionName: PCWSTR, Flags: UINT, RelativeKeyRoot: HKEY,
        SourceRootPath: PCWSTR, CopyFlags: UINT, MsgHandler: PSP_FILE_CALLBACK_W, Context: PVOID,
        DeviceInfoSet: HDEVINFO, DeviceInfoData: PSP_DEVINFO_DATA,
    ) -> BOOL;
    pub fn SetupInstallServicesFromInfSectionA(
        InfHandle: HINF, SectionName: PCSTR, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupInstallServicesFromInfSectionExA(
        InfHandle: HINF, SectionName: PCSTR, Flags: DWORD, DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA, Reserved1: PVOID, Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupInstallServicesFromInfSectionExW(
        InfHandle: HINF, SectionName: PCWSTR, Flags: DWORD, DeviceInfoSet: HDEVINFO,
        DeviceInfoData: PSP_DEVINFO_DATA, Reserved1: PVOID, Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupInstallServicesFromInfSectionW(
        InfHandle: HINF, SectionName: PCWSTR, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupIterateCabinetA(
        CabinetFile: PCSTR, Reserved: DWORD, MsgHandler: PSP_FILE_CALLBACK_A, Context: PVOID,
    ) -> BOOL;
    pub fn SetupIterateCabinetW(
        CabinetFile: PCWSTR, Reserved: DWORD, MsgHandler: PSP_FILE_CALLBACK_W, Context: PVOID,
    ) -> BOOL;
    pub fn SetupLogErrorA(MessageString: LPCSTR, Severity: LogSeverity) -> BOOL;
    pub fn SetupLogErrorW(MessageString: LPCWSTR, Severity: LogSeverity) -> BOOL;
    pub fn SetupLogFileA(
        FileLogHandle: HSPFILELOG, LogSectionName: PCSTR, SourceFilename: PCSTR,
        TargetFilename: PCSTR, Checksum: DWORD, DiskTagfile: PCSTR, DiskDescription: PCSTR,
        OtherInfo: PCSTR, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupLogFileW(
        FileLogHandle: HSPFILELOG, LogSectionName: PCWSTR, SourceFilename: PCWSTR,
        TargetFilename: PCWSTR, Checksum: DWORD, DiskTagfile: PCWSTR, DiskDescription: PCWSTR,
        OtherInfo: PCWSTR, Flags: DWORD,
    ) -> BOOL;
    pub fn SetupOpenAppendInfFileA(FileName: PCSTR, InfHandle: HINF, ErrorLine: PUINT) -> BOOL;
    pub fn SetupOpenAppendInfFileW(FileName: PCWSTR, InfHandle: HINF, ErrorLine: PUINT) -> BOOL;
    pub fn SetupOpenFileQueue() -> HSPFILEQ;
    pub fn SetupOpenInfFileA(
        FileName: PCSTR, InfClass: PCSTR, InfStyle: DWORD, ErrorLine: PUINT,
    ) -> HINF;
    pub fn SetupOpenInfFileW(
        FileName: PCWSTR, InfClass: PCWSTR, InfStyle: DWORD, ErrorLine: PUINT,
    ) -> HINF;
    pub fn SetupOpenLog(Erase: BOOL) -> BOOL;
    pub fn SetupOpenMasterInf() -> HINF;
    pub fn SetupPrepareQueueForRestoreA(
        QueueHandle: HSPFILEQ, BackupPath: PCSTR, RestoreFlags: DWORD,
    ) -> BOOL;
    pub fn SetupPrepareQueueForRestoreW(
        QueueHandle: HSPFILEQ, BackupPath: PCWSTR, RestoreFlags: DWORD,
    ) -> BOOL;
    pub fn SetupPromptForDiskA(
        hwndParent: HWND, DialogTitle: PCSTR, DiskName: PCSTR, PathToSource: PCSTR,
        FileSought: PCSTR, TagFile: PCSTR, DiskPromptStyle: DWORD, PathBuffer: PSTR,
        PathBufferSize: DWORD, PathRequiredSize: PDWORD,
    ) -> UINT;
    pub fn SetupPromptForDiskW(
        hwndParent: HWND, DialogTitle: PCWSTR, DiskName: PCWSTR, PathToSource: PCWSTR,
        FileSought: PCWSTR, TagFile: PCWSTR, DiskPromptStyle: DWORD, PathBuffer: PWSTR,
        PathBufferSize: DWORD, PathRequiredSize: PDWORD,
    ) -> UINT;
    pub fn SetupPromptReboot(FileQueue: HSPFILEQ, Owner: HWND, ScanOnly: BOOL) -> INT;
    pub fn SetupQueryDrivesInDiskSpaceListA(
        DiskSpace: HDSKSPC, ReturnBuffer: PSTR, ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryDrivesInDiskSpaceListW(
        DiskSpace: HDSKSPC, ReturnBuffer: PWSTR, ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryFileLogA(
        FileLogHandle: HSPFILELOG, LogSectionName: PCSTR, TargetFilename: PCSTR,
        DesiredInfo: SetupFileLogInfo, DataOut: PSTR, ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryFileLogW(
        FileLogHandle: HSPFILELOG, LogSectionName: PCWSTR, TargetFilename: PCWSTR,
        DesiredInfo: SetupFileLogInfo, DataOut: PWSTR, ReturnBufferSize: DWORD,
        RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryInfFileInformationA(
        InfInformation: PSP_INF_INFORMATION, InfIndex: UINT, ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryInfFileInformationW(
        InfInformation: PSP_INF_INFORMATION, InfIndex: UINT, ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryInfOriginalFileInformationA(
        InfInformation: PSP_INF_INFORMATION, InfIndex: UINT,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO, OriginalFileInfo: PSP_ORIGINAL_FILE_INFO_A,
    ) -> BOOL;
    pub fn SetupQueryInfOriginalFileInformationW(
        InfInformation: PSP_INF_INFORMATION, InfIndex: UINT,
        AlternatePlatformInfo: PSP_ALTPLATFORM_INFO, OriginalFileInfo: PSP_ORIGINAL_FILE_INFO_W,
    ) -> BOOL;
    pub fn SetupQueryInfVersionInformationA(
        InfInformation: PSP_INF_INFORMATION, InfIndex: UINT, Key: PCSTR, ReturnBuffer: PSTR,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQueryInfVersionInformationW(
        InfInformation: PSP_INF_INFORMATION, InfIndex: UINT, Key: PCWSTR, ReturnBuffer: PWSTR,
        ReturnBufferSize: DWORD, RequiredSize: PDWORD,
    ) -> BOOL;
    pub fn SetupQuerySourceListA(Flags: DWORD, List: *mut *mut PCSTR, Count: PUINT) -> BOOL;
    pub fn SetupQuerySourceListW(Flags: DWORD, List: *mut *mut PCWSTR, Count: PUINT) -> BOOL;
    pub fn SetupQuerySpaceRequiredOnDriveA(
        DiskSpace: HDSKSPC, DriveSpec: PCSTR, SpaceRequired: *mut LONGLONG, Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupQuerySpaceRequiredOnDriveW(
        DiskSpace: HDSKSPC, DriveSpec: PCWSTR, SpaceRequired: *mut LONGLONG, Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupQueueCopyA(
        QueueHandle: HSPFILEQ, SourceRootPath: PCSTR, SourcePath: PCSTR, SourceFilename: PCSTR,
        SourceDescription: PCSTR, SourceTagfile: PCSTR, TargetDirectory: PCSTR,
        TargetFilename: PCSTR, CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueCopyIndirectA(CopyParams: PSP_FILE_COPY_PARAMS_A) -> BOOL;
    pub fn SetupQueueCopyIndirectW(CopyParams: PSP_FILE_COPY_PARAMS_W) -> BOOL;
    pub fn SetupQueueCopySectionA(
        QueueHandle: HSPFILEQ, SourceRootPath: PCSTR, InfHandle: HINF, ListInfHandle: HINF,
        Section: PCSTR, CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueCopySectionW(
        QueueHandle: HSPFILEQ, SourceRootPath: PCWSTR, InfHandle: HINF, ListInfHandle: HINF,
        Section: PCWSTR, CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueCopyW(
        QueueHandle: HSPFILEQ, SourceRootPath: PCWSTR, SourcePath: PCWSTR, SourceFilename: PCWSTR,
        SourceDescription: PCWSTR, SourceTagfile: PCWSTR, TargetDirectory: PCWSTR,
        TargetFilename: PCWSTR, CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueDefaultCopyA(
        QueueHandle: HSPFILEQ, InfHandle: HINF, SourceRootPath: PCSTR, SourceFilename: PCSTR,
        TargetFilename: PCSTR, CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueDefaultCopyW(
        QueueHandle: HSPFILEQ, InfHandle: HINF, SourceRootPath: PCWSTR, SourceFilename: PCWSTR,
        TargetFilename: PCWSTR, CopyStyle: DWORD,
    ) -> BOOL;
    pub fn SetupQueueDeleteA(QueueHandle: HSPFILEQ, PathPart1: PCSTR, PathPart2: PCSTR) -> BOOL;
    pub fn SetupQueueDeleteSectionA(
        QueueHandle: HSPFILEQ, InfHandle: HINF, ListInfHandle: HINF, Section: PCSTR,
    ) -> BOOL;
    pub fn SetupQueueDeleteSectionW(
        QueueHandle: HSPFILEQ, InfHandle: HINF, ListInfHandle: HINF, Section: PCWSTR,
    ) -> BOOL;
    pub fn SetupQueueDeleteW(QueueHandle: HSPFILEQ, PathPart1: PCWSTR, PathPart2: PCWSTR) -> BOOL;
    pub fn SetupQueueRenameA(
        QueueHandle: HSPFILEQ, SourcePath: PCSTR, SourceFilename: PCSTR, TargetPath: PCSTR,
        TargetFilename: PCSTR,
    ) -> BOOL;
    pub fn SetupQueueRenameSectionA(
        QueueHandle: HSPFILEQ, InfHandle: HINF, ListInfHandle: HINF, Section: PCSTR,
    ) -> BOOL;
    pub fn SetupQueueRenameSectionW(
        QueueHandle: HSPFILEQ, InfHandle: HINF, ListInfHandle: HINF, Section: PCWSTR,
    ) -> BOOL;
    pub fn SetupQueueRenameW(
        QueueHandle: HSPFILEQ, SourcePath: PCWSTR, SourceFilename: PCWSTR, TargetPath: PCWSTR,
        TargetFilename: PCWSTR,
    ) -> BOOL;
    pub fn SetupRemoveFileLogEntryA(
        FileLogHandle: HSPFILELOG, LogSectionName: PCSTR, TargetFilename: PCSTR,
    ) -> BOOL;
    pub fn SetupRemoveFileLogEntryW(
        FileLogHandle: HSPFILELOG, LogSectionName: PCWSTR, TargetFilename: PCWSTR,
    ) -> BOOL;
    pub fn SetupRemoveFromDiskSpaceListA(
        DiskSpace: HDSKSPC, TargetFilespec: PCSTR, Operation: UINT, Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveFromDiskSpaceListW(
        DiskSpace: HDSKSPC, TargetFilespec: PCWSTR, Operation: UINT, Reserved1: PVOID,
        Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveFromSourceListA(Flags: DWORD, Source: PCSTR) -> BOOL;
    pub fn SetupRemoveFromSourceListW(Flags: DWORD, Source: PCWSTR) -> BOOL;
    pub fn SetupRemoveInstallSectionFromDiskSpaceListA(
        DiskSpace: HDSKSPC, InfHandle: HINF, LayoutInfHandle: HINF, SectionName: PCSTR,
        Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveInstallSectionFromDiskSpaceListW(
        DiskSpace: HDSKSPC, InfHandle: HINF, LayoutInfHandle: HINF, SectionName: PCWSTR,
        Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveSectionFromDiskSpaceListA(
        DiskSpace: HDSKSPC, InfHandle: HINF, ListInfHandle: HINF, SectionName: PCSTR,
        Operation: UINT, Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRemoveSectionFromDiskSpaceListW(
        DiskSpace: HDSKSPC, InfHandle: HINF, ListInfHandle: HINF, SectionName: PCWSTR,
        Operation: UINT, Reserved1: PVOID, Reserved2: UINT,
    ) -> BOOL;
    pub fn SetupRenameErrorA(
        hwndParent: HWND, DialogTitle: PCSTR, SourceFile: PCSTR, TargetFile: PCSTR,
        Win32ErrorCode: UINT, Style: DWORD,
    ) -> UINT;
    pub fn SetupRenameErrorW(
        hwndParent: HWND, DialogTitle: PCWSTR, SourceFile: PCWSTR, TargetFile: PCWSTR,
        Win32ErrorCode: UINT, Style: DWORD,
    ) -> UINT;
    pub fn SetupScanFileQueue(
        FileQueue: HSPFILEQ, Flags: DWORD, Window: HWND, CallbackRoutine: PSP_FILE_CALLBACK_A,
        CallbackContext: PVOID, Result: PDWORD,
    ) -> BOOL;
    pub fn SetupScanFileQueueA(
        FileQueue: HSPFILEQ, Flags: DWORD, Window: HWND, CallbackRoutine: PSP_FILE_CALLBACK_A,
        CallbackContext: PVOID, Result: PDWORD,
    ) -> BOOL;
    pub fn SetupScanFileQueueW(
        FileQueue: HSPFILEQ, Flags: DWORD, Window: HWND, CallbackRoutine: PSP_FILE_CALLBACK_W,
        CallbackContext: PVOID, Result: PDWORD,
    ) -> BOOL;
    pub fn SetupSetDirectoryIdA(InfHandle: HINF, Id: DWORD, Directory: PCSTR) -> BOOL;
    pub fn SetupSetDirectoryIdExA(
        InfHandle: HINF, Id: DWORD, Directory: PCSTR, Flags: DWORD, Reserved1: DWORD,
        Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupSetDirectoryIdExW(
        InfHandle: HINF, Id: DWORD, Directory: PCWSTR, Flags: DWORD, Reserved1: DWORD,
        Reserved2: PVOID,
    ) -> BOOL;
    pub fn SetupSetDirectoryIdW(InfHandle: HINF, Id: DWORD, Directory: PCWSTR) -> BOOL;
    pub fn SetupSetFileQueueAlternatePlatformA(
        QueueHandle: HSPFILEQ, AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        AlternateDefaultCatalogFile: PCSTR,
    ) -> BOOL;
    pub fn SetupSetFileQueueAlternatePlatformW(
        QueueHandle: HSPFILEQ, AlternatePlatformInfo: PSP_ALTPLATFORM_INFO,
        AlternateDefaultCatalogFile: PCWSTR,
    ) -> BOOL;
    pub fn SetupSetFileQueueFlags(FileQueue: HSPFILEQ, FlagMask: DWORD, Flags: DWORD) -> BOOL;
    pub fn SetupSetNonInteractiveMode(NonInteractiveFlag: BOOL) -> BOOL;
    pub fn SetupSetPlatformPathOverrideA(Override: PCSTR) -> BOOL;
    pub fn SetupSetPlatformPathOverrideW(Override: PCWSTR) -> BOOL;
    pub fn SetupSetSourceListA(Flags: DWORD, SourceList: *mut PCSTR, SourceCount: UINT) -> BOOL;
    pub fn SetupSetSourceListW(Flags: DWORD, SourceList: *mut PCWSTR, SourceCount: UINT) -> BOOL;
    pub fn SetupSetThreadLogToken(LogToken: SP_LOG_TOKEN);
    pub fn SetupTermDefaultQueueCallback(Context: PVOID);
    pub fn SetupTerminateFileLog(FileLogHandle: HSPFILELOG) -> BOOL;
    pub fn SetupUninstallNewlyCopiedInfs(
        FileQueue: HSPFILEQ, Flags: DWORD, Reserved: PVOID,
    ) -> BOOL;
    pub fn SetupUninstallOEMInfA(InfFileName: PCSTR, Flags: DWORD, Reserved: PVOID) -> BOOL;
    pub fn SetupUninstallOEMInfW(InfFileName: PCWSTR, Flags: DWORD, Reserved: PVOID) -> BOOL;
    pub fn SetupVerifyInfFileA(
        InfName: PCSTR, AltPlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSignerInfo: PSP_INF_SIGNER_INFO_A,
    ) -> BOOL;
    pub fn SetupVerifyInfFileW(
        InfName: PCWSTR, AltPlatformInfo: PSP_ALTPLATFORM_INFO,
        InfSignerInfo: PSP_INF_SIGNER_INFO_W,
    ) -> BOOL;
    // pub fn SetupWriteTextLog();
    // pub fn SetupWriteTextLogError();
    pub fn SetupWriteTextLogInfLine(
        LogToken: SP_LOG_TOKEN, Flags: DWORD, InfHandle: HINF, Context: PINFCONTEXT,
    );
    // pub fn pSetupDiCrimsonLogDeviceInstall();
}
