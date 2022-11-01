use crate::token::{Literal, LiteralKind};
use std::cmp::Ordering;

use crate::reporter::Reporter;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ObjectKind {
    Number,
    Float,
    String,
    Bool,
    #[default]
    Nil,
}

#[derive(Debug, Default, PartialEq)]
pub enum ObjectValue {
    Number(i32),
    Float(f32),
    String(String),
    Bool(bool),
    #[default]
    Nil,
}

impl From<Literal> for ObjectKind {
    fn from(v: Literal) -> Self {
        match v.value {
            LiteralKind::Number(_) => ObjectKind::Number,
            LiteralKind::Float(_) => ObjectKind::Float,
            LiteralKind::String(_) => ObjectKind::String,
            LiteralKind::Bool(_) => ObjectKind::Bool,
            LiteralKind::Nil => ObjectKind::Nil,
        }
    }
}

impl From<Literal> for ObjectValue {
    fn from(v: Literal) -> Self {
        match v.value {
            LiteralKind::Number(n) => ObjectValue::Number(n),
            LiteralKind::Float(f) => ObjectValue::Float(f),
            LiteralKind::String(s) => ObjectValue::String(s),
            LiteralKind::Bool(b) => ObjectValue::Bool(b),
            LiteralKind::Nil => ObjectValue::Nil,
        }
    }
}

impl Clone for ObjectValue {
    fn clone(&self) -> Self {
        todo!()
    }
}
impl Display for ObjectValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            ObjectValue::Float(fl) => write!(f, "{}", fl),
            ObjectValue::Number(n) => write!(f, "{}", n),
            ObjectValue::String(s) => write!(f, "{}", s),
            ObjectValue::Bool(b) => write!(f, "{}", b),
            ObjectValue::Nil => write!(f, "Nil"),
        }
    }
}

impl Sub for ObjectValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            ObjectValue::Number(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::Number(l - r),
                ObjectValue::Float(r) => ObjectValue::Float(l as f32 - r),
                _ => {
                    Reporter::arithmetic_error(&format!("{} - {}", l, rhs));
                    ObjectValue::Nil
                }
            },

            ObjectValue::Float(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::Float(l - r as f32),
                ObjectValue::Float(r) => ObjectValue::Float(l - r),
                _ => {
                    Reporter::arithmetic_error(&format!("{} - {}", l, rhs));
                    ObjectValue::Nil
                }
            },
            _ => {
                Reporter::arithmetic_error(&format!("{} - {}", self, rhs));
                ObjectValue::Nil
            }
        }
    }
}

impl Mul for ObjectValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            ObjectValue::Number(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::Number(l * r),
                ObjectValue::Float(r) => ObjectValue::Float(l as f32 * r),
                _ => {
                    Reporter::arithmetic_error(&format!("{} * {}", l, rhs));
                    ObjectValue::Nil
                }
            },

            ObjectValue::Float(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::Float(l * r as f32),
                ObjectValue::Float(r) => ObjectValue::Float(l * r),
                _ => {
                    Reporter::arithmetic_error(&format!("{} * {}", l, rhs));
                    ObjectValue::Nil
                }
            },
            _ => {
                Reporter::arithmetic_error(&format!("{} * {}", self, rhs));
                ObjectValue::Nil
            }
        }
    }
}

impl Div for ObjectValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // TODO: check for i32 / i32 that results in f32
        match self {
            ObjectValue::Number(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::Number(l / r),
                ObjectValue::Float(r) => ObjectValue::Float(l as f32 / r),
                _ => {
                    Reporter::arithmetic_error(&format!("{} / {}", l, rhs));
                    ObjectValue::Nil
                }
            },

            ObjectValue::Float(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::Float(l / r as f32),
                ObjectValue::Float(r) => ObjectValue::Float(l / r),
                _ => {
                    Reporter::arithmetic_error(&format!("{} / {}", l, rhs));
                    ObjectValue::Nil
                }
            },
            _ => {
                Reporter::arithmetic_error(&format!("{} / {}", self, rhs));
                ObjectValue::Nil
            }
        }
    }
}

