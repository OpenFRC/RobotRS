
use ni::fpga::fpga_types;

#[allow(dead_code,
        non_camel_case_types,
        non_snake_case_variables,
        non_upper_case_globals)]

pub const NiFpga_False: fpga_types::NiFpga_Bool = 0;
pub const NiFpga_True: fpga_types::NiFpga_Bool = 1;
pub const NiFpga_Status_FifoTimeout: fpga_types::NiFpga_Status = -50400;
pub const NiFpga_Status_TransferAborted: fpga_types::NiFpga_Status = -50405;
pub const NiFpga_Status_MemoryFull: fpga_types::NiFpga_Status = -52000;
pub const NiFpga_Status_SoftwareFault: fpga_types::NiFpga_Status = -52003;
pub const NiFpga_Status_InvalidParameter: fpga_types::NiFpga_Status = -52005;
pub const NiFpga_Status_ResourceNotFound: fpga_types::NiFpga_Status = -52006;
pub const NiFpga_Status_ResourceNotInitialized: fpga_types::NiFpga_Status = -52010;
pub const NiFpga_Status_FpgaAlreadyRunning: fpga_types::NiFpga_Status = -61003;
pub const NiFpga_Status_DownloadError: fpga_types::NiFpga_Status = -61018;
pub const NiFpga_Status_DeviceTypeMismatch: fpga_types::NiFpga_Status = -61024;
pub const NiFpga_Status_CommunicationTimeout: fpga_types::NiFpga_Status = -61046;
pub const NiFpga_Status_IrqTimeout: fpga_types::NiFpga_Status = -61060;
pub const NiFpga_Status_CorruptBitfile: fpga_types::NiFpga_Status = -61070;
pub const NiFpga_Status_BadDepth: fpga_types::NiFpga_Status = -61072;
pub const NiFpga_Status_BadReadWriteCount: fpga_types::NiFpga_Status = -61073;
pub const NiFpga_Status_ClockLostLoc: fpga_types::NiFpga_Status = -61083;
pub const NiFpga_Status_FpgaBusy: fpga_types::NiFpga_Status = -61141;
pub const NiFpga_Status_FpgaBusyFpgaInterfaceCApi: fpga_types::NiFpga_Status = -61200;
pub const NiFpga_Status_FpgaBusyScanInterface: fpga_types::NiFpga_Status = -61201;
pub const NiFpga_Status_FpgaBusyFpgaInterface: fpga_types::NiFpga_Status = -61202;
pub const NiFpga_Status_FpgaBusyFpgaInteractive: fpga_types::NiFpga_Status = -61203;
pub const NiFpga_Status_FpgaBusyFpgaEmulation: fpga_types::NiFpga_Status = -61204;
pub const NiFpga_Status_ResetCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61211;
pub const NiFpga_Status_AbortCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61212;
pub const NiFpga_Status_CloseAndResetCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61213;
pub const NiFpga_Status_ImplicitEnableRemovalButNotYetRun: fpga_types::NiFpga_Status = -61214;
pub const NiFpga_Status_RunAfterStoppedCalledWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61215;
pub const NiFpga_Status_GatedClockHandshakingViolation: fpga_types::NiFpga_Status = -61216;
pub const NiFpga_Status_ElementsNotPermissibleToBeAcquired: fpga_types::NiFpga_Status = -61219;
pub const NiFpga_Status_FpgaBusyConfiguration: fpga_types::NiFpga_Status = -61252;
pub const NiFpga_Status_CloseAndResetCallWithResetNotSupported: fpga_types::NiFpga_Status = -61253;
pub const NiFpga_Status_InteralError: fpga_types::NiFpga_Status = -61499;
pub const NiFpga_Status_TotalDmaFifoDepthExceeded: fpga_types::NiFpga_Status = -63003;
pub const NiFpga_Status_AccessDenied: fpga_types::NiFpga_Status = -63033;
pub const NiFpga_Status_HostVersionMismatch: fpga_types::NiFpga_Status = -63038;
pub const NiFpga_Status_RpcConnectionError: fpga_types::NiFpga_Status = -63040;
pub const NiFpga_Status_RpcSessionError: fpga_types::NiFpga_Status = -63043;
pub const NiFpga_Status_FifoReserved: fpga_types::NiFpga_Status = -63082;
pub const NiFpga_Status_FifoElementsCurrentlyAcquired: fpga_types::NiFpga_Status = -63083;
pub const NiFpga_Status_MisalignedAccess: fpga_types::NiFpga_Status = -63084;
pub const NiFpga_Status_ControlOrIndicatorTooLarge: fpga_types::NiFpga_Status = -63085;
pub const NiFpga_Status_BitfileReadError: fpga_types::NiFpga_Status = -63101;
pub const NiFpga_Status_SignatureMismatch: fpga_types::NiFpga_Status = -63106;
pub const NiFpga_Status_IncompatibleBitfile: fpga_types::NiFpga_Status = -63107;
pub const NiFpga_Status_HardwareFault: fpga_types::NiFpga_Status = -63150;
pub const NiFpga_Status_InvalidResourceName: fpga_types::NiFpga_Status = -63192;
pub const NiFpga_Status_FeatureNotSupported: fpga_types::NiFpga_Status = -63193;
pub const NiFpga_Status_InvalidSession: fpga_types::NiFpga_Status = -63195;
pub const NiFpga_Status_OutOfHandles: fpga_types::NiFpga_Status = -63198;
