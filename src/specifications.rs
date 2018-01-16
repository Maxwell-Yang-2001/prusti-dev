//! Data structures for storing information about specifications.
//!
//! Currently, we support preconditions, postconditions, and loop
//! invariants that can be specified by using the attribute syntax as
//! follows:
//!
//! ```rust
//! #[requires="0 < n && n < 10"]
//! #[ensures="result > 0"]
//! fn fib(mut n: i32) -> i32 {
//!     let mut i = 1;
//!     let mut j = 1;
//!     #[invariant="i > 0 && j > 0"]
//!     while n > 2 {
//!         let tmp = i + j;
//!         j = i;
//!         i = tmp;
//!         n -= 1;
//!     }
//!     i
//! }
//! ```
//!
//! The allowed assertions are of the form:
//!
//!     assertion := assertion && assertion
//!                | expression ==> assertion
//!                | (forall variable_name :: {expression} expression)
//!
//!  Here ``expression`` is a Rust expression that contains only
//!  elements that are considered expressions in Viper, plus ``match``
//!  expressions.


use rustc;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::string::ToString;
use syntax::{ast, ptr};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A specification type.
pub enum SpecType {
    /// Precondition of a procedure.
    Precondition,
    /// Postcondition of a procedure.
    Postcondition,
    /// Loop invariant.
    Invariant,
}

#[derive(Debug)]
/// A conversion from string into specification type error.
pub enum TryFromStringError {
    /// Reported when the string being converted is not one of the
    /// following: `requires`, `ensures`, `invariant`.
    UnknownSpecificationType,
}

impl<'a> TryFrom<&'a str> for SpecType {

    type Error = TryFromStringError;

    fn try_from(typ: &str) -> Result<SpecType, TryFromStringError> {
        match typ {
            "requires" => {Ok(SpecType::Precondition)},
            "ensures" => {Ok(SpecType::Postcondition)},
            "invariant" => {Ok(SpecType::Invariant)},
            _ => {Err(TryFromStringError::UnknownSpecificationType)},
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
/// A unique ID of the specification element such as entire precondition
/// or postcondition.
pub struct SpecID(u64);

impl SpecID {
    /// Constructor.
    pub fn new() -> Self {
        Self{ 0: 100 }
    }
    /// Increment ID and return a copy of the new value.
    pub fn inc(&mut self) -> Self {
        self.0 += 1;
        Self{ 0: self.0 }
    }
}

impl ToString for SpecID {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
/// A unique ID of the Rust expression used in the specification.
pub struct ExpressionId(usize);

impl ExpressionId {
    /// Constructor.
    pub fn new() -> Self {
        Self{ 0: 100 }
    }
    /// Increment ID and return a copy of the new value.
    pub fn inc(&mut self) -> Self {
        self.0 += 1;
        Self{ 0: self.0 }
    }
}

impl ToString for ExpressionId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Into<usize> for ExpressionId {
    fn into(self) -> usize {
        self.0
    }
}

impl From<u128> for ExpressionId {
    fn from(value: u128) -> Self {
        Self{ 0: value as usize }
    }
}

#[derive(Debug, Clone)]
/// A Rust expression used in the specification.
pub struct Expression<ET> {
    /// Unique identifier.
    pub id: ExpressionId,
    /// Actual expression.
    pub expr: ET,
}

#[derive(Debug, Clone)]
/// An assertion used in the specification.
pub struct Assertion<ET> {
    /// Subassertions.
    pub kind: Box<AssertionKind<ET>>,
}

#[derive(Debug, Clone)]
/// A single trigger for a quantifier.
pub struct Trigger<ET>(Vec<Expression<ET>>);

#[derive(Debug, Clone)]
/// A set of triggers used in the quantifier.
pub struct TriggerSet<ET>(Vec<Trigger<ET>>);

#[derive(Debug, Clone)]
/// An assertion kind used in the specification.
pub enum AssertionKind<ET> {
    /// A single Rust expression.
    Expr(Expression<ET>),
    /// Conjunction &&.
    And(Vec<Assertion<ET>>),
    /// Implication ==>
    Implies(Expression<ET>, Assertion<ET>),
    /// Quantifier (forall x :: {trigger(x)} body(x))
    ForAll(String, TriggerSet<ET>, Expression<ET>),
}


#[derive(Debug, Clone)]
/// Specification such as precondition, postcondition, or invariant.
pub struct Specification<ET> {
    /// Specification type.
    pub typ: SpecType,
    /// Actual specification.
    pub assertion: Assertion<ET>,
}


#[derive(Debug, Clone)]
/// Specification of a single element such as procedure or loop.
pub enum SpecificationSet<ET> {
    /// (Precondition, Postcondition)
    Procedure(Vec<Specification<ET>>, Vec<Specification<ET>>),
    /// Loop invariant.
    Loop(Vec<Specification<ET>>),
}


/// A specification that has no types associated with it.
pub type UntypedSpecification = Specification<ptr::P<ast::Expr>>;
/// A set of specifications associated with a single element.
pub type UntypedSpecificationSet = SpecificationSet<ptr::P<ast::Expr>>;
/// A map of specifications for a specific crate.
pub type UntypedSpecificationMap = HashMap<SpecID, UntypedSpecificationSet>;
/// An assertion that has no types associated with it.
pub type UntypedAssertion = Assertion<ptr::P<ast::Expr>>;
/// An assertion kind that has no types associated with it.
pub type UntypedAssertionKind = AssertionKind<ptr::P<ast::Expr>>;


/// A specification that has no types associated with it.
pub type TypedSpecification = Specification<rustc::hir::Expr>;
/// A set of specifications associated with a single element.
pub type TypedSpecificationSet = SpecificationSet<rustc::hir::Expr>;
/// A map of specifications for a specific crate.
pub type TypedSpecificationMap = HashMap<SpecID, TypedSpecificationSet>;
/// An assertion that has no types associated with it.
pub type TypedAssertion = Assertion<rustc::hir::Expr>;
/// An assertion kind that has no types associated with it.
pub type TypedAssertionKind = AssertionKind<rustc::hir::Expr>;
