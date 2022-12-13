use crate::*;
use ast::javascript as ast;

fn parse(input: &str) -> ast::Expression {
	let mut parser = Parser::new(input.to_owned());
	let expression = parser.parse_javascript().unwrap();
	assert!(parser.scanner.is_done());
	expression
}

#[test]
fn test_parse_identifier() {
	match parse("foo") {
		ast::Expression::Identifier(expr) => {
			assert_eq!(expr.name, "foo");
		}
		_ => panic!("Expected IdentifierExpression"),
	}
}

#[test]
fn test_parse_string_literal() {
	match parse("'foo'") {
		ast::Expression::Literal(expr) => match expr {
			ast::Literal::String(literal) => {
				assert_eq!(literal.value, "foo");
				assert_eq!(literal.quote, '\'');
			}
			_ => panic!("Expected StringLiteral"),
		},
		_ => panic!("Expected Literal"),
	}
}

#[test]
fn test_parse_number_literal() {
	match parse("123") {
		ast::Expression::Literal(expr) => match expr {
			ast::Literal::Number(literal) => {
				assert_eq!(literal.value, 123_f64);
			}
			_ => panic!("Expected NumberLiteral"),
		},
		_ => panic!("Expected Literal"),
	}
}

#[test]
fn test_parse_bool_literal() {
	match parse("false") {
		ast::Expression::Literal(expr) => match expr {
			ast::Literal::Boolean(literal) => {
				assert_eq!(literal.value, false);
			}
			_ => panic!("Expected BooleanLiteral"),
		},
		_ => panic!("Expected Literal"),
	}
}

#[test]
fn test_parse_null_literal() {
	match parse("null") {
		ast::Expression::Literal(expr) => match expr {
			ast::Literal::Null(_) => {}
			_ => panic!("Expected NullLiteral"),
		},
		_ => panic!("Expected Literal"),
	}
}

