//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CALayerContentsGravity = NSString;
);

typed_enum!(
    pub type CALayerContentsFormat = NSString;
);

typed_enum!(
    pub type CALayerContentsFilter = NSString;
);

typed_enum!(
    pub type CALayerCornerCurve = NSString;
);

ns_options!(
    #[underlying(c_uint)]
    pub enum CAAutoresizingMask {
        kCALayerNotSizable = 0,
        kCALayerMinXMargin = 1 << 0,
        kCALayerWidthSizable = 1 << 1,
        kCALayerMaxXMargin = 1 << 2,
        kCALayerMinYMargin = 1 << 3,
        kCALayerHeightSizable = 1 << 4,
        kCALayerMaxYMargin = 1 << 5,
    }
);

ns_options!(
    #[underlying(c_uint)]
    pub enum CAEdgeAntialiasingMask {
        kCALayerLeftEdge = 1 << 0,
        kCALayerRightEdge = 1 << 1,
        kCALayerBottomEdge = 1 << 2,
        kCALayerTopEdge = 1 << 3,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum CACornerMask {
        kCALayerMinXMinYCorner = 1 << 0,
        kCALayerMaxXMinYCorner = 1 << 1,
        kCALayerMinXMaxYCorner = 1 << 2,
        kCALayerMaxXMaxYCorner = 1 << 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CALayer;

    unsafe impl ClassType for CALayer {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "QuartzCore_CALayer")]
    unsafe impl CALayer {
        #[method_id(@__retain_semantics Other layer)]
        pub fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other presentationLayer)]
        pub unsafe fn presentationLayer(&self) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other modelLayer)]
        pub unsafe fn modelLayer(&self) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Id<Object, Shared>>;

        #[method(needsDisplayForKey:)]
        pub unsafe fn needsDisplayForKey(key: &NSString) -> bool;

        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[method(bounds)]
        pub fn bounds(&self) -> CGRect;

        #[method(setBounds:)]
        pub fn setBounds(&self, bounds: CGRect);

        #[method(position)]
        pub fn position(&self) -> CGPoint;

        #[method(setPosition:)]
        pub fn setPosition(&self, position: CGPoint);

        #[method(zPosition)]
        pub fn zPosition(&self) -> CGFloat;

        #[method(setZPosition:)]
        pub fn setZPosition(&self, zPosition: CGFloat);

        #[method(anchorPoint)]
        pub fn anchorPoint(&self) -> CGPoint;

        #[method(setAnchorPoint:)]
        pub fn setAnchorPoint(&self, anchorPoint: CGPoint);

        #[method(anchorPointZ)]
        pub fn anchorPointZ(&self) -> CGFloat;

        #[method(setAnchorPointZ:)]
        pub fn setAnchorPointZ(&self, anchorPointZ: CGFloat);

        #[method(transform)]
        pub fn transform(&self) -> CATransform3D;

        #[method(setTransform:)]
        pub fn setTransform(&self, transform: CATransform3D);

        #[method(frame)]
        pub fn frame(&self) -> CGRect;

        #[method(setFrame:)]
        pub fn setFrame(&self, frame: CGRect);

        #[method(isHidden)]
        pub fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub fn setHidden(&self, hidden: bool);

        #[method(isDoubleSided)]
        pub fn isDoubleSided(&self) -> bool;

        #[method(setDoubleSided:)]
        pub fn setDoubleSided(&self, doubleSided: bool);

        #[method(isGeometryFlipped)]
        pub fn isGeometryFlipped(&self) -> bool;

        #[method(setGeometryFlipped:)]
        pub fn setGeometryFlipped(&self, geometryFlipped: bool);

        #[method(contentsAreFlipped)]
        pub fn contentsAreFlipped(&self) -> bool;

        #[method_id(@__retain_semantics Other superlayer)]
        pub fn superlayer(&self) -> Option<Id<CALayer, Shared>>;

        #[method(removeFromSuperlayer)]
        pub fn removeFromSuperlayer(&self);

        #[method_id(@__retain_semantics Other sublayers)]
        pub unsafe fn sublayers(&self) -> Option<Id<NSArray<CALayer>, Shared>>;

        #[method(setSublayers:)]
        pub unsafe fn setSublayers(&self, sublayers: Option<&NSArray<CALayer>>);

        #[method(addSublayer:)]
        pub fn addSublayer(&self, layer: &CALayer);

        #[method(insertSublayer:atIndex:)]
        pub fn insertSublayer_atIndex(&self, layer: &CALayer, idx: c_uint);

        #[method(insertSublayer:below:)]
        pub fn insertSublayer_below(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(insertSublayer:above:)]
        pub fn insertSublayer_above(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(replaceSublayer:with:)]
        pub unsafe fn replaceSublayer_with(&self, oldLayer: &CALayer, newLayer: &CALayer);

        #[method(sublayerTransform)]
        pub fn sublayerTransform(&self) -> CATransform3D;

        #[method(setSublayerTransform:)]
        pub fn setSublayerTransform(&self, sublayerTransform: CATransform3D);

        #[method_id(@__retain_semantics Other mask)]
        pub fn mask(&self) -> Option<Id<CALayer, Shared>>;

        #[method(setMask:)]
        pub unsafe fn setMask(&self, mask: Option<&CALayer>);

        #[method(masksToBounds)]
        pub fn masksToBounds(&self) -> bool;

        #[method(setMasksToBounds:)]
        pub fn setMasksToBounds(&self, masksToBounds: bool);

        #[method(convertPoint:fromLayer:)]
        pub fn convertPoint_fromLayer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[method(convertPoint:toLayer:)]
        pub fn convertPoint_toLayer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[method(convertRect:fromLayer:)]
        pub fn convertRect_fromLayer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[method(convertRect:toLayer:)]
        pub fn convertRect_toLayer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[method(convertTime:fromLayer:)]
        pub fn convertTime_fromLayer(
            &self,
            t: CFTimeInterval,
            l: Option<&CALayer>,
        ) -> CFTimeInterval;

        #[method(convertTime:toLayer:)]
        pub fn convertTime_toLayer(&self, t: CFTimeInterval, l: Option<&CALayer>)
            -> CFTimeInterval;

        #[method_id(@__retain_semantics Other hitTest:)]
        pub fn hitTest(&self, p: CGPoint) -> Option<Id<CALayer, Shared>>;

        #[method(containsPoint:)]
        pub fn containsPoint(&self, p: CGPoint) -> bool;

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<Object, Shared>>;

        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&Object>);

        #[method(contentsRect)]
        pub fn contentsRect(&self) -> CGRect;

        #[method(setContentsRect:)]
        pub fn setContentsRect(&self, contentsRect: CGRect);

        #[cfg(feature = "QuartzCore_CALayerContentsGravity")]
        #[method_id(@__retain_semantics Other contentsGravity)]
        pub fn contentsGravity(&self) -> Id<CALayerContentsGravity, Shared>;

        #[cfg(feature = "QuartzCore_CALayerContentsGravity")]
        #[method(setContentsGravity:)]
        pub fn setContentsGravity(&self, contentsGravity: &CALayerContentsGravity);

        #[method(contentsScale)]
        pub fn contentsScale(&self) -> CGFloat;

        #[method(setContentsScale:)]
        pub fn setContentsScale(&self, contentsScale: CGFloat);

        #[method(contentsCenter)]
        pub fn contentsCenter(&self) -> CGRect;

        #[method(setContentsCenter:)]
        pub fn setContentsCenter(&self, contentsCenter: CGRect);

        #[cfg(feature = "QuartzCore_CALayerContentsFormat")]
        #[method_id(@__retain_semantics Other contentsFormat)]
        pub fn contentsFormat(&self) -> Id<CALayerContentsFormat, Shared>;

        #[cfg(feature = "QuartzCore_CALayerContentsFormat")]
        #[method(setContentsFormat:)]
        pub fn setContentsFormat(&self, contentsFormat: &CALayerContentsFormat);

        #[cfg(feature = "QuartzCore_CALayerContentsFilter")]
        #[method_id(@__retain_semantics Other minificationFilter)]
        pub fn minificationFilter(&self) -> Id<CALayerContentsFilter, Shared>;

        #[cfg(feature = "QuartzCore_CALayerContentsFilter")]
        #[method(setMinificationFilter:)]
        pub fn setMinificationFilter(&self, minificationFilter: &CALayerContentsFilter);

        #[cfg(feature = "QuartzCore_CALayerContentsFilter")]
        #[method_id(@__retain_semantics Other magnificationFilter)]
        pub fn magnificationFilter(&self) -> Id<CALayerContentsFilter, Shared>;

        #[cfg(feature = "QuartzCore_CALayerContentsFilter")]
        #[method(setMagnificationFilter:)]
        pub fn setMagnificationFilter(&self, magnificationFilter: &CALayerContentsFilter);

        #[method(minificationFilterBias)]
        pub fn minificationFilterBias(&self) -> c_float;

        #[method(setMinificationFilterBias:)]
        pub fn setMinificationFilterBias(&self, minificationFilterBias: c_float);

        #[method(isOpaque)]
        pub fn isOpaque(&self) -> bool;

        #[method(setOpaque:)]
        pub fn setOpaque(&self, opaque: bool);

        #[method(display)]
        pub fn display(&self);

        #[method(setNeedsDisplay)]
        pub fn setNeedsDisplay(&self);

        #[method(setNeedsDisplayInRect:)]
        pub fn setNeedsDisplayInRect(&self, r: CGRect);

        #[method(needsDisplay)]
        pub fn needsDisplay(&self) -> bool;

        #[method(displayIfNeeded)]
        pub fn displayIfNeeded(&self);

        #[method(needsDisplayOnBoundsChange)]
        pub fn needsDisplayOnBoundsChange(&self) -> bool;

        #[method(setNeedsDisplayOnBoundsChange:)]
        pub fn setNeedsDisplayOnBoundsChange(&self, needsDisplayOnBoundsChange: bool);

        #[method(drawsAsynchronously)]
        pub fn drawsAsynchronously(&self) -> bool;

        #[method(setDrawsAsynchronously:)]
        pub fn setDrawsAsynchronously(&self, drawsAsynchronously: bool);

        #[method(edgeAntialiasingMask)]
        pub fn edgeAntialiasingMask(&self) -> CAEdgeAntialiasingMask;

        #[method(setEdgeAntialiasingMask:)]
        pub fn setEdgeAntialiasingMask(&self, edgeAntialiasingMask: CAEdgeAntialiasingMask);

        #[method(allowsEdgeAntialiasing)]
        pub fn allowsEdgeAntialiasing(&self) -> bool;

        #[method(setAllowsEdgeAntialiasing:)]
        pub fn setAllowsEdgeAntialiasing(&self, allowsEdgeAntialiasing: bool);

        #[method(cornerRadius)]
        pub fn cornerRadius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub fn setCornerRadius(&self, cornerRadius: CGFloat);

        #[method(maskedCorners)]
        pub fn maskedCorners(&self) -> CACornerMask;

        #[method(setMaskedCorners:)]
        pub fn setMaskedCorners(&self, maskedCorners: CACornerMask);

        #[cfg(feature = "QuartzCore_CALayerCornerCurve")]
        #[method_id(@__retain_semantics Other cornerCurve)]
        pub fn cornerCurve(&self) -> Id<CALayerCornerCurve, Shared>;

        #[cfg(feature = "QuartzCore_CALayerCornerCurve")]
        #[method(setCornerCurve:)]
        pub fn setCornerCurve(&self, cornerCurve: &CALayerCornerCurve);

        #[cfg(feature = "QuartzCore_CALayerCornerCurve")]
        #[method(cornerCurveExpansionFactor:)]
        pub fn cornerCurveExpansionFactor(curve: &CALayerCornerCurve) -> CGFloat;

        #[method(borderWidth)]
        pub fn borderWidth(&self) -> CGFloat;

        #[method(setBorderWidth:)]
        pub fn setBorderWidth(&self, borderWidth: CGFloat);

        #[method(opacity)]
        pub fn opacity(&self) -> c_float;

        #[method(setOpacity:)]
        pub fn setOpacity(&self, opacity: c_float);

        #[method(allowsGroupOpacity)]
        pub fn allowsGroupOpacity(&self) -> bool;

        #[method(setAllowsGroupOpacity:)]
        pub fn setAllowsGroupOpacity(&self, allowsGroupOpacity: bool);

        #[method_id(@__retain_semantics Other compositingFilter)]
        pub unsafe fn compositingFilter(&self) -> Option<Id<Object, Shared>>;

        #[method(setCompositingFilter:)]
        pub unsafe fn setCompositingFilter(&self, compositingFilter: Option<&Object>);

        #[method_id(@__retain_semantics Other filters)]
        pub unsafe fn filters(&self) -> Option<Id<NSArray, Shared>>;

        #[method(setFilters:)]
        pub unsafe fn setFilters(&self, filters: Option<&NSArray>);

        #[method_id(@__retain_semantics Other backgroundFilters)]
        pub unsafe fn backgroundFilters(&self) -> Option<Id<NSArray, Shared>>;

        #[method(setBackgroundFilters:)]
        pub unsafe fn setBackgroundFilters(&self, backgroundFilters: Option<&NSArray>);

        #[method(shouldRasterize)]
        pub fn shouldRasterize(&self) -> bool;

        #[method(setShouldRasterize:)]
        pub fn setShouldRasterize(&self, shouldRasterize: bool);

        #[method(rasterizationScale)]
        pub fn rasterizationScale(&self) -> CGFloat;

        #[method(setRasterizationScale:)]
        pub fn setRasterizationScale(&self, rasterizationScale: CGFloat);

        #[method(shadowOpacity)]
        pub fn shadowOpacity(&self) -> c_float;

        #[method(setShadowOpacity:)]
        pub fn setShadowOpacity(&self, shadowOpacity: c_float);

        #[method(shadowOffset)]
        pub fn shadowOffset(&self) -> CGSize;

        #[method(setShadowOffset:)]
        pub fn setShadowOffset(&self, shadowOffset: CGSize);

        #[method(shadowRadius)]
        pub fn shadowRadius(&self) -> CGFloat;

        #[method(setShadowRadius:)]
        pub fn setShadowRadius(&self, shadowRadius: CGFloat);

        #[method(autoresizingMask)]
        pub fn autoresizingMask(&self) -> CAAutoresizingMask;

        #[method(setAutoresizingMask:)]
        pub fn setAutoresizingMask(&self, autoresizingMask: CAAutoresizingMask);

        #[cfg(feature = "QuartzCore_CALayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub fn layoutManager(&self) -> Option<Id<CALayoutManager, Shared>>;

        #[cfg(feature = "QuartzCore_CALayoutManager")]
        #[method(setLayoutManager:)]
        pub fn setLayoutManager(&self, layoutManager: Option<&CALayoutManager>);

        #[method(preferredFrameSize)]
        pub fn preferredFrameSize(&self) -> CGSize;

        #[method(setNeedsLayout)]
        pub fn setNeedsLayout(&self);

        #[method(needsLayout)]
        pub fn needsLayout(&self) -> bool;

        #[method(layoutIfNeeded)]
        pub fn layoutIfNeeded(&self);

        #[method(layoutSublayers)]
        pub fn layoutSublayers(&self);

        #[method(resizeSublayersWithOldSize:)]
        pub fn resizeSublayersWithOldSize(&self, size: CGSize);

        #[method(resizeWithOldSuperlayerSize:)]
        pub fn resizeWithOldSuperlayerSize(&self, size: CGSize);

        #[cfg(feature = "QuartzCore_CAAction")]
        #[method_id(@__retain_semantics Other defaultActionForKey:)]
        pub fn defaultActionForKey(event: &NSString) -> Option<Id<CAAction, Shared>>;

        #[cfg(feature = "QuartzCore_CAAction")]
        #[method_id(@__retain_semantics Other actionForKey:)]
        pub fn actionForKey(&self, event: &NSString) -> Option<Id<CAAction, Shared>>;

        #[cfg(feature = "QuartzCore_CAAction")]
        #[method_id(@__retain_semantics Other actions)]
        pub fn actions(&self) -> Option<Id<NSDictionary<NSString, CAAction>, Shared>>;

        #[cfg(feature = "QuartzCore_CAAction")]
        #[method(setActions:)]
        pub fn setActions(&self, actions: Option<&NSDictionary<NSString, CAAction>>);

        #[cfg(feature = "QuartzCore_CAAnimation")]
        #[method(addAnimation:forKey:)]
        pub fn addAnimation_forKey(&self, anim: &CAAnimation, key: Option<&NSString>);

        #[method(removeAllAnimations)]
        pub fn removeAllAnimations(&self);

        #[method(removeAnimationForKey:)]
        pub fn removeAnimationForKey(&self, key: &NSString);

        #[method_id(@__retain_semantics Other animationKeys)]
        pub fn animationKeys(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[cfg(feature = "QuartzCore_CAAnimation")]
        #[method_id(@__retain_semantics Other animationForKey:)]
        pub unsafe fn animationForKey(&self, key: &NSString) -> Option<Id<CAAnimation, Shared>>;

        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "QuartzCore_CALayerDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub fn delegate(&self) -> Option<Id<CALayerDelegate, Shared>>;

        #[cfg(feature = "QuartzCore_CALayerDelegate")]
        #[method(setDelegate:)]
        pub fn setDelegate(&self, delegate: Option<&CALayerDelegate>);

        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<NSDictionary, Shared>>;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: Option<&NSDictionary>);
    }
);

