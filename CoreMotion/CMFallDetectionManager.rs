//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMFallDetectionManager;

    unsafe impl ClassType for CMFallDetectionManager {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for CMFallDetectionManager {}

extern_methods!(
    unsafe impl CMFallDetectionManager {
        #[method(isAvailable)]
        pub unsafe fn isAvailable() -> bool;

        #[cfg(feature = "CMAuthorization")]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus(&self) -> CMAuthorizationStatus;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn CMFallDetectionDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CMFallDetectionDelegate>>,
        );

        #[cfg(all(feature = "CMAuthorization", feature = "block2"))]
        #[method(requestAuthorizationWithHandler:)]
        pub unsafe fn requestAuthorizationWithHandler(
            &self,
            handler: &block2::Block<dyn Fn(CMAuthorizationStatus)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMFallDetectionManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait CMFallDetectionDelegate: NSObjectProtocol {
        #[cfg(all(feature = "CMFallDetectionEvent", feature = "block2"))]
        #[optional]
        #[method(fallDetectionManager:didDetectEvent:completionHandler:)]
        unsafe fn fallDetectionManager_didDetectEvent_completionHandler(
            &self,
            fall_detection_manager: &CMFallDetectionManager,
            event: &CMFallDetectionEvent,
            handler: &block2::Block<dyn Fn()>,
        );

        #[optional]
        #[method(fallDetectionManagerDidChangeAuthorization:)]
        unsafe fn fallDetectionManagerDidChangeAuthorization(
            &self,
            fall_detection_manager: &CMFallDetectionManager,
        );
    }

    unsafe impl ProtocolType for dyn CMFallDetectionDelegate {}
);
