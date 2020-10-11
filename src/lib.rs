#[derive(Debug, Copy, Clone)]
pub enum Magnitude<T> {
    /// A finite value
    Finite(T),

    /// Positive infinity
    PosInfinite,

    /// Negative infinity
    NegInfinite,
}

impl<T> Magnitude<T> {
    pub fn is_pos_infinite(&self) -> bool {
        matches!(self, Magnitude::PosInfinite)
    }

    pub fn is_neg_infinite(&self) -> bool {
        matches!(self, Magnitude::NegInfinite)
    }

    pub fn is_finite(&self) -> bool {
        !(self.is_pos_infinite() && self.is_neg_infinite())
    }

    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Magnitude::Finite(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ref_mut(&mut self) -> Option<&mut T> {
        match self {
            Magnitude::Finite(ref mut value) => Some(value),
            _ => None,
        }
    }
}

// Implement From trait for more convenient use of Magnitude
use std::convert::From;

impl<T> From<T> for Magnitude<T> {
    fn from(value: T) -> Self {
        Magnitude::Finite(value)
    }
}

// implement comparison operators for Beyond
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

impl<T: PartialEq> PartialEq for Magnitude<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Magnitude::Finite(value1), Magnitude::Finite(value2)) => value1.eq(value2),

            (Magnitude::Finite(_), Magnitude::PosInfinite)
            | (Magnitude::Finite(_), Magnitude::NegInfinite)
            | (Magnitude::PosInfinite, Magnitude::Finite(_))
            | (Magnitude::PosInfinite, Magnitude::NegInfinite)
            | (Magnitude::NegInfinite, Magnitude::Finite(_))
            | (Magnitude::NegInfinite, Magnitude::PosInfinite) => false,

            (Magnitude::PosInfinite, Magnitude::PosInfinite) => {
                panic!("Can not compare two positive infinities")
            }

            (Magnitude::NegInfinite, Magnitude::NegInfinite) => {
                panic!("Can not compare two negative infinities")
            }
        }
    }
}

impl<T: PartialEq> Eq for Magnitude<T> {}

impl<T: PartialOrd> PartialOrd for Magnitude<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Magnitude::Finite(value1), Magnitude::Finite(value2)) => value1.partial_cmp(value2),

            (Magnitude::Finite(_), Magnitude::NegInfinite) => Some(Ordering::Greater),
            (Magnitude::PosInfinite, Magnitude::Finite(_)) => Some(Ordering::Greater),
            (Magnitude::PosInfinite, Magnitude::NegInfinite) => Some(Ordering::Greater),

            (Magnitude::Finite(_), Magnitude::PosInfinite) => Some(Ordering::Less),
            (Magnitude::NegInfinite, Magnitude::Finite(_)) => Some(Ordering::Less),
            (Magnitude::NegInfinite, Magnitude::PosInfinite) => Some(Ordering::Less),

            (Magnitude::PosInfinite, Magnitude::PosInfinite) => {
                panic!("Can not compare two positive infinities")
            }
            (Magnitude::NegInfinite, Magnitude::NegInfinite) => {
                panic!("Can not compare two negative infinities")
            }
        }
    }
}

impl<T: Ord> Ord for Magnitude<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// implement arithmetic operators for Beyond
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<T: Add<Output = T>> Add for Magnitude<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Magnitude::Finite(value1), Magnitude::Finite(value2)) => {
                Magnitude::Finite(value1 + value2)
            }

            (Magnitude::Finite(_), Magnitude::PosInfinite) => Magnitude::PosInfinite,
            (Magnitude::PosInfinite, Magnitude::Finite(_)) => Magnitude::PosInfinite,
            (Magnitude::PosInfinite, Magnitude::PosInfinite) => Magnitude::PosInfinite,

            (Magnitude::Finite(_), Magnitude::NegInfinite) => Magnitude::NegInfinite,
            (Magnitude::NegInfinite, Magnitude::Finite(_)) => Magnitude::NegInfinite,
            (Magnitude::NegInfinite, Magnitude::NegInfinite) => Magnitude::NegInfinite,

            (Magnitude::PosInfinite, Magnitude::NegInfinite)
            | (Magnitude::NegInfinite, Magnitude::PosInfinite) => {
                panic!("Can not add PosInfinite and NegInfinite")
            }
        }
    }
}

