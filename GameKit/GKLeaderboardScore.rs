//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKLeaderboardScore;
);

unsafe impl NSObjectProtocol for GKLeaderboardScore {}

extern_methods!(
    unsafe impl GKLeaderboardScore {
        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self) -> Retained<GKPlayer>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method(setPlayer:)]
        pub unsafe fn setPlayer(&self, player: &GKPlayer);

        #[method(value)]
        pub unsafe fn value(&self) -> NSInteger;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: NSInteger);

        #[method(context)]
        pub unsafe fn context(&self) -> NSUInteger;

        #[method(setContext:)]
        pub unsafe fn setContext(&self, context: NSUInteger);

        #[method_id(@__retain_semantics Other leaderboardID)]
        pub unsafe fn leaderboardID(&self) -> Retained<NSString>;

        #[method(setLeaderboardID:)]
        pub unsafe fn setLeaderboardID(&self, leaderboard_id: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKLeaderboardScore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
