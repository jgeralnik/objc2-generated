//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSBackgroundActivityResult {
        #[doc(alias = "NSBackgroundActivityResultFinished")]
        Finished = 1,
        #[doc(alias = "NSBackgroundActivityResultDeferred")]
        Deferred = 2,
    }
);

pub type NSBackgroundActivityCompletionHandler = *mut Block<dyn Fn(NSBackgroundActivityResult)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSBackgroundActivityScheduler")]
    pub struct NSBackgroundActivityScheduler;

    #[cfg(feature = "Foundation_NSBackgroundActivityScheduler")]
    unsafe impl ClassType for NSBackgroundActivityScheduler {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSBackgroundActivityScheduler")]
unsafe impl NSObjectProtocol for NSBackgroundActivityScheduler {}

extern_methods!(
    #[cfg(feature = "Foundation_NSBackgroundActivityScheduler")]
    unsafe impl NSBackgroundActivityScheduler {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(this: Allocated<Self>, identifier: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, quality_of_service: NSQualityOfService);

        #[method(repeats)]
        pub unsafe fn repeats(&self) -> bool;

        #[method(setRepeats:)]
        pub unsafe fn setRepeats(&self, repeats: bool);

        #[method(interval)]
        pub unsafe fn interval(&self) -> NSTimeInterval;

        #[method(setInterval:)]
        pub unsafe fn setInterval(&self, interval: NSTimeInterval);

        #[method(tolerance)]
        pub unsafe fn tolerance(&self) -> NSTimeInterval;

        #[method(setTolerance:)]
        pub unsafe fn setTolerance(&self, tolerance: NSTimeInterval);

        #[method(scheduleWithBlock:)]
        pub unsafe fn scheduleWithBlock(
            &self,
            block: &Block<dyn Fn(NSBackgroundActivityCompletionHandler)>,
        );

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(shouldDefer)]
        pub unsafe fn shouldDefer(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSBackgroundActivityScheduler")]
    unsafe impl NSBackgroundActivityScheduler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