impl<T: Sub<Output = T>> Sub for Magnitude<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Magnitude::Finite(value1), Magnitude::Finite(value2)) => {
                Magnitude::Finite(value1 - value2)
            }

            (Magnitude::Finite(_), Magnitude::NegInfinite) => Magnitude::PosInfinite,
            (Magnitude::PosInfinite, Magnitude::Finite(_)) => Magnitude::PosInfinite,
            (Magnitude::PosInfinite, Magnitude::NegInfinite) => Magnitude::PosInfinite,

            (Magnitude::Finite(_), Magnitude::PosInfinite) => Magnitude::NegInfinite,
            (Magnitude::NegInfinite, Magnitude::Finite(_)) => Magnitude::NegInfinite,
            (Magnitude::NegInfinite, Magnitude::PosInfinite) => Magnitude::NegInfinite,

            (Magnitude::PosInfinite, Magnitude::PosInfinite) => {
                panic!("Can not subtract two PosInfinities")
            }
            (Magnitude::NegInfinite, Magnitude::NegInfinite) => {
                panic!("Can not subtract two NegInfinities")
            }
        }
    }
}

use num_traits::identities::Zero;
impl<T: Mul<Output = T> + PartialOrd + Zero> Mul for Magnitude<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Magnitude::Finite(value1), Magnitude::Finite(value2)) => {
                Magnitude::Finite(value1 * value2)
            }

            (Magnitude::PosInfinite, Magnitude::Finite(value))
            | (Magnitude::Finite(value), Magnitude::PosInfinite) => {
                if value < T::zero() {
                    Magnitude::NegInfinite
                } else if value > T::zero() {
                    Magnitude::PosInfinite
                } else {
                    panic!("Can not multiply zero by Infinity");
                }
            }

            (Magnitude::NegInfinite, Magnitude::Finite(value))
            | (Magnitude::Finite(value), Magnitude::NegInfinite) => {
                if value < T::zero() {
                    Magnitude::PosInfinite
                } else if value > T::zero() {
                    Magnitude::NegInfinite
                } else {
                    panic!("Can not multiply zero by Infinity");
                }
            }

            (Magnitude::PosInfinite, Magnitude::PosInfinite) => Magnitude::PosInfinite,
            (Magnitude::PosInfinite, Magnitude::NegInfinite) => Magnitude::NegInfinite,
            (Magnitude::NegInfinite, Magnitude::PosInfinite) => Magnitude::NegInfinite,
            (Magnitude::NegInfinite, Magnitude::NegInfinite) => Magnitude::PosInfinite,
        }
    }
}

impl<T: Div<Output = T> + PartialOrd + Zero> Div for Magnitude<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Magnitude::Finite(value1), Magnitude::Finite(value2)) => {
                Magnitude::Finite(value1 / value2)
            }

            (Magnitude::Finite(value), Magnitude::PosInfinite) => {
                if value == T::zero() {
                    Magnitude::Finite(T::zero())
                } else {
                    panic!("Can not divide non-zero number by infinity")
                }
            }

            (Magnitude::PosInfinite, Magnitude::Finite(value)) => {
                if value < T::zero() {
                    Magnitude::NegInfinite
                } else if value > T::zero() {
                    Magnitude::PosInfinite
                } else {
                    panic!("Division by zero");
                }
            }

            (Magnitude::Finite(value), Magnitude::NegInfinite) => {
                if value == T::zero() {
                    Magnitude::Finite(T::zero())
                } else {
                    panic!("Can not divide non-zero number by infinity")
                }
            }

            (Magnitude::NegInfinite, Magnitude::Finite(value)) => {
                if value < T::zero() {
                    Magnitude::PosInfinite
                } else if value > T::zero() {
                    Magnitude::NegInfinite
                } else {
                    panic!("Division by zero");
                }
            }

            (Magnitude::PosInfinite, Magnitude::PosInfinite)
            | (Magnitude::PosInfinite, Magnitude::NegInfinite)
            | (Magnitude::NegInfinite, Magnitude::PosInfinite)
            | (Magnitude::NegInfinite, Magnitude::NegInfinite) => {
                panic!("Can not divide two infinities")
            }
        }
    }
}

impl<T: Neg<Output = T>> Neg for Magnitude<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Magnitude::Finite(value) => Magnitude::Finite(-value),
            Magnitude::PosInfinite => Magnitude::NegInfinite,
            Magnitude::NegInfinite => Magnitude::PosInfinite,
        }
    }
}

