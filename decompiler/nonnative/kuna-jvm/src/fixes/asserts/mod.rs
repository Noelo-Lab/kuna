//! Fix family `asserts` (Java output): `assert` statements: the `if` and
//! `throw` an assert is rebuilt from, and places that need an expression.

pub(crate) mod assert_dead_throw;
pub(crate) mod assert_throw_after_if;
pub(crate) mod statement_as_expression;
