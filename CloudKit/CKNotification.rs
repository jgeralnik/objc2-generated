//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cknotificationid?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKNotificationID;
);

unsafe impl NSCoding for CKNotificationID {}

unsafe impl NSCopying for CKNotificationID {}

unsafe impl CopyingHelper for CKNotificationID {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKNotificationID {}

unsafe impl NSSecureCoding for CKNotificationID {}

extern_methods!(
    unsafe impl CKNotificationID {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKNotificationID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cknotificationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKNotificationType(pub NSInteger);
impl CKNotificationType {
    #[doc(alias = "CKNotificationTypeQuery")]
    pub const Query: Self = Self(1);
    #[doc(alias = "CKNotificationTypeRecordZone")]
    pub const RecordZone: Self = Self(2);
    #[doc(alias = "CKNotificationTypeReadNotification")]
    pub const ReadNotification: Self = Self(3);
    #[doc(alias = "CKNotificationTypeDatabase")]
    pub const Database: Self = Self(4);
}

unsafe impl Encode for CKNotificationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKNotificationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cknotification?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKNotification;
);

unsafe impl Send for CKNotification {}

unsafe impl Sync for CKNotification {}

unsafe impl NSObjectProtocol for CKNotification {}

extern_methods!(
    unsafe impl CKNotification {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other notificationFromRemoteNotificationDictionary:)]
        pub unsafe fn notificationFromRemoteNotificationDictionary(
            notification_dictionary: &NSDictionary,
        ) -> Option<Retained<Self>>;

        #[method(notificationType)]
        pub unsafe fn notificationType(&self) -> CKNotificationType;

        #[method_id(@__retain_semantics Other notificationID)]
        pub unsafe fn notificationID(&self) -> Option<Retained<CKNotificationID>>;

        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other subscriptionOwnerUserRecordID)]
        pub unsafe fn subscriptionOwnerUserRecordID(&self) -> Option<Retained<CKRecordID>>;

        #[method(isPruned)]
        pub unsafe fn isPruned(&self) -> bool;

        #[cfg(feature = "CKSubscription")]
        #[method_id(@__retain_semantics Other subscriptionID)]
        pub unsafe fn subscriptionID(&self) -> Option<Retained<CKSubscriptionID>>;
    }
);

extern_methods!(
    /// DeprecatedAPSProperties
    unsafe impl CKNotification {
        #[deprecated = "Interact with UI elements of a CloudKit-server-generated push message via UserNotifications.framework"]
        #[method_id(@__retain_semantics Other alertBody)]
        pub unsafe fn alertBody(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Interact with UI elements of a CloudKit-server-generated push message via UserNotifications.framework"]
        #[method_id(@__retain_semantics Other alertLocalizationKey)]
        pub unsafe fn alertLocalizationKey(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Interact with UI elements of a CloudKit-server-generated push message via UserNotifications.framework"]
        #[method_id(@__retain_semantics Other alertLocalizationArgs)]
        pub unsafe fn alertLocalizationArgs(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other titleLocalizationKey)]
        pub unsafe fn titleLocalizationKey(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other titleLocalizationArgs)]
        pub unsafe fn titleLocalizationArgs(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other subtitleLocalizationKey)]
        pub unsafe fn subtitleLocalizationKey(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other subtitleLocalizationArgs)]
        pub unsafe fn subtitleLocalizationArgs(&self) -> Option<Retained<NSArray<NSString>>>;

        #[deprecated = "Interact with UI elements of a CloudKit-server-generated push message via UserNotifications.framework"]
        #[method_id(@__retain_semantics Other alertActionLocalizationKey)]
        pub unsafe fn alertActionLocalizationKey(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Interact with UI elements of a CloudKit-server-generated push message via UserNotifications.framework"]
        #[method_id(@__retain_semantics Other alertLaunchImage)]
        pub unsafe fn alertLaunchImage(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Interact with UI elements of a CloudKit-server-generated push message via UserNotifications.framework"]
        #[method_id(@__retain_semantics Other badge)]
        pub unsafe fn badge(&self) -> Option<Retained<NSNumber>>;

        #[deprecated = "Interact with UI elements of a CloudKit-server-generated push message via UserNotifications.framework"]
        #[method_id(@__retain_semantics Other soundName)]
        pub unsafe fn soundName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Option<Retained<NSString>>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckquerynotificationreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKQueryNotificationReason(pub NSInteger);
impl CKQueryNotificationReason {
    #[doc(alias = "CKQueryNotificationReasonRecordCreated")]
    pub const RecordCreated: Self = Self(1);
    #[doc(alias = "CKQueryNotificationReasonRecordUpdated")]
    pub const RecordUpdated: Self = Self(2);
    #[doc(alias = "CKQueryNotificationReasonRecordDeleted")]
    pub const RecordDeleted: Self = Self(3);
}

unsafe impl Encode for CKQueryNotificationReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKQueryNotificationReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckquerynotification?language=objc)
    #[unsafe(super(CKNotification, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKQueryNotification;
);

