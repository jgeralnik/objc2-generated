//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UITraitDefinition: MainThreadOnly {
        #[optional]
        #[method_id(@__retain_semantics Other identifier)]
        unsafe fn identifier(mtm: MainThreadMarker) -> Retained<NSString>;

        #[optional]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(mtm: MainThreadMarker) -> Retained<NSString>;

        #[optional]
        #[method(affectsColorAppearance)]
        unsafe fn affectsColorAppearance(mtm: MainThreadMarker) -> bool;
    }

    unsafe impl ProtocolType for dyn UITraitDefinition {}
);

pub type UITrait = *mut AnyClass;

extern_protocol!(
    pub unsafe trait UICGFloatTraitDefinition: UITraitDefinition + MainThreadOnly {
        #[method(defaultValue)]
        unsafe fn defaultValue(mtm: MainThreadMarker) -> CGFloat;
    }

    unsafe impl ProtocolType for dyn UICGFloatTraitDefinition {}
);

pub type UICGFloatTrait = *mut AnyClass;

extern_protocol!(
    pub unsafe trait UINSIntegerTraitDefinition: UITraitDefinition + MainThreadOnly {
        #[method(defaultValue)]
        unsafe fn defaultValue(mtm: MainThreadMarker) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn UINSIntegerTraitDefinition {}
);

pub type UINSIntegerTrait = *mut AnyClass;

extern_protocol!(
    pub unsafe trait UIObjectTraitDefinition: UITraitDefinition + MainThreadOnly {
        #[method_id(@__retain_semantics Other defaultValue)]
        unsafe fn defaultValue(mtm: MainThreadMarker) -> Option<Retained<NSObject>>;
    }

    unsafe impl ProtocolType for dyn UIObjectTraitDefinition {}
);

pub type UIObjectTrait = *mut AnyClass;

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceIdiom;
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceIdiom {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceIdiom {}

unsafe impl UITraitDefinition for UITraitUserInterfaceIdiom {}

extern_methods!(
    unsafe impl UITraitUserInterfaceIdiom {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitUserInterfaceIdiom {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceStyle;
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceStyle {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceStyle {}

unsafe impl UITraitDefinition for UITraitUserInterfaceStyle {}

extern_methods!(
    unsafe impl UITraitUserInterfaceStyle {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitUserInterfaceStyle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitLayoutDirection;
);

unsafe impl NSObjectProtocol for UITraitLayoutDirection {}

unsafe impl UINSIntegerTraitDefinition for UITraitLayoutDirection {}

unsafe impl UITraitDefinition for UITraitLayoutDirection {}

extern_methods!(
    unsafe impl UITraitLayoutDirection {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitLayoutDirection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitDisplayScale;
);

unsafe impl NSObjectProtocol for UITraitDisplayScale {}

unsafe impl UICGFloatTraitDefinition for UITraitDisplayScale {}

unsafe impl UITraitDefinition for UITraitDisplayScale {}

extern_methods!(
    unsafe impl UITraitDisplayScale {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitDisplayScale {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitHorizontalSizeClass;
);

unsafe impl NSObjectProtocol for UITraitHorizontalSizeClass {}

unsafe impl UINSIntegerTraitDefinition for UITraitHorizontalSizeClass {}

unsafe impl UITraitDefinition for UITraitHorizontalSizeClass {}

extern_methods!(
    unsafe impl UITraitHorizontalSizeClass {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitHorizontalSizeClass {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitVerticalSizeClass;
);

unsafe impl NSObjectProtocol for UITraitVerticalSizeClass {}

unsafe impl UINSIntegerTraitDefinition for UITraitVerticalSizeClass {}

unsafe impl UITraitDefinition for UITraitVerticalSizeClass {}

extern_methods!(
    unsafe impl UITraitVerticalSizeClass {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitVerticalSizeClass {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitForceTouchCapability;
);

unsafe impl NSObjectProtocol for UITraitForceTouchCapability {}

unsafe impl UINSIntegerTraitDefinition for UITraitForceTouchCapability {}

unsafe impl UITraitDefinition for UITraitForceTouchCapability {}

extern_methods!(
    unsafe impl UITraitForceTouchCapability {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitForceTouchCapability {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitPreferredContentSizeCategory;
);

unsafe impl NSObjectProtocol for UITraitPreferredContentSizeCategory {}

unsafe impl UIObjectTraitDefinition for UITraitPreferredContentSizeCategory {}

unsafe impl UITraitDefinition for UITraitPreferredContentSizeCategory {}

extern_methods!(
    unsafe impl UITraitPreferredContentSizeCategory {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitPreferredContentSizeCategory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitDisplayGamut;
);

unsafe impl NSObjectProtocol for UITraitDisplayGamut {}

unsafe impl UINSIntegerTraitDefinition for UITraitDisplayGamut {}

unsafe impl UITraitDefinition for UITraitDisplayGamut {}

extern_methods!(
    unsafe impl UITraitDisplayGamut {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitDisplayGamut {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitAccessibilityContrast;
);

unsafe impl NSObjectProtocol for UITraitAccessibilityContrast {}

unsafe impl UINSIntegerTraitDefinition for UITraitAccessibilityContrast {}

unsafe impl UITraitDefinition for UITraitAccessibilityContrast {}

extern_methods!(
    unsafe impl UITraitAccessibilityContrast {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitAccessibilityContrast {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitUserInterfaceLevel;
);

unsafe impl NSObjectProtocol for UITraitUserInterfaceLevel {}

unsafe impl UINSIntegerTraitDefinition for UITraitUserInterfaceLevel {}

unsafe impl UITraitDefinition for UITraitUserInterfaceLevel {}

extern_methods!(
    unsafe impl UITraitUserInterfaceLevel {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitUserInterfaceLevel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitLegibilityWeight;
);

unsafe impl NSObjectProtocol for UITraitLegibilityWeight {}

unsafe impl UINSIntegerTraitDefinition for UITraitLegibilityWeight {}

unsafe impl UITraitDefinition for UITraitLegibilityWeight {}

extern_methods!(
    unsafe impl UITraitLegibilityWeight {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitLegibilityWeight {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitActiveAppearance;
);

unsafe impl NSObjectProtocol for UITraitActiveAppearance {}

unsafe impl UINSIntegerTraitDefinition for UITraitActiveAppearance {}

unsafe impl UITraitDefinition for UITraitActiveAppearance {}

extern_methods!(
    unsafe impl UITraitActiveAppearance {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitActiveAppearance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitToolbarItemPresentationSize;
);

unsafe impl NSObjectProtocol for UITraitToolbarItemPresentationSize {}

unsafe impl UINSIntegerTraitDefinition for UITraitToolbarItemPresentationSize {}

unsafe impl UITraitDefinition for UITraitToolbarItemPresentationSize {}

extern_methods!(
    unsafe impl UITraitToolbarItemPresentationSize {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitToolbarItemPresentationSize {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitImageDynamicRange;
);

unsafe impl NSObjectProtocol for UITraitImageDynamicRange {}

unsafe impl UINSIntegerTraitDefinition for UITraitImageDynamicRange {}

unsafe impl UITraitDefinition for UITraitImageDynamicRange {}

extern_methods!(
    unsafe impl UITraitImageDynamicRange {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitImageDynamicRange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitTypesettingLanguage;
);

unsafe impl NSObjectProtocol for UITraitTypesettingLanguage {}

unsafe impl UIObjectTraitDefinition for UITraitTypesettingLanguage {}

unsafe impl UITraitDefinition for UITraitTypesettingLanguage {}

extern_methods!(
    unsafe impl UITraitTypesettingLanguage {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitTypesettingLanguage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitSceneCaptureState;
);

unsafe impl NSObjectProtocol for UITraitSceneCaptureState {}

unsafe impl UINSIntegerTraitDefinition for UITraitSceneCaptureState {}

unsafe impl UITraitDefinition for UITraitSceneCaptureState {}

extern_methods!(
    unsafe impl UITraitSceneCaptureState {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitSceneCaptureState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
