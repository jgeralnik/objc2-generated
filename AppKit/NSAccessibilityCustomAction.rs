//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAccessibilityCustomAction;
);

unsafe impl NSObjectProtocol for NSAccessibilityCustomAction {}

extern_methods!(
    unsafe impl NSAccessibilityCustomAction {
        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithName:handler:)]
        pub unsafe fn initWithName_handler(
            this: Allocated<Self>,
            name: &NSString,
            handler: Option<&block2::Block<dyn Fn() -> Bool>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithName:target:selector:)]
        pub unsafe fn initWithName_target_selector(
            this: Allocated<Self>,
            name: &NSString,
            target: &NSObject,
            selector: Sel,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[cfg(feature = "block2")]
        #[method(handler)]
        pub unsafe fn handler(&self) -> *mut block2::Block<dyn Fn() -> Bool>;

        #[cfg(feature = "block2")]
        #[method(setHandler:)]
        pub unsafe fn setHandler(&self, handler: Option<&block2::Block<dyn Fn() -> Bool>>);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Retained<NSObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSObject>);

        #[method(selector)]
        pub unsafe fn selector(&self) -> Option<Sel>;

        #[method(setSelector:)]
        pub unsafe fn setSelector(&self, selector: Option<Sel>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAccessibilityCustomAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
