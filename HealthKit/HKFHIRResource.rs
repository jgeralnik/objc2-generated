//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type HKFHIRResourceType = NSString;

extern "C" {
    pub static HKFHIRResourceTypeAllergyIntolerance: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeCondition: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeCoverage: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeDiagnosticReport: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeDocumentReference: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeImmunization: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeMedicationDispense: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeMedicationOrder: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeMedicationRequest: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeMedicationStatement: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeObservation: &'static HKFHIRResourceType;
}

extern "C" {
    pub static HKFHIRResourceTypeProcedure: &'static HKFHIRResourceType;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKFHIRResource;

    unsafe impl ClassType for HKFHIRResource {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

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
