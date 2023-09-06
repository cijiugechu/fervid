use swc_core::{ecma::{ast::{Expr, Lit, Str}, atoms::{JsWord, Atom}}, common::Span};

use crate::context::CodegenContext;

impl CodegenContext {
    pub fn generate_text_node(&mut self, contents: &str, span: Span) -> Expr {
        let has_start_whitespace = contents.starts_with(char::is_whitespace);
        let has_end_whitespace = contents.ends_with(char::is_whitespace);
        let needs_shortening = has_start_whitespace || has_end_whitespace;

        let value = if needs_shortening {
            let trimmed = contents.trim();
            let new_len = trimmed.len() + (has_start_whitespace as usize) + (has_end_whitespace as usize);

            // Re-create a string with all start and end whitespace replaced by a single space
            let mut shortened = String::with_capacity(new_len);
            if has_start_whitespace {
                shortened.push(' ');
            }
            shortened.push_str(trimmed);
            if has_end_whitespace && !trimmed.is_empty() {
                shortened.push(' ');
            }

            JsWord::from(shortened)
        } else {
            JsWord::from(contents)
        };

        Expr::Lit(Lit::Str(Str {
            span,
            value,
            raw: Some(Atom::from(contents)),
        }))
    }
}
