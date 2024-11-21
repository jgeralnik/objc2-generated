//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLineCapStyle(pub NSUInteger);
impl NSLineCapStyle {
    #[doc(alias = "NSLineCapStyleButt")]
    pub const Butt: Self = Self(0);
    #[doc(alias = "NSLineCapStyleRound")]
    pub const Round: Self = Self(1);
    #[doc(alias = "NSLineCapStyleSquare")]
    pub const Square: Self = Self(2);
}

unsafe impl Encode for NSLineCapStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLineCapStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLineJoinStyle(pub NSUInteger);
impl NSLineJoinStyle {
    #[doc(alias = "NSLineJoinStyleMiter")]
    pub const Miter: Self = Self(0);
    #[doc(alias = "NSLineJoinStyleRound")]
    pub const Round: Self = Self(1);
    #[doc(alias = "NSLineJoinStyleBevel")]
    pub const Bevel: Self = Self(2);
}

unsafe impl Encode for NSLineJoinStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLineJoinStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWindingRule(pub NSUInteger);
impl NSWindingRule {
    #[doc(alias = "NSWindingRuleNonZero")]
    pub const NonZero: Self = Self(0);
    #[doc(alias = "NSWindingRuleEvenOdd")]
    pub const EvenOdd: Self = Self(1);
}

