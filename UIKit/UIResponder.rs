//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "block2")]
pub type UITextAttributesConversionHandler = *mut block2::Block<
    dyn Fn(
        NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
    ) -> NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
>;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIEditingInteractionConfiguration(pub NSInteger);
impl UIEditingInteractionConfiguration {
    #[doc(alias = "UIEditingInteractionConfigurationNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIEditingInteractionConfigurationDefault")]
    pub const Default: Self = Self(1);
}

unsafe impl Encode for UIEditingInteractionConfiguration {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIEditingInteractionConfiguration {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UIResponderStandardEditActions:
        NSObjectProtocol + MainThreadOnly
    {
        #[optional]
        #[method(cut:)]
        unsafe fn cut(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(copy:)]
        unsafe fn copy(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(paste:)]
        unsafe fn paste(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(pasteAndMatchStyle:)]
        unsafe fn pasteAndMatchStyle(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(pasteAndGo:)]
        unsafe fn pasteAndGo(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(pasteAndSearch:)]
        unsafe fn pasteAndSearch(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(select:)]
        unsafe fn select(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(selectAll:)]
        unsafe fn selectAll(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(delete:)]
        unsafe fn delete(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(makeTextWritingDirectionLeftToRight:)]
        unsafe fn makeTextWritingDirectionLeftToRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(makeTextWritingDirectionRightToLeft:)]
        unsafe fn makeTextWritingDirectionRightToLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(toggleBoldface:)]
        unsafe fn toggleBoldface(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(toggleItalics:)]
        unsafe fn toggleItalics(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(toggleUnderline:)]
        unsafe fn toggleUnderline(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(increaseSize:)]
        unsafe fn increaseSize(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(decreaseSize:)]
        unsafe fn decreaseSize(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(find:)]
        unsafe fn find(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(findAndReplace:)]
        unsafe fn findAndReplace(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(findNext:)]
        unsafe fn findNext(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(findPrevious:)]
        unsafe fn findPrevious(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(useSelectionForFind:)]
        unsafe fn useSelectionForFind(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "block2")]
        #[optional]
        #[method(updateTextAttributesWithConversionHandler:)]
        unsafe fn updateTextAttributesWithConversionHandler(
            &self,
            conversion_handler: UITextAttributesConversionHandler,
        );

        #[optional]
        #[method(print:)]
        unsafe fn print(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(rename:)]
        unsafe fn rename(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(duplicate:)]
        unsafe fn duplicate(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(move:)]
        unsafe fn r#move(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(export:)]
        unsafe fn export(&self, sender: Option<&AnyObject>);
    }

    unsafe impl ProtocolType for dyn UIResponderStandardEditActions {}
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIResponder;
);

unsafe impl NSObjectProtocol for UIResponder {}

unsafe impl UIResponderStandardEditActions for UIResponder {}

