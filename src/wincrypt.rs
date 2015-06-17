// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Cryptographic API Prototypes and Definitions
//108
pub const ALG_CLASS_ANY: ALG_ID = 0;
pub const ALG_CLASS_SIGNATURE: ALG_ID = 1 << 13;
pub const ALG_CLASS_MSG_ENCRYPT: ALG_ID = 2 << 13;
pub const ALG_CLASS_DATA_ENCRYPT: ALG_ID = 3 << 13;
pub const ALG_CLASS_HASH: ALG_ID = 4 << 13;
pub const ALG_CLASS_KEY_EXCHANGE: ALG_ID = 5 << 13;
pub const ALG_CLASS_ALL: ALG_ID = 7 << 13;
pub const ALG_TYPE_ANY: ALG_ID = 0;
pub const ALG_TYPE_DSS: ALG_ID = 1 << 9;
pub const ALG_TYPE_RSA: ALG_ID = 2 << 9;
pub const ALG_TYPE_BLOCK: ALG_ID = 3 << 9;
pub const ALG_TYPE_STREAM: ALG_ID = 4 << 9;
pub const ALG_TYPE_DH: ALG_ID = 5 << 9;
pub const ALG_TYPE_SECURECHANNEL: ALG_ID = 6 << 9;
pub const ALG_SID_ANY: ALG_ID = 0;
pub const ALG_SID_RSA_ANY: ALG_ID = 0;
pub const ALG_SID_RSA_PKCS: ALG_ID = 1;
pub const ALG_SID_RSA_MSATWORK: ALG_ID = 2;
pub const ALG_SID_RSA_ENTRUST: ALG_ID = 3;
pub const ALG_SID_RSA_PGP: ALG_ID = 4;
pub const ALG_SID_DSS_ANY: ALG_ID = 0;
pub const ALG_SID_DSS_PKCS: ALG_ID = 1;
pub const ALG_SID_DSS_DMS: ALG_ID = 2;
pub const ALG_SID_ECDSA: ALG_ID = 3;
pub const ALG_SID_DES: ALG_ID = 1;
pub const ALG_SID_3DES: ALG_ID = 3;
pub const ALG_SID_DESX: ALG_ID = 4;
pub const ALG_SID_IDEA: ALG_ID = 5;
pub const ALG_SID_CAST: ALG_ID = 6;
pub const ALG_SID_SAFERSK64: ALG_ID = 7;
pub const ALG_SID_SAFERSK128: ALG_ID = 8;
pub const ALG_SID_3DES_112: ALG_ID = 9;
pub const ALG_SID_CYLINK_MEK: ALG_ID = 12;
pub const ALG_SID_RC5: ALG_ID = 13;
pub const ALG_SID_AES_128: ALG_ID = 14;
pub const ALG_SID_AES_192: ALG_ID = 15;
pub const ALG_SID_AES_256: ALG_ID = 16;
pub const ALG_SID_AES: ALG_ID = 17;
pub const ALG_SID_SKIPJACK: ALG_ID = 10;
pub const ALG_SID_TEK: ALG_ID = 11;
pub const CRYPT_MODE_CBCI: ALG_ID = 6;
pub const CRYPT_MODE_CFBP: ALG_ID = 7;
pub const CRYPT_MODE_OFBP: ALG_ID = 8;
pub const CRYPT_MODE_CBCOFM: ALG_ID = 9;
pub const CRYPT_MODE_CBCOFMI: ALG_ID = 10;
pub const ALG_SID_RC2: ALG_ID = 2;
pub const ALG_SID_RC4: ALG_ID = 1;
pub const ALG_SID_SEAL: ALG_ID = 2;
pub const ALG_SID_DH_SANDF: ALG_ID = 1;
pub const ALG_SID_DH_EPHEM: ALG_ID = 2;
pub const ALG_SID_AGREED_KEY_ANY: ALG_ID = 3;
pub const ALG_SID_KEA: ALG_ID = 4;
pub const ALG_SID_ECDH: ALG_ID = 5;
pub const ALG_SID_MD2: ALG_ID = 1;
pub const ALG_SID_MD4: ALG_ID = 2;
pub const ALG_SID_MD5: ALG_ID = 3;
pub const ALG_SID_SHA: ALG_ID = 4;
pub const ALG_SID_SHA1: ALG_ID = 4;
pub const ALG_SID_MAC: ALG_ID = 5;
pub const ALG_SID_RIPEMD: ALG_ID = 6;
pub const ALG_SID_RIPEMD160: ALG_ID = 7;
pub const ALG_SID_SSL3SHAMD5: ALG_ID = 8;
pub const ALG_SID_HMAC: ALG_ID = 9;
pub const ALG_SID_TLS1PRF: ALG_ID = 10;
pub const ALG_SID_HASH_REPLACE_OWF: ALG_ID = 11;
pub const ALG_SID_SHA_256: ALG_ID = 12;
pub const ALG_SID_SHA_384: ALG_ID = 13;
pub const ALG_SID_SHA_512: ALG_ID = 14;
pub const ALG_SID_SSL3_MASTER: ALG_ID = 1;
pub const ALG_SID_SCHANNEL_MASTER_HASH: ALG_ID = 2;
pub const ALG_SID_SCHANNEL_MAC_KEY: ALG_ID = 3;
pub const ALG_SID_PCT1_MASTER: ALG_ID = 4;
pub const ALG_SID_SSL2_MASTER: ALG_ID = 5;
pub const ALG_SID_TLS1_MASTER: ALG_ID = 6;
pub const ALG_SID_SCHANNEL_ENC_KEY: ALG_ID = 7;
pub const ALG_SID_ECMQV: ALG_ID = 1;
pub const ALG_SID_EXAMPLE: ALG_ID = 80;
pub type ALG_ID = ::c_uint;
pub const CALG_MD2: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD2;
pub const CALG_MD4: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD4;
pub const CALG_MD5: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD5;
pub const CALG_SHA: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA;
pub const CALG_SHA1: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA1;
pub const CALG_MAC: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MAC;
pub const CALG_RSA_SIGN: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_RSA | ALG_SID_RSA_ANY;
pub const CALG_DSS_SIGN: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_DSS | ALG_SID_DSS_ANY;
pub const CALG_NO_SIGN: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_ANY | ALG_SID_ANY;
pub const CALG_RSA_KEYX: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_RSA | ALG_SID_RSA_ANY;
pub const CALG_DES: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_DES;
pub const CALG_3DES_112: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_3DES_112;
pub const CALG_3DES: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_3DES;
pub const CALG_DESX: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_DESX;
pub const CALG_RC2: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_RC2;
pub const CALG_RC4: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_STREAM | ALG_SID_RC4;
pub const CALG_SEAL: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_STREAM | ALG_SID_SEAL;
pub const CALG_DH_SF: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_DH_SANDF;
pub const CALG_DH_EPHEM: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_DH_EPHEM;
pub const CALG_AGREEDKEY_ANY: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH
    | ALG_SID_AGREED_KEY_ANY;
