// Status code.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum EfiStatus{
    EfiSuccess,
    EfiLoadError,
    EfiInvalidParameter,
    EfiUnsupported,
    EfiBadBufferSize,
    EfiBufferTooSmall,
    EfiNotReady,
    EfiDeviceError,
    EfiWriteProtected,
    EfiOutOfResources,
    EfiVolumeCorrupted,
    EfiVolumeFull,
    EfiNoMedia,
    EfiMediaChanged,
    EfiNotFound,
    EfiAccessDenied,
    EfiNoResponse,
    EfiNoMapping,
    EfiTimeout,
    EfiNotStarted,
    EfiAlreadyStarted,
    EfiAborted,
    EfiIcmpError,
    EfiTftpError,
    EfiProtocolError,
    EfiIncompatibleVersion,
    EfiSecurityViolation,
    EfiCrcError,
    EfiEndOfMedia,
    EfiEndOfFile,
    EfiInvalidLanguage,
    EfiCompromisedData,
    EfiHttpError,
    EfiNetworkUnreachable,
    EfiHostUnreachable,
    EfiProtocolUnreachable,
    EfiPortUnreachable,
    EfiConnectioNFin,
    EfiConnectionReset,
    EfiConnectionRefused,
    EfiWarnUnknownGlyph,
    EfiWarnDeleteFailure,
    EfiWarnWriteFailure,
    EfiWarnBufferTooSmall,
    EfiWarnStaleData,
    EfiWarnFileSystem,
    InvalidEfiStatusCode(usize),
}

impl From<usize> for EfiStatus {
    fn from(val: usize) -> Self {
        use EfiStatus::*;
        match val {
            0x0 => EfiSuccess,
            0x8000000000000001 => EfiLoadError,
            0x8000000000000002 => EfiInvalidParameter,
            0x8000000000000003 => EfiUnsupported,
            0x8000000000000004 => EfiBadBufferSize,
            0x8000000000000005 => EfiBufferTooSmall,
            0x8000000000000006 => EfiNotReady,
            0x8000000000000007 => EfiDeviceError,
            0x8000000000000008 => EfiWriteProtected,
            0x8000000000000009 => EfiOutOfResources,
            0x800000000000000a => EfiVolumeCorrupted,
            0x800000000000000b => EfiVolumeFull,
            0x800000000000000c => EfiNoMedia,
            0x800000000000000d => EfiMediaChanged,
            0x800000000000000e => EfiNotFound,
            0x800000000000000f => EfiAccessDenied,
            0x8000000000000010 => EfiNoResponse,
            0x8000000000000011 => EfiNoMapping,
            0x8000000000000012 => EfiTimeout,
            0x8000000000000013 => EfiNotStarted,
            0x8000000000000014 => EfiAlreadyStarted,
            0x8000000000000015 => EfiAborted,
            0x8000000000000016 => EfiIcmpError,
            0x8000000000000017 => EfiTftpError,
            0x8000000000000018 => EfiProtocolError,
            0x8000000000000019 => EfiIncompatibleVersion,
            0x800000000000001a => EfiSecurityViolation,
            0x800000000000001b => EfiCrcError,
            0x800000000000001c => EfiEndOfMedia,
            0x800000000000001f => EfiEndOfFile,
            0x8000000000000020 => EfiInvalidLanguage,
            0x8000000000000021 => EfiCompromisedData,
            0x8000000000000023 => EfiHttpError,
            0x8000000000000064 => EfiNetworkUnreachable,
            0x8000000000000065 => EfiHostUnreachable,
            0x8000000000000066 => EfiProtocolUnreachable,
            0x8000000000000067 => EfiPortUnreachable,
            0x8000000000000068 => EfiConnectioNFin,
            0x8000000000000069 => EfiConnectionReset,
            0x800000000000006a => EfiConnectionRefused,
            0x1 => EfiWarnUnknownGlyph,
            0x2 => EfiWarnDeleteFailure,
            0x3 => EfiWarnWriteFailure,
            0x4 => EfiWarnBufferTooSmall,
            0x5 => EfiWarnStaleData,
            0x6 => EfiWarnFileSystem,
            _ => InvalidEfiStatusCode(val),
        }
    }
}