extern_methods!(
    unsafe impl UIResponder {
        #[method_id(@__retain_semantics Other nextResponder)]
        pub unsafe fn nextResponder(&self) -> Option<Retained<UIResponder>>;

        #[method(canBecomeFirstResponder)]
        pub unsafe fn canBecomeFirstResponder(&self) -> bool;

        #[method(becomeFirstResponder)]
        pub unsafe fn becomeFirstResponder(&self) -> bool;

        #[method(canResignFirstResponder)]
        pub unsafe fn canResignFirstResponder(&self) -> bool;

        #[method(resignFirstResponder)]
        pub unsafe fn resignFirstResponder(&self) -> bool;

        #[method(isFirstResponder)]
        pub unsafe fn isFirstResponder(&self) -> bool;

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[method(touchesBegan:withEvent:)]
        pub unsafe fn touchesBegan_withEvent(
            &self,
            touches: &NSSet<UITouch>,
            event: Option<&UIEvent>,
        );

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[method(touchesMoved:withEvent:)]
        pub unsafe fn touchesMoved_withEvent(
            &self,
            touches: &NSSet<UITouch>,
            event: Option<&UIEvent>,
        );

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[method(touchesEnded:withEvent:)]
        pub unsafe fn touchesEnded_withEvent(
            &self,
            touches: &NSSet<UITouch>,
            event: Option<&UIEvent>,
        );

        #[cfg(all(feature = "UIEvent", feature = "UITouch"))]
        #[method(touchesCancelled:withEvent:)]
        pub unsafe fn touchesCancelled_withEvent(
            &self,
            touches: &NSSet<UITouch>,
            event: Option<&UIEvent>,
        );

        #[cfg(feature = "UITouch")]
        #[method(touchesEstimatedPropertiesUpdated:)]
        pub unsafe fn touchesEstimatedPropertiesUpdated(&self, touches: &NSSet<UITouch>);

        #[cfg(all(feature = "UIEvent", feature = "UIPress", feature = "UIPressesEvent"))]
        #[method(pressesBegan:withEvent:)]
        pub unsafe fn pressesBegan_withEvent(
            &self,
            presses: &NSSet<UIPress>,
            event: Option<&UIPressesEvent>,
        );

        #[cfg(all(feature = "UIEvent", feature = "UIPress", feature = "UIPressesEvent"))]
        #[method(pressesChanged:withEvent:)]
        pub unsafe fn pressesChanged_withEvent(
            &self,
            presses: &NSSet<UIPress>,
            event: Option<&UIPressesEvent>,
        );

        #[cfg(all(feature = "UIEvent", feature = "UIPress", feature = "UIPressesEvent"))]
        #[method(pressesEnded:withEvent:)]
        pub unsafe fn pressesEnded_withEvent(
            &self,
            presses: &NSSet<UIPress>,
            event: Option<&UIPressesEvent>,
        );

        #[cfg(all(feature = "UIEvent", feature = "UIPress", feature = "UIPressesEvent"))]
        #[method(pressesCancelled:withEvent:)]
        pub unsafe fn pressesCancelled_withEvent(
            &self,
            presses: &NSSet<UIPress>,
            event: Option<&UIPressesEvent>,
        );

        #[cfg(feature = "UIEvent")]
        #[method(motionBegan:withEvent:)]
        pub unsafe fn motionBegan_withEvent(&self, motion: UIEventSubtype, event: Option<&UIEvent>);

        #[cfg(feature = "UIEvent")]
        #[method(motionEnded:withEvent:)]
        pub unsafe fn motionEnded_withEvent(&self, motion: UIEventSubtype, event: Option<&UIEvent>);

        #[cfg(feature = "UIEvent")]
        #[method(motionCancelled:withEvent:)]
        pub unsafe fn motionCancelled_withEvent(
            &self,
            motion: UIEventSubtype,
            event: Option<&UIEvent>,
        );

        #[cfg(feature = "UIEvent")]
        #[method(remoteControlReceivedWithEvent:)]
        pub unsafe fn remoteControlReceivedWithEvent(&self, event: Option<&UIEvent>);

        #[method(canPerformAction:withSender:)]
        pub unsafe fn canPerformAction_withSender(
            &self,
            action: Sel,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[method_id(@__retain_semantics Other targetForAction:withSender:)]
        pub unsafe fn targetForAction_withSender(
            &self,
            action: Sel,
            sender: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "UIMenuBuilder")]
        #[method(buildMenuWithBuilder:)]
        pub unsafe fn buildMenuWithBuilder(&self, builder: &ProtocolObject<dyn UIMenuBuilder>);

        #[cfg(all(feature = "UICommand", feature = "UIMenuElement"))]
        #[method(validateCommand:)]
        pub unsafe fn validateCommand(&self, command: &UICommand);

        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Retained<NSUndoManager>>;

        #[method(editingInteractionConfiguration)]
        pub unsafe fn editingInteractionConfiguration(&self) -> UIEditingInteractionConfiguration;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIResponder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// UIResponderKeyCommands
    unsafe impl UIResponder {
        #[cfg(all(
            feature = "UICommand",
            feature = "UIKeyCommand",
            feature = "UIMenuElement"
        ))]
        #[method_id(@__retain_semantics Other keyCommands)]
        pub unsafe fn keyCommands(&self) -> Option<Retained<NSArray<UIKeyCommand>>>;
    }
);

