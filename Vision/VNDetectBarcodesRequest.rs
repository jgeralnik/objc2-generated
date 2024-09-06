//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNDetectBarcodesRequest;

    #[cfg(feature = "VNRequest")]
    unsafe impl ClassType for VNDetectBarcodesRequest {
        #[inherits(VNRequest, NSObject)]
        type Super = VNImageBasedRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNDetectBarcodesRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNDetectBarcodesRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNDetectBarcodesRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectBarcodesRequest {
        #[cfg(feature = "VNTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other supportedSymbologies)]
        pub unsafe fn supportedSymbologies() -> Retained<NSArray<VNBarcodeSymbology>>;

        #[cfg(feature = "VNTypes")]
        #[method_id(@__retain_semantics Other supportedSymbologiesAndReturnError:_)]
        pub unsafe fn supportedSymbologiesAndReturnError(
            &self,
        ) -> Result<Retained<NSArray<VNBarcodeSymbology>>, Retained<NSError>>;

        #[cfg(feature = "VNTypes")]
        #[method_id(@__retain_semantics Other symbologies)]
        pub unsafe fn symbologies(&self) -> Retained<NSArray<VNBarcodeSymbology>>;

        #[cfg(feature = "VNTypes")]
        #[method(setSymbologies:)]
        pub unsafe fn setSymbologies(&self, symbologies: &NSArray<VNBarcodeSymbology>);

        #[method(coalesceCompositeSymbologies)]
        pub unsafe fn coalesceCompositeSymbologies(&self) -> bool;

        #[method(setCoalesceCompositeSymbologies:)]
        pub unsafe fn setCoalesceCompositeSymbologies(&self, coalesce_composite_symbologies: bool);

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNBarcodeObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectBarcodesRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNDetectBarcodesRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

pub static VNDetectBarcodesRequestRevision1: NSUInteger = 1;

pub static VNDetectBarcodesRequestRevision2: NSUInteger = 2;

pub static VNDetectBarcodesRequestRevision3: NSUInteger = 3;

pub static VNDetectBarcodesRequestRevision4: NSUInteger = 4;