impl Add for ObjectValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            ObjectValue::Number(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::Number(l + r),
                ObjectValue::Float(r) => ObjectValue::Float(l as f32 + r),
                ObjectValue::String(r) => ObjectValue::String(format!("{}{}", l, r)),
                _ => {
                    Reporter::arithmetic_error(&format!("{} + {}", l, rhs));
                    ObjectValue::Nil
                }
            },

            ObjectValue::Float(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::Float(l + r as f32),
                ObjectValue::Float(r) => ObjectValue::Float(l + r),
                ObjectValue::String(r) => ObjectValue::String(format!("{}{}", l, r)),
                _ => {
                    Reporter::arithmetic_error(&format!("{} + {}", l, rhs));
                    ObjectValue::Nil
                }
            },

            ObjectValue::String(l) => match rhs {
                ObjectValue::Number(r) => ObjectValue::String(format!("{}{}", l, r)),
                ObjectValue::Float(r) => ObjectValue::String(format!("{}{}", l, r)),
                ObjectValue::String(r) => ObjectValue::String(format!("{}{}", l, r)),
                _ => {
                    Reporter::arithmetic_error(&format!("{} + {}", l, rhs));
                    ObjectValue::Nil
                }
            },
            _ => {
                Reporter::arithmetic_error(&format!("{} + {}", self, rhs));
                ObjectValue::Nil
            }
        }
    }
}

