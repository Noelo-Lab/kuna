//! Port of `org/jetbrains/java/decompiler/api/plugin/LanguageSpec.java`.

use std::rc::Rc;

use crate::class_pass::ClassPass;
use crate::decompiler_context::DecompilerContext;
use crate::graph_parser::GraphParser;
use crate::kotlin_chooser::KotlinChooser;
use crate::pass::Pass;
use crate::statement_writer::StatementWriter;
use crate::struct_class::StructClass;

/// A whole decompilation pipeline for a JVM language: the classes `chooser` claims skip the Java
/// method schedule and the Java writer. The one language is Kotlin.
pub struct LanguageSpec {
    pub chooser: KotlinChooser,
    pub graph_parser: Rc<dyn GraphParser>,
    pub writer: Rc<dyn StatementWriter>,
    pub pass: Rc<dyn Pass>,
    pub cfg_pass: Rc<dyn Pass>,
    pub root_processor: Rc<dyn ClassPass>,
    pub extension: String,
}

impl LanguageSpec {
    pub fn new(
        chooser: KotlinChooser,
        graph_parser: Rc<dyn GraphParser>,
        writer: Rc<dyn StatementWriter>,
        pass: Rc<dyn Pass>,
        cfg_pass: Rc<dyn Pass>,
        root_processor: Rc<dyn ClassPass>,
        extension: &str,
    ) -> LanguageSpec {
        LanguageSpec {
            chooser,
            graph_parser,
            writer,
            pass,
            cfg_pass,
            root_processor,
            extension: extension.to_string(),
        }
    }

    /// The pipeline that claims `cl` (`PluginContext.getLanguageSpec`): the
    /// session's Kotlin spec when its chooser takes the class, else `None` (Java).
    pub fn of(cl: StructClass) -> Option<Rc<LanguageSpec>> {
        let spec = DecompilerContext::get_struct_context().with(|d| d.language_spec.clone())?;
        spec.chooser.is_language(cl).then_some(spec)
    }
}
