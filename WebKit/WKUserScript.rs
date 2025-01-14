//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkuserscriptinjectiontime?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKUserScriptInjectionTime(pub NSInteger);
impl WKUserScriptInjectionTime {
    #[doc(alias = "WKUserScriptInjectionTimeAtDocumentStart")]
    pub const AtDocumentStart: Self = Self(0);
    #[doc(alias = "WKUserScriptInjectionTimeAtDocumentEnd")]
    pub const AtDocumentEnd: Self = Self(1);
}

unsafe impl Encode for WKUserScriptInjectionTime {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKUserScriptInjectionTime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkuserscript?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKUserScript;
);

unsafe impl NSCopying for WKUserScript {}

unsafe impl CopyingHelper for WKUserScript {
    type Result = Self;
}

unsafe impl NSObjectProtocol for WKUserScript {}

extern_methods!(
    unsafe impl WKUserScript {
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Retained<NSString>;

        #[method(injectionTime)]
        pub unsafe fn injectionTime(&self) -> WKUserScriptInjectionTime;

        #[method(isForMainFrameOnly)]
        pub unsafe fn isForMainFrameOnly(&self) -> bool;

        #[method_id(@__retain_semantics Init initWithSource:injectionTime:forMainFrameOnly:)]
        pub unsafe fn initWithSource_injectionTime_forMainFrameOnly(
            this: Allocated<Self>,
            source: &NSString,
            injection_time: WKUserScriptInjectionTime,
            for_main_frame_only: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "WKContentWorld")]
        #[method_id(@__retain_semantics Init initWithSource:injectionTime:forMainFrameOnly:inContentWorld:)]
        pub unsafe fn initWithSource_injectionTime_forMainFrameOnly_inContentWorld(
            this: Allocated<Self>,
            source: &NSString,
            injection_time: WKUserScriptInjectionTime,
            for_main_frame_only: bool,
            content_world: &WKContentWorld,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKUserScript {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
