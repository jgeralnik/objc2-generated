//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSNotificationName = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNotification;

    unsafe impl ClassType for NSNotification {
        type Super = NSObject;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSNotification {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSNotification {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSNotification {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSNotification {}

extern_methods!(
    unsafe impl NSNotification {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSNotificationName>;

        #[method_id(@__retain_semantics Other object)]
        pub unsafe fn object(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithName:object:userInfo:)]
        pub unsafe fn initWithName_object_userInfo(
            this: Allocated<Self>,
            name: &NSNotificationName,
            object: Option<&AnyObject>,
            user_info: Option<&NSDictionary>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// NSNotificationCreation
    unsafe impl NSNotification {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other notificationWithName:object:)]
        pub unsafe fn notificationWithName_object(
            a_name: &NSNotificationName,
            an_object: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other notificationWithName:object:userInfo:)]
        pub unsafe fn notificationWithName_object_userInfo(
            a_name: &NSNotificationName,
            an_object: Option<&AnyObject>,
            a_user_info: Option<&NSDictionary>,
        ) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNotificationCenter;

    unsafe impl ClassType for NSNotificationCenter {
        type Super = NSObject;
    }
);

unsafe impl Send for NSNotificationCenter {}

unsafe impl Sync for NSNotificationCenter {}

unsafe impl NSObjectProtocol for NSNotificationCenter {}

extern_methods!(
    unsafe impl NSNotificationCenter {
        #[method_id(@__retain_semantics Other defaultCenter)]
        pub unsafe fn defaultCenter() -> Retained<NSNotificationCenter>;

        #[cfg(feature = "NSString")]
        #[method(addObserver:selector:name:object:)]
        pub unsafe fn addObserver_selector_name_object(
            &self,
            observer: &AnyObject,
            a_selector: Sel,
            a_name: Option<&NSNotificationName>,
            an_object: Option<&AnyObject>,
        );

        #[method(postNotification:)]
        pub unsafe fn postNotification(&self, notification: &NSNotification);

        #[cfg(feature = "NSString")]
        #[method(postNotificationName:object:)]
        pub unsafe fn postNotificationName_object(
            &self,
            a_name: &NSNotificationName,
            an_object: Option<&AnyObject>,
        );

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(postNotificationName:object:userInfo:)]
        pub unsafe fn postNotificationName_object_userInfo(
            &self,
            a_name: &NSNotificationName,
            an_object: Option<&AnyObject>,
            a_user_info: Option<&NSDictionary>,
        );

        #[method(removeObserver:)]
        pub unsafe fn removeObserver(&self, observer: &AnyObject);

        #[cfg(feature = "NSString")]
        #[method(removeObserver:name:object:)]
        pub unsafe fn removeObserver_name_object(
            &self,
            observer: &AnyObject,
            a_name: Option<&NSNotificationName>,
            an_object: Option<&AnyObject>,
        );

        #[cfg(all(feature = "NSOperation", feature = "NSString", feature = "block2"))]
        #[method_id(@__retain_semantics Other addObserverForName:object:queue:usingBlock:)]
        pub unsafe fn addObserverForName_object_queue_usingBlock(
            &self,
            name: Option<&NSNotificationName>,
            obj: Option<&AnyObject>,
            queue: Option<&NSOperationQueue>,
            block: &block2::Block<dyn Fn(NonNull<NSNotification>)>,
        ) -> Retained<NSObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNotificationCenter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
