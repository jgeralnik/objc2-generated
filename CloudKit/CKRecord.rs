//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

pub type CKRecordType = NSString;

pub type CKRecordFieldKey = NSString;

extern "C" {
    pub static CKRecordTypeUserRecord: &'static CKRecordType;
}

extern "C" {
    pub static CKRecordRecordIDKey: &'static CKRecordFieldKey;
}

extern "C" {
    pub static CKRecordCreatorUserRecordIDKey: &'static CKRecordFieldKey;
}

extern "C" {
    pub static CKRecordCreationDateKey: &'static CKRecordFieldKey;
}

extern "C" {
    pub static CKRecordLastModifiedUserRecordIDKey: &'static CKRecordFieldKey;
}

extern "C" {
    pub static CKRecordModificationDateKey: &'static CKRecordFieldKey;
}

extern "C" {
    pub static CKRecordParentKey: &'static CKRecordFieldKey;
}

extern "C" {
    pub static CKRecordShareKey: &'static CKRecordFieldKey;
}

extern_protocol!(
    pub unsafe trait CKRecordValue: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn CKRecordValue {}
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKRecord;
);

unsafe impl NSCoding for CKRecord {}

unsafe impl NSCopying for CKRecord {}

unsafe impl CopyingHelper for CKRecord {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKRecord {}

unsafe impl NSSecureCoding for CKRecord {}

extern_methods!(
    unsafe impl CKRecord {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithRecordType:)]
        pub unsafe fn initWithRecordType(
            this: Allocated<Self>,
            record_type: &CKRecordType,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Init initWithRecordType:recordID:)]
        pub unsafe fn initWithRecordType_recordID(
            this: Allocated<Self>,
            record_type: &CKRecordType,
            record_id: &CKRecordID,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithRecordType:zoneID:)]
        pub unsafe fn initWithRecordType_zoneID(
            this: Allocated<Self>,
            record_type: &CKRecordType,
            zone_id: &CKRecordZoneID,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Retained<CKRecordType>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other recordID)]
        pub unsafe fn recordID(&self) -> Retained<CKRecordID>;

        #[method_id(@__retain_semantics Other recordChangeTag)]
        pub unsafe fn recordChangeTag(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other creatorUserRecordID)]
        pub unsafe fn creatorUserRecordID(&self) -> Option<Retained<CKRecordID>>;

        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other lastModifiedUserRecordID)]
        pub unsafe fn lastModifiedUserRecordID(&self) -> Option<Retained<CKRecordID>>;

        #[method_id(@__retain_semantics Other modificationDate)]
        pub unsafe fn modificationDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            key: &CKRecordFieldKey,
        ) -> Option<Retained<ProtocolObject<dyn CKRecordValue>>>;

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(
            &self,
            object: Option<&ProtocolObject<dyn CKRecordValue>>,
            key: &CKRecordFieldKey,
        );

        #[method_id(@__retain_semantics Other allKeys)]
        pub unsafe fn allKeys(&self) -> Retained<NSArray<CKRecordFieldKey>>;

        #[method_id(@__retain_semantics Other allTokens)]
        pub unsafe fn allTokens(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &CKRecordFieldKey,
        ) -> Option<Retained<ProtocolObject<dyn CKRecordValue>>>;

        #[method(setObject:forKeyedSubscript:)]
        pub unsafe fn setObject_forKeyedSubscript(
            &self,
            object: Option<&ProtocolObject<dyn CKRecordValue>>,
            key: &CKRecordFieldKey,
        );

        #[method_id(@__retain_semantics Other changedKeys)]
        pub unsafe fn changedKeys(&self) -> Retained<NSArray<CKRecordFieldKey>>;

        #[method(encodeSystemFieldsWithCoder:)]
        pub unsafe fn encodeSystemFieldsWithCoder(&self, coder: &NSCoder);

        #[cfg(feature = "CKReference")]
        #[method_id(@__retain_semantics Other share)]
        pub unsafe fn share(&self) -> Option<Retained<CKReference>>;

        #[cfg(feature = "CKReference")]
        #[method_id(@__retain_semantics Other parent)]
        pub unsafe fn parent(&self) -> Option<Retained<CKReference>>;

        #[cfg(feature = "CKReference")]
        #[method(setParent:)]
        pub unsafe fn setParent(&self, parent: Option<&CKReference>);

        #[method(setParentReferenceFromRecord:)]
        pub unsafe fn setParentReferenceFromRecord(&self, parent_record: Option<&CKRecord>);

        #[cfg(feature = "CKRecordID")]
        #[method(setParentReferenceFromRecordID:)]
        pub unsafe fn setParentReferenceFromRecordID(&self, parent_record_id: Option<&CKRecordID>);
    }
);

unsafe impl CKRecordValue for NSString {}

unsafe impl CKRecordValue for NSNumber {}

unsafe impl CKRecordValue for NSArray {}

unsafe impl CKRecordValue for NSDate {}

unsafe impl CKRecordValue for NSData {}

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "CKReference")]
    unsafe impl CKReference {}
);

#[cfg(feature = "CKReference")]
unsafe impl CKRecordValue for CKReference {}

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "CKAsset")]
    unsafe impl CKAsset {}
);

#[cfg(feature = "CKAsset")]
unsafe impl CKRecordValue for CKAsset {}

#[cfg(feature = "objc2-core-location")]
unsafe impl CKRecordValue for CLLocation {}

extern_protocol!(
    pub unsafe trait CKRecordKeyValueSetting: NSObjectProtocol {
        #[method_id(@__retain_semantics Other objectForKey:)]
        unsafe fn objectForKey(
            &self,
            key: &CKRecordFieldKey,
        ) -> Option<Retained<ProtocolObject<dyn CKRecordValue>>>;

        #[method(setObject:forKey:)]
        unsafe fn setObject_forKey(
            &self,
            object: Option<&ProtocolObject<dyn CKRecordValue>>,
            key: &CKRecordFieldKey,
        );

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        unsafe fn objectForKeyedSubscript(
            &self,
            key: &CKRecordFieldKey,
        ) -> Option<Retained<ProtocolObject<dyn CKRecordValue>>>;

        #[method(setObject:forKeyedSubscript:)]
        unsafe fn setObject_forKeyedSubscript(
            &self,
            object: Option<&ProtocolObject<dyn CKRecordValue>>,
            key: &CKRecordFieldKey,
        );

        #[method_id(@__retain_semantics Other allKeys)]
        unsafe fn allKeys(&self) -> Retained<NSArray<CKRecordFieldKey>>;

        #[method_id(@__retain_semantics Other changedKeys)]
        unsafe fn changedKeys(&self) -> Retained<NSArray<CKRecordFieldKey>>;
    }

    unsafe impl ProtocolType for dyn CKRecordKeyValueSetting {}
);

extern_methods!(
    /// CKRecordKeyValueSettingConformance
    unsafe impl CKRecord {
        #[method_id(@__retain_semantics Other encryptedValues)]
        pub unsafe fn encryptedValues(
            &self,
        ) -> Retained<ProtocolObject<dyn CKRecordKeyValueSetting>>;
    }
);

unsafe impl CKRecordKeyValueSetting for CKRecord {}
