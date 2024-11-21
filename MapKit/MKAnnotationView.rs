//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static MKAnnotationCalloutInfoDidChangeNotification: &'static NSString;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKAnnotationViewDragState(pub NSUInteger);
impl MKAnnotationViewDragState {
    #[doc(alias = "MKAnnotationViewDragStateNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MKAnnotationViewDragStateStarting")]
    pub const Starting: Self = Self(1);
    #[doc(alias = "MKAnnotationViewDragStateDragging")]
    pub const Dragging: Self = Self(2);
    #[doc(alias = "MKAnnotationViewDragStateCanceling")]
    pub const Canceling: Self = Self(3);
    #[doc(alias = "MKAnnotationViewDragStateEnding")]
    pub const Ending: Self = Self(4);
}

unsafe impl Encode for MKAnnotationViewDragState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MKAnnotationViewDragState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type MKFeatureDisplayPriority = c_float;

pub static MKFeatureDisplayPriorityRequired: MKFeatureDisplayPriority = 1000 as _;

pub static MKFeatureDisplayPriorityDefaultHigh: MKFeatureDisplayPriority = 750 as _;

pub static MKFeatureDisplayPriorityDefaultLow: MKFeatureDisplayPriority = 250 as _;

// NS_TYPED_EXTENSIBLE_ENUM
pub type MKAnnotationViewZPriority = c_float;

pub static MKAnnotationViewZPriorityMax: MKAnnotationViewZPriority = 1000 as _;

pub static MKAnnotationViewZPriorityDefaultSelected: MKAnnotationViewZPriority = 1000 as _;

pub static MKAnnotationViewZPriorityDefaultUnselected: MKAnnotationViewZPriority = 500 as _;

pub static MKAnnotationViewZPriorityMin: MKAnnotationViewZPriority = 0 as _;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKAnnotationViewCollisionMode(pub NSInteger);
impl MKAnnotationViewCollisionMode {
    #[doc(alias = "MKAnnotationViewCollisionModeRectangle")]
    pub const Rectangle: Self = Self(0);
    #[doc(alias = "MKAnnotationViewCollisionModeCircle")]
    pub const Circle: Self = Self(1);
    #[doc(alias = "MKAnnotationViewCollisionModeNone")]
    pub const None: Self = Self(2);
}

unsafe impl Encode for MKAnnotationViewCollisionMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKAnnotationViewCollisionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct MKAnnotationView;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for MKAnnotationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for MKAnnotationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for MKAnnotationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for MKAnnotationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for MKAnnotationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for MKAnnotationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for MKAnnotationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for MKAnnotationView {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKAnnotationView {
        #[cfg(feature = "MKAnnotation")]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Allocated<Self>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other reuseIdentifier)]
        pub unsafe fn reuseIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(prepareForReuse)]
        pub unsafe fn prepareForReuse(&self);

        #[method(prepareForDisplay)]
        pub unsafe fn prepareForDisplay(&self);

        #[cfg(feature = "MKAnnotation")]
        #[method_id(@__retain_semantics Other annotation)]
        pub unsafe fn annotation(&self) -> Option<Retained<ProtocolObject<dyn MKAnnotation>>>;

