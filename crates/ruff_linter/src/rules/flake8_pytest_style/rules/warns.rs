use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::helpers::is_compound_statement;
use ruff_python_ast::{self as ast, Expr, Stmt, WithItem};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

use super::helpers::{is_non_trivial_with_body, is_pytest_warns};

/// ## What it does
/// Checks for `pytest.warns` context managers with multiple statements.
///
/// ## Why is this bad?
/// See [PytestRaisesWithMultipleStatements (PT012)](pytest-raises-with-multiple-statements.md).
///
/// ## References
/// - [`pytest` documentation: `pytest.warns`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-warns)
#[violation]
pub struct PytestWarnsWithMultipleStatements;

impl Violation for PytestWarnsWithMultipleStatements {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`pytest.warns()` block should contain a single simple statement".to_string()
    }
}

/// PT028
pub(crate) fn complex_warns(checker: &mut Checker, stmt: &Stmt, items: &[WithItem], body: &[Stmt]) {
    let warns_called = items.iter().any(|item| match &item.context_expr {
        Expr::Call(ast::ExprCall { func, .. }) => is_pytest_warns(func, checker.semantic()),
        _ => false,
    });

    // Check body for `pytest.warns` context manager
    if warns_called {
        let is_too_complex = if let [stmt] = body {
            match stmt {
                Stmt::With(ast::StmtWith { body, .. }) => is_non_trivial_with_body(body),
                // Allow function and class definitions to test decorators
                Stmt::ClassDef(_) | Stmt::FunctionDef(_) => false,
                stmt => is_compound_statement(stmt),
            }
        } else {
            true
        };

        if is_too_complex {
            checker.diagnostics.push(Diagnostic::new(
                PytestWarnsWithMultipleStatements,
                stmt.range(),
            ));
        }
    }
}
