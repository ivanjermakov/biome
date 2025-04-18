use super::{
    PatternCompiler, compilation_context::NodeCompilationContext,
    predicate_compiler::PredicateCompiler,
};
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::GritPatternWhere;
use grit_pattern_matcher::pattern::Where;

pub(crate) struct WhereCompiler;

impl WhereCompiler {
    pub(crate) fn from_node(
        node: &GritPatternWhere,
        context: &mut NodeCompilationContext,
    ) -> Result<Where<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_node(&node.pattern()?, context)?;
        let side_condition = PredicateCompiler::from_node(&node.side_condition()?, context)?;

        Ok(Where::new(pattern, side_condition))
    }
}
