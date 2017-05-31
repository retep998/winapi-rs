// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

ENUM!{enum PROCESS_DPI_AWARENESS {
    Process_DPI_Unaware = 0,
    Process_System_DPI_Aware = 1,
    Process_Per_Monitor_DPI_Aware = 2,
}}
ENUM!{enum MONITOR_DPI_TYPE {
    MDT_EFFECTIVE_DPI = 0,
    MDT_ANGULAR_DPI = 1,
    MDT_RAW_DPI = 2,
    MDT_DEFAULT = MDT_EFFECTIVE_DPI,
}}
ENUM!{enum SHELL_UI_COMPONENT {
    SHELL_UI_COMPONENT_TASKBARS = 0,
    SHELL_UI_COMPONENT_NOTIFICATIONAREA = 1,
    SHELL_UI_COMPONENT_DESKBAND = 2,
}}
