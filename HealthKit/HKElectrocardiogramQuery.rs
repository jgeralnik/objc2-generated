//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkelectrocardiogramvoltagemeasurement?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKElectrocardiogramVoltageMeasurement;
);

unsafe impl Send for HKElectrocardiogramVoltageMeasurement {}

unsafe impl Sync for HKElectrocardiogramVoltageMeasurement {}

unsafe impl NSCopying for HKElectrocardiogramVoltageMeasurement {}

unsafe impl CopyingHelper for HKElectrocardiogramVoltageMeasurement {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKElectrocardiogramVoltageMeasurement {}

extern_methods!(
    unsafe impl HKElectrocardiogramVoltageMeasurement {
        #[method(timeSinceSampleStart)]
        pub unsafe fn timeSinceSampleStart(&self) -> NSTimeInterval;

        #[cfg(all(feature = "HKElectrocardiogram", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other quantityForLead:)]
        pub unsafe fn quantityForLead(
            &self,
            lead: HKElectrocardiogramLead,
        ) -> Option<Retained<HKQuantity>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKElectrocardiogramVoltageMeasurement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkelectrocardiogramquery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKElectrocardiogramQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKElectrocardiogramQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKElectrocardiogramQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKElectrocardiogramQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKElectrocardiogramQuery {
        #[cfg(all(
            feature = "HKElectrocardiogram",
            feature = "HKObject",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithElectrocardiogram:dataHandler:)]
        pub unsafe fn initWithElectrocardiogram_dataHandler(
            this: Allocated<Self>,
            electrocardiogram: &HKElectrocardiogram,
            data_handler: &block2::Block<
                dyn Fn(
                    NonNull<HKElectrocardiogramQuery>,
                    *mut HKElectrocardiogramVoltageMeasurement,
                    Bool,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKElectrocardiogramQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKElectrocardiogramQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
