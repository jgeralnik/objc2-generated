//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uialertactionstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAlertActionStyle(pub NSInteger);
impl UIAlertActionStyle {
    #[doc(alias = "UIAlertActionStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UIAlertActionStyleCancel")]
    pub const Cancel: Self = Self(1);
    #[doc(alias = "UIAlertActionStyleDestructive")]
    pub const Destructive: Self = Self(2);
}

unsafe impl Encode for UIAlertActionStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAlertActionStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uialertcontrollerstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAlertControllerStyle(pub NSInteger);
impl UIAlertControllerStyle {
    #[doc(alias = "UIAlertControllerStyleActionSheet")]
    pub const ActionSheet: Self = Self(0);
    #[doc(alias = "UIAlertControllerStyleAlert")]
    pub const Alert: Self = Self(1);
}

unsafe impl Encode for UIAlertControllerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAlertControllerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uialertcontrollerseverity?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAlertControllerSeverity(pub NSInteger);
impl UIAlertControllerSeverity {
    #[doc(alias = "UIAlertControllerSeverityDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "UIAlertControllerSeverityCritical")]
    pub const Critical: Self = Self(1);
}

unsafe impl Encode for UIAlertControllerSeverity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAlertControllerSeverity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uialertaction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIAlertAction;
);

unsafe impl NSCopying for UIAlertAction {}

unsafe impl CopyingHelper for UIAlertAction {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIAlertAction {}

extern_methods!(
    unsafe impl UIAlertAction {
        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other actionWithTitle:style:handler:)]
        pub unsafe fn actionWithTitle_style_handler(
            title: Option<&NSString>,
            style: UIAlertActionStyle,
            handler: Option<&block2::Block<dyn Fn(NonNull<UIAlertAction>)>>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method(style)]
        pub unsafe fn style(&self) -> UIAlertActionStyle;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIAlertAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uialertcontroller?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UIAlertController;
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIAlertController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIAlertController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIAlertController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIAlertController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIAlertController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIAlertController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIAlertController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIAlertController {
        #[method_id(@__retain_semantics Other alertControllerWithTitle:message:preferredStyle:)]
        pub unsafe fn alertControllerWithTitle_message_preferredStyle(
            title: Option<&NSString>,
            message: Option<&NSString>,
            preferred_style: UIAlertControllerStyle,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method(addAction:)]
        pub unsafe fn addAction(&self, action: &UIAlertAction);

        #[method_id(@__retain_semantics Other actions)]
        pub unsafe fn actions(&self) -> Retained<NSArray<UIAlertAction>>;

        #[method_id(@__retain_semantics Other preferredAction)]
        pub unsafe fn preferredAction(&self) -> Option<Retained<UIAlertAction>>;

        #[method(setPreferredAction:)]
        pub unsafe fn setPreferredAction(&self, preferred_action: Option<&UIAlertAction>);

        #[cfg(all(
            feature = "UIControl",
            feature = "UITextField",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method(addTextFieldWithConfigurationHandler:)]
        pub unsafe fn addTextFieldWithConfigurationHandler(
            &self,
            configuration_handler: Option<&block2::Block<dyn Fn(NonNull<UITextField>)>>,
        );

        #[cfg(all(feature = "UIControl", feature = "UITextField", feature = "UIView"))]
        #[method_id(@__retain_semantics Other textFields)]
        pub unsafe fn textFields(&self) -> Option<Retained<NSArray<UITextField>>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other message)]
        pub unsafe fn message(&self) -> Option<Retained<NSString>>;

        #[method(setMessage:)]
        pub unsafe fn setMessage(&self, message: Option<&NSString>);

        #[method(preferredStyle)]
        pub unsafe fn preferredStyle(&self) -> UIAlertControllerStyle;

        #[method(severity)]
        pub unsafe fn severity(&self) -> UIAlertControllerSeverity;

        #[method(setSeverity:)]
        pub unsafe fn setSeverity(&self, severity: UIAlertControllerSeverity);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIAlertController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIAlertController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// SpringLoading
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIAlertController {}
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UISpringLoadedInteractionSupporting",
    feature = "UIViewController"
))]
unsafe impl UISpringLoadedInteractionSupporting for UIAlertController {}
