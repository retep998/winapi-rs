// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Cryptographic Primitive API Prototypes and Definitions
pub const KDF_HASH_ALGORITHM: ::ULONG = 0x0;
pub const KDF_SECRET_PREPEND: ::ULONG = 0x1;
pub const KDF_SECRET_APPEND: ::ULONG = 0x2;
pub const KDF_HMAC_KEY: ::ULONG = 0x3;
pub const KDF_TLS_PRF_LABEL: ::ULONG = 0x4;
pub const KDF_TLS_PRF_SEED: ::ULONG = 0x5;
pub const KDF_SECRET_HANDLE: ::ULONG = 0x6;
pub const KDF_TLS_PRF_PROTOCOL: ::ULONG = 0x7;
pub const KDF_ALGORITHMID: ::ULONG = 0x8;
pub const KDF_PARTYUINFO: ::ULONG = 0x9;
pub const KDF_PARTYVINFO: ::ULONG = 0xA;
pub const KDF_SUPPPUBINFO: ::ULONG = 0xB;
pub const KDF_SUPPPRIVINFO: ::ULONG = 0xC;
pub const KDF_LABEL: ::ULONG = 0xD;
pub const KDF_CONTEXT: ::ULONG = 0xE;
pub const KDF_SALT: ::ULONG = 0xF;
pub const KDF_ITERATION_COUNT: ::ULONG = 0x10;
pub const KDF_GENERIC_PARAMETER: ::ULONG = 0x11;
pub const KDF_KEYBITLENGTH: ::ULONG = 0x12;

