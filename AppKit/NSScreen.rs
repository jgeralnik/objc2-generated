//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScreen;

    unsafe impl ClassType for NSScreen {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSScreen")]
    unsafe impl NSScreen {
        #[method_id(@__retain_semantics Other screens)]
        pub unsafe fn screens() -> Id<NSArray<NSScreen>, Shared>;

        #[method_id(@__retain_semantics Other mainScreen)]
        pub unsafe fn mainScreen() -> Option<Id<NSScreen, Shared>>;

        #[method_id(@__retain_semantics Other deepestScreen)]
        pub unsafe fn deepestScreen() -> Option<Id<NSScreen, Shared>>;

        #[method(screensHaveSeparateSpaces)]
        pub unsafe fn screensHaveSeparateSpaces() -> bool;

        #[method(depth)]
        pub unsafe fn depth(&self) -> NSWindowDepth;

        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(visibleFrame)]
        pub unsafe fn visibleFrame(&self) -> NSRect;

        #[cfg(feature = "AppKit_NSDeviceDescriptionKey")]
        #[method_id(@__retain_semantics Other deviceDescription)]
        pub unsafe fn deviceDescription(
            &self,
        ) -> Id<NSDictionary<NSDeviceDescriptionKey, Object>, Shared>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Option<Id<NSColorSpace, Shared>>;

        #[method(supportedWindowDepths)]
        pub unsafe fn supportedWindowDepths(&self) -> NonNull<NSWindowDepth>;

        #[method(canRepresentDisplayGamut:)]
        pub unsafe fn canRepresentDisplayGamut(&self, displayGamut: NSDisplayGamut) -> bool;

        #[method(convertRectToBacking:)]
        pub unsafe fn convertRectToBacking(&self, rect: NSRect) -> NSRect;

        #[method(convertRectFromBacking:)]
        pub unsafe fn convertRectFromBacking(&self, rect: NSRect) -> NSRect;

        #[method(backingAlignedRect:options:)]
        pub unsafe fn backingAlignedRect_options(
            &self,
            rect: NSRect,
            options: NSAlignmentOptions,
        ) -> NSRect;

        #[method(backingScaleFactor)]
        pub unsafe fn backingScaleFactor(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Id<NSString, Shared>;

        #[method(safeAreaInsets)]
        pub unsafe fn safeAreaInsets(&self) -> NSEdgeInsets;

        #[method(auxiliaryTopLeftArea)]
        pub unsafe fn auxiliaryTopLeftArea(&self) -> NSRect;

        #[method(auxiliaryTopRightArea)]
        pub unsafe fn auxiliaryTopRightArea(&self) -> NSRect;
    }
);

extern_static!(NSScreenColorSpaceDidChangeNotification: &'static NSNotificationName);

extern_methods!(
    #[cfg(feature = "AppKit_NSScreen")]
    unsafe impl NSScreen {
        #[method(maximumExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;

        #[method(maximumPotentialExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumPotentialExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;

        #[method(maximumReferenceExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumReferenceExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSScreen")]
    unsafe impl NSScreen {
        #[method(maximumFramesPerSecond)]
        pub unsafe fn maximumFramesPerSecond(&self) -> NSInteger;

        #[method(minimumRefreshInterval)]
        pub unsafe fn minimumRefreshInterval(&self) -> NSTimeInterval;

        #[method(maximumRefreshInterval)]
        pub unsafe fn maximumRefreshInterval(&self) -> NSTimeInterval;

        #[method(displayUpdateGranularity)]
        pub unsafe fn displayUpdateGranularity(&self) -> NSTimeInterval;

        #[method(lastDisplayUpdateTimestamp)]
        pub unsafe fn lastDisplayUpdateTimestamp(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSScreen")]
    unsafe impl NSScreen {
        #[method(userSpaceScaleFactor)]
        pub unsafe fn userSpaceScaleFactor(&self) -> CGFloat;
    }
);
