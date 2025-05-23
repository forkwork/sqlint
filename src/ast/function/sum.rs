use super::Function;
use crate::ast::Expression;

/// A represention of the `SUM` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Sum<'a> {
    pub(crate) expr: Box<Expression<'a>>,
}

/// Calculates the sum value of a numeric column.
///
/// ```rust
/// # use sqlint::{ast::*, visitor::{Visitor, Sqlite}, col};
/// # fn main() -> Result<(), sqlint::error::Error> {
/// let query = Select::from_table("users").value(sum(col!("age")).alias("sum"));
/// let (sql, _) = Sqlite::build(query)?;
/// assert_eq!("SELECT SUM(`age`) AS `sum` FROM `users`", sql);
/// # Ok(())
/// # }
/// ```
pub fn sum<'a, E>(expr: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = Sum { expr: Box::new(expr.into()) };

    fun.into()
}
