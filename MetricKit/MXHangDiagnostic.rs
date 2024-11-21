//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(MXDiagnostic, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXDiagnostic")]
    pub struct MXHangDiagnostic;
);

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSCoding for MXHangDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSObjectProtocol for MXHangDiagnostic {}

#[cfg(feature = "MXDiagnostic")]
unsafe impl NSSecureCoding for MXHangDiagnostic {}

extern_methods!(
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXHangDiagnostic {
        #[cfg(feature = "MXCallStackTree")]
        #[method_id(@__retain_semantics Other callStackTree)]
        pub unsafe fn callStackTree(&self) -> Retained<MXCallStackTree>;

        #[method_id(@__retain_semantics Other hangDuration)]
        pub unsafe fn hangDuration(&self) -> Retained<NSMeasurement<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXDiagnostic")]
    unsafe impl MXHangDiagnostic {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
