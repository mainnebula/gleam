use insta::assert_debug_snapshot;
use lsp_types::{FoldingRange, FoldingRangeParams};

use super::*;

fn folding_ranges(tester: TestProject<'_>) -> Vec<FoldingRange> {
    tester.at(Position::default(), |engine, param, _| {
        let params = FoldingRangeParams {
            text_document: param.text_document,
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        let response = engine.folding_range(params);

        response.result.unwrap()
    })
}

#[test]
fn no_folding_ranges_for_empty_module() {
    let code = "";

    assert_debug_snapshot!(folding_ranges(TestProject::for_source(code)))
}

#[test]
fn no_folding_ranges_for_single_import() {
    let code = "import dep/module_a";

    assert_debug_snapshot!(folding_ranges(
        TestProject::for_source(code).add_dep_module("module_a", "")
    ))
}

#[test]
fn folding_range_for_multiple_imports() {
    let code = "import dep/module_a
import dep/module_b
import dep/module_c";

    assert_debug_snapshot!(folding_ranges(
        TestProject::for_source(code)
            .add_dep_module("module_a", "")
            .add_dep_module("module_b", "")
            .add_dep_module("module_c", "")
    ))
}

#[test]
fn folding_range_for_function() {
    let code = "
pub fn hello() -> String {
  \"hello\"
}";

    assert_debug_snapshot!(folding_ranges(TestProject::for_source(code)))
}

#[test]
fn no_folding_range_for_single_line_function() {
    let code = "
pub fn hello() -> String { \"hello\" }";

    assert_debug_snapshot!(folding_ranges(TestProject::for_source(code)))
}

#[test]
fn folding_range_for_custom_type() {
    let code = "
pub type Color {
  Red
  Green
  Blue
}";

    assert_debug_snapshot!(folding_ranges(TestProject::for_source(code)))
}

#[test]
fn folding_range_for_multiline_constant() {
    let code = "
pub const colors = [
  \"red\",
  \"green\",
  \"blue\",
]";

    assert_debug_snapshot!(folding_ranges(TestProject::for_source(code)))
}

#[test]
fn no_folding_range_for_single_line_constant() {
    let code = "
pub const name = \"gleam\"";

    assert_debug_snapshot!(folding_ranges(TestProject::for_source(code)))
}

#[test]
fn folding_range_for_type_alias() {
    let code = "
pub type Headers =
  List(#(String, String))";

    assert_debug_snapshot!(folding_ranges(TestProject::for_source(code)))
}

#[test]
fn folding_ranges_for_mixed_definitions() {
    let code = "import dep/module_a
import dep/module_b

pub type Color {
  Red
  Green
}

pub fn hello() -> String {
  \"hello\"
}

pub const name = \"gleam\"";

    assert_debug_snapshot!(folding_ranges(
        TestProject::for_source(code)
            .add_dep_module("module_a", "")
            .add_dep_module("module_b", "")
    ))
}
