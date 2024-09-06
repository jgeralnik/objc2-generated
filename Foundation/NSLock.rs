//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    pub unsafe trait NSLocking {
        #[method(lock)]
        unsafe fn lock(&self);

        #[method(unlock)]
        unsafe fn unlock(&self);
    }

    unsafe impl ProtocolType for dyn NSLocking {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLock;

    unsafe impl ClassType for NSLock {
        type Super = NSObject;
    }
);

unsafe impl Send for NSLock {}

unsafe impl Sync for NSLock {}

unsafe impl NSLocking for NSLock {}

unsafe impl NSObjectProtocol for NSLock {}

extern_methods!(
    unsafe impl NSLock {
        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(lockBeforeDate:)]
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setName:)]
        pub fn setName(&self, name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLock {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for NSLock {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSConditionLock;

    unsafe impl ClassType for NSConditionLock {
        type Super = NSObject;
    }
);

unsafe impl Send for NSConditionLock {}

unsafe impl Sync for NSConditionLock {}

unsafe impl NSLocking for NSConditionLock {}

unsafe impl NSObjectProtocol for NSConditionLock {}

extern_methods!(
    unsafe impl NSConditionLock {
        #[method_id(@__retain_semantics Init initWithCondition:)]
        pub unsafe fn initWithCondition(
            this: Allocated<Self>,
            condition: NSInteger,
        ) -> Retained<Self>;

        #[method(condition)]
        pub unsafe fn condition(&self) -> NSInteger;

        #[method(lockWhenCondition:)]
        pub unsafe fn lockWhenCondition(&self, condition: NSInteger);

        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[method(tryLockWhenCondition:)]
        pub unsafe fn tryLockWhenCondition(&self, condition: NSInteger) -> bool;

        #[method(unlockWithCondition:)]
        pub unsafe fn unlockWithCondition(&self, condition: NSInteger);

        #[cfg(feature = "NSDate")]
        #[method(lockBeforeDate:)]
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(lockWhenCondition:beforeDate:)]
        pub unsafe fn lockWhenCondition_beforeDate(
            &self,
            condition: NSInteger,
            limit: &NSDate,
        ) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSConditionLock {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRecursiveLock;

    unsafe impl ClassType for NSRecursiveLock {
        type Super = NSObject;
    }
);

unsafe impl Send for NSRecursiveLock {}

unsafe impl Sync for NSRecursiveLock {}

unsafe impl NSLocking for NSRecursiveLock {}

unsafe impl NSObjectProtocol for NSRecursiveLock {}

extern_methods!(
    unsafe impl NSRecursiveLock {
        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[cfg(feature = "NSDate")]
        #[method(lockBeforeDate:)]
        pub unsafe fn lockBeforeDate(&self, limit: &NSDate) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSRecursiveLock {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCondition;

    unsafe impl ClassType for NSCondition {
        type Super = NSObject;
    }
);

unsafe impl Send for NSCondition {}

unsafe impl Sync for NSCondition {}

unsafe impl NSLocking for NSCondition {}

unsafe impl NSObjectProtocol for NSCondition {}

extern_methods!(
    unsafe impl NSCondition {
        #[method(wait)]
        pub unsafe fn wait(&self);

        #[cfg(feature = "NSDate")]
        #[method(waitUntilDate:)]
        pub unsafe fn waitUntilDate(&self, limit: &NSDate) -> bool;

        #[method(signal)]
        pub unsafe fn signal(&self);

        #[method(broadcast)]
        pub unsafe fn broadcast(&self);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCondition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
