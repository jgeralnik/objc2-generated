//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFAcousticFeature;
);

unsafe impl NSCoding for SFAcousticFeature {}

unsafe impl NSCopying for SFAcousticFeature {}

unsafe impl CopyingHelper for SFAcousticFeature {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SFAcousticFeature {}

unsafe impl NSSecureCoding for SFAcousticFeature {}

extern_methods!(
    unsafe impl SFAcousticFeature {
        #[method_id(@__retain_semantics Other acousticFeatureValuePerFrame)]
        pub unsafe fn acousticFeatureValuePerFrame(&self) -> Retained<NSArray<NSNumber>>;

        #[method(frameDuration)]
        pub unsafe fn frameDuration(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFAcousticFeature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFVoiceAnalytics;
);

unsafe impl NSCoding for SFVoiceAnalytics {}

unsafe impl NSCopying for SFVoiceAnalytics {}

unsafe impl CopyingHelper for SFVoiceAnalytics {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SFVoiceAnalytics {}

unsafe impl NSSecureCoding for SFVoiceAnalytics {}

extern_methods!(
    unsafe impl SFVoiceAnalytics {
        #[method_id(@__retain_semantics Other jitter)]
        pub unsafe fn jitter(&self) -> Retained<SFAcousticFeature>;

        #[method_id(@__retain_semantics Other shimmer)]
        pub unsafe fn shimmer(&self) -> Retained<SFAcousticFeature>;

        #[method_id(@__retain_semantics Other pitch)]
        pub unsafe fn pitch(&self) -> Retained<SFAcousticFeature>;

        #[method_id(@__retain_semantics Other voicing)]
        pub unsafe fn voicing(&self) -> Retained<SFAcousticFeature>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFVoiceAnalytics {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
