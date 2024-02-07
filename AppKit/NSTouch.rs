//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTouchPhase {
        NSTouchPhaseBegan = 1 << 0,
        NSTouchPhaseMoved = 1 << 1,
        NSTouchPhaseStationary = 1 << 2,
        NSTouchPhaseEnded = 1 << 3,
        NSTouchPhaseCancelled = 1 << 4,
        NSTouchPhaseTouching = NSTouchPhase::NSTouchPhaseBegan.0
            | NSTouchPhase::NSTouchPhaseMoved.0
            | NSTouchPhase::NSTouchPhaseStationary.0,
        NSTouchPhaseAny = NSUIntegerMax as _,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTouchType {
        NSTouchTypeDirect = 0,
        NSTouchTypeIndirect = 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTouchTypeMask {
        NSTouchTypeMaskDirect = 1 << NSTouchType::NSTouchTypeDirect.0,
        NSTouchTypeMaskIndirect = 1 << NSTouchType::NSTouchTypeIndirect.0,
    }
);

inline_fn!(
    pub unsafe fn NSTouchTypeMaskFromType(r#type: NSTouchType) -> NSTouchTypeMask {
        todo!()
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTouch")]
    pub struct NSTouch;

    #[cfg(feature = "AppKit_NSTouch")]
    unsafe impl ClassType for NSTouch {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

#[cfg(feature = "AppKit_NSTouch")]
unsafe impl Send for NSTouch {}

#[cfg(feature = "AppKit_NSTouch")]
unsafe impl Sync for NSTouch {}

#[cfg(feature = "AppKit_NSTouch")]
unsafe impl NSCopying for NSTouch {}

#[cfg(feature = "AppKit_NSTouch")]
unsafe impl NSObjectProtocol for NSTouch {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTouch")]
    unsafe impl NSTouch {
        #[method_id(@__retain_semantics Other identity)]
        pub unsafe fn identity(&self) -> Id<TodoProtocols>;

        #[method(phase)]
        pub unsafe fn phase(&self) -> NSTouchPhase;

        #[method(normalizedPosition)]
        pub unsafe fn normalizedPosition(&self) -> NSPoint;

        #[method(isResting)]
        pub unsafe fn isResting(&self) -> bool;

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<AnyObject>>;

        #[method(deviceSize)]
        pub unsafe fn deviceSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTouch")]
    unsafe impl NSTouch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSTouchBar
    #[cfg(feature = "AppKit_NSTouch")]
    unsafe impl NSTouch {
        #[method(type)]
        pub unsafe fn r#type(&self) -> NSTouchType;

        #[cfg(feature = "AppKit_NSView")]
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;

        #[cfg(feature = "AppKit_NSView")]
        #[method(previousLocationInView:)]
        pub unsafe fn previousLocationInView(&self, view: Option<&NSView>) -> NSPoint;
    }
);
