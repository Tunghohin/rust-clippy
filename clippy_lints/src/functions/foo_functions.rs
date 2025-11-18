use rustc_ast::ItemKind;
use rustc_hir::FnDecl;
use rustc_lint::{LateContext, LintContext};

use super::FOO_FUNCTIONS;

// TODO: Adjust the parameters as necessary
pub(super) fn check_fn(cx: &LateContext<'_>, decl: &'_ FnDecl<'_>) {}
