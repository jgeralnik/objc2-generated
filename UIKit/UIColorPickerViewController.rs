//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UIColorPickerViewControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated]
        #[optional]
        #[method(colorPickerViewControllerDidSelectColor:)]
        unsafe fn colorPickerViewControllerDidSelectColor(
            &self,
            view_controller: &UIColorPickerViewController,
        );

        #[cfg(all(
            feature = "UIColor",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(colorPickerViewController:didSelectColor:continuously:)]
        unsafe fn colorPickerViewController_didSelectColor_continuously(
            &self,
            view_controller: &UIColorPickerViewController,
            color: &UIColor,
            continuously: bool,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(colorPickerViewControllerDidFinish:)]
        unsafe fn colorPickerViewControllerDidFinish(
            &self,
            view_controller: &UIColorPickerViewController,
        );
    }

    unsafe impl ProtocolType for dyn UIColorPickerViewControllerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UIColorPickerViewController;

    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl ClassType for UIColorPickerViewController {
        #[inherits(UIResponder, NSObject)]
        type Super = UIViewController;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIColorPickerViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIColorPickerViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIColorPickerViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIColorPickerViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIColorPickerViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIColorPickerViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIColorPickerViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIColorPickerViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIColorPickerViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIColorPickerViewControllerDelegate>>,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other selectedColor)]
        pub unsafe fn selectedColor(&self) -> Retained<UIColor>;

        #[cfg(feature = "UIColor")]
        #[method(setSelectedColor:)]
        pub unsafe fn setSelectedColor(&self, selected_color: &UIColor);

        #[method(supportsAlpha)]
        pub unsafe fn supportsAlpha(&self) -> bool;

        #[method(setSupportsAlpha:)]
        pub unsafe fn setSupportsAlpha(&self, supports_alpha: bool);

        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIColorPickerViewController {
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
    unsafe impl UIColorPickerViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
