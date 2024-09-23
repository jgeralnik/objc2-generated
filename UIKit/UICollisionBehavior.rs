//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollisionBehaviorMode(pub NSUInteger);
bitflags::bitflags! {
    impl UICollisionBehaviorMode: NSUInteger {
        #[doc(alias = "UICollisionBehaviorModeItems")]
        const Items = 1<<0;
        #[doc(alias = "UICollisionBehaviorModeBoundaries")]
        const Boundaries = 1<<1;
        #[doc(alias = "UICollisionBehaviorModeEverything")]
        const Everything = NSUIntegerMax as _;
    }
}

unsafe impl Encode for UICollisionBehaviorMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UICollisionBehaviorMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UICollisionBehaviorDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "UIDynamicBehavior")]
        #[optional]
        #[method(collisionBehavior:beganContactForItem:withItem:atPoint:)]
        unsafe fn collisionBehavior_beganContactForItem_withItem_atPoint(
            &self,
            behavior: &UICollisionBehavior,
            item1: &ProtocolObject<dyn UIDynamicItem>,
            item2: &ProtocolObject<dyn UIDynamicItem>,
            p: CGPoint,
        );

        #[cfg(feature = "UIDynamicBehavior")]
        #[optional]
        #[method(collisionBehavior:endedContactForItem:withItem:)]
        unsafe fn collisionBehavior_endedContactForItem_withItem(
            &self,
            behavior: &UICollisionBehavior,
            item1: &ProtocolObject<dyn UIDynamicItem>,
            item2: &ProtocolObject<dyn UIDynamicItem>,
        );

        #[cfg(feature = "UIDynamicBehavior")]
        #[optional]
        #[method(collisionBehavior:beganContactForItem:withBoundaryIdentifier:atPoint:)]
        unsafe fn collisionBehavior_beganContactForItem_withBoundaryIdentifier_atPoint(
            &self,
            behavior: &UICollisionBehavior,
            item: &ProtocolObject<dyn UIDynamicItem>,
            identifier: Option<&ProtocolObject<dyn NSCopying>>,
            p: CGPoint,
        );

        #[cfg(feature = "UIDynamicBehavior")]
        #[optional]
        #[method(collisionBehavior:endedContactForItem:withBoundaryIdentifier:)]
        unsafe fn collisionBehavior_endedContactForItem_withBoundaryIdentifier(
            &self,
            behavior: &UICollisionBehavior,
            item: &ProtocolObject<dyn UIDynamicItem>,
            identifier: Option<&ProtocolObject<dyn NSCopying>>,
        );
    }

    unsafe impl ProtocolType for dyn UICollisionBehaviorDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UICollisionBehavior;

    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl ClassType for UICollisionBehavior {
        #[inherits(NSObject)]
        type Super = UIDynamicBehavior;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(feature = "UIDynamicBehavior")]
unsafe impl NSObjectProtocol for UICollisionBehavior {}

extern_methods!(
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UICollisionBehavior {
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Allocated<Self>,
            items: &NSArray<ProtocolObject<dyn UIDynamicItem>>,
        ) -> Retained<Self>;

        #[method(addItem:)]
        pub unsafe fn addItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[method(collisionMode)]
        pub unsafe fn collisionMode(&self) -> UICollisionBehaviorMode;

        #[method(setCollisionMode:)]
        pub unsafe fn setCollisionMode(&self, collision_mode: UICollisionBehaviorMode);

        #[method(translatesReferenceBoundsIntoBoundary)]
        pub unsafe fn translatesReferenceBoundsIntoBoundary(&self) -> bool;

        #[method(setTranslatesReferenceBoundsIntoBoundary:)]
        pub unsafe fn setTranslatesReferenceBoundsIntoBoundary(
            &self,
            translates_reference_bounds_into_boundary: bool,
        );

        #[cfg(feature = "UIGeometry")]
        #[method(setTranslatesReferenceBoundsIntoBoundaryWithInsets:)]
        pub unsafe fn setTranslatesReferenceBoundsIntoBoundaryWithInsets(
            &self,
            insets: UIEdgeInsets,
        );

        #[cfg(feature = "UIBezierPath")]
        #[method(addBoundaryWithIdentifier:forPath:)]
        pub unsafe fn addBoundaryWithIdentifier_forPath(
            &self,
            identifier: &ProtocolObject<dyn NSCopying>,
            bezier_path: &UIBezierPath,
        );

        #[method(addBoundaryWithIdentifier:fromPoint:toPoint:)]
        pub unsafe fn addBoundaryWithIdentifier_fromPoint_toPoint(
            &self,
            identifier: &ProtocolObject<dyn NSCopying>,
            p1: CGPoint,
            p2: CGPoint,
        );

        #[cfg(feature = "UIBezierPath")]
        #[method_id(@__retain_semantics Other boundaryWithIdentifier:)]
        pub unsafe fn boundaryWithIdentifier(
            &self,
            identifier: &ProtocolObject<dyn NSCopying>,
        ) -> Option<Retained<UIBezierPath>>;

        #[method(removeBoundaryWithIdentifier:)]
        pub unsafe fn removeBoundaryWithIdentifier(
            &self,
            identifier: &ProtocolObject<dyn NSCopying>,
        );

        #[method_id(@__retain_semantics Other boundaryIdentifiers)]
        pub unsafe fn boundaryIdentifiers(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn NSCopying>>>>;

        #[method(removeAllBoundaries)]
        pub unsafe fn removeAllBoundaries(&self);

        #[method_id(@__retain_semantics Other collisionDelegate)]
        pub unsafe fn collisionDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UICollisionBehaviorDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setCollisionDelegate:)]
        pub unsafe fn setCollisionDelegate(
            &self,
            collision_delegate: Option<&ProtocolObject<dyn UICollisionBehaviorDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UICollisionBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
