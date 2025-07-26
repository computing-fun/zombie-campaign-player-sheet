use std::ops::{Add, Neg, Not, Sub};

#[derive(
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    derive_more::Debug,
    derive_more::Not,
    derive_more::Add,
    derive_more::Sum,
    derive_more::Display,
)]
pub struct ActionPoints(pub usize);

const ACTION_POINTS_DEFAULT: ActionPoints = ActionPoints(3);

#[derive(
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    derive_more::Debug,
    derive_more::Not,
    derive_more::Add,
    derive_more::Sum,
    derive_more::Display,
)]
pub struct Movement(pub isize);

pub trait AbilityScore: Into<isize> + From<Self> + Add + Sub + Not + Neg + Sized {}

macro_rules! ability_score {
    ($name:ident) => {
        #[derive(
            derive_more::From,
            derive_more::Into,
            derive_more::FromStr,
            derive_more::Add,
            derive_more::Sub,
            derive_more::Not,
            derive_more::Neg,
            Copy,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Debug,
            derive_more::Display,
        )]
        pub struct $name(pub isize);
    };
}

ability_score!(Strength);
ability_score!(Dexterity);
ability_score!(Wisdom);
ability_score!(Intelligence);
