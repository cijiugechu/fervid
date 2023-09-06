use fervid_core::{AttributeOrBinding, ElementNode};
use swc_core::{ecma::{ast::{Expr, ObjectLit, PropOrSpread, Prop, KeyValueProp, PropName, Ident, Lit, Number}, atoms::JsWord}, common::{Span, DUMMY_SP}};

use crate::CodegenContext;

impl CodegenContext {
    /// Generates attributes if any are present, returns `None` otherwise
    pub(crate) fn generate_builtin_attrs(&mut self, attributes: &[AttributeOrBinding], span: Span) -> Option<Expr> {
        if !attributes.is_empty() {
            let mut attrs = Vec::with_capacity(attributes.len());
            self.generate_attributes(&attributes, &mut attrs);
            Some(Expr::Object(ObjectLit { span, props: attrs }))
        } else {
            None
        }
    }

    /// Generates the slots expression for builtins.
    ///
    /// Additionally adds `_: 1` to the slots object.
    pub(crate) fn generate_builtin_slots(&mut self, element_node: &ElementNode) -> Option<Expr> {
        let mut slots = self.generate_component_children(element_node);
        if let Some(Expr::Object(ref mut obj)) = slots {
            obj.props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: JsWord::from("_"),
                    optional: false,
                }),
                value: Box::new(Expr::Lit(Lit::Num(Number {
                    span: DUMMY_SP,
                    value: 1.0,
                    raw: None,
                }))),
            }))));
        }

        slots
    }
}