        #[cfg(feature = "MKAnnotation")]
        #[method(setAnnotation:)]
        pub unsafe fn setAnnotation(&self, annotation: Option<&ProtocolObject<dyn MKAnnotation>>);

        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(centerOffset)]
        pub unsafe fn centerOffset(&self) -> CGPoint;

        #[method(setCenterOffset:)]
        pub unsafe fn setCenterOffset(&self, center_offset: CGPoint);

        #[method(accessoryOffset)]
        pub unsafe fn accessoryOffset(&self) -> CGPoint;

        #[method(setAccessoryOffset:)]
        pub unsafe fn setAccessoryOffset(&self, accessory_offset: CGPoint);

        #[method(calloutOffset)]
        pub unsafe fn calloutOffset(&self) -> CGPoint;

        #[method(setCalloutOffset:)]
        pub unsafe fn setCalloutOffset(&self, callout_offset: CGPoint);

        #[method(leftCalloutOffset)]
        pub unsafe fn leftCalloutOffset(&self) -> CGPoint;

        #[method(setLeftCalloutOffset:)]
        pub unsafe fn setLeftCalloutOffset(&self, left_callout_offset: CGPoint);

        #[method(rightCalloutOffset)]
        pub unsafe fn rightCalloutOffset(&self) -> CGPoint;

        #[method(setRightCalloutOffset:)]
        pub unsafe fn setRightCalloutOffset(&self, right_callout_offset: CGPoint);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(setSelected:animated:)]
        pub unsafe fn setSelected_animated(&self, selected: bool, animated: bool);

        #[method(canShowCallout)]
        pub unsafe fn canShowCallout(&self) -> bool;

        #[method(setCanShowCallout:)]
        pub unsafe fn setCanShowCallout(&self, can_show_callout: bool);

        #[method_id(@__retain_semantics Other leftCalloutAccessoryView)]
        pub unsafe fn leftCalloutAccessoryView(&self) -> Option<Retained<NSView>>;

        #[method(setLeftCalloutAccessoryView:)]
        pub unsafe fn setLeftCalloutAccessoryView(
            &self,
            left_callout_accessory_view: Option<&NSView>,
        );

        #[method_id(@__retain_semantics Other rightCalloutAccessoryView)]
        pub unsafe fn rightCalloutAccessoryView(&self) -> Option<Retained<NSView>>;

        #[method(setRightCalloutAccessoryView:)]
        pub unsafe fn setRightCalloutAccessoryView(
            &self,
            right_callout_accessory_view: Option<&NSView>,
        );

        #[method_id(@__retain_semantics Other detailCalloutAccessoryView)]
        pub unsafe fn detailCalloutAccessoryView(&self) -> Option<Retained<NSView>>;

        #[method(setDetailCalloutAccessoryView:)]
        pub unsafe fn setDetailCalloutAccessoryView(
            &self,
            detail_callout_accessory_view: Option<&NSView>,
        );

        #[method(isDraggable)]
        pub unsafe fn isDraggable(&self) -> bool;

        #[method(setDraggable:)]
        pub unsafe fn setDraggable(&self, draggable: bool);

        #[method(dragState)]
        pub unsafe fn dragState(&self) -> MKAnnotationViewDragState;

        #[method(setDragState:)]
        pub unsafe fn setDragState(&self, drag_state: MKAnnotationViewDragState);

        #[method(setDragState:animated:)]
        pub unsafe fn setDragState_animated(
            &self,
            new_drag_state: MKAnnotationViewDragState,
            animated: bool,
        );

        #[method_id(@__retain_semantics Other clusteringIdentifier)]
        pub unsafe fn clusteringIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setClusteringIdentifier:)]
        pub unsafe fn setClusteringIdentifier(&self, clustering_identifier: Option<&NSString>);

        #[method_id(@__retain_semantics Other clusterAnnotationView)]
        pub unsafe fn clusterAnnotationView(&self) -> Option<Retained<MKAnnotationView>>;

        #[method(displayPriority)]
        pub unsafe fn displayPriority(&self) -> MKFeatureDisplayPriority;

        #[method(setDisplayPriority:)]
        pub unsafe fn setDisplayPriority(&self, display_priority: MKFeatureDisplayPriority);

        #[method(zPriority)]
        pub unsafe fn zPriority(&self) -> MKAnnotationViewZPriority;

        #[method(setZPriority:)]
        pub unsafe fn setZPriority(&self, z_priority: MKAnnotationViewZPriority);

        #[method(selectedZPriority)]
        pub unsafe fn selectedZPriority(&self) -> MKAnnotationViewZPriority;

        #[method(setSelectedZPriority:)]
        pub unsafe fn setSelectedZPriority(&self, selected_z_priority: MKAnnotationViewZPriority);

        #[method(collisionMode)]
        pub unsafe fn collisionMode(&self) -> MKAnnotationViewCollisionMode;

        #[method(setCollisionMode:)]
        pub unsafe fn setCollisionMode(&self, collision_mode: MKAnnotationViewCollisionMode);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKAnnotationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKAnnotationView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKAnnotationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
