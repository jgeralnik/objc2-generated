//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[deprecated]
pub const DOM_NONE: c_uint = 0;
#[deprecated]
pub const DOM_CAPTURING_PHASE: c_uint = 1;
#[deprecated]
pub const DOM_AT_TARGET: c_uint = 2;
#[deprecated]
pub const DOM_BUBBLING_PHASE: c_uint = 3;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMEvent;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMEvent {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
    }
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMEvent {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMEvent {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMEvent {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMEvent {
        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[cfg(feature = "DOMEventTarget")]
        #[deprecated]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Retained<ProtocolObject<dyn DOMEventTarget>>>;

        #[cfg(feature = "DOMEventTarget")]
        #[deprecated]
        #[method_id(@__retain_semantics Other currentTarget)]
        pub unsafe fn currentTarget(&self) -> Option<Retained<ProtocolObject<dyn DOMEventTarget>>>;

        #[deprecated]
        #[method(eventPhase)]
        pub unsafe fn eventPhase(&self) -> c_ushort;

        #[deprecated]
        #[method(bubbles)]
        pub unsafe fn bubbles(&self) -> bool;

        #[deprecated]
        #[method(cancelable)]
        pub unsafe fn cancelable(&self) -> bool;

        #[deprecated]
        #[method(timeStamp)]
        pub unsafe fn timeStamp(&self) -> DOMTimeStamp;

        #[cfg(feature = "DOMEventTarget")]
        #[method_id(@__retain_semantics Other srcElement)]
        pub unsafe fn srcElement(&self) -> Option<Retained<ProtocolObject<dyn DOMEventTarget>>>;

        #[method(returnValue)]
        pub unsafe fn returnValue(&self) -> bool;

        #[method(setReturnValue:)]
        pub unsafe fn setReturnValue(&self, return_value: bool);

        #[method(cancelBubble)]
        pub unsafe fn cancelBubble(&self) -> bool;

        #[method(setCancelBubble:)]
        pub unsafe fn setCancelBubble(&self, cancel_bubble: bool);

        #[deprecated]
        #[method(stopPropagation)]
        pub unsafe fn stopPropagation(&self);

        #[deprecated]
        #[method(preventDefault)]
        pub unsafe fn preventDefault(&self);

        #[method(initEvent:canBubbleArg:cancelableArg:)]
        pub unsafe fn initEvent_canBubbleArg_cancelableArg(
            &self,
            event_type_arg: Option<&NSString>,
            can_bubble_arg: bool,
            cancelable_arg: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMEvent {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMEvent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMEventDeprecated
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMEvent {
        #[deprecated]
        #[method(initEvent:::)]
        pub unsafe fn initEvent(
            &self,
            event_type_arg: Option<&NSString>,
            can_bubble_arg: bool,
            cancelable_arg: bool,
        );
    }
);
