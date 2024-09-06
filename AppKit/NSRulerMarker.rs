//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRulerMarker;

    unsafe impl ClassType for NSRulerMarker {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for NSRulerMarker {}

unsafe impl NSCopying for NSRulerMarker {}

unsafe impl CopyingHelper for NSRulerMarker {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSRulerMarker {}

extern_methods!(
    unsafe impl NSRulerMarker {
        #[cfg(all(
            feature = "NSImage",
            feature = "NSResponder",
            feature = "NSRulerView",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Init initWithRulerView:markerLocation:image:imageOrigin:)]
        pub unsafe fn initWithRulerView_markerLocation_image_imageOrigin(
            this: Allocated<Self>,
            ruler: &NSRulerView,
            location: CGFloat,
            image: &NSImage,
            image_origin: NSPoint,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSRulerView", feature = "NSView"))]
        #[method_id(@__retain_semantics Other ruler)]
        pub unsafe fn ruler(&self, mtm: MainThreadMarker) -> Option<Retained<NSRulerView>>;

        #[method(markerLocation)]
        pub unsafe fn markerLocation(&self) -> CGFloat;

        #[method(setMarkerLocation:)]
        pub unsafe fn setMarkerLocation(&self, marker_location: CGFloat);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Retained<NSImage>;

        #[cfg(feature = "NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: &NSImage);

        #[method(imageOrigin)]
        pub unsafe fn imageOrigin(&self) -> NSPoint;

        #[method(setImageOrigin:)]
        pub unsafe fn setImageOrigin(&self, image_origin: NSPoint);

        #[method(isMovable)]
        pub unsafe fn isMovable(&self) -> bool;

        #[method(setMovable:)]
        pub unsafe fn setMovable(&self, movable: bool);

        #[method(isRemovable)]
        pub unsafe fn isRemovable(&self) -> bool;

        #[method(setRemovable:)]
        pub unsafe fn setRemovable(&self, removable: bool);

        #[method(isDragging)]
        pub unsafe fn isDragging(&self) -> bool;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Retained<ProtocolObject<dyn NSCopying>>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(
            &self,
            represented_object: Option<&ProtocolObject<dyn NSCopying>>,
        );

        #[method(imageRectInRuler)]
        pub unsafe fn imageRectInRuler(&self) -> NSRect;

        #[method(thicknessRequiredInRuler)]
        pub unsafe fn thicknessRequiredInRuler(&self) -> CGFloat;

        #[method(drawRect:)]
        pub unsafe fn drawRect(&self, rect: NSRect);

        #[cfg(feature = "NSEvent")]
        #[method(trackMouse:adding:)]
        pub unsafe fn trackMouse_adding(&self, mouse_down_event: &NSEvent, is_adding: bool)
            -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSRulerMarker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
