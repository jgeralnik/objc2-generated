//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static BGTaskSchedulerErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BGTaskSchedulerErrorCode(pub NSInteger);
impl BGTaskSchedulerErrorCode {
    #[doc(alias = "BGTaskSchedulerErrorCodeUnavailable")]
    pub const Unavailable: Self = Self(1);
    #[doc(alias = "BGTaskSchedulerErrorCodeTooManyPendingTaskRequests")]
    pub const TooManyPendingTaskRequests: Self = Self(2);
    #[doc(alias = "BGTaskSchedulerErrorCodeNotPermitted")]
    pub const NotPermitted: Self = Self(3);
}

unsafe impl Encode for BGTaskSchedulerErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for BGTaskSchedulerErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGTaskScheduler;
);

unsafe impl NSObjectProtocol for BGTaskScheduler {}

extern_methods!(
    unsafe impl BGTaskScheduler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other sharedScheduler)]
        pub unsafe fn sharedScheduler() -> Retained<BGTaskScheduler>;

        #[cfg(feature = "BGTaskRequest")]
        #[method(submitTaskRequest:error:_)]
        pub unsafe fn submitTaskRequest_error(
            &self,
            task_request: &BGTaskRequest,
        ) -> Result<(), Retained<NSError>>;

        #[method(cancelTaskRequestWithIdentifier:)]
        pub unsafe fn cancelTaskRequestWithIdentifier(&self, identifier: &NSString);

        #[method(cancelAllTaskRequests)]
        pub unsafe fn cancelAllTaskRequests(&self);

        #[cfg(all(feature = "BGTaskRequest", feature = "block2"))]
        #[method(getPendingTaskRequestsWithCompletionHandler:)]
        pub unsafe fn getPendingTaskRequestsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(NonNull<NSArray<BGTaskRequest>>)>,
        );
    }
);
