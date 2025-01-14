//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspictimagerep?language=objc)
    #[unsafe(super(NSImageRep, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSImageRep")]
    pub struct NSPICTImageRep;
);

#[cfg(feature = "NSImageRep")]
unsafe impl NSCoding for NSPICTImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl NSCopying for NSPICTImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl CopyingHelper for NSPICTImageRep {
    type Result = Self;
}

#[cfg(feature = "NSImageRep")]
unsafe impl NSObjectProtocol for NSPICTImageRep {}

extern_methods!(
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSPICTImageRep {
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(pict_data: &NSData) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Allocated<Self>,
            pict_data: &NSData,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other PICTRepresentation)]
        pub unsafe fn PICTRepresentation(&self) -> Retained<NSData>;

        #[method(boundingBox)]
        pub unsafe fn boundingBox(&self) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSImageRep`
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSPICTImageRep {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSPICTImageRep {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