pub const KDF_USE_SECRET_AS_HMAC_KEY_FLAG: ::ULONG = 0x1;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_KEY_LENGTHS_STRUCT {
    pub dwMinLength: ::ULONG,
    pub dwMaxLength: ::ULONG,
    pub dwIncrement: ::ULONG,
}
pub type BCRYPT_AUTH_TAG_LENGTHS_STRUCT = BCRYPT_KEY_LENGTHS_STRUCT;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_OID {
    pub cbOID: ::ULONG,
    pub pbOID: ::PUCHAR,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_OID_LIST {
    pub dwOIDCount: ::ULONG,
    pub pOIDs: *mut BCRYPT_OID,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_PKCS1_PADDING_INFO {
    pub pszAlgId: ::LPCWSTR,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_PSS_PADDING_INFO {
    pub pszAlgId: ::LPCWSTR,
    pub cbSalt: ::ULONG,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_OAEP_PADDING_INFO {
    pub pszAlgId: ::LPCWSTR,
    pub pbLabel: ::PUCHAR,
    pub cbLabel: ::ULONG,
}

pub const BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO_VERSION: ::ULONG = 1;

pub const BCRYPT_AUTH_MODE_CHAIN_CALLS_FLAG: ::ULONG = 0x00000001;
pub const BCRYPT_AUTH_MODE_IN_PROGRESS_FLAG: ::ULONG = 0x00000002;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    pub cbSize: ::ULONG,
    pub dwInfoVersion: ::ULONG,
    pub pbNonce: ::PUCHAR,
    pub cbNonce: ::ULONG,
    pub pbAuthData: ::PUCHAR,
    pub cbAuthData: ::ULONG,
    pub pbTag: ::PUCHAR,
    pub cbTag: ::ULONG,
    pub pbMacContext: ::PUCHAR,
    pub cbMacContext: ::ULONG,
    pub cbAAD: ::ULONG,
    pub cbData: ::ULONGLONG,
    pub dwFlags: ::ULONG,
}
pub type PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO = *mut BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO;

pub const BCRYPT_PROV_DISPATCH: ::ULONG = 0x00000001;

pub const BCRYPT_BLOCK_PADDING: ::ULONG = 0x00000001;

pub const BCRYPT_PAD_NONE: ::ULONG = 0x00000001;
pub const BCRYPT_PAD_PKCS1: ::ULONG = 0x00000002;
pub const BCRYPT_PAD_OAEP: ::ULONG = 0x00000004;
pub const BCRYPT_PAD_PSS: ::ULONG = 0x00000008;
pub const BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID: ::ULONG = 0x00000010;

pub const BCRYPTBUFFER_VERSION: ::ULONG = 0;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCryptBuffer {
    pub cbBuffer: ::ULONG,
    pub BufferType: ::ULONG,
    pub pvBuffer: ::PVOID,
}
pub type PBCryptBuffer = *mut BCryptBuffer;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCryptBufferDesc {
    pub ulVersion: ::ULONG,
    pub cBuffers: ::ULONG,
    pub pBuffers: PBCryptBuffer,
}
pub type PBCryptBufferDesc = *mut BCryptBufferDesc;

//321
pub type BCRYPT_HANDLE = ::PVOID;
pub type BCRYPT_ALG_HANDLE = ::PVOID;
pub type BCRYPT_KEY_HANDLE = ::PVOID;
pub type BCRYPT_HASH_HANDLE = ::PVOID;
pub type BCRYPT_SECRET_HANDLE = ::PVOID;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_KEY_BLOB {
    pub Magic: ::ULONG,
}

pub const BCRYPT_RSAPUBLIC_MAGIC: ::ULONG = 0x31415352;
pub const BCRYPT_RSAPRIVATE_MAGIC: ::ULONG = 0x32415352;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_RSAKEY_BLOB {
    pub Magic: ::ULONG,
    pub BitLength: ::ULONG,
    pub cbPublicExp: ::ULONG,
    pub cbModulus: ::ULONG,
    pub cbPrime1: ::ULONG,
    pub cbPrime2: ::ULONG,
}

pub const BCRYPT_RSAFULLPRIVATE_MAGIC: ::ULONG = 0x33415352;

pub const BCRYPT_ECDH_PUBLIC_P256_MAGIC: ::ULONG = 0x314B4345;
pub const BCRYPT_ECDH_PRIVATE_P256_MAGIC: ::ULONG = 0x324B4345;
pub const BCRYPT_ECDH_PUBLIC_P384_MAGIC: ::ULONG = 0x334B4345;
pub const BCRYPT_ECDH_PRIVATE_P384_MAGIC: ::ULONG = 0x344B4345;
pub const BCRYPT_ECDH_PUBLIC_P521_MAGIC: ::ULONG = 0x354B4345;
pub const BCRYPT_ECDH_PRIVATE_P521_MAGIC: ::ULONG = 0x364B4345;

pub const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: ::ULONG = 0x31534345;
pub const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: ::ULONG = 0x32534345;
pub const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: ::ULONG = 0x33534345;
pub const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: ::ULONG = 0x34534345;
pub const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: ::ULONG = 0x35534345;
pub const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: ::ULONG = 0x36534345;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_ECCKEY_BLOB {
    pub dwMagic: ::ULONG,
    pub cbKey: ::ULONG,
}
pub type PBCRYPT_ECCKEY_BLOB = *mut BCRYPT_ECCKEY_BLOB;

pub const BCRYPT_DH_PUBLIC_MAGIC: ::ULONG = 0x42504844;
pub const BCRYPT_DH_PRIVATE_MAGIC: ::ULONG = 0x56504844;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_DH_KEY_BLOB {
    pub dwMagic: ::ULONG,
    pub cbKey: ::ULONG,
}
pub type PBCRYPT_DH_KEY_BLOB = *mut BCRYPT_DH_KEY_BLOB;

pub const BCRYPT_DH_PARAMETERS_MAGIC: ::ULONG = 0x4d504844;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_DH_PARAMETER_HEADER {
    pub cbLength: ::ULONG,
    pub dwMagic: ::ULONG,
    pub cbKeyLength: ::ULONG,
}

pub const BCRYPT_DSA_PUBLIC_MAGIC: ::ULONG = 0x42505344;
pub const BCRYPT_DSA_PRIVATE_MAGIC: ::ULONG = 0x56505344;
pub const BCRYPT_DSA_PUBLIC_MAGIC_V2: ::ULONG = 0x32425044;
pub const BCRYPT_DSA_PRIVATE_MAGIC_V2: ::ULONG = 0x32565044;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_DSA_KEY_BLOB {
    pub dwMagic: ::ULONG,
    pub cbKey: ::ULONG,
    pub Count: [::UCHAR; 4],
    pub Seed: [::UCHAR; 20],
    pub q: [::UCHAR; 20],
}
pub type PBCRYPT_DSA_KEY_BLOB = *mut BCRYPT_DSA_KEY_BLOB;

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum HASHALGORITHM_ENUM {
    DSA_HASH_ALGORITHM_SHA1,
    DSA_HASH_ALGORITHM_SHA256,
    DSA_HASH_ALGORITHM_SHA512,
}

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum DSAFIPSVERSION_ENUM {
    DSA_FIPS186_2,
    DSA_FIPS186_3,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_DSA_KEY_BLOB_V2 {
    pub dwMagic: ::ULONG,
    pub cbKey: ::ULONG,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: ::ULONG,
    pub cbGroupSize: ::ULONG,
    pub Count: [::UCHAR; 4],
}
pub type PBCRYPT_DSA_KEY_BLOB_V2 = *mut BCRYPT_DSA_KEY_BLOB_V2;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_KEY_DATA_BLOB_HEADER {
    pub dwMagic: ::ULONG,
    pub dwVersion: ::ULONG,
    pub cbKeyData: ::ULONG,
}
pub type PBCRYPT_KEY_DATA_BLOB_HEADER = *mut BCRYPT_KEY_DATA_BLOB_HEADER;

pub const BCRYPT_KEY_DATA_BLOB_MAGIC: ::ULONG = 0x4d42444b;

pub const BCRYPT_KEY_DATA_BLOB_VERSION1: ::ULONG = 0x1;

pub const BCRYPT_DSA_PARAMETERS_MAGIC: ::ULONG = 0x4d505344;
pub const BCRYPT_DSA_PARAMETERS_MAGIC_V2: ::ULONG = 0x324d5044;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_DSA_PARAMETER_HEADER {
    pub cbLength: ::ULONG,
    pub dwMagic: ::ULONG,
    pub cbKeyLength: ::ULONG,
    pub Count: [::UCHAR; 4],
    pub Seed: [::UCHAR; 20],
    pub q: [::UCHAR; 20],
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_DSA_PARAMETER_HEADER_V2 {
    pub cbLength: ::ULONG,
    pub dwMagic: ::ULONG,
    pub cbKeyLength: ::ULONG,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: ::ULONG,
    pub cbGroupSize: ::ULONG,
    pub Count: [::UCHAR; 4],
}

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum BCRYPT_HASH_OPERATION_TYPE {
    BCRYPT_HASH_OPERATION_HASH_DATA = 1,
    BCRYPT_HASH_OPERATION_FINISH_HASH = 2,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_MULTI_HASH_OPERATION {
    pub iHash: ::ULONG,
    pub hashOperation: BCRYPT_HASH_OPERATION_TYPE,
    pub pbBuffer: ::PUCHAR,
    pub cbBuffer: ::ULONG,
}

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum BCRYPT_MULTI_OPERATION_TYPE {
    BCRYPT_OPERATION_TYPE_HASH = 1,
    DUMMY, // FIXME: due to `rustc --explain E0083`
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    pub cbPerObject: ::ULONG,
    pub cbPerElement: ::ULONG,
}

pub const BCRYPT_CIPHER_INTERFACE: ::ULONG = 0x00000001;
pub const BCRYPT_HASH_INTERFACE: ::ULONG = 0x00000002;
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: ::ULONG = 0x00000003;
pub const BCRYPT_SECRET_AGREEMENT_INTERFACE: ::ULONG = 0x00000004;
pub const BCRYPT_SIGNATURE_INTERFACE: ::ULONG = 0x00000005;
pub const BCRYPT_RNG_INTERFACE: ::ULONG = 0x00000006;
pub const BCRYPT_KEY_DERIVATION_INTERFACE: ::ULONG = 0x00000007;

pub const BCRYPT_ALG_HANDLE_HMAC_FLAG: ::ULONG = 0x00000008;
pub const BCRYPT_CAPI_AES_FLAG: ::ULONG = 0x00000010;
pub const BCRYPT_HASH_REUSABLE_FLAG: ::ULONG = 0x00000020;
pub const BCRYPT_BUFFERS_LOCKED_FLAG: ::ULONG = 0x00000040;
pub const BCRYPT_EXTENDED_KEYSIZE: ::ULONG = 0x00000080;

pub const BCRYPT_CIPHER_OPERATION: ::ULONG = 0x00000001;
pub const BCRYPT_HASH_OPERATION: ::ULONG = 0x00000002;
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: ::ULONG = 0x00000004;
pub const BCRYPT_SECRET_AGREEMENT_OPERATION: ::ULONG = 0x00000008;
pub const BCRYPT_SIGNATURE_OPERATION: ::ULONG = 0x00000010;
pub const BCRYPT_RNG_OPERATION: ::ULONG = 0x00000020;
pub const BCRYPT_KEY_DERIVATION_OPERATION: ::ULONG = 0x00000040;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_ALGORITHM_IDENTIFIER {
    pub pszName: ::LPWSTR,
    pub dwClass: ::ULONG,
    pub dwFlags: ::ULONG,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_PROVIDER_NAME {
    pub pszProviderName: ::LPWSTR,
}

pub const BCRYPT_PUBLIC_KEY_FLAG: ::ULONG = 0x00000001;
pub const BCRYPT_PRIVATE_KEY_FLAG: ::ULONG = 0x00000002;

pub const BCRYPT_RNG_USE_ENTROPY_IN_BUFFER: ::ULONG = 0x00000001;
pub const BCRYPT_USE_SYSTEM_PREFERRED_RNG: ::ULONG = 0x00000002;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BCRYPT_INTERFACE_VERSION {
    pub MajorVersion: ::USHORT,
    pub MinorVersion: ::USHORT,
}
pub type PBCRYPT_INTERFACE_VERSION = *mut BCRYPT_INTERFACE_VERSION;

#[inline] #[allow(dead_code)]
pub fn BCRYPT_MAKE_INTERFACE_VERSION(major: ::USHORT, minor: ::USHORT) -> BCRYPT_INTERFACE_VERSION {
    BCRYPT_INTERFACE_VERSION{ MajorVersion: major, MinorVersion: minor }
}

pub const BCRYPT_CIPHER_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION = 
    BCRYPT_INTERFACE_VERSION{ MajorVersion: 1, MinorVersion: 0 };
pub const BCRYPT_HASH_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION = 
    BCRYPT_INTERFACE_VERSION{ MajorVersion: 1, MinorVersion: 0 };
pub const BCRYPT_HASH_INTERFACE_MAJORVERSION_2: ::USHORT = 2;
pub const BCRYPT_HASH_INTERFACE_VERSION_2: BCRYPT_INTERFACE_VERSION = 
    BCRYPT_INTERFACE_VERSION{ MajorVersion: BCRYPT_HASH_INTERFACE_MAJORVERSION_2, MinorVersion: 0 };
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION = 
    BCRYPT_INTERFACE_VERSION{ MajorVersion: 1, MinorVersion: 0 };
pub const BCRYPT_SECRET_AGREEMENT_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION = 
    BCRYPT_INTERFACE_VERSION{ MajorVersion: 1, MinorVersion: 0 };
pub const BCRYPT_SIGNATURE_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION = 
    BCRYPT_INTERFACE_VERSION{ MajorVersion: 1, MinorVersion: 0 };
pub const BCRYPT_RNG_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION = 
    BCRYPT_INTERFACE_VERSION{ MajorVersion: 1, MinorVersion: 0 };

pub const CRYPT_MIN_DEPENDENCIES: ::ULONG = 0x00000001;
pub const CRYPT_PROCESS_ISOLATE: ::ULONG = 0x00010000;
pub const CRYPT_UM: ::ULONG = 0x00000001;
pub const CRYPT_KM: ::ULONG = 0x00000002;
pub const CRYPT_MM: ::ULONG = 0x00000003;
pub const CRYPT_ANY: ::ULONG = 0x00000004;
pub const CRYPT_OVERWRITE: ::ULONG = 0x00000001;
pub const CRYPT_LOCAL: ::ULONG = 0x00000001;
pub const CRYPT_DOMAIN: ::ULONG = 0x00000002;
pub const CRYPT_EXCLUSIVE: ::ULONG = 0x00000001;
pub const CRYPT_OVERRIDE: ::ULONG = 0x00010000;
pub const CRYPT_ALL_FUNCTIONS: ::ULONG = 0x00000001;
pub const CRYPT_ALL_PROVIDERS: ::ULONG = 0x00000002;
pub const CRYPT_PRIORITY_TOP: ::ULONG = 0x00000000;
pub const CRYPT_PRIORITY_BOTTOM: ::ULONG = 0xFFFFFFFF;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_INTERFACE_REG {
    pub dwInterface: ::ULONG,
    pub dwFlags: ::ULONG,
    pub cFunctions: ::ULONG,
    pub rgpszFunctions: *mut ::PWSTR,
}
pub type PCRYPT_INTERFACE_REG = *mut CRYPT_INTERFACE_REG;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_IMAGE_REG {
    pub pszImage: ::PWSTR,
    pub cInterfaces: ::ULONG,
    pub rgpInterfaces: *mut PCRYPT_INTERFACE_REG,
}
pub type PCRYPT_IMAGE_REG = *mut CRYPT_IMAGE_REG;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_PROVIDER_REG {
    pub cAliases: ::ULONG,
    pub rgpszAliases: *mut ::PWSTR,
    pub pUM: PCRYPT_IMAGE_REG,
    pub pKM: PCRYPT_IMAGE_REG,
}
pub type PCRYPT_PROVIDER_REG = *mut CRYPT_PROVIDER_REG;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_PROVIDERS {
    pub cProviders: ::ULONG,
    pub rgpszProviders: *mut ::PWSTR,
}
pub type PCRYPT_PROVIDERS = *mut CRYPT_PROVIDERS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_CONTEXT_CONFIG {
    pub dwFlags: ::ULONG,
    pub dwReserved: ::ULONG,
}
pub type PCRYPT_CONTEXT_CONFIG = *mut CRYPT_CONTEXT_CONFIG;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_CONTEXT_FUNCTION_CONFIG {
    pub dwFlags: ::ULONG,
    pub dwReserved: ::ULONG,
}
pub type PCRYPT_CONTEXT_FUNCTION_CONFIG = *mut CRYPT_CONTEXT_FUNCTION_CONFIG;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_CONTEXTS {
    pub cContexts: ::ULONG,
    pub rgpszContexts: *mut ::PWSTR,
}
pub type PCRYPT_CONTEXTS = *mut CRYPT_CONTEXTS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_CONTEXT_FUNCTIONS {
    pub cFunctions: ::ULONG,
    pub rgpszFunctions: *mut ::PWSTR,
}
pub type PCRYPT_CONTEXT_FUNCTIONS = *mut CRYPT_CONTEXT_FUNCTIONS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    pub cProviders: ::ULONG,
    pub rgpszProviders: *mut ::PWSTR,
}
pub type PCRYPT_CONTEXT_FUNCTION_PROVIDERS = *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_PROPERTY_REF {
    pub pszProperty: ::PWSTR,
    pub cbValue: ::ULONG,
    pub pbValue: ::PUCHAR,
}
pub type PCRYPT_PROPERTY_REF = *mut CRYPT_PROPERTY_REF;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_IMAGE_REF {
    pub pszImage: ::PWSTR,
    pub dwFlags: ::ULONG,
}
pub type PCRYPT_IMAGE_REF = *mut CRYPT_IMAGE_REF;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_PROVIDER_REF {
    pub dwInterface: ::ULONG,
    pub pszFunction: ::PWSTR,
    pub pszProvider: ::PWSTR,
    pub cProperties: ::ULONG,
    pub rgpProperties: *mut PCRYPT_PROPERTY_REF,
    pub pUM: PCRYPT_IMAGE_REF,
    pub pKM: PCRYPT_IMAGE_REF,
}
pub type PCRYPT_PROVIDER_REF = *mut CRYPT_PROVIDER_REF;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPT_PROVIDER_REFS {
    pub cProviders: ::ULONG,
    pub rgpProviders: *mut PCRYPT_PROVIDER_REF,
}
pub type PCRYPT_PROVIDER_REFS = *mut CRYPT_PROVIDER_REFS;
