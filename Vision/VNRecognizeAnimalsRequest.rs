//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type VNAnimalIdentifier = NSString;

extern "C" {
    pub static VNAnimalIdentifierDog: &'static VNAnimalIdentifier;
}

extern "C" {
    pub static VNAnimalIdentifierCat: &'static VNAnimalIdentifier;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNRecognizeAnimalsRequest;

    #[cfg(feature = "VNRequest")]
    unsafe impl ClassType for VNRecognizeAnimalsRequest {
        #[inherits(VNRequest, NSObject)]
        type Super = VNImageBasedRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VNRequest")]
unsafe impl NSCopying for VNRecognizeAnimalsRequest {}

#[cfg(feature = "VNRequest")]
unsafe impl NSObjectProtocol for VNRecognizeAnimalsRequest {}

extern_methods!(
    #[cfg(feature = "VNRequest")]
    unsafe impl VNRecognizeAnimalsRequest {
        #[deprecated]
        #[method_id(@__retain_semantics Other knownAnimalIdentifiersForRevision:error:_)]
        pub unsafe fn knownAnimalIdentifiersForRevision_error(
            request_revision: NSUInteger,
        ) -> Result<Id<NSArray<VNAnimalIdentifier>>, Id<NSError>>;

        #[method_id(@__retain_semantics Other supportedIdentifiersAndReturnError:_)]
        pub unsafe fn supportedIdentifiersAndReturnError(
            &self,
        ) -> Result<Id<NSArray<VNAnimalIdentifier>>, Id<NSError>>;

        #[cfg(feature = "VNObservation")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Option<Id<NSArray<VNRecognizedObjectObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNRequest`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNRecognizeAnimalsRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "VNRequest")]
    unsafe impl VNRecognizeAnimalsRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

pub static VNRecognizeAnimalsRequestRevision1: NSUInteger = 1;

pub static VNRecognizeAnimalsRequestRevision2: NSUInteger = 2;