pub const CALG_KEA_KEYX: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_KEA;
pub const CALG_HUGHES_MD5: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_ANY | ALG_SID_MD5;
pub const CALG_SKIPJACK: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_SKIPJACK;
pub const CALG_TEK: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_TEK;
pub const CALG_CYLINK_MEK: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_CYLINK_MEK;
pub const CALG_SSL3_SHAMD5: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SSL3SHAMD5;
pub const CALG_SSL3_MASTER: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SSL3_MASTER;
pub const CALG_SCHANNEL_MASTER_HASH: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SCHANNEL_MASTER_HASH;
pub const CALG_SCHANNEL_MAC_KEY: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SCHANNEL_MAC_KEY;
pub const CALG_SCHANNEL_ENC_KEY: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SCHANNEL_ENC_KEY;
pub const CALG_PCT1_MASTER: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_PCT1_MASTER;
pub const CALG_SSL2_MASTER: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SSL2_MASTER;
pub const CALG_TLS1_MASTER: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_TLS1_MASTER;
pub const CALG_RC5: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_RC5;
pub const CALG_HMAC: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_HMAC;
pub const CALG_TLS1PRF: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_TLS1PRF;
pub const CALG_HASH_REPLACE_OWF: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_HASH_REPLACE_OWF;
pub const CALG_AES_128: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_128;
pub const CALG_AES_192: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_192;
pub const CALG_AES_256: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_256;
pub const CALG_AES: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES;
pub const CALG_SHA_256: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_256;
pub const CALG_SHA_384: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_384;
pub const CALG_SHA_512: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_512;
pub const CALG_ECDH: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_ECDH;
pub const CALG_ECMQV: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_ANY | ALG_SID_ECMQV;
pub const CALG_ECDSA: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_DSS | ALG_SID_ECDSA;
pub type HCRYPTPROV = ::ULONG_PTR;
pub type HCRYPTKEY = ::ULONG_PTR;
pub type HCRYPTHASH = ::ULONG_PTR;
pub const CRYPT_VERIFYCONTEXT: ::DWORD = 0xF0000000;
pub const CRYPT_NEWKEYSET: ::DWORD = 0x00000008;
pub const CRYPT_DELETEKEYSET: ::DWORD = 0x00000010;
pub const CRYPT_MACHINE_KEYSET: ::DWORD = 0x00000020;
pub const CRYPT_SILENT: ::DWORD = 0x00000040;
pub const CRYPT_DEFAULT_CONTAINER_OPTIONAL: ::DWORD = 0x00000080;
pub const CRYPT_EXPORTABLE: ::DWORD = 0x00000001;
pub const CRYPT_USER_PROTECTED: ::DWORD = 0x00000002;
pub const CRYPT_CREATE_SALT: ::DWORD = 0x00000004;
pub const CRYPT_UPDATE_KEY: ::DWORD = 0x00000008;
pub const CRYPT_NO_SALT: ::DWORD = 0x00000010;
pub const CRYPT_PREGEN: ::DWORD = 0x00000040;
pub const CRYPT_RECIPIENT: ::DWORD = 0x00000010;
pub const CRYPT_INITIATOR: ::DWORD = 0x00000040;
pub const CRYPT_ONLINE: ::DWORD = 0x00000080;
pub const CRYPT_SF: ::DWORD = 0x00000100;
pub const CRYPT_CREATE_IV: ::DWORD = 0x00000200;
pub const CRYPT_KEK: ::DWORD = 0x00000400;
pub const CRYPT_DATA_KEY: ::DWORD = 0x00000800;
pub const CRYPT_VOLATILE: ::DWORD = 0x00001000;
pub const CRYPT_SGCKEY: ::DWORD = 0x00002000;
pub const CRYPT_USER_PROTECTED_STRONG: ::DWORD = 0x00100000;
pub const CRYPT_ARCHIVABLE: ::DWORD = 0x00004000;
pub const CRYPT_FORCE_KEY_PROTECTION_HIGH: ::DWORD = 0x00008000;
pub const RSA1024BIT_KEY: ::DWORD = 0x04000000;
pub const CRYPT_SERVER: ::DWORD = 0x00000400;
pub const KEY_LENGTH_MASK: ::DWORD = 0xFFFF0000;
pub const CRYPT_Y_ONLY: ::DWORD = 0x00000001;
pub const CRYPT_SSL2_FALLBACK: ::DWORD = 0x00000002;
pub const CRYPT_DESTROYKEY: ::DWORD = 0x00000004;
pub const CRYPT_OAEP: ::DWORD = 0x00000040;
pub const CRYPT_BLOB_VER3: ::DWORD = 0x00000080;
pub const CRYPT_IPSEC_HMAC_KEY: ::DWORD = 0x00000100;
pub const CRYPT_DECRYPT_RSA_NO_PADDING_CHECK: ::DWORD = 0x00000020;
pub const CRYPT_SECRETDIGEST: ::DWORD = 0x00000001;
pub const CRYPT_OWF_REPL_LM_HASH: ::DWORD = 0x00000001;
pub const CRYPT_LITTLE_ENDIAN: ::DWORD = 0x00000001;
pub const CRYPT_NOHASHOID: ::DWORD = 0x00000001;
pub const CRYPT_TYPE2_FORMAT: ::DWORD = 0x00000002;
pub const CRYPT_X931_FORMAT: ::DWORD = 0x00000004;
pub const CRYPT_MACHINE_DEFAULT: ::DWORD = 0x00000001;
pub const CRYPT_USER_DEFAULT: ::DWORD = 0x00000002;
pub const CRYPT_DELETE_DEFAULT: ::DWORD = 0x00000004;
pub const SIMPLEBLOB: ::DWORD = 0x1;
pub const PUBLICKEYBLOB: ::DWORD = 0x6;
pub const PRIVATEKEYBLOB: ::DWORD = 0x7;
pub const PLAINTEXTKEYBLOB: ::DWORD = 0x8;
pub const OPAQUEKEYBLOB: ::DWORD = 0x9;
pub const PUBLICKEYBLOBEX: ::DWORD = 0xA;
pub const SYMMETRICWRAPKEYBLOB: ::DWORD = 0xB;
pub const KEYSTATEBLOB: ::DWORD = 0xC;
pub const AT_KEYEXCHANGE: ::DWORD = 1;
pub const AT_SIGNATURE: ::DWORD = 2;
pub const CRYPT_USERDATA: ::DWORD = 1;
pub const KP_IV: ::DWORD = 1;
pub const KP_SALT: ::DWORD = 2;
pub const KP_PADDING: ::DWORD = 3;
pub const KP_MODE: ::DWORD = 4;
pub const KP_MODE_BITS: ::DWORD = 5;
pub const KP_PERMISSIONS: ::DWORD = 6;
pub const KP_ALGID: ::DWORD = 7;
pub const KP_BLOCKLEN: ::DWORD = 8;
pub const KP_KEYLEN: ::DWORD = 9;
pub const KP_SALT_EX: ::DWORD = 10;
pub const KP_P: ::DWORD = 11;
pub const KP_G: ::DWORD = 12;
pub const KP_Q: ::DWORD = 13;
pub const KP_X: ::DWORD = 14;
pub const KP_Y: ::DWORD = 15;
pub const KP_RA: ::DWORD = 16;
pub const KP_RB: ::DWORD = 17;
pub const KP_INFO: ::DWORD = 18;
pub const KP_EFFECTIVE_KEYLEN: ::DWORD = 19;
pub const KP_SCHANNEL_ALG: ::DWORD = 20;
pub const KP_CLIENT_RANDOM: ::DWORD = 21;
pub const KP_SERVER_RANDOM: ::DWORD = 22;
pub const KP_RP: ::DWORD = 23;
pub const KP_PRECOMP_MD5: ::DWORD = 24;
pub const KP_PRECOMP_SHA: ::DWORD = 25;
pub const KP_CERTIFICATE: ::DWORD = 26;
pub const KP_CLEAR_KEY: ::DWORD = 27;
pub const KP_PUB_EX_LEN: ::DWORD = 28;
pub const KP_PUB_EX_VAL: ::DWORD = 29;
pub const KP_KEYVAL: ::DWORD = 30;
pub const KP_ADMIN_PIN: ::DWORD = 31;
pub const KP_KEYEXCHANGE_PIN: ::DWORD = 32;
pub const KP_SIGNATURE_PIN: ::DWORD = 33;
pub const KP_PREHASH: ::DWORD = 34;
pub const KP_ROUNDS: ::DWORD = 35;
pub const KP_OAEP_PARAMS: ::DWORD = 36;
pub const KP_CMS_KEY_INFO: ::DWORD = 37;
pub const KP_CMS_DH_KEY_INFO: ::DWORD = 38;
pub const KP_PUB_PARAMS: ::DWORD = 39;
pub const KP_VERIFY_PARAMS: ::DWORD = 40;
pub const KP_HIGHEST_VERSION: ::DWORD = 41;
pub const KP_GET_USE_COUNT: ::DWORD = 42;
pub const KP_PIN_ID: ::DWORD = 43;
pub const KP_PIN_INFO: ::DWORD = 44;
pub const PKCS5_PADDING: ::DWORD = 1;
pub const RANDOM_PADDING: ::DWORD = 2;
pub const ZERO_PADDING: ::DWORD = 3;
pub const CRYPT_MODE_CBC: ::DWORD = 1;
pub const CRYPT_MODE_ECB: ::DWORD = 2;
pub const CRYPT_MODE_OFB: ::DWORD = 3;
pub const CRYPT_MODE_CFB: ::DWORD = 4;
pub const CRYPT_MODE_CTS: ::DWORD = 5;
pub const CRYPT_ENCRYPT: ::DWORD = 0x0001;
pub const CRYPT_DECRYPT: ::DWORD = 0x0002;
pub const CRYPT_EXPORT: ::DWORD = 0x0004;
pub const CRYPT_READ: ::DWORD = 0x0008;
pub const CRYPT_WRITE: ::DWORD = 0x0010;
pub const CRYPT_MAC: ::DWORD = 0x0020;
pub const CRYPT_EXPORT_KEY: ::DWORD = 0x0040;
pub const CRYPT_IMPORT_KEY: ::DWORD = 0x0080;
pub const CRYPT_ARCHIVE: ::DWORD = 0x0100;
pub const HP_ALGID: ::DWORD = 0x0001;
pub const HP_HASHVAL: ::DWORD = 0x0002;
pub const HP_HASHSIZE: ::DWORD = 0x0004;
pub const HP_HMAC_INFO: ::DWORD = 0x0005;
pub const HP_TLS1PRF_LABEL: ::DWORD = 0x0006;
pub const HP_TLS1PRF_SEED: ::DWORD = 0x0007;
pub const CRYPT_FAILED: ::BOOL = ::FALSE;
pub const CRYPT_SUCCEED: ::BOOL = ::TRUE;
pub const PP_ENUMALGS: ::DWORD = 1;
pub const PP_ENUMCONTAINERS: ::DWORD = 2;
pub const PP_IMPTYPE: ::DWORD = 3;
pub const PP_NAME: ::DWORD = 4;
pub const PP_VERSION: ::DWORD = 5;
pub const PP_CONTAINER: ::DWORD = 6;
pub const PP_CHANGE_PASSWORD: ::DWORD = 7;
pub const PP_KEYSET_SEC_DESCR: ::DWORD = 8;
pub const PP_CERTCHAIN: ::DWORD = 9;
pub const PP_KEY_TYPE_SUBTYPE: ::DWORD = 10;
pub const PP_PROVTYPE: ::DWORD = 16;
pub const PP_KEYSTORAGE: ::DWORD = 17;
pub const PP_APPLI_CERT: ::DWORD = 18;
pub const PP_SYM_KEYSIZE: ::DWORD = 19;
pub const PP_SESSION_KEYSIZE: ::DWORD = 20;
pub const PP_UI_PROMPT: ::DWORD = 21;
pub const PP_ENUMALGS_EX: ::DWORD = 22;
pub const PP_ENUMMANDROOTS: ::DWORD = 25;
pub const PP_ENUMELECTROOTS: ::DWORD = 26;
pub const PP_KEYSET_TYPE: ::DWORD = 27;
pub const PP_ADMIN_PIN: ::DWORD = 31;
pub const PP_KEYEXCHANGE_PIN: ::DWORD = 32;
pub const PP_SIGNATURE_PIN: ::DWORD = 33;
pub const PP_SIG_KEYSIZE_INC: ::DWORD = 34;
pub const PP_KEYX_KEYSIZE_INC: ::DWORD = 35;
pub const PP_UNIQUE_CONTAINER: ::DWORD = 36;
pub const PP_SGC_INFO: ::DWORD = 37;
pub const PP_USE_HARDWARE_RNG: ::DWORD = 38;
pub const PP_KEYSPEC: ::DWORD = 39;
pub const PP_ENUMEX_SIGNING_PROT: ::DWORD = 40;
pub const PP_CRYPT_COUNT_KEY_USE: ::DWORD = 41;
pub const PP_USER_CERTSTORE: ::DWORD = 42;
pub const PP_SMARTCARD_READER: ::DWORD = 43;
pub const PP_SMARTCARD_GUID: ::DWORD = 45;
pub const PP_ROOT_CERTSTORE: ::DWORD = 46;
pub const PP_SMARTCARD_READER_ICON: ::DWORD = 47;
pub const CRYPT_FIRST: ::DWORD = 1;
pub const CRYPT_NEXT: ::DWORD = 2;
pub const CRYPT_SGC_ENUM: ::DWORD = 4;
pub const CRYPT_IMPL_HARDWARE: ::DWORD = 1;
pub const CRYPT_IMPL_SOFTWARE: ::DWORD = 2;
pub const CRYPT_IMPL_MIXED: ::DWORD = 3;
pub const CRYPT_IMPL_UNKNOWN: ::DWORD = 4;
pub const CRYPT_IMPL_REMOVABLE: ::DWORD = 8;
pub const CRYPT_SEC_DESCR: ::DWORD = 0x00000001;
pub const CRYPT_PSTORE: ::DWORD = 0x00000002;
pub const CRYPT_UI_PROMPT: ::DWORD = 0x00000004;
pub const CRYPT_FLAG_PCT1: ::DWORD = 0x0001;
pub const CRYPT_FLAG_SSL2: ::DWORD = 0x0002;
pub const CRYPT_FLAG_SSL3: ::DWORD = 0x0004;
pub const CRYPT_FLAG_TLS1: ::DWORD = 0x0008;
pub const CRYPT_FLAG_IPSEC: ::DWORD = 0x0010;
pub const CRYPT_FLAG_SIGNING: ::DWORD = 0x0020;
pub const CRYPT_SGC: ::DWORD = 0x0001;
pub const CRYPT_FASTSGC: ::DWORD = 0x0002;
pub const PP_CLIENT_HWND: ::DWORD = 1;
pub const PP_CONTEXT_INFO: ::DWORD = 11;
pub const PP_KEYEXCHANGE_KEYSIZE: ::DWORD = 12;
pub const PP_SIGNATURE_KEYSIZE: ::DWORD = 13;
pub const PP_KEYEXCHANGE_ALG: ::DWORD = 14;
pub const PP_SIGNATURE_ALG: ::DWORD = 15;
pub const PP_DELETEKEY: ::DWORD = 24;
pub const PP_PIN_PROMPT_STRING: ::DWORD = 44;
pub const PP_SECURE_KEYEXCHANGE_PIN: ::DWORD = 47;
pub const PP_SECURE_SIGNATURE_PIN: ::DWORD = 48;
pub const PROV_RSA_FULL: ::DWORD = 1;
pub const PROV_RSA_SIG: ::DWORD = 2;
pub const PROV_DSS: ::DWORD = 3;
pub const PROV_FORTEZZA: ::DWORD = 4;
pub const PROV_MS_EXCHANGE: ::DWORD = 5;
pub const PROV_SSL: ::DWORD = 6;
pub const PROV_RSA_SCHANNEL: ::DWORD = 12;
pub const PROV_DSS_DH: ::DWORD = 13;
pub const PROV_EC_ECDSA_SIG: ::DWORD = 14;
pub const PROV_EC_ECNRA_SIG: ::DWORD = 15;
pub const PROV_EC_ECDSA_FULL: ::DWORD = 16;
pub const PROV_EC_ECNRA_FULL: ::DWORD = 17;
pub const PROV_DH_SCHANNEL: ::DWORD = 18;
pub const PROV_SPYRUS_LYNKS: ::DWORD = 20;
pub const PROV_RNG: ::DWORD = 21;
pub const PROV_INTEL_SEC: ::DWORD = 22;
pub const PROV_REPLACE_OWF: ::DWORD = 23;
pub const PROV_RSA_AES: ::DWORD = 24;
pub const MS_DEF_PROV: &'static str = "Microsoft Base Cryptographic Provider v1.0";
pub const MS_ENHANCED_PROV: &'static str = "Microsoft Enhanced Cryptographic Provider v1.0";
pub const MS_STRONG_PROV: &'static str = "Microsoft Strong Cryptographic Provider";
pub const MS_DEF_RSA_SIG_PROV: &'static str = "Microsoft RSA Signature Cryptographic Provider";
pub const MS_DEF_RSA_SCHANNEL_PROV: &'static str = "Microsoft RSA SChannel Cryptographic Provider";
pub const MS_DEF_DSS_PROV: &'static str = "Microsoft Base DSS Cryptographic Provider";
pub const MS_DEF_DSS_DH_PROV: &'static str = "Microsoft Base DSS and Diffie-Hellman Cryptographic Provider";
pub const MS_ENH_DSS_DH_PROV: &'static str = "Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider";
pub const MS_DEF_DH_SCHANNEL_PROV: &'static str = "Microsoft DH SChannel Cryptographic Provider";
pub const MS_SCARD_PROV: &'static str = "Microsoft Base Smart Card Crypto Provider";
pub const MS_ENH_RSA_AES_PROV: &'static str = "Microsoft Enhanced RSA and AES Cryptographic Provider";
pub const MS_ENH_RSA_AES_PROV_XP: &'static str = "Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)";
pub const MAXUIDLEN: usize = 64;
pub const EXPO_OFFLOAD_REG_VALUE: &'static str = "ExpoOffload";
pub const EXPO_OFFLOAD_FUNC_NAME: &'static str = "OffloadModExpo";
pub const szKEY_CRYPTOAPI_PRIVATE_KEY_OPTIONS: &'static str = "Software\\Policies\\Microsoft\\Cryptography";
pub const szKEY_CACHE_ENABLED: &'static str = "CachePrivateKeys";
pub const szKEY_CACHE_SECONDS: &'static str = "PrivateKeyLifetimeSeconds";
pub const szPRIV_KEY_CACHE_MAX_ITEMS: &'static str = "PrivKeyCacheMaxItems";
pub const cPRIV_KEY_CACHE_MAX_ITEMS_DEFAULT: ::DWORD = 20;
pub const szPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS: &'static str = "PrivKeyCachePurgeIntervalSeconds";
pub const cPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS_DEFAULT: ::DWORD = 86400;
pub const CUR_BLOB_VERSION: ::DWORD = 2;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CMS_KEY_INFO {
    pub dwVersion: ::DWORD,
    pub Algid: ::ALG_ID,
    pub pbOID: *mut ::BYTE,
    pub cbOID: ::DWORD,
}
pub type PCMS_KEY_INFO = *mut CMS_KEY_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HMAC_INFO {
    pub HashAlgid: ::ALG_ID,
    pub pbInnerString: *mut ::BYTE,
    pub cbInnerString: ::DWORD,
    pub pbOuterString: *mut ::BYTE,
    pub cbOuterString: ::DWORD,
}
pub type PHMAC_INFO = *mut HMAC_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCHANNEL_ALG {
    pub dwUse: ::DWORD,
    pub Algid: ::ALG_ID,
    pub cBits: ::DWORD,
    pub dwFlags: ::DWORD,
    pub dwReserved: ::DWORD,
}
pub type PSCHANNEL_ALG = *mut SCHANNEL_ALG;
pub const SCHANNEL_MAC_KEY: ::DWORD = 0x00000000;
pub const SCHANNEL_ENC_KEY: ::DWORD = 0x00000001;
pub const INTERNATIONAL_USAGE: ::DWORD = 0x00000001;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROV_ENUMALGS {
    pub aiAlgid: ::ALG_ID,
    pub dwBitLen: ::DWORD,
    pub dwNameLen: ::DWORD,
    pub szName: [::CHAR; 20],
}
#[repr(C)] #[derive(Copy)]
pub struct PROV_ENUMALGS_EX {
    pub aiAlgid: ::ALG_ID,
    pub dwDefaultLen: ::DWORD,
    pub dwMinLen: ::DWORD,
    pub dwMaxLen: ::DWORD,
    pub dwProtocols: ::DWORD,
    pub dwNameLen: ::DWORD,
    pub szName: [::CHAR; 20],
    pub dwLongNameLen: ::DWORD,
    pub szLongName: [::CHAR; 40],
}
impl Clone for PROV_ENUMALGS_EX { fn clone(&self) -> PROV_ENUMALGS_EX { *self } }
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BLOBHEADER {
    pub bType: ::BYTE,
    pub bVersion: ::BYTE,
    pub reserved: ::WORD,
    pub aiKeyAlg: ::ALG_ID,
}
pub type PUBLICKEYSTRUC = BLOBHEADER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RSAPUBKEY {
    pub magic: ::DWORD,
    pub bitlen: ::DWORD,
    pub pubexp: ::DWORD,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DHPUBKEY {
    pub magic: ::DWORD,
    pub bitlen: ::DWORD,
}
pub type DSSPUBKEY = DHPUBKEY;
pub type KEAPUBKEY = DHPUBKEY;
pub type TEKPUBKEY = DHPUBKEY;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DSSSEED {
    pub counter: ::DWORD,
    pub seed: [::BYTE; 20],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DHPUBKEY_VER3 {
    pub magic: ::DWORD,
    pub bitlenP: ::DWORD,
    pub bitlenQ: ::DWORD,
    pub bitlenJ: ::DWORD,
    pub DSSSeed: ::DSSSEED,
}
pub type DSSPUBKEY_VER3 = DHPUBKEY_VER3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DHPRIVKEY_VER3 {
    pub magic: ::DWORD,
    pub bitlenP: ::DWORD,
    pub bitlenQ: ::DWORD,
    pub bitlenJ: ::DWORD,
    pub bitlenX: ::DWORD,
    pub DSSSeed: ::DSSSEED,
}
pub type DSSPRIVKEY_VER3 = DHPRIVKEY_VER3;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KEY_TYPE_SUBTYPE {
    pub dwKeySpec: ::DWORD,
    pub Type: ::GUID,
    pub Subtype: ::GUID,
}
pub type PKEY_TYPE_SUBTYPE = *mut KEY_TYPE_SUBTYPE;
#[repr(C)] #[derive(Copy)]
pub struct CERT_FORTEZZA_DATA_PROP {
    pub SerialNumber: [::c_uchar; 8],
    pub CertIndex: ::c_int,
    pub CertLabel: [::c_uchar; 36],
}
impl Clone for CERT_FORTEZZA_DATA_PROP { fn clone(&self) -> CERT_FORTEZZA_DATA_PROP { *self } }
#[repr(C)] #[derive(Copy)]
pub struct CRYPT_RC4_KEY_STATE {
    pub Key: [::c_uchar; 16],
    pub SBox: [::c_uchar; 256],
    pub i: ::c_uchar,
    pub j: ::c_uchar,
}
impl Clone for CRYPT_RC4_KEY_STATE { fn clone(&self) -> CRYPT_RC4_KEY_STATE { *self } }
pub type PCRYPT_RC4_KEY_STATE = *mut CRYPT_RC4_KEY_STATE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_DES_KEY_STATE {
    pub Key: [::c_uchar; 8],
    pub IV: [::c_uchar; 8],
    pub Feedback: [::c_uchar; 8],
}
pub type PCRYPT_DES_KEY_STATE = *mut CRYPT_DES_KEY_STATE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_3DES_KEY_STATE {
    pub Key: [::c_uchar; 24],
    pub IV: [::c_uchar; 8],
    pub Feedback: [::c_uchar; 8],
}
pub type PCRYPT_3DES_KEY_STATE = *mut CRYPT_3DES_KEY_STATE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_AES_128_KEY_STATE {
    pub Key: [::c_uchar; 16],
    pub IV: [::c_uchar; 16],
    pub EncryptionState: [[::c_uchar; 16]; 11],
    pub DecryptionState: [[::c_uchar; 16]; 11],
    pub Feedback: [::c_uchar; 16],
}
pub type PCRYPT_AES_128_KEY_STATE = *mut CRYPT_AES_128_KEY_STATE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_AES_256_KEY_STATE {
    pub Key: [::c_uchar; 32],
    pub IV: [::c_uchar; 16],
    pub EncryptionState: [[::c_uchar; 16]; 15],
    pub DecryptionState: [[::c_uchar; 16]; 15],
    pub Feedback: [::c_uchar; 16],
}
pub type PCRYPT_AES_256_KEY_STATE = *mut CRYPT_AES_256_KEY_STATE;