extern_protocol!(
    pub struct CALayoutManager;

    unsafe impl ProtocolType for CALayoutManager {
        #[optional]
        #[method(preferredSizeOfLayer:)]
        pub unsafe fn preferredSizeOfLayer(&self, layer: &CALayer) -> CGSize;

        #[optional]
        #[method(invalidateLayoutOfLayer:)]
        pub unsafe fn invalidateLayoutOfLayer(&self, layer: &CALayer);

        #[optional]
        #[method(layoutSublayersOfLayer:)]
        pub unsafe fn layoutSublayersOfLayer(&self, layer: &CALayer);
    }
);

extern_protocol!(
    pub struct CAAction;

    unsafe impl ProtocolType for CAAction {
        #[method(runActionForKey:object:arguments:)]
        pub unsafe fn runActionForKey_object_arguments(
            &self,
            event: &NSString,
            anObject: &Object,
            dict: Option<&NSDictionary>,
        );
    }
);

extern_methods!(
    /// CAActionAdditions
    #[cfg(feature = "QuartzCore_NSNull")]
    unsafe impl NSNull {}
);

extern_protocol!(
    pub struct CALayerDelegate;

    unsafe impl ProtocolType for CALayerDelegate {
        #[optional]
        #[method(displayLayer:)]
        pub unsafe fn displayLayer(&self, layer: &CALayer);

        #[optional]
        #[method(layerWillDraw:)]
        pub unsafe fn layerWillDraw(&self, layer: &CALayer);

        #[optional]
        #[method(layoutSublayersOfLayer:)]
        pub unsafe fn layoutSublayersOfLayer(&self, layer: &CALayer);

        #[optional]
        #[method_id(@__retain_semantics Other actionForLayer:forKey:)]
        pub unsafe fn actionForLayer_forKey(
            &self,
            layer: &CALayer,
            event: &NSString,
        ) -> Option<Id<CAAction, Shared>>;
    }
);

