// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, WORD};
pub const WAVE_FORMAT_UNKNOWN: WORD = 0x0000;
pub const WAVE_FORMAT_PCM: WORD = 0x0001;
pub const WAVE_FORMAT_ADPCM: WORD = 0x0002;
pub const WAVE_FORMAT_IEEE_FLOAT: WORD = 0x0003;
pub const WAVE_FORMAT_VSELP: WORD = 0x0004;
pub const WAVE_FORMAT_IBM_CVSD: WORD = 0x0005;
pub const WAVE_FORMAT_ALAW: WORD = 0x0006;
pub const WAVE_FORMAT_MULAW: WORD = 0x0007;
pub const WAVE_FORMAT_DTS: WORD = 0x0008;
pub const WAVE_FORMAT_DRM: WORD = 0x0009;
pub const WAVE_FORMAT_WMAVOICE9: WORD = 0x000A;
pub const WAVE_FORMAT_WMAVOICE10: WORD = 0x000B;
pub const WAVE_FORMAT_OKI_ADPCM: WORD = 0x0010;
pub const WAVE_FORMAT_DVI_ADPCM: WORD = 0x0011;
pub const WAVE_FORMAT_IMA_ADPCM: WORD = WAVE_FORMAT_DVI_ADPCM;
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: WORD = 0x0012;
pub const WAVE_FORMAT_SIERRA_ADPCM: WORD = 0x0013;
pub const WAVE_FORMAT_G723_ADPCM: WORD = 0x0014;
pub const WAVE_FORMAT_DIGISTD: WORD = 0x0015;
pub const WAVE_FORMAT_DIGIFIX: WORD = 0x0016;
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: WORD = 0x0017;
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: WORD = 0x0018;
pub const WAVE_FORMAT_CU_CODEC: WORD = 0x0019;
pub const WAVE_FORMAT_HP_DYN_VOICE: WORD = 0x001A;
pub const WAVE_FORMAT_YAMAHA_ADPCM: WORD = 0x0020;
pub const WAVE_FORMAT_SONARC: WORD = 0x0021;
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: WORD = 0x0022;
pub const WAVE_FORMAT_ECHOSC1: WORD = 0x0023;
pub const WAVE_FORMAT_AUDIOFILE_AF36: WORD = 0x0024;
pub const WAVE_FORMAT_APTX: WORD = 0x0025;
pub const WAVE_FORMAT_AUDIOFILE_AF10: WORD = 0x0026;
pub const WAVE_FORMAT_PROSODY_1612: WORD = 0x0027;
pub const WAVE_FORMAT_LRC: WORD = 0x0028;
pub const WAVE_FORMAT_DOLBY_AC2: WORD = 0x0030;
pub const WAVE_FORMAT_GSM610: WORD = 0x0031;
pub const WAVE_FORMAT_MSNAUDIO: WORD = 0x0032;
pub const WAVE_FORMAT_ANTEX_ADPCME: WORD = 0x0033;
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: WORD = 0x0034;
pub const WAVE_FORMAT_DIGIREAL: WORD = 0x0035;
pub const WAVE_FORMAT_DIGIADPCM: WORD = 0x0036;
pub const WAVE_FORMAT_CONTROL_RES_CR10: WORD = 0x0037;
pub const WAVE_FORMAT_NMS_VBXADPCM: WORD = 0x0038;
pub const WAVE_FORMAT_CS_IMAADPCM: WORD = 0x0039;
pub const WAVE_FORMAT_ECHOSC3: WORD = 0x003A;
pub const WAVE_FORMAT_ROCKWELL_ADPCM: WORD = 0x003B;
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: WORD = 0x003C;
pub const WAVE_FORMAT_XEBEC: WORD = 0x003D;
pub const WAVE_FORMAT_G721_ADPCM: WORD = 0x0040;
pub const WAVE_FORMAT_G728_CELP: WORD = 0x0041;
pub const WAVE_FORMAT_MSG723: WORD = 0x0042;
pub const WAVE_FORMAT_INTEL_G723_1: WORD = 0x0043;
pub const WAVE_FORMAT_INTEL_G729: WORD = 0x0044;
pub const WAVE_FORMAT_SHARP_G726: WORD = 0x0045;
pub const WAVE_FORMAT_MPEG: WORD = 0x0050;
pub const WAVE_FORMAT_RT24: WORD = 0x0052;
pub const WAVE_FORMAT_PAC: WORD = 0x0053;
pub const WAVE_FORMAT_MPEGLAYER3: WORD = 0x0055;
pub const WAVE_FORMAT_LUCENT_G723: WORD = 0x0059;
pub const WAVE_FORMAT_CIRRUS: WORD = 0x0060;
pub const WAVE_FORMAT_ESPCM: WORD = 0x0061;
pub const WAVE_FORMAT_VOXWARE: WORD = 0x0062;
pub const WAVE_FORMAT_CANOPUS_ATRAC: WORD = 0x0063;
pub const WAVE_FORMAT_G726_ADPCM: WORD = 0x0064;
pub const WAVE_FORMAT_G722_ADPCM: WORD = 0x0065;
pub const WAVE_FORMAT_DSAT: WORD = 0x0066;
pub const WAVE_FORMAT_DSAT_DISPLAY: WORD = 0x0067;
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: WORD = 0x0069;
pub const WAVE_FORMAT_VOXWARE_AC8: WORD = 0x0070;
pub const WAVE_FORMAT_VOXWARE_AC10: WORD = 0x0071;
pub const WAVE_FORMAT_VOXWARE_AC16: WORD = 0x0072;
pub const WAVE_FORMAT_VOXWARE_AC20: WORD = 0x0073;
pub const WAVE_FORMAT_VOXWARE_RT24: WORD = 0x0074;
pub const WAVE_FORMAT_VOXWARE_RT29: WORD = 0x0075;
pub const WAVE_FORMAT_VOXWARE_RT29HW: WORD = 0x0076;
pub const WAVE_FORMAT_VOXWARE_VR12: WORD = 0x0077;
pub const WAVE_FORMAT_VOXWARE_VR18: WORD = 0x0078;
pub const WAVE_FORMAT_VOXWARE_TQ40: WORD = 0x0079;
pub const WAVE_FORMAT_VOXWARE_SC3: WORD = 0x007A;
pub const WAVE_FORMAT_VOXWARE_SC3_1: WORD = 0x007B;
pub const WAVE_FORMAT_SOFTSOUND: WORD = 0x0080;
pub const WAVE_FORMAT_VOXWARE_TQ60: WORD = 0x0081;
pub const WAVE_FORMAT_MSRT24: WORD = 0x0082;
pub const WAVE_FORMAT_G729A: WORD = 0x0083;
pub const WAVE_FORMAT_MVI_MVI2: WORD = 0x0084;
pub const WAVE_FORMAT_DF_G726: WORD = 0x0085;
pub const WAVE_FORMAT_DF_GSM610: WORD = 0x0086;
pub const WAVE_FORMAT_ISIAUDIO: WORD = 0x0088;
pub const WAVE_FORMAT_ONLIVE: WORD = 0x0089;
pub const WAVE_FORMAT_MULTITUDE_FT_SX20: WORD = 0x008A;
pub const WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM: WORD = 0x008B;
pub const WAVE_FORMAT_CONVEDIA_G729: WORD = 0x008C;
pub const WAVE_FORMAT_CONGRUENCY: WORD = 0x008D;
pub const WAVE_FORMAT_SBC24: WORD = 0x0091;
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: WORD = 0x0092;
pub const WAVE_FORMAT_MEDIASONIC_G723: WORD = 0x0093;
pub const WAVE_FORMAT_PROSODY_8KBPS: WORD = 0x0094;
pub const WAVE_FORMAT_ZYXEL_ADPCM: WORD = 0x0097;
pub const WAVE_FORMAT_PHILIPS_LPCBB: WORD = 0x0098;
pub const WAVE_FORMAT_PACKED: WORD = 0x0099;
pub const WAVE_FORMAT_MALDEN_PHONYTALK: WORD = 0x00A0;
pub const WAVE_FORMAT_RACAL_RECORDER_GSM: WORD = 0x00A1;
pub const WAVE_FORMAT_RACAL_RECORDER_G720_A: WORD = 0x00A2;
pub const WAVE_FORMAT_RACAL_RECORDER_G723_1: WORD = 0x00A3;
pub const WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP: WORD = 0x00A4;
pub const WAVE_FORMAT_NEC_AAC: WORD = 0x00B0;
pub const WAVE_FORMAT_RAW_AAC1: WORD = 0x00FF;
pub const WAVE_FORMAT_RHETOREX_ADPCM: WORD = 0x0100;
pub const WAVE_FORMAT_IRAT: WORD = 0x0101;
pub const WAVE_FORMAT_VIVO_G723: WORD = 0x0111;
pub const WAVE_FORMAT_VIVO_SIREN: WORD = 0x0112;
pub const WAVE_FORMAT_PHILIPS_CELP: WORD = 0x0120;
pub const WAVE_FORMAT_PHILIPS_GRUNDIG: WORD = 0x0121;
pub const WAVE_FORMAT_DIGITAL_G723: WORD = 0x0123;
pub const WAVE_FORMAT_SANYO_LD_ADPCM: WORD = 0x0125;
pub const WAVE_FORMAT_SIPROLAB_ACEPLNET: WORD = 0x0130;
pub const WAVE_FORMAT_SIPROLAB_ACELP4800: WORD = 0x0131;
pub const WAVE_FORMAT_SIPROLAB_ACELP8V3: WORD = 0x0132;
pub const WAVE_FORMAT_SIPROLAB_G729: WORD = 0x0133;
pub const WAVE_FORMAT_SIPROLAB_G729A: WORD = 0x0134;
pub const WAVE_FORMAT_SIPROLAB_KELVIN: WORD = 0x0135;
pub const WAVE_FORMAT_VOICEAGE_AMR: WORD = 0x0136;
pub const WAVE_FORMAT_G726ADPCM: WORD = 0x0140;
pub const WAVE_FORMAT_DICTAPHONE_CELP68: WORD = 0x0141;
pub const WAVE_FORMAT_DICTAPHONE_CELP54: WORD = 0x0142;
pub const WAVE_FORMAT_QUALCOMM_PUREVOICE: WORD = 0x0150;
pub const WAVE_FORMAT_QUALCOMM_HALFRATE: WORD = 0x0151;
pub const WAVE_FORMAT_TUBGSM: WORD = 0x0155;
pub const WAVE_FORMAT_MSAUDIO1: WORD = 0x0160;
pub const WAVE_FORMAT_WMAUDIO2: WORD = 0x0161;
pub const WAVE_FORMAT_WMAUDIO3: WORD = 0x0162;
pub const WAVE_FORMAT_WMAUDIO_LOSSLESS: WORD = 0x0163;
pub const WAVE_FORMAT_WMASPDIF: WORD = 0x0164;
pub const WAVE_FORMAT_UNISYS_NAP_ADPCM: WORD = 0x0170;
pub const WAVE_FORMAT_UNISYS_NAP_ULAW: WORD = 0x0171;
pub const WAVE_FORMAT_UNISYS_NAP_ALAW: WORD = 0x0172;
pub const WAVE_FORMAT_UNISYS_NAP_16K: WORD = 0x0173;
pub const WAVE_FORMAT_SYCOM_ACM_SYC008: WORD = 0x0174;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_G726L: WORD = 0x0175;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54: WORD = 0x0176;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68: WORD = 0x0177;
pub const WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM: WORD = 0x0178;
pub const WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC: WORD = 0x0180;
pub const WAVE_FORMAT_DTS_DS: WORD = 0x0190;
pub const WAVE_FORMAT_CREATIVE_ADPCM: WORD = 0x0200;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: WORD = 0x0202;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: WORD = 0x0203;
pub const WAVE_FORMAT_UHER_ADPCM: WORD = 0x0210;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO: WORD = 0x0215;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO_1: WORD = 0x0216;
pub const WAVE_FORMAT_QUARTERDECK: WORD = 0x0220;
pub const WAVE_FORMAT_ILINK_VC: WORD = 0x0230;
pub const WAVE_FORMAT_RAW_SPORT: WORD = 0x0240;
pub const WAVE_FORMAT_ESST_AC3: WORD = 0x0241;
pub const WAVE_FORMAT_GENERIC_PASSTHRU: WORD = 0x0249;
pub const WAVE_FORMAT_IPI_HSX: WORD = 0x0250;
pub const WAVE_FORMAT_IPI_RPELP: WORD = 0x0251;
pub const WAVE_FORMAT_CS2: WORD = 0x0260;
pub const WAVE_FORMAT_SONY_SCX: WORD = 0x0270;
pub const WAVE_FORMAT_SONY_SCY: WORD = 0x0271;
pub const WAVE_FORMAT_SONY_ATRAC3: WORD = 0x0272;
pub const WAVE_FORMAT_SONY_SPC: WORD = 0x0273;
pub const WAVE_FORMAT_TELUM_AUDIO: WORD = 0x0280;
pub const WAVE_FORMAT_TELUM_IA_AUDIO: WORD = 0x0281;
pub const WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM: WORD = 0x0285;
pub const WAVE_FORMAT_FM_TOWNS_SND: WORD = 0x0300;
pub const WAVE_FORMAT_MICRONAS: WORD = 0x0350;
pub const WAVE_FORMAT_MICRONAS_CELP833: WORD = 0x0351;
pub const WAVE_FORMAT_BTV_DIGITAL: WORD = 0x0400;
pub const WAVE_FORMAT_INTEL_MUSIC_CODER: WORD = 0x0401;
pub const WAVE_FORMAT_INDEO_AUDIO: WORD = 0x0402;
pub const WAVE_FORMAT_QDESIGN_MUSIC: WORD = 0x0450;
pub const WAVE_FORMAT_ON2_VP7_AUDIO: WORD = 0x0500;
pub const WAVE_FORMAT_ON2_VP6_AUDIO: WORD = 0x0501;
pub const WAVE_FORMAT_VME_VMPCM: WORD = 0x0680;
pub const WAVE_FORMAT_TPC: WORD = 0x0681;
pub const WAVE_FORMAT_LIGHTWAVE_LOSSLESS: WORD = 0x08AE;
pub const WAVE_FORMAT_OLIGSM: WORD = 0x1000;
pub const WAVE_FORMAT_OLIADPCM: WORD = 0x1001;
pub const WAVE_FORMAT_OLICELP: WORD = 0x1002;
pub const WAVE_FORMAT_OLISBC: WORD = 0x1003;
pub const WAVE_FORMAT_OLIOPR: WORD = 0x1004;
pub const WAVE_FORMAT_LH_CODEC: WORD = 0x1100;
pub const WAVE_FORMAT_LH_CODEC_CELP: WORD = 0x1101;
pub const WAVE_FORMAT_LH_CODEC_SBC8: WORD = 0x1102;
pub const WAVE_FORMAT_LH_CODEC_SBC12: WORD = 0x1103;
pub const WAVE_FORMAT_LH_CODEC_SBC16: WORD = 0x1104;
pub const WAVE_FORMAT_NORRIS: WORD = 0x1400;
pub const WAVE_FORMAT_ISIAUDIO_2: WORD = 0x1401;
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: WORD = 0x1500;
pub const WAVE_FORMAT_MPEG_ADTS_AAC: WORD = 0x1600;
pub const WAVE_FORMAT_MPEG_RAW_AAC: WORD = 0x1601;
pub const WAVE_FORMAT_MPEG_LOAS: WORD = 0x1602;
pub const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: WORD = 0x1608;
pub const WAVE_FORMAT_NOKIA_MPEG_RAW_AAC: WORD = 0x1609;
pub const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: WORD = 0x160A;
pub const WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC: WORD = 0x160B;
pub const WAVE_FORMAT_MPEG_HEAAC: WORD = 0x1610;
pub const WAVE_FORMAT_VOXWARE_RT24_SPEECH: WORD = 0x181C;
pub const WAVE_FORMAT_SONICFOUNDRY_LOSSLESS: WORD = 0x1971;
pub const WAVE_FORMAT_INNINGS_TELECOM_ADPCM: WORD = 0x1979;
pub const WAVE_FORMAT_LUCENT_SX8300P: WORD = 0x1C07;
pub const WAVE_FORMAT_LUCENT_SX5363S: WORD = 0x1C0C;
pub const WAVE_FORMAT_CUSEEME: WORD = 0x1F03;
pub const WAVE_FORMAT_NTCSOFT_ALF2CM_ACM: WORD = 0x1FC4;
pub const WAVE_FORMAT_DVM: WORD = 0x2000;
pub const WAVE_FORMAT_DTS2: WORD = 0x2001;
pub const WAVE_FORMAT_MAKEAVIS: WORD = 0x3313;
pub const WAVE_FORMAT_DIVIO_MPEG4_AAC: WORD = 0x4143;
pub const WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE: WORD = 0x4201;
pub const WAVE_FORMAT_DIVIO_G726: WORD = 0x4243;
pub const WAVE_FORMAT_LEAD_SPEECH: WORD = 0x434C;
pub const WAVE_FORMAT_LEAD_VORBIS: WORD = 0x564C;
pub const WAVE_FORMAT_WAVPACK_AUDIO: WORD = 0x5756;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1: WORD = 0x674F;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2: WORD = 0x6750;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3: WORD = 0x6751;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS: WORD = 0x676F;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS: WORD = 0x6770;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS: WORD = 0x6771;
pub const WAVE_FORMAT_3COM_NBX: WORD = 0x7000;
pub const WAVE_FORMAT_FAAD_AAC: WORD = 0x706D;
pub const WAVE_FORMAT_AMR_NB: WORD = 0x7361;
pub const WAVE_FORMAT_AMR_WB: WORD = 0x7362;
pub const WAVE_FORMAT_AMR_WP: WORD = 0x7363;
pub const WAVE_FORMAT_GSM_AMR_CBR: WORD = 0x7A21;
pub const WAVE_FORMAT_GSM_AMR_VBR_SID: WORD = 0x7A22;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_G723_1: WORD = 0xA100;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC: WORD = 0xA101;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_SBC: WORD = 0xA102;
pub const WAVE_FORMAT_SYMBOL_G729_A: WORD = 0xA103;
pub const WAVE_FORMAT_VOICEAGE_AMR_WB: WORD = 0xA104;
pub const WAVE_FORMAT_INGENIENT_G726: WORD = 0xA105;
pub const WAVE_FORMAT_MPEG4_AAC: WORD = 0xA106;
pub const WAVE_FORMAT_ENCORE_G726: WORD = 0xA107;
pub const WAVE_FORMAT_ZOLL_ASAO: WORD = 0xA108;
pub const WAVE_FORMAT_SPEEX_VOICE: WORD = 0xA109;
pub const WAVE_FORMAT_VIANIX_MASC: WORD = 0xA10A;
pub const WAVE_FORMAT_WM9_SPECTRUM_ANALYZER: WORD = 0xA10B;
pub const WAVE_FORMAT_WMF_SPECTRUM_ANAYZER: WORD = 0xA10C;
pub const WAVE_FORMAT_GSM_610: WORD = 0xA10D;
pub const WAVE_FORMAT_GSM_620: WORD = 0xA10E;
pub const WAVE_FORMAT_GSM_660: WORD = 0xA10F;
pub const WAVE_FORMAT_GSM_690: WORD = 0xA110;
pub const WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB: WORD = 0xA111;
pub const WAVE_FORMAT_POLYCOM_G722: WORD = 0xA112;
pub const WAVE_FORMAT_POLYCOM_G728: WORD = 0xA113;
pub const WAVE_FORMAT_POLYCOM_G729_A: WORD = 0xA114;
pub const WAVE_FORMAT_POLYCOM_SIREN: WORD = 0xA115;
pub const WAVE_FORMAT_GLOBAL_IP_ILBC: WORD = 0xA116;
pub const WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO: WORD = 0xA117;
pub const WAVE_FORMAT_NICE_ACA: WORD = 0xA118;
pub const WAVE_FORMAT_NICE_ADPCM: WORD = 0xA119;
pub const WAVE_FORMAT_VOCORD_G721: WORD = 0xA11A;
pub const WAVE_FORMAT_VOCORD_G726: WORD = 0xA11B;
pub const WAVE_FORMAT_VOCORD_G722_1: WORD = 0xA11C;
pub const WAVE_FORMAT_VOCORD_G728: WORD = 0xA11D;
pub const WAVE_FORMAT_VOCORD_G729: WORD = 0xA11E;
pub const WAVE_FORMAT_VOCORD_G729_A: WORD = 0xA11F;
pub const WAVE_FORMAT_VOCORD_G723_1: WORD = 0xA120;
pub const WAVE_FORMAT_VOCORD_LBC: WORD = 0xA121;
pub const WAVE_FORMAT_NICE_G728: WORD = 0xA122;
pub const WAVE_FORMAT_FRACE_TELECOM_G729: WORD = 0xA123;
pub const WAVE_FORMAT_CODIAN: WORD = 0xA124;
pub const WAVE_FORMAT_FLAC: WORD = 0xF1AC;
pub const WAVE_FORMAT_EXTENSIBLE: WORD = 0xFFFE;
pub const WAVE_FORMAT_DEVELOPMENT: WORD = 0xFFFF;
//2557
pub const SPEAKER_FRONT_LEFT: DWORD = 0x1;
pub const SPEAKER_FRONT_RIGHT: DWORD = 0x2;
pub const SPEAKER_FRONT_CENTER: DWORD = 0x4;
pub const SPEAKER_LOW_FREQUENCY: DWORD = 0x8;
pub const SPEAKER_BACK_LEFT: DWORD = 0x10;
pub const SPEAKER_BACK_RIGHT: DWORD = 0x20;
pub const SPEAKER_FRONT_LEFT_OF_CENTER: DWORD = 0x40;
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: DWORD = 0x80;
pub const SPEAKER_BACK_CENTER: DWORD = 0x100;
pub const SPEAKER_SIDE_LEFT: DWORD = 0x200;
pub const SPEAKER_SIDE_RIGHT: DWORD = 0x400;
pub const SPEAKER_TOP_CENTER: DWORD = 0x800;
pub const SPEAKER_TOP_FRONT_LEFT: DWORD = 0x1000;
pub const SPEAKER_TOP_FRONT_CENTER: DWORD = 0x2000;
pub const SPEAKER_TOP_FRONT_RIGHT: DWORD = 0x4000;
pub const SPEAKER_TOP_BACK_LEFT: DWORD = 0x8000;
pub const SPEAKER_TOP_BACK_CENTER: DWORD = 0x10000;
pub const SPEAKER_TOP_BACK_RIGHT: DWORD = 0x20000;
pub const SPEAKER_RESERVED: DWORD = 0x7FFC0000;
pub const SPEAKER_ALL: DWORD = 0x80000000;
STRUCT! {#[repr(packed)] struct WAVEFORMATEX {
    wFormatTag: WORD,
    nChannels: WORD,
    nSamplesPerSec: DWORD,
    nAvgBytesPerSec: DWORD,
    nBlockAlign: WORD,
    wBitsPerSample: WORD,
    cbSize: WORD,
}}
STRUCT! {#[repr(packed)] struct WAVEFORMATEXTENSIBLE {
    Format: WAVEFORMATEX,
    Samples: WORD,
    dwChannelMask: DWORD,
    SubFormat: GUID,
}}