unsafe impl Encode for NSWindingRule {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSWindingRule {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBezierPathElement(pub NSUInteger);
impl NSBezierPathElement {
    #[doc(alias = "NSBezierPathElementMoveTo")]
    pub const MoveTo: Self = Self(0);
    #[doc(alias = "NSBezierPathElementLineTo")]
    pub const LineTo: Self = Self(1);
    #[doc(alias = "NSBezierPathElementCubicCurveTo")]
    pub const CubicCurveTo: Self = Self(2);
    #[doc(alias = "NSBezierPathElementClosePath")]
    pub const ClosePath: Self = Self(3);
    #[doc(alias = "NSBezierPathElementQuadraticCurveTo")]
    pub const QuadraticCurveTo: Self = Self(4);
    #[deprecated]
    #[doc(alias = "NSBezierPathElementCurveTo")]
    pub const CurveTo: Self = Self(NSBezierPathElement::CubicCurveTo.0);
}

unsafe impl Encode for NSBezierPathElement {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBezierPathElement {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBezierPath;
);

unsafe impl NSCoding for NSBezierPath {}

unsafe impl NSCopying for NSBezierPath {}

unsafe impl CopyingHelper for NSBezierPath {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSBezierPath {}

unsafe impl NSSecureCoding for NSBezierPath {}

extern_methods!(
    unsafe impl NSBezierPath {
        #[method_id(@__retain_semantics Other bezierPath)]
        pub unsafe fn bezierPath() -> Retained<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathWithRect:)]
        pub unsafe fn bezierPathWithRect(rect: NSRect) -> Retained<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathWithOvalInRect:)]
        pub unsafe fn bezierPathWithOvalInRect(rect: NSRect) -> Retained<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathWithRoundedRect:xRadius:yRadius:)]
        pub unsafe fn bezierPathWithRoundedRect_xRadius_yRadius(
            rect: NSRect,
            x_radius: CGFloat,
            y_radius: CGFloat,
        ) -> Retained<NSBezierPath>;

        #[method(fillRect:)]
        pub unsafe fn fillRect(rect: NSRect);

        #[method(strokeRect:)]
        pub unsafe fn strokeRect(rect: NSRect);

        #[method(clipRect:)]
        pub unsafe fn clipRect(rect: NSRect);

        #[method(strokeLineFromPoint:toPoint:)]
        pub unsafe fn strokeLineFromPoint_toPoint(point1: NSPoint, point2: NSPoint);

        #[method(drawPackedGlyphs:atPoint:)]
        pub unsafe fn drawPackedGlyphs_atPoint(packed_glyphs: NonNull<c_char>, point: NSPoint);

        #[method(defaultMiterLimit)]
        pub unsafe fn defaultMiterLimit() -> CGFloat;

        #[method(setDefaultMiterLimit:)]
        pub unsafe fn setDefaultMiterLimit(default_miter_limit: CGFloat);

        #[method(defaultFlatness)]
        pub unsafe fn defaultFlatness() -> CGFloat;

        #[method(setDefaultFlatness:)]
        pub unsafe fn setDefaultFlatness(default_flatness: CGFloat);

        #[method(defaultWindingRule)]
        pub unsafe fn defaultWindingRule() -> NSWindingRule;

        #[method(setDefaultWindingRule:)]
        pub unsafe fn setDefaultWindingRule(default_winding_rule: NSWindingRule);

        #[method(defaultLineCapStyle)]
        pub unsafe fn defaultLineCapStyle() -> NSLineCapStyle;

        #[method(setDefaultLineCapStyle:)]
        pub unsafe fn setDefaultLineCapStyle(default_line_cap_style: NSLineCapStyle);

        #[method(defaultLineJoinStyle)]
        pub unsafe fn defaultLineJoinStyle() -> NSLineJoinStyle;

        #[method(setDefaultLineJoinStyle:)]
        pub unsafe fn setDefaultLineJoinStyle(default_line_join_style: NSLineJoinStyle);

        #[method(defaultLineWidth)]
        pub unsafe fn defaultLineWidth() -> CGFloat;

        #[method(setDefaultLineWidth:)]
        pub unsafe fn setDefaultLineWidth(default_line_width: CGFloat);

        #[method(moveToPoint:)]
        pub unsafe fn moveToPoint(&self, point: NSPoint);

        #[method(lineToPoint:)]
        pub unsafe fn lineToPoint(&self, point: NSPoint);

        #[method(curveToPoint:controlPoint1:controlPoint2:)]
        pub unsafe fn curveToPoint_controlPoint1_controlPoint2(
            &self,
            end_point: NSPoint,
            control_point1: NSPoint,
            control_point2: NSPoint,
        );

        #[method(curveToPoint:controlPoint:)]
        pub unsafe fn curveToPoint_controlPoint(&self, end_point: NSPoint, control_point: NSPoint);

        #[method(closePath)]
        pub unsafe fn closePath(&self);

        #[method(removeAllPoints)]
        pub unsafe fn removeAllPoints(&self);

        #[method(relativeMoveToPoint:)]
        pub unsafe fn relativeMoveToPoint(&self, point: NSPoint);

        #[method(relativeLineToPoint:)]
        pub unsafe fn relativeLineToPoint(&self, point: NSPoint);

        #[method(relativeCurveToPoint:controlPoint1:controlPoint2:)]
        pub unsafe fn relativeCurveToPoint_controlPoint1_controlPoint2(
            &self,
            end_point: NSPoint,
            control_point1: NSPoint,
            control_point2: NSPoint,
        );

        #[method(relativeCurveToPoint:controlPoint:)]
        pub unsafe fn relativeCurveToPoint_controlPoint(
            &self,
            end_point: NSPoint,
            control_point: NSPoint,
        );

        #[method(lineWidth)]
        pub unsafe fn lineWidth(&self) -> CGFloat;

        #[method(setLineWidth:)]
        pub unsafe fn setLineWidth(&self, line_width: CGFloat);

        #[method(lineCapStyle)]
        pub unsafe fn lineCapStyle(&self) -> NSLineCapStyle;

        #[method(setLineCapStyle:)]
        pub unsafe fn setLineCapStyle(&self, line_cap_style: NSLineCapStyle);

        #[method(lineJoinStyle)]
        pub unsafe fn lineJoinStyle(&self) -> NSLineJoinStyle;

        #[method(setLineJoinStyle:)]
        pub unsafe fn setLineJoinStyle(&self, line_join_style: NSLineJoinStyle);

        #[method(windingRule)]
        pub unsafe fn windingRule(&self) -> NSWindingRule;

        #[method(setWindingRule:)]
        pub unsafe fn setWindingRule(&self, winding_rule: NSWindingRule);

        #[method(miterLimit)]
        pub unsafe fn miterLimit(&self) -> CGFloat;

        #[method(setMiterLimit:)]
        pub unsafe fn setMiterLimit(&self, miter_limit: CGFloat);

        #[method(flatness)]
        pub unsafe fn flatness(&self) -> CGFloat;

        #[method(setFlatness:)]
        pub unsafe fn setFlatness(&self, flatness: CGFloat);

        #[method(getLineDash:count:phase:)]
        pub unsafe fn getLineDash_count_phase(
            &self,
            pattern: *mut CGFloat,
            count: *mut NSInteger,
            phase: *mut CGFloat,
        );

        #[method(setLineDash:count:phase:)]
        pub unsafe fn setLineDash_count_phase(
            &self,
            pattern: *mut CGFloat,
            count: NSInteger,
            phase: CGFloat,
        );

        #[method(stroke)]
        pub unsafe fn stroke(&self);

        #[method(fill)]
        pub unsafe fn fill(&self);

        #[method(addClip)]
        pub unsafe fn addClip(&self);

        #[method(setClip)]
        pub unsafe fn setClip(&self);

        #[method_id(@__retain_semantics Other bezierPathByFlatteningPath)]
        pub unsafe fn bezierPathByFlatteningPath(&self) -> Retained<NSBezierPath>;

        #[method_id(@__retain_semantics Other bezierPathByReversingPath)]
        pub unsafe fn bezierPathByReversingPath(&self) -> Retained<NSBezierPath>;

        #[method(transformUsingAffineTransform:)]
        pub unsafe fn transformUsingAffineTransform(&self, transform: &NSAffineTransform);

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method(currentPoint)]
        pub unsafe fn currentPoint(&self) -> NSPoint;

        #[method(controlPointBounds)]
        pub unsafe fn controlPointBounds(&self) -> NSRect;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;

        #[method(elementCount)]
        pub unsafe fn elementCount(&self) -> NSInteger;

        #[method(elementAtIndex:associatedPoints:)]
        pub unsafe fn elementAtIndex_associatedPoints(
            &self,
            index: NSInteger,
            points: NSPointArray,
        ) -> NSBezierPathElement;

        #[method(elementAtIndex:)]
        pub unsafe fn elementAtIndex(&self, index: NSInteger) -> NSBezierPathElement;

        #[method(setAssociatedPoints:atIndex:)]
        pub unsafe fn setAssociatedPoints_atIndex(&self, points: NSPointArray, index: NSInteger);

        #[method(appendBezierPath:)]
        pub unsafe fn appendBezierPath(&self, path: &NSBezierPath);

        #[method(appendBezierPathWithRect:)]
        pub unsafe fn appendBezierPathWithRect(&self, rect: NSRect);

        #[method(appendBezierPathWithPoints:count:)]
        pub unsafe fn appendBezierPathWithPoints_count(
            &self,
            points: NSPointArray,
            count: NSInteger,
        );

        #[method(appendBezierPathWithOvalInRect:)]
        pub unsafe fn appendBezierPathWithOvalInRect(&self, rect: NSRect);

        #[method(appendBezierPathWithArcWithCenter:radius:startAngle:endAngle:clockwise:)]
        pub unsafe fn appendBezierPathWithArcWithCenter_radius_startAngle_endAngle_clockwise(
            &self,
            center: NSPoint,
            radius: CGFloat,
            start_angle: CGFloat,
            end_angle: CGFloat,
            clockwise: bool,
        );

        #[method(appendBezierPathWithArcWithCenter:radius:startAngle:endAngle:)]
        pub unsafe fn appendBezierPathWithArcWithCenter_radius_startAngle_endAngle(
            &self,
            center: NSPoint,
            radius: CGFloat,
            start_angle: CGFloat,
            end_angle: CGFloat,
        );

        #[method(appendBezierPathWithArcFromPoint:toPoint:radius:)]
        pub unsafe fn appendBezierPathWithArcFromPoint_toPoint_radius(
            &self,
            point1: NSPoint,
            point2: NSPoint,
            radius: CGFloat,
        );

        #[method(appendBezierPathWithRoundedRect:xRadius:yRadius:)]
        pub unsafe fn appendBezierPathWithRoundedRect_xRadius_yRadius(
            &self,
            rect: NSRect,
            x_radius: CGFloat,
            y_radius: CGFloat,
        );

        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: NSPoint) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBezierPath {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSBezierPathDeprecated
    unsafe impl NSBezierPath {
        #[deprecated]
        #[method(cachesBezierPath)]
        pub unsafe fn cachesBezierPath(&self) -> bool;

        #[deprecated]
        #[method(setCachesBezierPath:)]
        pub unsafe fn setCachesBezierPath(&self, flag: bool);

        #[cfg(feature = "NSFont")]
        #[deprecated = "Use -appendBezierPathWithCGGlyph:inFont: instead"]
        #[method(appendBezierPathWithGlyph:inFont:)]
        pub unsafe fn appendBezierPathWithGlyph_inFont(&self, glyph: NSGlyph, font: &NSFont);

        #[cfg(feature = "NSFont")]
        #[deprecated = "Use -appendBezierPathWithCGGlyphs:count:inFont: instead"]
        #[method(appendBezierPathWithGlyphs:count:inFont:)]
        pub unsafe fn appendBezierPathWithGlyphs_count_inFont(
            &self,
            glyphs: NonNull<NSGlyph>,
            count: NSInteger,
            font: &NSFont,
        );

        #[deprecated = "Use -appendBezierPathWithCGGlyphs:count:inFont: instead"]
        #[method(appendBezierPathWithPackedGlyphs:)]
        pub unsafe fn appendBezierPathWithPackedGlyphs(&self, packed_glyphs: NonNull<c_char>);
    }
);

