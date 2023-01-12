use num::One as _;
use numwit::{Negative, Positive};

type PosU8 = Positive<u8>;
type PosI8 = Positive<i8>;
type NegI8 = Negative<i8>;

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Add       | `Positive` | `Positive`      | `Positive` | Yes         |
#[test]
fn add_pos_pos() {
    assert_eq!(PosU8::one() + PosU8::one(), 2);
}
#[test]
fn add_assign_pos_pos() {
    let mut n = PosU8::one();
    n += PosU8::one();
    assert_eq!(n, 2);
}
// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Add       | `Negative` | `Negative`      | `Negative` | Yes         |
#[test]
fn add_neg_neg() {
    assert_eq!(NegI8::one() + NegI8::one(), -2);
}
#[test]
fn add_assign_neg_neg() {
    let mut n = NegI8::one();
    n += NegI8::one();
    assert_eq!(n, -2);
}
// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Add       | `Positive` | `Negative`      | ?          | No          |
#[test]
fn add_pos_neg() {
    assert_eq!(PosI8::one() + NegI8::one(), 0);
}
// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Add       | `Negative` | `Positive`      | ?          | No          |
#[test]
fn add_neg_pos() {
    assert_eq!(NegI8::one() + PosI8::one(), 0);
}
// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Add       | `Positive` | `impl Unsigned` | `Positive` | Yes         |
#[test]
fn add_pos_unsigned() {
    assert_eq!(PosU8::one() + 1, 2);
}
#[test]
fn add_assign_pos_unsigned() {
    let mut n = PosU8::one();
    n += 1;
    assert_eq!(n, 2);
}
// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Add       | `Negative` | `impl Unsigned` | ?          | No          |
#[test]
#[ignore = "Negative<impl Unsigned> cannot be constructed"]
fn add_neg_unsigned() {
    // assert_eq!(NegI8::one() + 1, 0);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Sub       | `Positive` | `Positive`      | ?          | No          |
#[test]
fn sub_pos_pos() {
    assert_eq!(PosU8::one() - PosU8::one(), 0);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Sub       | `Negative` | `Negative`      | ?          | No          |
#[test]
fn sub_neg_neg() {
    assert_eq!(NegI8::one() - NegI8::one(), 0);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Sub       | `Positive` | `Negative`      | `Positive` | Yes         |
#[test]
fn sub_pos_neg() {
    assert_eq!(PosI8::one() - NegI8::one(), 2);
}

#[test]
fn sub_assign_pos_neg() {
    let mut n = PosI8::one();
    n -= NegI8::one();
    assert_eq!(n, 2);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Sub       | `Negative` | `Positive`      | `Negative` | Yes         |
#[test]
fn sub_neg_pos() {
    assert_eq!(NegI8::one() - PosI8::one(), -2);
}

#[test]
fn sub_assign_neg_pos() {
    let mut n = NegI8::one();
    n -= PosI8::one();
    assert_eq!(n, -2);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Sub       | `Positive` | `impl Unsigned` | ?          | No          |
#[test]
fn sub_pos_unsigned() {
    assert_eq!(PosU8::one() - 1, 0);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Sub       | `Negative` | `impl Unsigned` | `Negative` | Yes         |
#[test]
#[ignore = "Negative<impl Unsigned> cannot be constructed"]
fn sub_neg_unsigned() {
    // assert_eq!(NegI8::one() - 1, -2);
}

#[test]
#[ignore = "Negative<impl Unsigned> cannot be constructed"]
fn sub_assign_neg_unsigned() {
    // let mut n = NegI8::one();
    // n -= 1;
    // assert_eq!(n, -2);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Mul       | `Positive` | `Positive`      | `Positive` | Yes         |
#[test]
fn mul_pos_pos() {
    assert_eq!(PosU8::one() * PosU8::one(), 1);
}

#[test]
fn mul_assign_pos_pos() {
    let mut n = PosU8::one();
    n *= PosU8::one();
    assert_eq!(n, 1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Mul       | `Negative` | `Negative`      | `Positive` | No          |
#[test]
fn mul_neg_neg() {
    assert_eq!(NegI8::one() * NegI8::one(), 1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Mul       | `Positive` | `Negative`      | `Negative` | No          |
#[test]
fn mul_pos_neg() {
    assert_eq!(PosI8::one() * NegI8::one(), -1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Mul       | `Negative` | `Positive`      | `Negative` | Yes         |
#[test]
fn mul_neg_pos() {
    assert_eq!(NegI8::one() * PosI8::one(), -1);
}

#[test]
fn mul_assign_neg_pos() {
    let mut n = NegI8::one();
    n *= PosI8::one();
    assert_eq!(n, -1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Mul       | `Positive` | `impl Unsigned` | ?          | No          |
#[test]
fn mul_pos_unsigned() {
    assert_eq!(PosU8::one() * 0, 0);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Mul       | `Negative` | `impl Unsigned` | ?          | No          |
#[test]
#[ignore = "Negative<impl Unsigned> cannot be constructed"]
fn mul_neg_unsigned() {
    // assert_eq!(NegI8::one() * 0, 0);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Div       | `Positive` | `Positive`      | `Positive` | Yes         |
#[test]
fn div_pos_pos() {
    assert_eq!(PosI8::one() / PosI8::one(), 1);
}

#[test]
fn div_assign_pos_pos() {
    let mut n = PosI8::one();
    n /= PosI8::one();
    assert_eq!(n, 1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Div       | `Negative` | `Negative`      | `Positive` | No          |
#[test]
fn div_neg_neg() {
    assert_eq!(NegI8::one() / NegI8::one(), 1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Div       | `Positive` | `Negative`      | `Negative` | No          |
#[test]
fn div_pos_neg() {
    assert_eq!(PosI8::one() / NegI8::one(), -1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Div       | `Negative` | `Positive`      | `Negative` | Yes         |
#[test]
fn div_neg_pos() {
    assert_eq!(NegI8::one() / PosI8::one(), -1);
}

#[test]
fn div_assign_neg_pos() {
    let mut n = NegI8::one();
    n /= PosI8::one();
    assert_eq!(n, -1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Div       | `Positive` | `impl Unsigned` | `Positive` | Yes         |
#[test]
fn div_pos_unsigned() {
    assert_eq!(PosU8::one() / 1, 1)
}

#[test]
fn div_assign_pos_unsigned() {
    let mut n = PosU8::one();
    n /= 1;
    assert_eq!(n, 1);
}

// | Operation | LHS        | RHS             | Output     | Assignable? |
// | --------- | ---------- | --------------- | ---------- | ----------- |
// | Div       | `Negative` | `impl Unsigned` | `Negative` | Yes         |
#[test]
#[ignore = "Negative<impl Unsigned> cannot be constructed"]
fn div_neg_unsigned() {}

#[test]
#[ignore = "Negative<impl Unsigned> cannot be constructed"]
fn div_assign_neg_unsigned() {}
