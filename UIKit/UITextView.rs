//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_protocol!(
    #[cfg(feature = "UIScrollView")]
    pub unsafe trait UITextViewDelegate:
        NSObjectProtocol + UIScrollViewDelegate + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textViewShouldBeginEditing:)]
        unsafe fn textViewShouldBeginEditing(&self, text_view: &UITextView) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textViewShouldEndEditing:)]
        unsafe fn textViewShouldEndEditing(&self, text_view: &UITextView) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textViewDidBeginEditing:)]
        unsafe fn textViewDidBeginEditing(&self, text_view: &UITextView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textViewDidEndEditing:)]
        unsafe fn textViewDidEndEditing(&self, text_view: &UITextView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textView:shouldChangeTextInRange:replacementText:)]
        unsafe fn textView_shouldChangeTextInRange_replacementText(
            &self,
            text_view: &UITextView,
            range: NSRange,
            text: &NSString,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textViewDidChange:)]
        unsafe fn textViewDidChange(&self, text_view: &UITextView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textViewDidChangeSelection:)]
        unsafe fn textViewDidChangeSelection(&self, text_view: &UITextView);

        #[cfg(all(
            feature = "UIMenu",
            feature = "UIMenuElement",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textView:editMenuForTextInRange:suggestedActions:)]
        unsafe fn textView_editMenuForTextInRange_suggestedActions(
            &self,
            text_view: &UITextView,
            range: NSRange,
            suggested_actions: &NSArray<UIMenuElement>,
        ) -> Option<Retained<UIMenu>>;

        #[cfg(all(
            feature = "UIEditMenuInteraction",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textView:willPresentEditMenuWithAnimator:)]
        unsafe fn textView_willPresentEditMenuWithAnimator(
            &self,
            text_view: &UITextView,
            animator: &ProtocolObject<dyn UIEditMenuInteractionAnimating>,
        );

        #[cfg(all(
            feature = "UIEditMenuInteraction",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textView:willDismissEditMenuWithAnimator:)]
        unsafe fn textView_willDismissEditMenuWithAnimator(
            &self,
            text_view: &UITextView,
            animator: &ProtocolObject<dyn UIEditMenuInteractionAnimating>,
        );

        #[cfg(all(
            feature = "UIAction",
            feature = "UIMenuElement",
            feature = "UIResponder",
            feature = "UITextItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textView:primaryActionForTextItem:defaultAction:)]
        unsafe fn textView_primaryActionForTextItem_defaultAction(
            &self,
            text_view: &UITextView,
            text_item: &UITextItem,
            default_action: &UIAction,
        ) -> Option<Retained<UIAction>>;

        #[cfg(all(
            feature = "UIMenu",
            feature = "UIMenuElement",
            feature = "UIResponder",
            feature = "UITextItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textView:menuConfigurationForTextItem:defaultMenu:)]
        unsafe fn textView_menuConfigurationForTextItem_defaultMenu(
            &self,
            text_view: &UITextView,
            text_item: &UITextItem,
            default_menu: &UIMenu,
        ) -> Option<Retained<UITextItemMenuConfiguration>>;

        #[cfg(all(
            feature = "UIContextMenuInteraction",
            feature = "UIResponder",
            feature = "UITextItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textView:textItemMenuWillDisplayForTextItem:animator:)]
        unsafe fn textView_textItemMenuWillDisplayForTextItem_animator(
            &self,
            text_view: &UITextView,
            text_item: &UITextItem,
            animator: &ProtocolObject<dyn UIContextMenuInteractionAnimating>,
        );

        #[cfg(all(
            feature = "UIContextMenuInteraction",
            feature = "UIResponder",
            feature = "UITextItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textView:textItemMenuWillEndForTextItem:animator:)]
        unsafe fn textView_textItemMenuWillEndForTextItem_animator(
            &self,
            text_view: &UITextView,
            text_item: &UITextItem,
            animator: &ProtocolObject<dyn UIContextMenuInteractionAnimating>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textViewWritingToolsWillBegin:)]
        unsafe fn textViewWritingToolsWillBegin(&self, text_view: &UITextView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(textViewWritingToolsDidEnd:)]
        unsafe fn textViewWritingToolsDidEnd(&self, text_view: &UITextView);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method_id(@__retain_semantics Other textView:writingToolsIgnoredRangesInEnclosingRange:)]
        unsafe fn textView_writingToolsIgnoredRangesInEnclosingRange(
            &self,
            text_view: &UITextView,
            enclosing_range: NSRange,
        ) -> Retained<NSArray<NSValue>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextItemInteraction",
            feature = "UIView"
        ))]
        #[deprecated = "Replaced by primaryActionForTextItem: and menuConfigurationForTextItem: for additional customization options."]
        #[optional]
        #[method(textView:shouldInteractWithURL:inRange:interaction:)]
        unsafe fn textView_shouldInteractWithURL_inRange_interaction(
            &self,
            text_view: &UITextView,
            url: &NSURL,
            character_range: NSRange,
            interaction: UITextItemInteraction,
        ) -> bool;

        #[cfg(all(
            feature = "NSTextAttachment",
            feature = "UIResponder",
            feature = "UITextItemInteraction",
            feature = "UIView"
        ))]
        #[deprecated = "Replaced by primaryActionForTextItem: and menuConfigurationForTextItem: for additional customization options."]
        #[optional]
        #[method(textView:shouldInteractWithTextAttachment:inRange:interaction:)]
        unsafe fn textView_shouldInteractWithTextAttachment_inRange_interaction(
            &self,
            text_view: &UITextView,
            text_attachment: &NSTextAttachment,
            character_range: NSRange,
            interaction: UITextItemInteraction,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated]
        #[optional]
        #[method(textView:shouldInteractWithURL:inRange:)]
        unsafe fn textView_shouldInteractWithURL_inRange(
            &self,
            text_view: &UITextView,
            url: &NSURL,
            character_range: NSRange,
        ) -> bool;

        #[cfg(all(
            feature = "NSTextAttachment",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[deprecated]
        #[optional]
        #[method(textView:shouldInteractWithTextAttachment:inRange:)]
        unsafe fn textView_shouldInteractWithTextAttachment_inRange(
            &self,
            text_view: &UITextView,
            text_attachment: &NSTextAttachment,
            character_range: NSRange,
        ) -> bool;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextFormattingViewController",
            feature = "UIView",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(textView:willBeginFormattingWithViewController:)]
        unsafe fn textView_willBeginFormattingWithViewController(
            &self,
            text_view: &UITextView,
            view_controller: &UITextFormattingViewController,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextFormattingViewController",
            feature = "UIView",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(textView:didBeginFormattingWithViewController:)]
        unsafe fn textView_didBeginFormattingWithViewController(
            &self,
            text_view: &UITextView,
            view_controller: &UITextFormattingViewController,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextFormattingViewController",
            feature = "UIView",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(textView:willEndFormattingWithViewController:)]
        unsafe fn textView_willEndFormattingWithViewController(
            &self,
            text_view: &UITextView,
            view_controller: &UITextFormattingViewController,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextFormattingViewController",
            feature = "UIView",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(textView:didEndFormattingWithViewController:)]
        unsafe fn textView_didEndFormattingWithViewController(
            &self,
            text_view: &UITextView,
            view_controller: &UITextFormattingViewController,
        );
    }

    #[cfg(feature = "UIScrollView")]
    unsafe impl ProtocolType for dyn UITextViewDelegate {}
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextViewBorderStyle(pub NSInteger);
impl UITextViewBorderStyle {
    #[doc(alias = "UITextViewBorderStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UITextViewBorderStyleRoundedRect")]
    pub const RoundedRect: Self = Self(1);
}