#[test]
fn test_parse_unary() {
	match parse("void foo") {
		ast::Expression::Unary(expr) => {
			assert!(matches!(expr.operator, ast::UnaryOperator::Void));
			match *expr.operand {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
		}
		_ => panic!("Expected UnaryExpression"),
	}
}

#[test]
fn test_parse_binary() {
	match parse("foo + bar") {
		ast::Expression::Binary(expr) => {
			assert!(matches!(expr.operator, ast::BinaryOperator::Plus));
			match *expr.left {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
			match *expr.right {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
		}
		_ => panic!("Expected BinaryExpression"),
	}
}

#[test]
fn test_parse_conditional() {
	match parse("foo ? bar : baz") {
		ast::Expression::Conditional(expr) => {
			match *expr.condition {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
			match *expr.consequent {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
			match *expr.alternate {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "baz");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
		}
		_ => panic!("Expected ConditionalExpression"),
	}
}

#[test]
fn test_parse_call() {
	match parse("foo(bar, baz)") {
		ast::Expression::Call(expr) => {
			assert_eq!(expr.arguments.len(), 2);
			match *expr.callee {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
			match expr.arguments.get(0) {
				Some(ast::Expression::Identifier(expr)) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
			match expr.arguments.get(1) {
				Some(ast::Expression::Identifier(expr)) => {
					assert_eq!(expr.name, "baz");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
		}
		_ => panic!("Expected CallExpression"),
	}
}

#[test]
fn test_parse_call_without_args() {
	match parse("foo()") {
		ast::Expression::Call(expr) => {
			assert_eq!(expr.arguments.len(), 0);
			match *expr.callee {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
		}
		_ => panic!("Expected CallExpression"),
	}
}

#[test]
fn test_parse_new() {
	match parse("new Foo(bar, baz)") {
		ast::Expression::New(expr) => {
			assert_eq!(expr.arguments.len(), 2);
			match *expr.callee {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "Foo");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
			match expr.arguments.get(0) {
				Some(ast::Expression::Identifier(expr)) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
			match expr.arguments.get(1) {
				Some(ast::Expression::Identifier(expr)) => {
					assert_eq!(expr.name, "baz");
				}
				_ => panic!("Expected IdentifierExpression"),
			}
		}
		_ => panic!("Expected NewExpression"),
	}
}

#[test]
fn test_parse_member() {
	match parse("foo.bar") {
		ast::Expression::Member(expr) => {
			assert_eq!(expr.optional, false);
			assert_eq!(expr.computed, false);
			match *expr.object {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
			match *expr.property {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected MemberExpression"),
	}
}

#[test]
fn test_parse_member_triple() {
	match parse("foo.bar.baz") {
		ast::Expression::Member(expr) => {
			assert_eq!(expr.optional, false);
			assert_eq!(expr.computed, false);
			match *expr.object {
				ast::Expression::Member(member) => {
					assert_eq!(member.optional, false);
					assert_eq!(member.computed, false);
					match *member.object {
						ast::Expression::Identifier(expr) => {
							assert_eq!(expr.name, "foo");
						}
						_ => panic!("Expected Identifier"),
					}
					match *member.property {
						ast::Expression::Identifier(expr) => {
							assert_eq!(expr.name, "bar");
						}
						_ => panic!("Expected Identifier"),
					}
				}
				_ => panic!("Expected Member"),
			}
			match *expr.property {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "baz");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected Member"),
	}
}

#[test]
fn test_parse_optional_member() {
	match parse("foo?.bar") {
		ast::Expression::Member(expr) => {
			assert_eq!(expr.optional, true);
			assert_eq!(expr.computed, false);
			match *expr.object {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
			match *expr.property {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected MemberExpression"),
	}
}

#[test]
fn test_parse_computed_member() {
	match parse("foo[bar]") {
		ast::Expression::Member(expr) => {
			assert_eq!(expr.optional, false);
			assert_eq!(expr.computed, true);
			match *expr.object {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
			match *expr.property {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected MemberExpression"),
	}
}

#[test]
fn test_parse_optional_computed_member() {
	match parse("foo?.[bar]") {
		ast::Expression::Member(expr) => {
			assert_eq!(expr.optional, true);
			assert_eq!(expr.computed, true);
			match *expr.object {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
			match *expr.property {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected MemberExpression"),
	}
}

#[test]
fn test_parse_function() {
	match parse("(foo) => bar") {
		ast::Expression::Function(expr) => {
			assert_eq!(expr.parameters.len(), 1);

			match expr.parameters.get(0).unwrap() {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}

			match *expr.body {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected FunctionExpression"),
	}
}

#[test]
fn test_parse_parenthesized() {
	match parse("(foo)") {
		ast::Expression::Parenthesized(expr) => match *expr.expression {
			ast::Expression::Identifier(expr) => {
				assert_eq!(expr.name, "foo");
			}
			_ => panic!("Expected Identifier"),
		},
		_ => panic!("Expected ParenthesizedExpression"),
	}
}

#[test]
fn test_parse_assignment() {
	match parse("foo = bar") {
		ast::Expression::Assignment(expr) => {
			assert!(matches!(expr.operator, ast::AssignmentOperator::Equal));
			match *expr.left {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
			match *expr.right {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected AssignmentExpression"),
	}
}

#[test]
fn test_parse_spread() {
	match parse("foo(...bar)") {
		ast::Expression::Call(expr) => {
			assert_eq!(expr.arguments.len(), 1);
			match expr.arguments.get(0).unwrap() {
				ast::Expression::Spread(expr) => match expr.argument.as_ref() {
					ast::Expression::Identifier(expr) => {
						assert_eq!(expr.name, "bar");
					}
					_ => panic!("Expected Identifier"),
				},
				_ => panic!("Expected SpreadExpression"),
			}
		}
		_ => panic!("Expected CallExpression"),
	}
}

#[test]
fn test_parse_template() {
	match parse("`foo`") {
		ast::Expression::Template(expr) => {
			assert_eq!(expr.raw, "foo");
		}
		_ => panic!("Expected TemplateExpression"),
	}
}

#[test]
fn test_parse_tagged_template() {
	match parse("foo`bar`") {
		ast::Expression::TaggedTemplate(expr) => {
			assert_eq!(expr.quasi.raw, "bar");
			match *expr.tag {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected TaggedTemplateExpression"),
	}
}

#[test]
fn test_parse_object() {
	match parse("{foo: bar, [baz]: qux}") {
		ast::Expression::Object(expr) => {
			assert_eq!(expr.properties.len(), 2);

			match &expr.properties.get(0).unwrap() {
				ast::ObjectProperty::Keyed(expr) => {
					assert_eq!(expr.key.name, "foo");

					match &expr.value {
						Some(ast::Expression::Identifier(expr)) => {
							assert_eq!(expr.name, "bar");
						}
						_ => panic!("Expected Identifier"),
					}
				}
				_ => panic!("Expected keyed object property"),
			}

			match &expr.properties.get(1).unwrap() {
				ast::ObjectProperty::Computed(expr) => {
					match &*expr.key {
						ast::Expression::Identifier(expr) => {
							assert_eq!(expr.name, "baz");
						}
						_ => panic!("Expected Identifier"),
					}

					match &*expr.value {
						ast::Expression::Identifier(expr) => {
							assert_eq!(expr.name, "qux");
						}
						_ => panic!("Expected Identifier"),
					}
				}
				_ => panic!("Expected keyed object property"),
			}
		}
		_ => panic!("Expected ObjectExpression"),
	}
}

#[test]
fn test_parse_object_trailing_comma() {
	match parse("{foo: bar,}") {
		ast::Expression::Object(expr) => {
			assert_eq!(expr.properties.len(), 1);

			match expr.properties.get(0).unwrap() {
				ast::ObjectProperty::Keyed(expr) => {
					assert_eq!(expr.key.name, "foo");

					match &expr.value {
						Some(ast::Expression::Identifier(expr)) => {
							assert_eq!(expr.name, "bar");
						}
						_ => panic!("Expected Identifier"),
					}
				}
				_ => panic!("Expected keyed object property"),
			}
		}
		_ => panic!("Expected ObjectExpression"),
	}
}

#[test]
fn test_parse_array() {
	match parse("[foo, bar]") {
		ast::Expression::Array(expr) => {
			assert_eq!(expr.elements.len(), 2);
			match expr.elements.get(0).unwrap() {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
			match expr.elements.get(1).unwrap() {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected ArrayExpression"),
	}
}

#[test]
fn test_parse_array_trailing_comma() {
	match parse("[foo, bar,]") {
		ast::Expression::Array(expr) => {
			assert_eq!(expr.elements.len(), 2);
			match expr.elements.get(0).unwrap() {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
			match expr.elements.get(1).unwrap() {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected ArrayExpression"),
	}
}
