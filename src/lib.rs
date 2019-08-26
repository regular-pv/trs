extern crate terms;

use std::collections::HashSet;
use terms::Pattern;

/// Rewriting rule.
pub type Rule<F, X> = (Pattern<F, X>, Pattern<F, X>);

/// Term rewriting system.
pub type TRS<F, X> = HashSet<Rule<F, X>>;
