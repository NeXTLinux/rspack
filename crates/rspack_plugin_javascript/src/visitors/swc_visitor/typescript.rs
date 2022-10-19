use std::sync::Arc;
use swc_common::{comments::SingleThreadedComments, Mark, SourceMap};
use swc_ecma_transforms::{
  react::{default_pragma, default_pragma_frag},
  typescript::{self, TsEnumConfig, TsImportExportAssignConfig},
};
use swc_ecma_visit::Fold;

pub fn typescript<'a>(
  top_level_mark: Mark,
  comments: Option<&'a SingleThreadedComments>,
  cm: &Arc<SourceMap>,
) -> impl Fold + 'a {
  /*  let import_export_assign_config = match module {
    Some(ModuleConfig::Es6) => TsImportExportAssignConfig::EsNext,
    Some(ModuleConfig::CommonJs(..))
    | Some(ModuleConfig::Amd(..))
    | Some(ModuleConfig::Umd(..)) => TsImportExportAssignConfig::Preserve,
    Some(ModuleConfig::NodeNext) => TsImportExportAssignConfig::NodeNext,
    // TODO: should Preserve for SystemJS
    _ => TsImportExportAssignConfig::Classic,
  };*/

  typescript::strip_with_jsx(
    cm.clone(),
    typescript::Config {
      pragma: Some(default_pragma()),
      pragma_frag: Some(default_pragma_frag()),
      ts_enum_config: TsEnumConfig {
        treat_const_enum_as_enum: false,
        ts_enum_is_readonly: false,
      },
      use_define_for_class_fields: true,
      import_export_assign_config: TsImportExportAssignConfig::Classic,
      ..Default::default()
    },
    comments,
    top_level_mark,
  )
}