impl PartialOrd for ObjectValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            ObjectValue::Number(l) => match other {
                ObjectValue::Number(r) => {
                    if l < r {
                        return Some(Ordering::Less);
                    }
                    if l == r {
                        return Some(Ordering::Equal);
                    }
                    if l > r {
                        return Some(Ordering::Greater);
                    }
                    None
                }
                ObjectValue::Float(r) => {
                    if l < &(*r as i32) {
                        return Some(Ordering::Less);
                    }
                    if l == &(*r as i32) {
                        return Some(Ordering::Equal);
                    }
                    if l > &(*r as i32) {
                        return Some(Ordering::Greater);
                    }
                    None
                }
                _ => None,
            },
            ObjectValue::Float(l) => match other {
                ObjectValue::Number(r) => {
                    if l < &(*r as f32) {
                        return Some(Ordering::Less);
                    }
                    if l == &(*r as f32) {
                        return Some(Ordering::Equal);
                    }
                    if l > &(*r as f32) {
                        return Some(Ordering::Greater);
                    }
                    None
                }
                ObjectValue::Float(r) => {
                    if l < r {
                        return Some(Ordering::Less);
                    }
                    if l == r {
                        return Some(Ordering::Equal);
                    }
                    if l > r {
                        return Some(Ordering::Greater);
                    }
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self {
            ObjectValue::Number(l) => match other {
                ObjectValue::Number(r) => l < r,
                ObjectValue::Float(r) => l < &(*r as i32),
                _ => {
                    Reporter::arithmetic_error(&format!("{} > {}", l, other));
                    false
                }
            },
            ObjectValue::Float(l) => match other {
                ObjectValue::Number(r) => l < &(*r as f32),
                ObjectValue::Float(r) => l < r,
                _ => {
                    Reporter::arithmetic_error(&format!("{} > {}", l, other));
                    false
                }
            },
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match self {
            ObjectValue::Number(l) => match other {
                ObjectValue::Number(r) => l <= r,
                ObjectValue::Float(r) => l <= &(*r as i32),
                _ => {
                    Reporter::arithmetic_error(&format!("{} > {}", l, other));
                    false
                }
            },
            ObjectValue::Float(l) => match other {
                ObjectValue::Number(r) => l <= &(*r as f32),
                ObjectValue::Float(r) => l <= r,
                _ => {
                    Reporter::arithmetic_error(&format!("{} > {}", l, other));
                    false
                }
            },
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            ObjectValue::Number(l) => match other {
                ObjectValue::Number(r) => l > r,
                ObjectValue::Float(r) => l > &(*r as i32),
                _ => {
                    Reporter::arithmetic_error(&format!("{} > {}", l, other));
                    false
                }
            },
            ObjectValue::Float(l) => match other {
                ObjectValue::Number(r) => l > &(*r as f32),
                ObjectValue::Float(r) => l > r,
                _ => {
                    Reporter::arithmetic_error(&format!("{} > {}", l, other));
                    false
                }
            },
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match self {
            ObjectValue::Number(l) => match other {
                ObjectValue::Number(r) => l >= r,
                ObjectValue::Float(r) => l >= &(*r as i32),
                _ => {
                    Reporter::arithmetic_error(&format!("{} > {}", l, other));
                    false
                }
            },
            ObjectValue::Float(l) => match other {
                ObjectValue::Number(r) => l >= &(*r as f32),
                ObjectValue::Float(r) => l >= r,
                _ => {
                    Reporter::arithmetic_error(&format!("{} > {}", l, other));
                    false
                }
            },
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object {
    pub kind: ObjectKind,
    pub value: ObjectValue,
}

/// member function
impl Object {
    /// return true if the object is of the same kind
    ///
    /// # Supported kinds
    /// `string, number, float, boolean, nil`
    ///
    /// # Example
    /// ```
    /// use oxa::object::{Object, ObjectKind};
    /// let obj = Object::from(10);
    ///
    /// assert!(obj.is_kind(ObjectKind::Number));
    /// ```
    pub fn is_kind(&self, kind: ObjectKind) -> bool {
        self.kind == kind
    }

    /// return true if the object is empty
    ///
    /// # Rule
    /// nil - true
    /// empty string - true
    /// nan number - true
    ///
    /// # Example
    /// ```
    /// use oxa::object::{Object, ObjectKind};
    /// let obj = Object::from("");
    ///
    /// assert!(obj.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        if self.is_kind(ObjectKind::Nil) || self.value == ObjectValue::Nil {
            return true;
        }

        match &self.value {
            ObjectValue::Float(f) => {
                if f.is_nan() {
                    return true;
                }
                false
            }
            ObjectValue::Number(_) => false,
            ObjectValue::String(s) => s.is_empty(),
            ObjectValue::Bool(_) => false,
            _ => true,
        }
    }
}

impl From<Literal> for Object {
    fn from(value: Literal) -> Self {
        Object {
            kind: value.clone().into(),
            value: value.into(),
        }
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Object {
            kind: ObjectKind::Bool,
            value: ObjectValue::Bool(value),
        }
    }
}

impl From<i32> for Object {
    fn from(value: i32) -> Self {
        Object {
            kind: ObjectKind::Number,
            value: ObjectValue::Number(value),
        }
    }
}

impl From<f32> for Object {
    fn from(value: f32) -> Self {
        Object {
            kind: ObjectKind::Float,
            value: ObjectValue::Float(value),
        }
    }
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Object {
            kind: ObjectKind::String,
            value: ObjectValue::String(value.to_string()),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Sub for Object {
    type Output = Self;

    /// subtract one object from another and returns object of the same type or `ObjectKind::Nil`
    /// if the operation cannot succeed
    ///
    /// # Example
    /// ```
    /// use oxa::object::Object;
    /// let obj_1 = Object::from(10);
    /// let obj_2 = Object::from(20);
    /// assert_eq!(obj_1 - obj_2, Object::from(-10));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let value = self.value - rhs.value;

        match value {
            ObjectValue::Number(n) => Object::from(n),
            ObjectValue::Float(f) => Object::from(f),
            _ => Object::default(),
        }
    }
}

impl Add for Object {
    type Output = Self;

    /// add one object to another and returns object of the same type or `ObjectKind::Nil`
    /// if the operation cannot succeed
    ///
    /// # Example
    /// ```
    /// use oxa::object::Object;
    /// let obj_1 = Object::from(2);
    /// let obj_2 = Object::from(4);
    /// assert_eq!(obj_1 + obj_2, Object::from(6));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let value = self.value + rhs.value;

        match value {
            ObjectValue::Number(n) => Object::from(n),
            ObjectValue::Float(f) => Object::from(f),
            ObjectValue::String(s) => Object::from(s.as_ref()),
            _ => Object::default(),
        }
    }
}

impl Mul for Object {
    type Output = Self;

    /// multiply one object with another and returns object of the same type or `ObjectKind::Nil`
    /// if the operation cannot succeed
    ///
    /// # Example
    /// ```
    /// use oxa::object::Object;
    /// let obj_1 = Object::from(2);
    /// let obj_2 = Object::from(4);
    /// assert_eq!(obj_1 * obj_2, Object::from(8));
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let value = self.value * rhs.value;

        match value {
            ObjectValue::Number(n) => Object::from(n),
            ObjectValue::Float(f) => Object::from(f),
            _ => Object::default(),
        }
    }
}

impl Div for Object {
    type Output = Self;

    /// divides one object from another and returns object of the same type or `ObjectKind::Nil`
    /// if the operation cannot succeed
    ///
    /// # Example
    /// ```
    /// use oxa::object::Object;
    /// let obj_1 = Object::from(10.0);
    /// let obj_2 = Object::from(5.0);
    /// assert_eq!(obj_1 / obj_2, Object::from(2.0));
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        let value = self.value / rhs.value;

        match value {
            ObjectValue::Number(n) => Object::from(n),
            ObjectValue::Float(f) => Object::from(f),
            _ => Object::default(),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.kind {
            ObjectKind::Number | ObjectKind::Float => self.value.partial_cmp(&other.value),
            _ => None,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self.kind {
            ObjectKind::Number | ObjectKind::Float => self.value < other.value,
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match self.kind {
            ObjectKind::Number | ObjectKind::Float => self.value <= other.value,
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self.kind {
            ObjectKind::Number | ObjectKind::Float => self.value > other.value,
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match self.kind {
            ObjectKind::Number | ObjectKind::Float => self.value >= other.value,
            _ => false,
        }
    }
}

#[cfg(test)]
mod object_tests {
    use super::*;

    #[test]
    fn has_correct_object_kind() {
        let data = vec![
            (Object::from(10), ObjectKind::Number),
            (Object::from(10.2), ObjectKind::Float),
            (Object::from("string"), ObjectKind::String),
            (Object::from(false), ObjectKind::Bool),
            (Object::default(), ObjectKind::Nil),
        ];

        for (obj, kind) in data {
            assert!(obj.is_kind(kind))
        }
    }

    #[test]
    fn check_for_empty() {
        let data = vec![
            (Object::from(10), false),
            (Object::from(10.2), false),
            (Object::from(f32::NAN), true),
            (Object::from("string"), false),
            (Object::from(""), true),
            (Object::from(false), false),
            (Object::default(), true),
        ];

        for (obj, exp) in data {
            assert_eq!(obj.is_empty(), exp)
        }
    }

    #[test]
    fn same_type_arithmetic() {
        let obj_1 = Object::from(10);
        let obj_2 = Object::from(20);
        assert_eq!(obj_1 + obj_2, Object::from(30));

        let obj_3 = Object::from(10);
        let obj_4 = Object::from(20);
        assert_eq!(obj_3 - obj_4, Object::from(-10));

        let obj_5 = Object::from(10);
        let obj_6 = Object::from(20);
        assert_eq!(obj_5 * obj_6, Object::from(200));

        let obj_5 = Object::from(10.0);
        let obj_6 = Object::from(20.0);
        assert_eq!(obj_5 / obj_6, Object::from(0.5));

        let obj_7 = Object::from("string");
        let obj_8 = Object::from("concat");
        assert_eq!(obj_7 + obj_8, Object::from("stringconcat"));
    }

    #[test]
    fn returns_nil_for_wrong_operation() {
        let obj_1 = Object::from(10);
        let obj_2 = Object::default();
        assert_eq!(obj_1 + obj_2, Object::default());

        let obj_3 = Object::from("string");
        let obj_4 = Object::from(false);
        assert_eq!(obj_3 - obj_4, Object::default());
    }

    #[test]
    fn concat_string_with_other_type() {
        let obj_1 = Object::from("string");
        let obj_2 = Object::from("string");
        assert_eq!(obj_1 + obj_2, Object::from("stringstring"));

        let obj_3 = Object::from("string");
        let obj_4 = Object::from(10);
        assert_eq!(obj_3 + obj_4, Object::from("string10"));
    }

    #[test]
    fn logical_same_type() {
        let obj_1 = Object::from("string");
        let obj_2 = Object::from("string");
        assert!(obj_1 == obj_2);

        let obj_3 = Object::from("string");
        let obj_4 = Object::from("string2");
        assert!(obj_3 != obj_4);

        let obj_5 = Object::from(10);
        let obj_6 = Object::from(20);
        assert!(obj_5 != obj_6);

        let obj_7 = Object::from(10);
        let obj_8 = Object::from(20);
        assert!(obj_7 < obj_8);

        let obj_9 = Object::from(10);
        let obj_10 = Object::from(20);
        assert!(obj_9 < obj_10);

        let obj_1 = Object::default();
        let obj_2 = Object::default();
        assert!(obj_1 == obj_2);
    }

    #[test]
    fn logical_wrong_type() {
        let obj_1 = Object::default();
        let obj_2 = Object::from("string");
        assert!(obj_1 != obj_2);

        let obj_3 = Object::from("string");
        let obj_4 = Object::from(10.5);
        assert!(obj_3 != obj_4);
    }
}
