//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
    pub struct UIStoryboardSegue;

    unsafe impl ClassType for UIStoryboardSegue {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIStoryboardSegue {}

extern_methods!(
    unsafe impl UIStoryboardSegue {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other segueWithIdentifier:source:destination:performHandler:)]
        pub unsafe fn segueWithIdentifier_source_destination_performHandler(
            identifier: Option<&NSString>,
            source: &UIViewController,
            destination: &UIViewController,
            perform_handler: &block2::Block<dyn Fn()>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Init initWithIdentifier:source:destination:)]
        pub unsafe fn initWithIdentifier_source_destination(
            this: Allocated<Self>,
            identifier: Option<&NSString>,
            source: &UIViewController,
            destination: &UIViewController,
        ) -> Retained<Self>;

        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Other sourceViewController)]
        pub unsafe fn sourceViewController(&self) -> Retained<UIViewController>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Other destinationViewController)]
        pub unsafe fn destinationViewController(&self) -> Retained<UIViewController>;

        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method(perform)]
        pub unsafe fn perform(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIStoryboardSegue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIStoryboardUnwindSegueSource;

    unsafe impl ClassType for UIStoryboardUnwindSegueSource {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIStoryboardUnwindSegueSource {}

extern_methods!(
    unsafe impl UIStoryboardUnwindSegueSource {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other sourceViewController)]
        pub unsafe fn sourceViewController(&self) -> Retained<UIViewController>;

        #[method(unwindAction)]
        pub unsafe fn unwindAction(&self) -> Sel;

        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(&self) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIStoryboardUnwindSegueSource {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
