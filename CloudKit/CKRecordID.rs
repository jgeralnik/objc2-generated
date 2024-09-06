//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKRecordID;

    unsafe impl ClassType for CKRecordID {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKRecordID {}

unsafe impl Sync for CKRecordID {}

unsafe impl NSCoding for CKRecordID {}

unsafe impl NSCopying for CKRecordID {}

unsafe impl CopyingHelper for CKRecordID {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKRecordID {}

unsafe impl NSSecureCoding for CKRecordID {}

extern_methods!(
    unsafe impl CKRecordID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithRecordName:)]
        pub unsafe fn initWithRecordName(
            this: Allocated<Self>,
            record_name: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithRecordName:zoneID:)]
        pub unsafe fn initWithRecordName_zoneID(
            this: Allocated<Self>,
            record_name: &NSString,
            zone_id: &CKRecordZoneID,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other recordName)]
        pub unsafe fn recordName(&self) -> Retained<NSString>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Retained<CKRecordZoneID>;
    }
);
