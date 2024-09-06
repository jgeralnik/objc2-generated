//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait NSInputServiceProvider {
        #[deprecated]
        #[method(insertText:client:)]
        unsafe fn insertText_client(&self, string: Option<&AnyObject>, sender: Option<&AnyObject>);

        #[deprecated]
        #[method(doCommandBySelector:client:)]
        unsafe fn doCommandBySelector_client(
            &self,
            selector: Option<Sel>,
            sender: Option<&AnyObject>,
        );

        #[deprecated]
        #[method(markedTextAbandoned:)]
        unsafe fn markedTextAbandoned(&self, sender: Option<&AnyObject>);

        #[deprecated]
        #[method(markedTextSelectionChanged:client:)]
        unsafe fn markedTextSelectionChanged_client(
            &self,
            new_sel: NSRange,
            sender: Option<&AnyObject>,
        );

        #[deprecated]
        #[method(terminate:)]
        unsafe fn terminate(&self, sender: Option<&AnyObject>);

        #[deprecated]
        #[method(canBeDisabled)]
        unsafe fn canBeDisabled(&self) -> bool;

        #[deprecated]
        #[method(wantsToInterpretAllKeystrokes)]
        unsafe fn wantsToInterpretAllKeystrokes(&self) -> bool;

        #[deprecated]
        #[method(wantsToHandleMouseEvents)]
        unsafe fn wantsToHandleMouseEvents(&self) -> bool;

        #[deprecated]
        #[method(wantsToDelayTextChangeNotifications)]
        unsafe fn wantsToDelayTextChangeNotifications(&self) -> bool;

        #[deprecated]
        #[method(inputClientBecomeActive:)]
        unsafe fn inputClientBecomeActive(&self, sender: Option<&AnyObject>);

        #[deprecated]
        #[method(inputClientResignActive:)]
        unsafe fn inputClientResignActive(&self, sender: Option<&AnyObject>);

        #[deprecated]
        #[method(inputClientEnabled:)]
        unsafe fn inputClientEnabled(&self, sender: Option<&AnyObject>);

        #[deprecated]
        #[method(inputClientDisabled:)]
        unsafe fn inputClientDisabled(&self, sender: Option<&AnyObject>);

        #[deprecated]
        #[method(activeConversationWillChange:fromOldConversation:)]
        unsafe fn activeConversationWillChange_fromOldConversation(
            &self,
            sender: Option<&AnyObject>,
            old_conversation: NSInteger,
        );

        #[deprecated]
        #[method(activeConversationChanged:toNewConversation:)]
        unsafe fn activeConversationChanged_toNewConversation(
            &self,
            sender: Option<&AnyObject>,
            new_conversation: NSInteger,
        );
    }

    unsafe impl ProtocolType for dyn NSInputServiceProvider {}
);

extern_protocol!(
    pub unsafe trait NSInputServerMouseTracker {
        #[deprecated]
        #[method(mouseDownOnCharacterIndex:atCoordinate:withModifier:client:)]
        unsafe fn mouseDownOnCharacterIndex_atCoordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[deprecated]
        #[method(mouseDraggedOnCharacterIndex:atCoordinate:withModifier:client:)]
        unsafe fn mouseDraggedOnCharacterIndex_atCoordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&AnyObject>,
        ) -> bool;

        #[deprecated]
        #[method(mouseUpOnCharacterIndex:atCoordinate:withModifier:client:)]
        unsafe fn mouseUpOnCharacterIndex_atCoordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&AnyObject>,
        );
    }

    unsafe impl ProtocolType for dyn NSInputServerMouseTracker {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct NSInputServer;

    unsafe impl ClassType for NSInputServer {
        type Super = NSObject;
    }
);

unsafe impl NSInputServerMouseTracker for NSInputServer {}

unsafe impl NSInputServiceProvider for NSInputServer {}

unsafe impl NSObjectProtocol for NSInputServer {}

extern_methods!(
    unsafe impl NSInputServer {
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithDelegate:name:)]
        pub unsafe fn initWithDelegate_name(
            this: Allocated<Self>,
            delegate: Option<&AnyObject>,
            name: Option<&NSString>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSInputServer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
