// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
// This module contains the definitions for portable NetBIOS 3.0 support.
pub const NCBNAMSZ: usize = 16;
pub const MAX_LANA: usize = 254;
pub type PFPOST = Option<unsafe extern "system" fn(*mut NCB)>;
#[repr(C)] #[derive(Copy)]
pub struct NCB {
    pub ncb_command: ::UCHAR,
    pub ncb_retcode: ::UCHAR,
    pub ncb_lsn: ::UCHAR,
    pub ncb_num: ::UCHAR,
    pub ncb_buffer: ::PUCHAR,
    pub ncb_length: ::WORD,
    pub ncb_callname: [::UCHAR; NCBNAMSZ],
    pub ncb_name: [::UCHAR; NCBNAMSZ],
    pub ncb_rto: ::UCHAR,
    pub ncb_sto: ::UCHAR,
    pub ncb_post: PFPOST,
    pub ncb_lana_num: ::UCHAR,
    pub ncb_cmd_cplt: ::UCHAR,
    #[cfg(target_arch="x86")]
    pub ncb_reserve: [::UCHAR; 10],
    #[cfg(target_arch="x86_64")]
    pub ncb_reserve: [::UCHAR; 18],
    pub ncb_event: ::HANDLE,
}
impl Clone for NCB { fn clone(&self) -> NCB { *self } }
pub type PNCB = *mut NCB;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ADAPTER_STATUS {
    pub adapter_address: [::UCHAR; 6],
    pub rev_major: ::UCHAR,
    pub reserved0: ::UCHAR,
    pub adapter_type: ::UCHAR,
    pub rev_minor: ::UCHAR,
    pub duration: ::WORD,
    pub frmr_recv: ::WORD,
    pub frmr_xmit: ::WORD,
    pub iframe_recv_err: ::WORD,
    pub xmit_aborts: ::WORD,
    pub xmit_success: ::DWORD,
    pub recv_success: ::DWORD,
    pub iframe_xmit_err: ::WORD,
    pub recv_buff_unavail: ::WORD,
    pub t1_timeouts: ::WORD,
    pub ti_timeouts: ::WORD,
    pub reserved1: ::DWORD,
    pub free_ncbs: ::WORD,
    pub max_cfg_ncbs: ::WORD,
    pub max_ncbs: ::WORD,
    pub xmit_buf_unavail: ::WORD,
    pub max_dgram_size: ::WORD,
    pub pending_sess: ::WORD,
    pub max_cfg_sess: ::WORD,
    pub max_sess: ::WORD,
    pub max_sess_pkt_size: ::WORD,
    pub name_count: ::WORD,
}
pub type PADAPTER_STATUS = *mut ADAPTER_STATUS;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NAME_BUFFER {
    pub name: [::UCHAR; NCBNAMSZ],
    pub name_num: ::UCHAR,
    pub name_flags: ::UCHAR,
}
pub type PNAME_BUFFER = *mut NAME_BUFFER;
pub const NAME_FLAGS_MASK: ::UCHAR = 0x87;
pub const GROUP_NAME: ::UCHAR = 0x80;
pub const UNIQUE_NAME: ::UCHAR = 0x00;
pub const REGISTERING: ::UCHAR = 0x00;
pub const REGISTERED: ::UCHAR = 0x04;
pub const DEREGISTERED: ::UCHAR = 0x05;
pub const DUPLICATE: ::UCHAR = 0x06;
pub const DUPLICATE_DEREG: ::UCHAR = 0x07;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SESSION_HEADER {
    pub sess_name: ::UCHAR,
    pub num_sess: ::UCHAR,
    pub rcv_dg_outstanding: ::UCHAR,
    pub rcv_any_outstanding: ::UCHAR,
}
pub type PSESSION_HEADER = *mut SESSION_HEADER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SESSION_BUFFER {
    pub lsn: ::UCHAR,
    pub state: ::UCHAR,
    pub local_name: [::UCHAR; NCBNAMSZ],
    pub remote_name: [::UCHAR; NCBNAMSZ],
    pub rcvs_outstanding: ::UCHAR,
    pub sends_outstanding: ::UCHAR,
}
pub type PSESSION_BUFFER = *mut SESSION_BUFFER;
pub const LISTEN_OUTSTANDING: ::UCHAR = 0x01;
pub const CALL_PENDING: ::UCHAR = 0x02;
pub const SESSION_ESTABLISHED: ::UCHAR = 0x03;
pub const HANGUP_PENDING: ::UCHAR = 0x04;
pub const HANGUP_COMPLETE: ::UCHAR = 0x05;
pub const SESSION_ABORTED: ::UCHAR = 0x06;
#[repr(C)] #[derive(Copy)]
pub struct LANA_ENUM {
    pub length: ::UCHAR,
    pub lana: [::UCHAR; MAX_LANA + 1],
}
impl Clone for LANA_ENUM { fn clone(&self) -> LANA_ENUM { *self } }
pub type PLANA_ENUM = *mut LANA_ENUM;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FIND_NAME_HEADER {
    pub node_count: ::WORD,
    pub reserved: ::UCHAR,
    pub unique_group: ::UCHAR,
}
pub type PFIND_NAME_HEADER = *mut FIND_NAME_HEADER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FIND_NAME_BUFFER {
    pub length: ::UCHAR,
    pub access_control: ::UCHAR,
    pub frame_control: ::UCHAR,
    pub destination_addr: [::UCHAR; 6],
    pub source_addr: [::UCHAR; 6],
    pub routing_info: [::UCHAR; 18],
}
pub type PFIND_NAME_BUFFER = *mut FIND_NAME_BUFFER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ACTION_HEADER {
    pub transport_id: ::ULONG,
    pub action_code: ::USHORT,
    pub reserved: ::USHORT,
}
pub type PACTION_HEADER = *mut ACTION_HEADER;
pub const NCBCALL: ::UCHAR = 0x10;
pub const NCBLISTEN: ::UCHAR = 0x11;
pub const NCBHANGUP: ::UCHAR = 0x12;
pub const NCBSEND: ::UCHAR = 0x14;
pub const NCBRECV: ::UCHAR = 0x15;
pub const NCBRECVANY: ::UCHAR = 0x16;
pub const NCBCHAINSEND: ::UCHAR = 0x17;
pub const NCBDGSEND: ::UCHAR = 0x20;
pub const NCBDGRECV: ::UCHAR = 0x21;
pub const NCBDGSENDBC: ::UCHAR = 0x22;
pub const NCBADDNAME: ::UCHAR = 0x30;
pub const NCBDELNAME: ::UCHAR = 0x31;
pub const NCBRESET: ::UCHAR = 0x32;
pub const NCBASTAT: ::UCHAR = 0x33;
pub const NCBSSTAT: ::UCHAR = 0x34;
pub const NCBCANCEL: ::UCHAR = 0x35;
pub const NCBADDGRNAME: ::UCHAR = 0x36;
pub const NCBENUM: ::UCHAR = 0x37;
pub const NCBUNLINK: ::UCHAR = 0x70;
pub const NCBSENDNA: ::UCHAR = 0x71;
pub const NCBCHAINSENDNA: ::UCHAR = 0x72;
pub const NCBLANSTALERT: ::UCHAR = 0x73;
pub const NCBACTION: ::UCHAR = 0x77;
pub const NCBFINDNAME: ::UCHAR = 0x78;
pub const NCBTRACE: ::UCHAR = 0x79;
pub const ASYNCH: ::UCHAR = 0x80;
pub const NRC_GOODRET: ::UCHAR = 0x00;
pub const NRC_BUFLEN: ::UCHAR = 0x01;
pub const NRC_ILLCMD: ::UCHAR = 0x03;
pub const NRC_CMDTMO: ::UCHAR = 0x05;
pub const NRC_INCOMP: ::UCHAR = 0x06;
pub const NRC_BADDR: ::UCHAR = 0x07;
pub const NRC_SNUMOUT: ::UCHAR = 0x08;
pub const NRC_NORES: ::UCHAR = 0x09;
pub const NRC_SCLOSED: ::UCHAR = 0x0a;
pub const NRC_CMDCAN: ::UCHAR = 0x0b;
pub const NRC_DUPNAME: ::UCHAR = 0x0d;
pub const NRC_NAMTFUL: ::UCHAR = 0x0e;
pub const NRC_ACTSES: ::UCHAR = 0x0f;
pub const NRC_LOCTFUL: ::UCHAR = 0x11;
pub const NRC_REMTFUL: ::UCHAR = 0x12;
pub const NRC_ILLNN: ::UCHAR = 0x13;
pub const NRC_NOCALL: ::UCHAR = 0x14;
pub const NRC_NOWILD: ::UCHAR = 0x15;
pub const NRC_INUSE: ::UCHAR = 0x16;
pub const NRC_NAMERR: ::UCHAR = 0x17;
pub const NRC_SABORT: ::UCHAR = 0x18;
pub const NRC_NAMCONF: ::UCHAR = 0x19;
pub const NRC_IFBUSY: ::UCHAR = 0x21;
pub const NRC_TOOMANY: ::UCHAR = 0x22;
pub const NRC_BRIDGE: ::UCHAR = 0x23;
pub const NRC_CANOCCR: ::UCHAR = 0x24;
pub const NRC_CANCEL: ::UCHAR = 0x26;
pub const NRC_DUPENV: ::UCHAR = 0x30;
pub const NRC_ENVNOTDEF: ::UCHAR = 0x34;
pub const NRC_OSRESNOTAV: ::UCHAR = 0x35;
pub const NRC_MAXAPPS: ::UCHAR = 0x36;
pub const NRC_NOSAPS: ::UCHAR = 0x37;
pub const NRC_NORESOURCES: ::UCHAR = 0x38;
pub const NRC_INVADDRESS: ::UCHAR = 0x39;
pub const NRC_INVDDID: ::UCHAR = 0x3B;
pub const NRC_LOCKFAIL: ::UCHAR = 0x3C;
pub const NRC_OPENERR: ::UCHAR = 0x3f;
pub const NRC_SYSTEM: ::UCHAR = 0x40;
pub const NRC_PENDING: ::UCHAR = 0xff;
