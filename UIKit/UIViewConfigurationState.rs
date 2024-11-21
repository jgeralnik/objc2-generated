//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIViewConfigurationState;
);

unsafe impl NSCoding for UIViewConfigurationState {}

unsafe impl NSCopying for UIViewConfigurationState {}

unsafe impl CopyingHelper for UIViewConfigurationState {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIViewConfigurationState {}

unsafe impl NSSecureCoding for UIViewConfigurationState {}

#[cfg(feature = "UIConfigurationState")]
unsafe impl UIConfigurationState for UIViewConfigurationState {}

extern_methods!(
    unsafe impl UIViewConfigurationState {
        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Init initWithTraitCollection:)]
        pub unsafe fn initWithTraitCollection(
            this: Allocated<Self>,
            trait_collection: &UITraitCollection,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other traitCollection)]
        pub unsafe fn traitCollection(&self) -> Retained<UITraitCollection>;

        #[cfg(feature = "UITraitCollection")]
        #[method(setTraitCollection:)]
        pub unsafe fn setTraitCollection(&self, trait_collection: &UITraitCollection);

        #[method(isDisabled)]
        pub unsafe fn isDisabled(&self) -> bool;

        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(isFocused)]
        pub unsafe fn isFocused(&self) -> bool;

        #[method(setFocused:)]
        pub unsafe fn setFocused(&self, focused: bool);

        #[method(isPinned)]
        pub unsafe fn isPinned(&self) -> bool;

        #[method(setPinned:)]
        pub unsafe fn setPinned(&self, pinned: bool);
    }
);
