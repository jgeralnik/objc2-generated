//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mailkit/meextensionmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEExtensionManager;
);

unsafe impl NSObjectProtocol for MEExtensionManager {}

extern_methods!(
    unsafe impl MEExtensionManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(reloadContentBlockerWithIdentifier:completionHandler:)]
        pub unsafe fn reloadContentBlockerWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(reloadVisibleMessagesWithCompletionHandler:)]
        pub unsafe fn reloadVisibleMessagesWithCompletionHandler(
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);
