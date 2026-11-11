//! Port of `org/jetbrains/java/decompiler/api/plugin/LanguageSpec.java`.

use std::rc::Rc;

use crate::class_pass::ClassPass;
use crate::graph_parser::GraphParser;
use crate::language_chooser::LanguageChooser;
use crate::pass::Pass;
use crate::statement_writer::StatementWriter;

/// A whole decompilation pipeline for a JVM language: the classes `chooser` claims skip the Java
/// method schedule and the Java writer.
pub struct LanguageSpec {
    pub name: String,
    pub chooser: Rc<dyn LanguageChooser>,
    pub graph_parser: Rc<dyn GraphParser>,
    pub writer: Rc<dyn StatementWriter>,
    pub pass: Rc<dyn Pass>,
    pub cfg_pass: Rc<dyn Pass>,
    pub root_processor: Rc<dyn ClassPass>,
    pub extension: String,
}

impl LanguageSpec {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        chooser: Rc<dyn LanguageChooser>,
        graph_parser: Rc<dyn GraphParser>,
        writer: Rc<dyn StatementWriter>,
        pass: Rc<dyn Pass>,
        cfg_pass: Rc<dyn Pass>,
        root_processor: Rc<dyn ClassPass>,
        extension: &str,
    ) -> LanguageSpec {
        LanguageSpec {
            name: name.to_string(),
            chooser,
            graph_parser,
            writer,
            pass,
            cfg_pass,
            root_processor,
            extension: extension.to_string(),
        }
    }
}