unsafe impl Encode for UITextViewBorderStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextViewBorderStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
    pub struct UITextView;

    #[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
    unsafe impl ClassType for UITextView {
        #[inherits(UIView, UIResponder, NSObject)]
        type Super = UIScrollView;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UITextView {}

#[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
unsafe impl NSCoding for UITextView {}

#[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
unsafe impl NSObjectProtocol for UITextView {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIAppearance for UITextView {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UITextView {}

#[cfg(all(
    feature = "UIContentSizeCategoryAdjusting",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIContentSizeCategoryAdjusting for UITextView {}

#[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
unsafe impl UICoordinateSpace for UITextView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UITextView {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UITextView {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UITextView {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UITextView {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIFocusItemScrollableContainer for UITextView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
unsafe impl UIKeyInput for UITextView {}

#[cfg(all(
    feature = "UILetterformAwareAdjusting",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UILetterformAwareAdjusting for UITextView {}

#[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UITextView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
unsafe impl UITextInput for UITextView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
unsafe impl UITextInputTraits for UITextView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UITextView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
    unsafe impl UITextView {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn UITextViewDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn UITextViewDelegate>>);

        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Retained<UIFont>>;

        #[cfg(feature = "UIFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&UIFont>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: Option<&UIColor>);

        #[cfg(feature = "NSText")]
        #[method(textAlignment)]
        pub unsafe fn textAlignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        #[method(setTextAlignment:)]
        pub unsafe fn setTextAlignment(&self, text_alignment: NSTextAlignment);

        #[method(selectedRange)]
        pub unsafe fn selectedRange(&self) -> NSRange;

        #[method(setSelectedRange:)]
        pub unsafe fn setSelectedRange(&self, selected_range: NSRange);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;

        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);

        #[cfg(feature = "UIDataDetectors")]
        #[method(dataDetectorTypes)]
        pub unsafe fn dataDetectorTypes(&self) -> UIDataDetectorTypes;

        #[cfg(feature = "UIDataDetectors")]
        #[method(setDataDetectorTypes:)]
        pub unsafe fn setDataDetectorTypes(&self, data_detector_types: UIDataDetectorTypes);

        #[method(allowsEditingTextAttributes)]
        pub unsafe fn allowsEditingTextAttributes(&self) -> bool;

        #[method(setAllowsEditingTextAttributes:)]
        pub unsafe fn setAllowsEditingTextAttributes(&self, allows_editing_text_attributes: bool);

        #[method_id(@__retain_semantics Other attributedText)]
        pub unsafe fn attributedText(&self) -> Retained<NSAttributedString>;

        #[method(setAttributedText:)]
        pub unsafe fn setAttributedText(&self, attributed_text: Option<&NSAttributedString>);

        #[method_id(@__retain_semantics Other typingAttributes)]
        pub unsafe fn typingAttributes(
            &self,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method(setTypingAttributes:)]
        pub unsafe fn setTypingAttributes(
            &self,
            typing_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        );

        #[method(scrollRangeToVisible:)]
        pub unsafe fn scrollRangeToVisible(&self, range: NSRange);

        #[method_id(@__retain_semantics Other inputView)]
        pub unsafe fn inputView(&self) -> Option<Retained<UIView>>;

        #[method(setInputView:)]
        pub unsafe fn setInputView(&self, input_view: Option<&UIView>);

        #[method_id(@__retain_semantics Other inputAccessoryView)]
        pub unsafe fn inputAccessoryView(&self) -> Option<Retained<UIView>>;

        #[method(setInputAccessoryView:)]
        pub unsafe fn setInputAccessoryView(&self, input_accessory_view: Option<&UIView>);

        #[method(clearsOnInsertion)]
        pub unsafe fn clearsOnInsertion(&self) -> bool;

        #[method(setClearsOnInsertion:)]
        pub unsafe fn setClearsOnInsertion(&self, clears_on_insertion: bool);

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Init initWithFrame:textContainer:)]
        pub unsafe fn initWithFrame_textContainer(
            this: Allocated<Self>,
            frame: CGRect,
            text_container: Option<&NSTextContainer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other textViewUsingTextLayoutManager:)]
        pub unsafe fn textViewUsingTextLayoutManager(
            using_text_layout_manager: bool,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other textContainer)]
        pub unsafe fn textContainer(&self) -> Retained<NSTextContainer>;

        #[cfg(feature = "UIGeometry")]
        #[method(textContainerInset)]
        pub unsafe fn textContainerInset(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setTextContainerInset:)]
        pub unsafe fn setTextContainerInset(&self, text_container_inset: UIEdgeInsets);

        #[cfg(feature = "NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Retained<NSTextLayoutManager>>;

        #[cfg(feature = "NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager(&self) -> Retained<NSLayoutManager>;

        #[cfg(feature = "NSTextStorage")]
        #[method_id(@__retain_semantics Other textStorage)]
        pub unsafe fn textStorage(&self) -> Retained<NSTextStorage>;

        #[method_id(@__retain_semantics Other linkTextAttributes)]
        pub unsafe fn linkTextAttributes(
            &self,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method(setLinkTextAttributes:)]
        pub unsafe fn setLinkTextAttributes(
            &self,
            link_text_attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[method(usesStandardTextScaling)]
        pub unsafe fn usesStandardTextScaling(&self) -> bool;

        #[method(setUsesStandardTextScaling:)]
        pub unsafe fn setUsesStandardTextScaling(&self, uses_standard_text_scaling: bool);

        #[cfg(feature = "UIFindInteraction")]
        #[method_id(@__retain_semantics Other findInteraction)]
        pub unsafe fn findInteraction(&self) -> Option<Retained<UIFindInteraction>>;

        #[method(isFindInteractionEnabled)]
        pub unsafe fn isFindInteractionEnabled(&self) -> bool;

        #[method(setFindInteractionEnabled:)]
        pub unsafe fn setFindInteractionEnabled(&self, find_interaction_enabled: bool);

        #[method(borderStyle)]
        pub unsafe fn borderStyle(&self) -> UITextViewBorderStyle;

        #[method(setBorderStyle:)]
        pub unsafe fn setBorderStyle(&self, border_style: UITextViewBorderStyle);

        #[method_id(@__retain_semantics Other textHighlightAttributes)]
        pub unsafe fn textHighlightAttributes(
            &self,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[method(setTextHighlightAttributes:)]
        pub unsafe fn setTextHighlightAttributes(
            &self,
            text_highlight_attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[cfg(feature = "NSTextRange")]
        #[method(drawTextHighlightBackgroundForTextRange:origin:)]
        pub unsafe fn drawTextHighlightBackgroundForTextRange_origin(
            &self,
            text_range: &NSTextRange,
            origin: CGPoint,
        );

        #[method(isWritingToolsActive)]
        pub unsafe fn isWritingToolsActive(&self) -> bool;

        #[cfg(feature = "UITextInputTraits")]
        #[method(writingToolsBehavior)]
        pub unsafe fn writingToolsBehavior(&self) -> UIWritingToolsBehavior;

        #[cfg(feature = "UITextInputTraits")]
        #[method(setWritingToolsBehavior:)]
        pub unsafe fn setWritingToolsBehavior(
            &self,
            writing_tools_behavior: UIWritingToolsBehavior,
        );

        #[cfg(feature = "UITextInputTraits")]
        #[method(allowedWritingToolsResultOptions)]
        pub unsafe fn allowedWritingToolsResultOptions(&self) -> UIWritingToolsResultOptions;

        #[cfg(feature = "UITextInputTraits")]
        #[method(setAllowedWritingToolsResultOptions:)]
        pub unsafe fn setAllowedWritingToolsResultOptions(
            &self,
            allowed_writing_tools_result_options: UIWritingToolsResultOptions,
        );

        #[cfg(feature = "UITextFormattingViewControllerConfiguration")]
        #[method_id(@__retain_semantics Other textFormattingConfiguration)]
        pub unsafe fn textFormattingConfiguration(
            &self,
        ) -> Option<Retained<UITextFormattingViewControllerConfiguration>>;

        #[cfg(feature = "UITextFormattingViewControllerConfiguration")]
        #[method(setTextFormattingConfiguration:)]
        pub unsafe fn setTextFormattingConfiguration(
            &self,
            text_formatting_configuration: Option<&UITextFormattingViewControllerConfiguration>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
    unsafe impl UITextView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
    unsafe impl UITextView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
    unsafe impl UITextView {}
);

#[cfg(all(
    feature = "UIFindInteraction",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView"
))]
unsafe impl UIFindInteractionDelegate for UITextView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITextDragging",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UIView"
))]
unsafe impl UITextDraggable for UITextView {}

#[cfg(all(
    feature = "UIPasteConfigurationSupporting",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITextDropping",
    feature = "UITextInput",
    feature = "UITextInputTraits",
    feature = "UITextPasteConfigurationSupporting",
    feature = "UIView"
))]
unsafe impl UITextDroppable for UITextView {}

#[cfg(all(
    feature = "UIPasteConfigurationSupporting",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITextPasteConfigurationSupporting",
    feature = "UIView"
))]
unsafe impl UITextPasteConfigurationSupporting for UITextView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITextSearching",
    feature = "UIView"
))]
unsafe impl UITextSearching for UITextView {}

extern_methods!(
    /// UIInteractionStateRestorable
    #[cfg(all(feature = "UIResponder", feature = "UIScrollView", feature = "UIView"))]
    unsafe impl UITextView {
        #[method_id(@__retain_semantics Other interactionState)]
        pub unsafe fn interactionState(&self) -> Retained<AnyObject>;

        #[method(setInteractionState:)]
        pub unsafe fn setInteractionState(&self, interaction_state: &AnyObject);
    }
);

extern "C" {
    pub static UITextViewTextDidBeginEditingNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UITextViewTextDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UITextViewTextDidEndEditingNotification: &'static NSNotificationName;
}
