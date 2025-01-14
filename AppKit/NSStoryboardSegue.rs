//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstoryboardsegueidentifier?language=objc)
pub type NSStoryboardSegueIdentifier = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstoryboardsegue?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStoryboardSegue;
);

unsafe impl NSObjectProtocol for NSStoryboardSegue {}

extern_methods!(
    unsafe impl NSStoryboardSegue {
        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other segueWithIdentifier:source:destination:performHandler:)]
        pub unsafe fn segueWithIdentifier_source_destination_performHandler(
            identifier: &NSStoryboardSegueIdentifier,
            source_controller: &AnyObject,
            destination_controller: &AnyObject,
            perform_handler: &block2::Block<dyn Fn()>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIdentifier:source:destination:)]
        pub unsafe fn initWithIdentifier_source_destination(
            this: Allocated<Self>,
            identifier: &NSStoryboardSegueIdentifier,
            source_controller: &AnyObject,
            destination_controller: &AnyObject,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSStoryboardSegueIdentifier>>;

        #[method_id(@__retain_semantics Other sourceController)]
        pub unsafe fn sourceController(&self) -> Retained<AnyObject>;

        #[method_id(@__retain_semantics Other destinationController)]
        pub unsafe fn destinationController(&self) -> Retained<AnyObject>;

        #[method(perform)]
        pub unsafe fn perform(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStoryboardSegue {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssegueperforming?language=objc)
    pub unsafe trait NSSeguePerforming: NSObjectProtocol + MainThreadOnly {
        #[optional]
        #[method(prepareForSegue:sender:)]
        unsafe fn prepareForSegue_sender(
            &self,
            segue: &NSStoryboardSegue,
            sender: Option<&AnyObject>,
        );

        #[optional]
        #[method(performSegueWithIdentifier:sender:)]
        unsafe fn performSegueWithIdentifier_sender(
            &self,
            identifier: &NSStoryboardSegueIdentifier,
            sender: Option<&AnyObject>,
        );

        #[optional]
        #[method(shouldPerformSegueWithIdentifier:sender:)]
        unsafe fn shouldPerformSegueWithIdentifier_sender(
            &self,
            identifier: &NSStoryboardSegueIdentifier,
            sender: Option<&AnyObject>,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSSeguePerforming {}
);
