use fervid_core::ElementNode;
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{ArrayLit, Expr, ExprOrSpread, Ident},
};

use crate::{imports::VueImports, CodegenContext};

impl CodegenContext {
    /// Generates `(_openBlock(), _createBlock(_KeepAlive, null, [keepalive_children], 1024))`
    pub fn generate_keepalive(&mut self, element_node: &ElementNode) -> Expr {
        let span = DUMMY_SP; // TODO

        // _KeepAlive
        let keepalive_identifier = Expr::Ident(Ident {
            span,
            sym: self.get_and_add_import_ident(VueImports::KeepAlive),
            optional: false,
        });

        let keepalive_attrs =
            self.generate_builtin_attrs(&element_node.starting_tag.attributes, span);

        let generated_children = self.generate_element_children(element_node, false);
        let keepalive_children = if !generated_children.0.is_empty() {
            Some(Expr::Array(ArrayLit {
                span,
                elems: generated_children
                    .0
                    .into_iter()
                    .map(|c| {
                        Some(ExprOrSpread {
                            spread: None,
                            expr: Box::new(c),
                        })
                    })
                    .collect(),
            }))
        } else {
            None
        };

        let should_wrap_in_block = keepalive_children.is_some();
        let patch_flag = if should_wrap_in_block { 1024 } else { 0 };

        self.generate_componentlike(
            keepalive_identifier,
            keepalive_attrs,
            keepalive_children,
            patch_flag,
            should_wrap_in_block,
            span,
        )
    }
}

#[cfg(test)]
mod tests {
    use fervid_core::{AttributeOrBinding, BuiltinType, ElementKind, Node, StartingTag};

    use crate::test_utils::js;

    use super::*;

    #[test]
    fn it_generates_empty_keepalive() {
        // <keep-alive></keep-alive>
        test_out(
            ElementNode {
                kind: ElementKind::Builtin(BuiltinType::KeepAlive),
                starting_tag: StartingTag {
                    tag_name: "keep-alive",
                    attributes: vec![],
                    directives: None,
                },
                children: vec![],
                template_scope: 0,
            },
            r#"_createVNode(_KeepAlive)"#,
        )
    }

    #[test]
    fn it_generates_keepalive_attrs() {
        // <keep-alive foo="bar" :baz="qux"></keep-alive>
        test_out(
            ElementNode {
                kind: ElementKind::Builtin(BuiltinType::KeepAlive),
                starting_tag: StartingTag {
                    tag_name: "keep-alive",
                    attributes: vec![
                        AttributeOrBinding::RegularAttribute {
                            name: "foo",
                            value: "bar",
                        },
                        AttributeOrBinding::VBind(fervid_core::VBindDirective {
                            argument: Some("baz".into()),
                            value: js("qux"),
                            is_camel: false,
                            is_prop: false,
                            is_attr: false,
                        }),
                    ],
                    directives: None,
                },
                children: vec![],
                template_scope: 0,
            },
            r#"_createVNode(_KeepAlive,{foo:"bar",baz:qux})"#,
        )
    }

    #[test]
    fn it_generates_keepalive_children() {
        // <keep-alive>foobar</keep-alive>
        test_out(
            ElementNode {
                kind: ElementKind::Builtin(BuiltinType::KeepAlive),
                starting_tag: StartingTag {
                    tag_name: "keep-alive",
                    attributes: vec![],
                    directives: None,
                },
                children: vec![Node::Text("foobar")],
                template_scope: 0,
            },
            r#"(_openBlock(),_createBlock(_KeepAlive,null,[_createTextVNode("foobar")],1024))"#,
        )
    }

    #[test]
    fn it_generates_full_keepalive() {
        // <keep-alive foo="bar" :baz="qux">foobar</keep-alive>
        test_out(
            ElementNode {
                kind: ElementKind::Builtin(BuiltinType::KeepAlive),
                starting_tag: StartingTag {
                    tag_name: "keep-alive",
                    attributes: vec![
                        AttributeOrBinding::RegularAttribute {
                            name: "foo",
                            value: "bar",
                        },
                        AttributeOrBinding::VBind(fervid_core::VBindDirective {
                            argument: Some("baz".into()),
                            value: js("qux"),
                            is_camel: false,
                            is_prop: false,
                            is_attr: false,
                        }),
                    ],
                    directives: None,
                },
                children: vec![Node::Text("foobar")],
                template_scope: 0,
            },
            r#"(_openBlock(),_createBlock(_KeepAlive,{foo:"bar",baz:qux},[_createTextVNode("foobar")],1024))"#,
        )
    }

    fn test_out(input: ElementNode, expected: &str) {
        let mut ctx = CodegenContext::default();
        let out = ctx.generate_keepalive(&input);
        assert_eq!(crate::test_utils::to_str(out), expected)
    }
}
