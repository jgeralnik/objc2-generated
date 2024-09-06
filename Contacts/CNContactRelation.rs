//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactRelation;

    unsafe impl ClassType for CNContactRelation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CNContactRelation {}

unsafe impl NSCopying for CNContactRelation {}

unsafe impl CopyingHelper for CNContactRelation {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNContactRelation {}

unsafe impl NSSecureCoding for CNContactRelation {}

extern_methods!(
    unsafe impl CNContactRelation {
        #[method_id(@__retain_semantics Other contactRelationWithName:)]
        pub unsafe fn contactRelationWithName(name: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(this: Allocated<Self>, name: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactRelation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static CNLabelContactRelationAssistant: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationManager: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationColleague: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationTeacher: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungestSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationEldestSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungestBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationEldestBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFriend: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationMaleFriend: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFemaleFriend: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSpouse: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationPartner: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationMalePartner: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFemalePartner: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGirlfriendOrBoyfriend: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGirlfriend: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBoyfriend: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParent: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationMother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationChild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandparent: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandmother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandmotherMothersMother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandmotherFathersMother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandfather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandfatherMothersFather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandfatherFathersFather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGreatGrandparent: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGreatGrandmother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGreatGrandfather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandchild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGranddaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGranddaughterDaughtersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGranddaughterSonsDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandson: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandsonDaughtersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandsonSonsSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGreatGrandchild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGreatGranddaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGreatGrandson: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationMotherInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationMotherInLawWifesMother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationMotherInLawHusbandsMother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFatherInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFatherInLawWifesFather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFatherInLawHusbandsFather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCoParentInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCoMotherInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCoFatherInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSiblingInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerSiblingInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderSiblingInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerSisterInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderSisterInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLawSpousesSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLawWifesSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLawHusbandsSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLawBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLawYoungerBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLawElderBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerBrotherInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderBrotherInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLawSpousesBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLawHusbandsBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLawWifesBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLawSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLawYoungerSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLawElderSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLawWifesBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSisterInLawHusbandsBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLawWifesSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationBrotherInLawHusbandsSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCoSiblingInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCoSisterInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCoBrotherInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationChildInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationDaughterInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSonInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousin: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousin: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousin: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationMaleCousin: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFemaleCousin: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinParentsSiblingsChild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinParentsSiblingsSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinParentsSiblingsSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinParentsSiblingsSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinParentsSiblingsDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinParentsSiblingsDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinParentsSiblingsDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinMothersSistersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersSistersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinMothersSistersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinMothersSistersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersSistersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinMothersSistersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinMothersBrothersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersBrothersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinMothersBrothersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinMothersBrothersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersBrothersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinMothersBrothersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinFathersSistersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinFathersSistersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinFathersSistersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinFathersSistersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinFathersSistersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinFathersSistersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinFathersBrothersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinFathersBrothersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinFathersBrothersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinFathersBrothersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinFathersBrothersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinFathersBrothersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinGrandparentsSiblingsChild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinGrandparentsSiblingsDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinGrandparentsSiblingsSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersSiblingsSonOrFathersSistersSon:
        &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinMothersSiblingsSonOrFathersSistersSon:
        &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter:
        &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationElderCousinMothersSiblingsDaughterOrFathersSistersDaughter:
        &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsYoungerSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsElderSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsSiblingMothersSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsSiblingMothersYoungerSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsSiblingMothersElderSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsSiblingFathersSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsSiblingFathersYoungerSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationParentsSiblingFathersElderSibling: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAunt: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntParentsSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntParentsYoungerSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntParentsElderSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntFathersSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntFathersYoungerSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntFathersElderSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntFathersBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntFathersYoungerBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntFathersElderBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntMothersSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntMothersYoungerSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntMothersElderSister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationAuntMothersBrothersWife: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandaunt: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncle: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleParentsBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleParentsYoungerBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleParentsElderBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleMothersBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleMothersYoungerBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleMothersElderBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleMothersSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleFathersBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleFathersYoungerBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleFathersElderBrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleFathersSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleFathersYoungerSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationUncleFathersElderSistersHusband: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGranduncle: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSiblingsChild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNiece: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNieceSistersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNieceBrothersDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNieceSistersDaughterOrWifesSiblingsDaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNieceBrothersDaughterOrHusbandsSiblingsDaughter:
        &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNephew: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNephewSistersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNephewBrothersSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNephewBrothersSonOrHusbandsSiblingsSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNephewSistersSonOrWifesSiblingsSon: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandniece: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandnieceSistersGranddaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandnieceBrothersGranddaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandnephew: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandnephewSistersGrandson: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandnephewBrothersGrandson: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationStepparent: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationStepmother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationStepfather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationStepchild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationStepdaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationStepson: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationStepsister: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationStepbrother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationMotherInLawOrStepmother: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationFatherInLawOrStepfather: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationDaughterInLawOrStepdaughter: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSonInLawOrStepson: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationCousinOrSiblingsChild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNieceOrCousin: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationNephewOrCousin: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandchildOrSiblingsChild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGranddaughterOrNiece: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGrandsonOrNephew: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationGreatGrandchildOrSiblingsGrandchild: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationDaughterInLawOrSisterInLaw: &'static NSString;
}

extern "C" {
    pub static CNLabelContactRelationSonInLawOrBrotherInLaw: &'static NSString;
}