unsafe impl Send for CKQueryNotification {}

unsafe impl Sync for CKQueryNotification {}

unsafe impl NSObjectProtocol for CKQueryNotification {}

extern_methods!(
    unsafe impl CKQueryNotification {
        #[method(queryNotificationReason)]
        pub unsafe fn queryNotificationReason(&self) -> CKQueryNotificationReason;

        #[method_id(@__retain_semantics Other recordFields)]
        pub unsafe fn recordFields(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other recordID)]
        pub unsafe fn recordID(&self) -> Option<Retained<CKRecordID>>;

        #[cfg(feature = "CKDatabase")]
        #[method(databaseScope)]
        pub unsafe fn databaseScope(&self) -> CKDatabaseScope;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKNotification`
    unsafe impl CKQueryNotification {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other notificationFromRemoteNotificationDictionary:)]
        pub unsafe fn notificationFromRemoteNotificationDictionary(
            notification_dictionary: &NSDictionary,
        ) -> Option<Retained<Self>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckrecordzonenotification?language=objc)
    #[unsafe(super(CKNotification, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKRecordZoneNotification;
);

unsafe impl Send for CKRecordZoneNotification {}

unsafe impl Sync for CKRecordZoneNotification {}

unsafe impl NSObjectProtocol for CKRecordZoneNotification {}

extern_methods!(
    unsafe impl CKRecordZoneNotification {
        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other recordZoneID)]
        pub unsafe fn recordZoneID(&self) -> Option<Retained<CKRecordZoneID>>;

        #[cfg(feature = "CKDatabase")]
        #[method(databaseScope)]
        pub unsafe fn databaseScope(&self) -> CKDatabaseScope;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKNotification`
    unsafe impl CKRecordZoneNotification {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other notificationFromRemoteNotificationDictionary:)]
        pub unsafe fn notificationFromRemoteNotificationDictionary(
            notification_dictionary: &NSDictionary,
        ) -> Option<Retained<Self>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckdatabasenotification?language=objc)
    #[unsafe(super(CKNotification, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKDatabaseNotification;
);

unsafe impl Send for CKDatabaseNotification {}

unsafe impl Sync for CKDatabaseNotification {}

unsafe impl NSObjectProtocol for CKDatabaseNotification {}

extern_methods!(
    unsafe impl CKDatabaseNotification {
        #[cfg(feature = "CKDatabase")]
        #[method(databaseScope)]
        pub unsafe fn databaseScope(&self) -> CKDatabaseScope;
    }
);

extern_methods!(
    /// Methods declared on superclass `CKNotification`
    unsafe impl CKDatabaseNotification {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other notificationFromRemoteNotificationDictionary:)]
        pub unsafe fn notificationFromRemoteNotificationDictionary(
            notification_dictionary: &NSDictionary,
        ) -> Option<Retained<Self>>;
    }
);
