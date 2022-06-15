use swc_common::SyntaxContext;
use {
  swc_ecma_ast::Ident,
  swc_ecma_visit::{noop_visit_mut_type, VisitMut},
};

#[derive(Clone, Copy)]
pub struct ClearMark;
impl VisitMut for ClearMark {
  noop_visit_mut_type!();

  fn visit_mut_ident(&mut self, ident: &mut Ident) {
    ident.span.ctxt = SyntaxContext::empty();
  }
}