extern_methods!(
    /// UIResponderInputViewAdditions
    unsafe impl UIResponder {
        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other inputView)]
        pub unsafe fn inputView(&self) -> Option<Retained<UIView>>;

        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other inputAccessoryView)]
        pub unsafe fn inputAccessoryView(&self) -> Option<Retained<UIView>>;

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other inputAssistantItem)]
        pub unsafe fn inputAssistantItem(&self) -> Retained<UITextInputAssistantItem>;

        #[cfg(all(feature = "UIInputViewController", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other inputViewController)]
        pub unsafe fn inputViewController(&self) -> Option<Retained<UIInputViewController>>;

        #[cfg(all(feature = "UIInputViewController", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other inputAccessoryViewController)]
        pub unsafe fn inputAccessoryViewController(
            &self,
        ) -> Option<Retained<UIInputViewController>>;

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other textInputMode)]
        pub unsafe fn textInputMode(&self) -> Option<Retained<UITextInputMode>>;

        #[method_id(@__retain_semantics Other textInputContextIdentifier)]
        pub unsafe fn textInputContextIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(clearTextInputContextIdentifier:)]
        pub unsafe fn clearTextInputContextIdentifier(identifier: &NSString, mtm: MainThreadMarker);

        #[method(reloadInputViews)]
        pub unsafe fn reloadInputViews(&self);
    }
);

extern "C" {
    pub static UIKeyInputUpArrow: &'static NSString;
}

extern "C" {
    pub static UIKeyInputDownArrow: &'static NSString;
}

extern "C" {
    pub static UIKeyInputLeftArrow: &'static NSString;
}

extern "C" {
    pub static UIKeyInputRightArrow: &'static NSString;
}

extern "C" {
    pub static UIKeyInputEscape: &'static NSString;
}

extern "C" {
    pub static UIKeyInputPageUp: &'static NSString;
}

extern "C" {
    pub static UIKeyInputPageDown: &'static NSString;
}

extern "C" {
    pub static UIKeyInputHome: &'static NSString;
}

extern "C" {
    pub static UIKeyInputEnd: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF2: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF3: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF4: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF5: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF6: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF7: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF8: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF9: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF10: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF11: &'static NSString;
}

extern "C" {
    pub static UIKeyInputF12: &'static NSString;
}

extern "C" {
    pub static UIKeyInputDelete: &'static NSString;
}

extern_methods!(
    /// ActivityContinuation
    unsafe impl UIResponder {
        #[method_id(@__retain_semantics Other userActivity)]
        pub unsafe fn userActivity(&self) -> Option<Retained<NSUserActivity>>;

        #[method(setUserActivity:)]
        pub unsafe fn setUserActivity(&self, user_activity: Option<&NSUserActivity>);

        #[method(updateUserActivityState:)]
        pub unsafe fn updateUserActivityState(&self, activity: &NSUserActivity);

        #[method(restoreUserActivityState:)]
        pub unsafe fn restoreUserActivityState(&self, activity: &NSUserActivity);
    }
);

#[cfg(feature = "UIUserActivity")]
unsafe impl UIUserActivityRestoring for UIResponder {}

extern_methods!(
    /// UIPasteConfigurationSupporting
    unsafe impl UIResponder {}
);

#[cfg(feature = "UIPasteConfigurationSupporting")]
unsafe impl UIPasteConfigurationSupporting for UIResponder {}

extern_methods!(
    /// UICaptureTextFromCameraSupporting
    unsafe impl UIResponder {
        #[method(captureTextFromCamera:)]
        pub unsafe fn captureTextFromCamera(&self, sender: Option<&AnyObject>);
    }
);
