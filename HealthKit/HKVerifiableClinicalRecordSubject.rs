//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKVerifiableClinicalRecordSubject;

    unsafe impl ClassType for HKVerifiableClinicalRecordSubject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKVerifiableClinicalRecordSubject {}

unsafe impl NSCopying for HKVerifiableClinicalRecordSubject {}

unsafe impl CopyingHelper for HKVerifiableClinicalRecordSubject {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKVerifiableClinicalRecordSubject {}

unsafe impl NSSecureCoding for HKVerifiableClinicalRecordSubject {}

extern_methods!(
    unsafe impl HKVerifiableClinicalRecordSubject {
        #[method_id(@__retain_semantics Other fullName)]
        pub unsafe fn fullName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other dateOfBirthComponents)]
        pub unsafe fn dateOfBirthComponents(&self) -> Option<Retained<NSDateComponents>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
