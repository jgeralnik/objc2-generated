//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UICalendarView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UICalendarView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UICalendarView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UICalendarView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UICalendarView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UICalendarView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UICalendarView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UICalendarView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UICalendarView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UICalendarView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UICalendarView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UICalendarView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UICalendarView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UICalendarView {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UICalendarViewDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UICalendarViewDelegate>>,
        );

        #[cfg(feature = "UICalendarSelection")]
        #[method_id(@__retain_semantics Other selectionBehavior)]
        pub unsafe fn selectionBehavior(&self) -> Option<Retained<UICalendarSelection>>;

        #[cfg(feature = "UICalendarSelection")]
        #[method(setSelectionBehavior:)]
        pub unsafe fn setSelectionBehavior(&self, selection_behavior: Option<&UICalendarSelection>);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Retained<NSLocale>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: &NSLocale);

        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Retained<NSCalendar>;

        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: &NSCalendar);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "UIFontDescriptor")]
        #[method_id(@__retain_semantics Other fontDesign)]
        pub unsafe fn fontDesign(&self) -> Retained<UIFontDescriptorSystemDesign>;

        #[cfg(feature = "UIFontDescriptor")]
        #[method(setFontDesign:)]
        pub unsafe fn setFontDesign(&self, font_design: &UIFontDescriptorSystemDesign);

        #[method_id(@__retain_semantics Other availableDateRange)]
        pub unsafe fn availableDateRange(&self) -> Retained<NSDateInterval>;

        #[method(setAvailableDateRange:)]
        pub unsafe fn setAvailableDateRange(&self, available_date_range: &NSDateInterval);

        #[method_id(@__retain_semantics Other visibleDateComponents)]
        pub unsafe fn visibleDateComponents(&self) -> Retained<NSDateComponents>;

        #[method(setVisibleDateComponents:)]
        pub unsafe fn setVisibleDateComponents(&self, visible_date_components: &NSDateComponents);

        #[method(setVisibleDateComponents:animated:)]
        pub unsafe fn setVisibleDateComponents_animated(
            &self,
            date_components: &NSDateComponents,
            animated: bool,
        );

        #[method(wantsDateDecorations)]
        pub unsafe fn wantsDateDecorations(&self) -> bool;

        #[method(setWantsDateDecorations:)]
        pub unsafe fn setWantsDateDecorations(&self, wants_date_decorations: bool);

        #[method(reloadDecorationsForDateComponents:animated:)]
        pub unsafe fn reloadDecorationsForDateComponents_animated(
            &self,
            dates: &NSArray<NSDateComponents>,
            animated: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UICalendarView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UICalendarView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarviewdelegate?language=objc)
    pub unsafe trait UICalendarViewDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "UICalendarViewDecoration",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other calendarView:decorationForDateComponents:)]
        unsafe fn calendarView_decorationForDateComponents(
            &self,
            calendar_view: &UICalendarView,
            date_components: &NSDateComponents,
        ) -> Option<Retained<UICalendarViewDecoration>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(calendarView:didChangeVisibleDateComponentsFrom:)]
        unsafe fn calendarView_didChangeVisibleDateComponentsFrom(
            &self,
            calendar_view: &UICalendarView,
            previous_date_components: &NSDateComponents,
        );
    }

    unsafe impl ProtocolType for dyn UICalendarViewDelegate {}
);
