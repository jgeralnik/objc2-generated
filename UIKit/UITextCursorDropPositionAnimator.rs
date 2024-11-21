//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextCursorDropPositionAnimator;
);

unsafe impl NSObjectProtocol for UITextCursorDropPositionAnimator {}

extern_methods!(
    unsafe impl UITextCursorDropPositionAnimator {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextCursorView",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Other cursorView)]
        pub unsafe fn cursorView(&self) -> Option<Retained<UIView>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Other textInput)]
        pub unsafe fn textInput(&self) -> Option<Retained<UIView>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextCursorView",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Init initWithTextCursorView:textInput:)]
        pub unsafe fn initWithTextCursorView_textInput(
            this: Allocated<Self>,
            cursor_view: Option<&UIView>,
            text_input: Option<&UIView>,
        ) -> Option<Retained<Self>>;

        #[method(setCursorVisible:animated:)]
        pub unsafe fn setCursorVisible_animated(&self, visible: bool, animated: bool);

        #[cfg(feature = "UITextInput")]
        #[method(placeCursorAtPosition:animated:)]
        pub unsafe fn placeCursorAtPosition_animated(
            &self,
            position: Option<&UITextPosition>,
            animated: bool,
        );

        #[cfg(feature = "block2")]
        #[method(animateAlongsideChanges:completion:)]
        pub unsafe fn animateAlongsideChanges_completion(
            &self,
            animation: Option<&block2::Block<dyn Fn()>>,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextCursorDropPositionAnimator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
