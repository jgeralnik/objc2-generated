//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NEProvider")]
    pub struct NEAppPushProvider;

    #[cfg(feature = "NEProvider")]
    unsafe impl ClassType for NEAppPushProvider {
        #[inherits(NSObject)]
        type Super = NEProvider;
    }
);

#[cfg(feature = "NEProvider")]
unsafe impl NSObjectProtocol for NEAppPushProvider {}

extern_methods!(
    #[cfg(feature = "NEProvider")]
    unsafe impl NEAppPushProvider {
        #[method_id(@__retain_semantics Other providerConfiguration)]
        pub unsafe fn providerConfiguration(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(startWithCompletionHandler:)]
        pub unsafe fn startWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method(start)]
        pub unsafe fn start(&self);

        #[cfg(feature = "block2")]
        #[method(stopWithReason:completionHandler:)]
        pub unsafe fn stopWithReason_completionHandler(
            &self,
            reason: NEProviderStopReason,
            completion_handler: &block2::Block<dyn Fn()>,
        );

        #[method(reportIncomingCallWithUserInfo:)]
        pub unsafe fn reportIncomingCallWithUserInfo(&self, user_info: &NSDictionary);

        #[method(reportPushToTalkMessageWithUserInfo:)]
        pub unsafe fn reportPushToTalkMessageWithUserInfo(&self, user_info: &NSDictionary);

        #[method(handleTimerEvent)]
        pub unsafe fn handleTimerEvent(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NEProvider")]
    unsafe impl NEAppPushProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
