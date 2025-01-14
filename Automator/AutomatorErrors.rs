//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/automator/amerrorcode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AMErrorCode(pub NSInteger);
impl AMErrorCode {
    pub const AMWorkflowNewerVersionError: Self = Self(-100);
    pub const AMWorkflowPropertyListInvalidError: Self = Self(-101);
    pub const AMWorkflowNewerActionVersionError: Self = Self(-111);
    pub const AMWorkflowOlderActionVersionError: Self = Self(-112);
    pub const AMWorkflowActionsNotLoadedError: Self = Self(-113);
    pub const AMWorkflowNoEnabledActionsError: Self = Self(-114);
    pub const AMUserCanceledError: Self = Self(-128);
    pub const AMNoSuchActionError: Self = Self(-200);
    pub const AMActionNotLoadableError: Self = Self(-201);
    pub const AMActionArchitectureMismatchError: Self = Self(-202);
    pub const AMActionRuntimeMismatchError: Self = Self(-203);
    pub const AMActionLoadError: Self = Self(-204);
    pub const AMActionLinkError: Self = Self(-205);
    pub const AMActionApplicationResourceError: Self = Self(-206);
    pub const AMActionApplicationVersionResourceError: Self = Self(-207);
    pub const AMActionFileResourceError: Self = Self(-208);
    pub const AMActionLicenseResourceError: Self = Self(-209);
    pub const AMActionRequiredActionResourceError: Self = Self(-210);
    pub const AMActionInitializationError: Self = Self(-211);
    pub const AMActionExecutionError: Self = Self(-212);
    pub const AMActionExceptionError: Self = Self(-213);
    pub const AMActionPropertyListInvalidError: Self = Self(-214);
    pub const AMActionInsufficientDataError: Self = Self(-215);
    pub const AMActionIsDeprecatedError: Self = Self(-216);
    pub const AMActionFailedGatekeeperError: Self = Self(-217);
    pub const AMActionSignatureCorruptError: Self = Self(-218);
    pub const AMActionQuarantineError: Self = Self(-219);
    pub const AMActionXProtectError: Self = Self(-220);
    pub const AMActionMalwareError: Self = Self(-221);
    pub const AMActionThirdPartyActionsNotAllowedError: Self = Self(-222);
    pub const AMActionXPCError: Self = Self(-223);
    pub const AMConversionNotPossibleError: Self = Self(-300);
    pub const AMConversionNoDataError: Self = Self(-301);
    pub const AMConversionFailedError: Self = Self(-302);
}

unsafe impl Encode for AMErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AMErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
