//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVVideoFrameAnalysisType(pub NSUInteger);
impl AVVideoFrameAnalysisType {
    #[doc(alias = "AVVideoFrameAnalysisTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "AVVideoFrameAnalysisTypeDefault")]
    pub const Default: Self = Self(1 << 0);
    #[doc(alias = "AVVideoFrameAnalysisTypeText")]
    pub const Text: Self = Self(1 << 1);
    #[doc(alias = "AVVideoFrameAnalysisTypeSubject")]
    pub const Subject: Self = Self(1 << 2);
    #[doc(alias = "AVVideoFrameAnalysisTypeVisualSearch")]
    pub const VisualSearch: Self = Self(1 << 3);
    #[doc(alias = "AVVideoFrameAnalysisTypeMachineReadableCode")]
    pub const MachineReadableCode: Self = Self(1 << 4);
}

unsafe impl Encode for AVVideoFrameAnalysisType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVVideoFrameAnalysisType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
