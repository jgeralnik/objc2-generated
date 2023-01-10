//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSDistributedNotificationCenterType = NSString;
);

extern_static!(NSLocalNotificationCenterType: &'static NSDistributedNotificationCenterType);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSNotificationSuspensionBehavior {
        NSNotificationSuspensionBehaviorDrop = 1,
        NSNotificationSuspensionBehaviorCoalesce = 2,
        NSNotificationSuspensionBehaviorHold = 3,
        NSNotificationSuspensionBehaviorDeliverImmediately = 4,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDistributedNotificationOptions {
        NSDistributedNotificationDeliverImmediately = 1 << 0,
        NSDistributedNotificationPostToAllSessions = 1 << 1,
    }
);

extern_static!(
    NSNotificationDeliverImmediately: NSDistributedNotificationOptions =
        NSDistributedNotificationDeliverImmediately
);

extern_static!(
    NSNotificationPostToAllSessions: NSDistributedNotificationOptions =
        NSDistributedNotificationPostToAllSessions
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDistributedNotificationCenter;

    unsafe impl ClassType for NSDistributedNotificationCenter {
        #[inherits(NSObject)]
        type Super = NSNotificationCenter;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSDistributedNotificationCenter")]
    unsafe impl NSDistributedNotificationCenter {
        #[method_id(@__retain_semantics Other notificationCenterForType:)]
        pub unsafe fn notificationCenterForType(
            notificationCenterType: &NSDistributedNotificationCenterType,
        ) -> Id<NSDistributedNotificationCenter, Shared>;

        #[method_id(@__retain_semantics Other defaultCenter)]
        pub unsafe fn defaultCenter() -> Id<NSDistributedNotificationCenter, Shared>;

        #[method(addObserver:selector:name:object:suspensionBehavior:)]
        pub unsafe fn addObserver_selector_name_object_suspensionBehavior(
            &self,
            observer: &Object,
            selector: Sel,
            name: Option<&NSNotificationName>,
            object: Option<&NSString>,
            suspensionBehavior: NSNotificationSuspensionBehavior,
        );

        #[method(postNotificationName:object:userInfo:deliverImmediately:)]
        pub unsafe fn postNotificationName_object_userInfo_deliverImmediately(
            &self,
            name: &NSNotificationName,
            object: Option<&NSString>,
            userInfo: Option<&NSDictionary>,
            deliverImmediately: bool,
        );

        #[method(postNotificationName:object:userInfo:options:)]
        pub unsafe fn postNotificationName_object_userInfo_options(
            &self,
            name: &NSNotificationName,
            object: Option<&NSString>,
            userInfo: Option<&NSDictionary>,
            options: NSDistributedNotificationOptions,
        );

        #[method(suspended)]
        pub unsafe fn suspended(&self) -> bool;

        #[method(setSuspended:)]
        pub unsafe fn setSuspended(&self, suspended: bool);

        #[method(addObserver:selector:name:object:)]
        pub unsafe fn addObserver_selector_name_object(
            &self,
            observer: &Object,
            aSelector: Sel,
            aName: Option<&NSNotificationName>,
            anObject: Option<&NSString>,
        );

        #[method(postNotificationName:object:)]
        pub unsafe fn postNotificationName_object(
            &self,
            aName: &NSNotificationName,
            anObject: Option<&NSString>,
        );

        #[method(postNotificationName:object:userInfo:)]
        pub unsafe fn postNotificationName_object_userInfo(
            &self,
            aName: &NSNotificationName,
            anObject: Option<&NSString>,
            aUserInfo: Option<&NSDictionary>,
        );

        #[method(removeObserver:name:object:)]
        pub unsafe fn removeObserver_name_object(
            &self,
            observer: &Object,
            aName: Option<&NSNotificationName>,
            anObject: Option<&NSString>,
        );
    }
);
