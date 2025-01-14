//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundoclosegroupingrunloopordering?language=objc)
pub static NSUndoCloseGroupingRunLoopOrdering: NSUInteger = 350000;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanageruserinfokey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSUndoManagerUserInfoKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagergroupisdiscardablekey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSUndoManagerGroupIsDiscardableKey: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUndoManager;
);

unsafe impl NSObjectProtocol for NSUndoManager {}

extern_methods!(
    unsafe impl NSUndoManager {
        #[method(beginUndoGrouping)]
        pub unsafe fn beginUndoGrouping(&self);

        #[method(endUndoGrouping)]
        pub unsafe fn endUndoGrouping(&self);

        #[method(groupingLevel)]
        pub unsafe fn groupingLevel(&self) -> NSInteger;

        #[method(disableUndoRegistration)]
        pub unsafe fn disableUndoRegistration(&self);

        #[method(enableUndoRegistration)]
        pub unsafe fn enableUndoRegistration(&self);

        #[method(isUndoRegistrationEnabled)]
        pub unsafe fn isUndoRegistrationEnabled(&self) -> bool;

        #[method(groupsByEvent)]
        pub unsafe fn groupsByEvent(&self) -> bool;

        #[method(setGroupsByEvent:)]
        pub unsafe fn setGroupsByEvent(&self, groups_by_event: bool);

        #[method(levelsOfUndo)]
        pub unsafe fn levelsOfUndo(&self) -> NSUInteger;

        #[method(setLevelsOfUndo:)]
        pub unsafe fn setLevelsOfUndo(&self, levels_of_undo: NSUInteger);

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "NSString"))]
        #[method_id(@__retain_semantics Other runLoopModes)]
        pub unsafe fn runLoopModes(&self) -> Retained<NSArray<NSRunLoopMode>>;

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "NSString"))]
        #[method(setRunLoopModes:)]
        pub unsafe fn setRunLoopModes(&self, run_loop_modes: &NSArray<NSRunLoopMode>);

        #[method(undo)]
        pub unsafe fn undo(&self);

        #[method(redo)]
        pub unsafe fn redo(&self);

        #[method(undoNestedGroup)]
        pub unsafe fn undoNestedGroup(&self);

        #[method(canUndo)]
        pub unsafe fn canUndo(&self) -> bool;

        #[method(canRedo)]
        pub unsafe fn canRedo(&self) -> bool;

        #[method(undoCount)]
        pub unsafe fn undoCount(&self) -> NSUInteger;

        #[method(redoCount)]
        pub unsafe fn redoCount(&self) -> NSUInteger;

        #[method(isUndoing)]
        pub unsafe fn isUndoing(&self) -> bool;

        #[method(isRedoing)]
        pub unsafe fn isRedoing(&self) -> bool;

        #[method(removeAllActions)]
        pub unsafe fn removeAllActions(&self);

        #[method(removeAllActionsWithTarget:)]
        pub unsafe fn removeAllActionsWithTarget(&self, target: &AnyObject);

        #[method(registerUndoWithTarget:selector:object:)]
        pub unsafe fn registerUndoWithTarget_selector_object(
            &self,
            target: &AnyObject,
            selector: Sel,
            object: Option<&AnyObject>,
        );

        #[method_id(@__retain_semantics Other prepareWithInvocationTarget:)]
        pub unsafe fn prepareWithInvocationTarget(&self, target: &AnyObject)
            -> Retained<AnyObject>;

        #[cfg(feature = "block2")]
        #[method(registerUndoWithTarget:handler:)]
        pub unsafe fn registerUndoWithTarget_handler(
            &self,
            target: &AnyObject,
            undo_handler: &block2::Block<dyn Fn(NonNull<AnyObject>)>,
        );

        #[method(setActionIsDiscardable:)]
        pub unsafe fn setActionIsDiscardable(&self, discardable: bool);

        #[method(undoActionIsDiscardable)]
        pub unsafe fn undoActionIsDiscardable(&self) -> bool;

        #[method(redoActionIsDiscardable)]
        pub unsafe fn redoActionIsDiscardable(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other undoActionName)]
        pub unsafe fn undoActionName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other redoActionName)]
        pub unsafe fn redoActionName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method(setActionName:)]
        pub unsafe fn setActionName(&self, action_name: &NSString);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other undoActionUserInfoValueForKey:)]
        pub unsafe fn undoActionUserInfoValueForKey(
            &self,
            key: &NSUndoManagerUserInfoKey,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other redoActionUserInfoValueForKey:)]
        pub unsafe fn redoActionUserInfoValueForKey(
            &self,
            key: &NSUndoManagerUserInfoKey,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method(setActionUserInfoValue:forKey:)]
        pub unsafe fn setActionUserInfoValue_forKey(
            &self,
            info: Option<&AnyObject>,
            key: &NSUndoManagerUserInfoKey,
        );

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other undoMenuItemTitle)]
        pub unsafe fn undoMenuItemTitle(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other redoMenuItemTitle)]
        pub unsafe fn redoMenuItemTitle(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other undoMenuTitleForUndoActionName:)]
        pub unsafe fn undoMenuTitleForUndoActionName(
            &self,
            action_name: &NSString,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other redoMenuTitleForUndoActionName:)]
        pub unsafe fn redoMenuTitleForUndoActionName(
            &self,
            action_name: &NSString,
        ) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUndoManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagercheckpointnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerCheckpointNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerwillundochangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerWillUndoChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerwillredochangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerWillRedoChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerdidundochangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerDidUndoChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerdidredochangenotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerDidRedoChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerdidopenundogroupnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerDidOpenUndoGroupNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerwillcloseundogroupnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerWillCloseUndoGroupNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsundomanagerdidcloseundogroupnotification?language=objc)
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSUndoManagerDidCloseUndoGroupNotification: &'static NSNotificationName;
}