pub static NSButtLineCapStyle: NSLineCapStyle = NSLineCapStyle(NSLineCapStyle::Butt.0);

pub static NSRoundLineCapStyle: NSLineCapStyle = NSLineCapStyle(NSLineCapStyle::Round.0);

pub static NSSquareLineCapStyle: NSLineCapStyle = NSLineCapStyle(NSLineCapStyle::Square.0);

pub static NSMiterLineJoinStyle: NSLineJoinStyle = NSLineJoinStyle(NSLineJoinStyle::Miter.0);

pub static NSRoundLineJoinStyle: NSLineJoinStyle = NSLineJoinStyle(NSLineJoinStyle::Round.0);

pub static NSBevelLineJoinStyle: NSLineJoinStyle = NSLineJoinStyle(NSLineJoinStyle::Bevel.0);

pub static NSNonZeroWindingRule: NSWindingRule = NSWindingRule(NSWindingRule::NonZero.0);

pub static NSEvenOddWindingRule: NSWindingRule = NSWindingRule(NSWindingRule::EvenOdd.0);

pub static NSMoveToBezierPathElement: NSBezierPathElement =
    NSBezierPathElement(NSBezierPathElement::MoveTo.0);

pub static NSLineToBezierPathElement: NSBezierPathElement =
    NSBezierPathElement(NSBezierPathElement::LineTo.0);

pub static NSCurveToBezierPathElement: NSBezierPathElement =
    NSBezierPathElement(NSBezierPathElement::CurveTo.0);

pub static NSClosePathBezierPathElement: NSBezierPathElement =
    NSBezierPathElement(NSBezierPathElement::ClosePath.0);
