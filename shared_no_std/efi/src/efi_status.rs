// Status code.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum EfiStatus{
    /// EFI_SUCCESS, The operation completed successfully.
    EfiSuccess,

    /// EFI_LOAD_ERROR, The image failed to load.
    EfiLoadError,
    /// EFI_INVALID_PARAMETER, A parameter was incorrect.
    EfiInvalidParameter,
    /// EFI_UNSUPPORTED, The operation is not supported.
    EfiUnsupported,
    /// EFI_BAD_BUFFER_SIZE, The buffer was not the proper size for the request.
    EfiBadBufferSize,
    /// EFI_BUFFER_TOO_SMALL, The buffer is not large enough to hold the 
    /// requested data. The required buffer size is returned in the appropriate
    ///  parameter when this error occurs.
    EfiBufferTooSmall,
    /// EFI_NOT_READY, There is no data pending upon return.
    EfiNotReady,
    /// EFI_DEVICE_ERROR, The physical device reported an error while attempting 
    /// the operation.
    EfiDeviceError,
    /// EFI_WRITE_PROTECTED, The device cannot be written to.
    EfiWriteProtected,
    /// EFI_OUT_OF_RESOURCES, A resource has run out.
    EfiOutOfResources,
    /// EFI_VOLUME_CORRUPTED, An inconstancy was detected on the file system 
    /// causing the operating to fail.
    EfiVolumeCorrupted,
    /// EFI_VOLUME_FULL, There is no more space on the file system.
    EfiVolumeFull,
    /// EFI_NO_MEDIA, The device does not contain any medium to perform the
    /// operation.
    EfiNoMedia,
    /// EFI_MEDIA_CHANGED, The medium in the device has changed since the last 
    /// access.
    EfiMediaChanged,
    /// EFI_NOT_FOUND, The item was not found.
    EfiNotFound,
    /// EFI_ACCESS_DENIED, Access was denied.
    EfiAccessDenied,
    /// EFI_NO_RESPONSE, The server was not found or did not respond to the 
    /// request.
    EfiNoResponse,
    /// EFI_NO_MAPPING, A mapping to a device does not exist.
    EfiNoMapping,
    /// EFI_TIMEOUT, The timeout time expired.
    EfiTimeout,
    /// EFI_NOT_STARTED, The protocol has not been started.
    EfiNotStarted,
    /// EFI_ALREADY_STARTED, The protocol has already been started.
    EfiAlreadyStarted,
    /// EFI_ABORTED, The operation was aborted.
    EfiAborted,
    /// EFI_ICMP_ERROR, An ICMP error occurred during the network operation.
    EfiIcmpError,
    /// EFI_TFTP_ERROR, A TFTP error occurred during the network operation.
    EfiTftpError,
    /// EFI_PROTOCOL_ERROR, A protocol error occurred during the network 
    /// operation.
    EfiProtocolError,
    /// EFI_INCOMPATIBLE_VERSION, The function encountered an internal version 
    /// that was incompatible with a version requested by the caller.
    EfiIncompatibleVersion,
    /// EFI_SECURITY_VIOLATION, The function was not performed due to a security 
    /// violation.
    EfiSecurityViolation,
    /// EFI_CRC_ERROR, A CRC error was detected.
    EfiCrcError,
    /// EFI_END_OF_MEDIA, Beginning or end of media was reached.
    EfiEndOfMedia,
    /// EFI_END_OF_FILE, The end of the file was reached.
    EfiEndOfFile,
    /// EFI_INVALID_LANGUAGE, The language specified was invalid.
    EfiInvalidLanguage,
    /// EFI_COMPROMISED_DATA, The security status of the data is unknown or 
    /// compromised and the data must be updated or replaced to restore a valid 
    /// security status.
    EfiCompromisedData,
    /// EFI_IP_ADDRESS_CONFLICT, There is an address conflict address allocation
    EfiIpAddressConflict,
    /// EFI_HTTP_ERROR, A HTTP error occurred during the network operation.
    EfiHttpError,
    /// EFI_NETWORK_UNREACHABLE, The transmission fails because an ICMP network 
    /// unreachable error is received.
    EfiNetworkUnreachable,
    /// EFI_HOST_UNREACHABLE, The transmission fails because an ICMP host 
    /// unreachable error is received.
    EfiHostUnreachable,
    /// EFI_PROTOCOL_UNREACHABLE, The transmission fails because an ICMP 
    /// protocol unreachable error is received.
    EfiProtocolUnreachable,
    /// EFI_PORT_UNREACHABLE, The transmission fails and an ICMP port 
    /// unreachable error is received.
    EfiPortUnreachable,
    /// EFI_CONNECTION_FIN, The communication peer has closed the connection and 
    /// there is no any buffered data in the receive buffer of this instance.
    EfiConnectionFin,
    /// EFI_CONNECTION_RESET, The receiving or transmission operation fails 
    /// because this connection is reset either by instance itself or the 
    /// communication peer.
    EfiConnectionReset,
    /// EFI_CONNECTION_REFUSED: The connect fails because this connection is 
    /// initiated with an active open and the connection is refused.
    EfiConnectionRefused,
    
    /// EFI_WARN_UNKNOWN_GLYPH, The string contained one or more characters that
    /// the device could not render and were skipped.
    EfiWarnUnknownGlyph,
    /// EFI_WARN_DELETE_FAILURE, The handle was closed, but the file was not 
    /// deleted.
    EfiWarnDeleteFailure,
    /// EFI_WARN_WRITE_FAILURE, The handle was closed, but the data to the file 
    /// was not flushed properly.
    EfiWarnWriteFailure,
    /// EFI_WARN_BUFFER_TOO_SMALL, The resulting buffer was too small, and the 
    /// data was truncated to the buffer size.
    EfiWarnBufferTooSmall,
    /// EFI_WARN_STALE_DATA, The data has not been updated within the timeframe 
    /// set by local policy for this type of data.
    EfiWarnStaleData,
    /// EFI_WARN_FILE_SYSTEM, The resulting buffer contains UEFI-compliant file 
    /// system.
    EfiWarnFileSystem,
    /// EFI_WARN_RESET_REQUIRED, The operation will be processed across a system 
    /// reset.
    EfiWarnResetRequired,
    
    /// Invalid code
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
            0x8000000000000022 => EfiIpAddressConflict,
            0x8000000000000023 => EfiHttpError,
            0x8000000000000064 => EfiNetworkUnreachable,
            0x8000000000000065 => EfiHostUnreachable,
            0x8000000000000066 => EfiProtocolUnreachable,
            0x8000000000000067 => EfiPortUnreachable,
            0x8000000000000068 => EfiConnectionFin,
            0x8000000000000069 => EfiConnectionReset,
            0x800000000000006a => EfiConnectionRefused,

            0x1 => EfiWarnUnknownGlyph,
            0x2 => EfiWarnDeleteFailure,
            0x3 => EfiWarnWriteFailure,
            0x4 => EfiWarnBufferTooSmall,
            0x5 => EfiWarnStaleData,
            0x6 => EfiWarnFileSystem,
            0x7 => EfiWarnResetRequired,

            _ => InvalidEfiStatusCode(val),
        }
    }
}