//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetype?language=objc)
// NS_TYPED_ENUM
pub type HKFHIRResourceType = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypeallergyintolerance?language=objc)
    pub static HKFHIRResourceTypeAllergyIntolerance: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypecondition?language=objc)
    pub static HKFHIRResourceTypeCondition: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypecoverage?language=objc)
    pub static HKFHIRResourceTypeCoverage: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypediagnosticreport?language=objc)
    pub static HKFHIRResourceTypeDiagnosticReport: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypedocumentreference?language=objc)
    pub static HKFHIRResourceTypeDocumentReference: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypeimmunization?language=objc)
    pub static HKFHIRResourceTypeImmunization: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypemedicationdispense?language=objc)
    pub static HKFHIRResourceTypeMedicationDispense: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypemedicationorder?language=objc)
    pub static HKFHIRResourceTypeMedicationOrder: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypemedicationrequest?language=objc)
    pub static HKFHIRResourceTypeMedicationRequest: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypemedicationstatement?language=objc)
    pub static HKFHIRResourceTypeMedicationStatement: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypeobservation?language=objc)
    pub static HKFHIRResourceTypeObservation: &'static HKFHIRResourceType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresourcetypeprocedure?language=objc)
    pub static HKFHIRResourceTypeProcedure: &'static HKFHIRResourceType;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkfhirresource?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKFHIRResource;
);

unsafe impl Send for HKFHIRResource {}

unsafe impl Sync for HKFHIRResource {}

unsafe impl NSCoding for HKFHIRResource {}

unsafe impl NSCopying for HKFHIRResource {}

unsafe impl CopyingHelper for HKFHIRResource {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKFHIRResource {}

unsafe impl NSSecureCoding for HKFHIRResource {}

extern_methods!(
    unsafe impl HKFHIRResource {
        #[cfg(feature = "HKFHIRVersion")]
        #[method_id(@__retain_semantics Other FHIRVersion)]
        pub unsafe fn FHIRVersion(&self) -> Retained<HKFHIRVersion>;

        #[method_id(@__retain_semantics Other resourceType)]
        pub unsafe fn resourceType(&self) -> Retained<HKFHIRResourceType>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other sourceURL)]
        pub unsafe fn sourceURL(&self) -> Option<Retained<NSURL>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKFHIRResource {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