// implement auxiliary operators
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

impl<T: Add<Output = T> + Clone> AddAssign for Magnitude<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<T: Sub<Output = T> + Clone> SubAssign for Magnitude<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<T: Mul<Output = T> + Clone + PartialOrd + Zero> MulAssign for Magnitude<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<T: Div<Output = T> + Clone + PartialOrd + Zero> DivAssign for Magnitude<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_comparisons() {
        let pos_inf = Magnitude::<usize>::PosInfinite;
        let neg_inf = Magnitude::<usize>::NegInfinite;

        assert!(pos_inf > neg_inf);
        assert!(neg_inf < pos_inf);

        let two = Magnitude::Finite(2);
        assert!(pos_inf > two);
        assert!(neg_inf < two);
        assert!(two == two);
        assert!(pos_inf != two);
        assert!(neg_inf != two);
        assert!(pos_inf != neg_inf);
    }

    #[test]
    #[should_panic(expected = "Can not compare two positive infinities")]
    fn compare_two_positive_infinities() {
        let pos_inf = Magnitude::<usize>::PosInfinite;

        let _ = pos_inf == pos_inf;
    }

    #[test]
    #[should_panic(expected = "Can not compare two negative infinities")]
    fn compare_two_negative_infinities() {
        let neg_inf = Magnitude::<usize>::NegInfinite;

        let _ = neg_inf == neg_inf;
    }

    #[test]
    fn correct_add() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let two = Magnitude::Finite(2);
        let neg_two = Magnitude::Finite(-2);
        let zero = Magnitude::Finite(0);

        assert!((pos_inf + two).is_pos_infinite());
        assert!((pos_inf + neg_two).is_pos_infinite());

        assert!((neg_inf + two).is_neg_infinite());
        assert!((neg_inf + neg_two).is_neg_infinite());

        assert!(two + neg_two == zero);
    }

    #[test]
    #[should_panic(expected = "Can not add PosInfinite and NegInfinite")]
    fn add_positive_and_negative_inf() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let _ = pos_inf + neg_inf;
    }

    #[test]
    fn correct_sub() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let two = Magnitude::Finite(2);
        let neg_two = Magnitude::Finite(-2);
        let four = Magnitude::Finite(4);
        let neg_four = Magnitude::Finite(-4);
        let zero = Magnitude::Finite(0);

        assert!((pos_inf - two).is_pos_infinite());
        assert!((pos_inf - neg_two).is_pos_infinite());
        assert!((two - pos_inf).is_neg_infinite());
        assert!((neg_two - pos_inf).is_neg_infinite());

        assert!((neg_inf - two).is_neg_infinite());
        assert!((neg_inf - neg_two).is_neg_infinite());
        assert!((two - neg_inf).is_pos_infinite());
        assert!((neg_two - neg_inf).is_pos_infinite());

        assert!(two - neg_two == four);
        assert!(neg_two - two == neg_four);
        assert!(two - zero == two);
    }

    #[test]
    #[should_panic(expected = "Can not subtract two PosInfinities")]
    fn subtract_two_positive_infinities() {
        let pos_inf = Magnitude::<i32>::PosInfinite;

        let _ = pos_inf - pos_inf;
    }

    #[test]
    #[should_panic(expected = "Can not subtract two NegInfinities")]
    fn subtract_two_negative_infinities() {
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let _ = neg_inf - neg_inf;
    }

    #[test]
    fn correct_multiply() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let two = Magnitude::Finite(2);
        let neg_two = Magnitude::Finite(-2);
        let neg_four = Magnitude::Finite(-4);
        let zero = Magnitude::Finite(0);

        assert!((pos_inf * pos_inf).is_pos_infinite());
        assert!((pos_inf * neg_inf).is_neg_infinite());
        assert!((neg_inf * pos_inf).is_neg_infinite());
        assert!((neg_inf * neg_inf).is_pos_infinite());

        assert!((pos_inf * two).is_pos_infinite());
        assert!((pos_inf * neg_two).is_neg_infinite());

        assert!((neg_inf * two).is_neg_infinite());
        assert!((neg_inf * neg_two).is_pos_infinite());

        assert!(two * neg_two == neg_four);
        assert!(two * zero == zero);
    }

    #[test]
    #[should_panic(expected = "Can not multiply zero by Infinity")]
    fn multiply_pos_infinity_by_zero() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let zero = Magnitude::<i32>::Finite(0);

        let _ = pos_inf * zero;
    }

    #[test]
    #[should_panic(expected = "Can not multiply zero by Infinity")]
    fn multiply_neg_infinity_by_zero() {
        let neg_inf = Magnitude::<i32>::NegInfinite;
        let zero = Magnitude::<i32>::Finite(0);

        let _ = neg_inf * zero;
    }

    #[test]
    fn correct_div() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let two = Magnitude::Finite(2);
        let neg_two = Magnitude::Finite(-2);
        let one = Magnitude::Finite(1);
        let neg_one = Magnitude::Finite(-1);
        let zero = Magnitude::Finite(0);

        assert!((pos_inf / two).is_pos_infinite());
        assert!((pos_inf / neg_two).is_neg_infinite());
        assert!(zero / pos_inf == zero);

        assert!((neg_inf / two).is_neg_infinite());
        assert!((neg_inf / neg_two).is_pos_infinite());
        assert!(zero / neg_inf == zero);

        assert!(two / two == one);
        assert!(two / neg_two == neg_one);
        assert!(zero / two == zero);
    }

    #[test]
    #[should_panic(expected = "Can not divide non-zero number by infinity")]
    fn divide_non_zero_number_by_pos_infinity() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let one = Magnitude::Finite(1);

        let _ = one / pos_inf;
    }

    #[test]
    #[should_panic(expected = "Can not divide non-zero number by infinity")]
    fn divide_non_zero_number_by_neg_infinity() {
        let neg_inf = Magnitude::<i32>::NegInfinite;
        let one = Magnitude::Finite(1);

        let _ = one / neg_inf;
    }

    #[test]
    #[should_panic(expected = "Can not divide two infinities")]
    fn divide_two_pos_infinities() {
        let pos_inf = Magnitude::<i32>::PosInfinite;

        let _ = pos_inf / pos_inf;
    }

    #[test]
    #[should_panic(expected = "Can not divide two infinities")]
    fn divide_two_neg_infinities() {
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let _ = neg_inf / neg_inf;
    }

    #[test]
    #[should_panic(expected = "Can not divide two infinities")]
    fn divide_pos_inf_by_neg_int() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let _ = pos_inf / neg_inf;
    }

    #[test]
    #[should_panic(expected = "Can not divide two infinities")]
    fn divide_neg_inf_by_pos_int() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let _ = neg_inf / pos_inf;
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn divide_pos_inf_by_zero() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let zero = Magnitude::Finite(0);

        let _ = pos_inf / zero;
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn divide_neg_inf_by_zero() {
        let neg_inf = Magnitude::<i32>::NegInfinite;
        let zero = Magnitude::Finite(0);

        let _ = neg_inf / zero;
    }

    #[test]
    fn neg_operator() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let neg_inf = Magnitude::<i32>::NegInfinite;

        let two = Magnitude::Finite(2);
        let neg_two = Magnitude::Finite(-2);
        let zero = Magnitude::Finite(0);

        assert!((-pos_inf).is_neg_infinite());
        assert!((-neg_inf).is_pos_infinite());

        assert!(-two == neg_two);
        assert!(-neg_two == two);
        assert!(-zero == zero);
    }

    #[test]
    fn as_ref() {
        let pos_inf = Magnitude::<i32>::PosInfinite;
        let two = Magnitude::Finite(2);

        assert_eq!(pos_inf.as_ref(), None);
        assert_eq!(two.as_ref(), Some(&2));
    }

    #[test]
    fn as_ref_mut() {
        let mut pos_inf = Magnitude::<i32>::PosInfinite;
        let mut two = Magnitude::Finite(2);

        assert_eq!(pos_inf.as_ref_mut(), None);
        assert_eq!(two.as_ref_mut(), Some(&mut 2));
    }

    #[test]
    fn collect_to_vec_of_magnitude() {
        let numbers = [0, 1, 2, 3, 4];

        let mags: Vec<Magnitude<i32>> = numbers
            .iter()
            .map(|value| Magnitude::Finite(*value))
            .collect();

        for i in 0..4 {
            assert!(*mags[i].as_ref().unwrap() == i as i32);
        }
    }
}