extern_static!(kCAGravityCenter: &'static CALayerContentsGravity);

extern_static!(kCAGravityTop: &'static CALayerContentsGravity);

extern_static!(kCAGravityBottom: &'static CALayerContentsGravity);

extern_static!(kCAGravityLeft: &'static CALayerContentsGravity);

extern_static!(kCAGravityRight: &'static CALayerContentsGravity);

extern_static!(kCAGravityTopLeft: &'static CALayerContentsGravity);

extern_static!(kCAGravityTopRight: &'static CALayerContentsGravity);

extern_static!(kCAGravityBottomLeft: &'static CALayerContentsGravity);

extern_static!(kCAGravityBottomRight: &'static CALayerContentsGravity);

extern_static!(kCAGravityResize: &'static CALayerContentsGravity);

extern_static!(kCAGravityResizeAspect: &'static CALayerContentsGravity);

extern_static!(kCAGravityResizeAspectFill: &'static CALayerContentsGravity);

extern_static!(kCAContentsFormatRGBA8Uint: &'static CALayerContentsFormat);

extern_static!(kCAContentsFormatRGBA16Float: &'static CALayerContentsFormat);

extern_static!(kCAContentsFormatGray8Uint: &'static CALayerContentsFormat);

extern_static!(kCAFilterNearest: &'static CALayerContentsFilter);

extern_static!(kCAFilterLinear: &'static CALayerContentsFilter);

extern_static!(kCAFilterTrilinear: &'static CALayerContentsFilter);

extern_static!(kCACornerCurveCircular: &'static CALayerCornerCurve);

extern_static!(kCACornerCurveContinuous: &'static CALayerCornerCurve);

extern_static!(kCAOnOrderIn: &'static NSString);

extern_static!(kCAOnOrderOut: &'static NSString);

extern_static!(kCATransition: &'static NSString);
