use super::super::*;

fn parse(input: &str) -> ast::Expression {
	let mut parser = Parser::new(input);
	let expression = parser.parse_javascript_expression(&[]).unwrap();
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

#[ignore]
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

#[ignore]
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

#[ignore]
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

#[ignore]
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

#[ignore]
#[test]
fn test_parse_member() {
	match parse("foo.bar") {
		ast::Expression::Member(expr) => {
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

#[ignore]
#[test]
fn test_parse_sequence() {
	match parse("foo, bar, baz") {
		ast::Expression::Sequence(expr) => {
			assert_eq!(expr.expressions.len(), 3);
			match expr.expressions.get(0).unwrap() {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}
			match expr.expressions.get(1).unwrap() {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
			}
			match expr.expressions.get(2).unwrap() {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "baz");
				}
				_ => panic!("Expected Identifier"),
			}
		}
		_ => panic!("Expected SequenceExpression"),
	}
}

#[ignore]
#[test]
fn test_parse_function() {
	match parse("(foo) => bar") {
		ast::Expression::Function(expr) => {
			assert_eq!(expr.parameters.len(), 1);
			assert_eq!(expr.parameters.get(0).unwrap().name, "foo");

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

#[ignore]
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

#[ignore]
#[test]
fn test_parse_spread() {
	match parse("...foo") {
		ast::Expression::Spread(expr) => match *expr.argument {
			ast::Expression::Identifier(expr) => {
				assert_eq!(expr.name, "foo");
			}
			_ => panic!("Expected Identifier"),
		},
		_ => panic!("Expected SpreadExpression"),
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

#[ignore]
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
	match parse("{foo: bar}") {
		ast::Expression::Object(expr) => {
			assert_eq!(expr.properties.len(), 1);

			match &expr.properties.get(0).unwrap().key {
				Some(ast::Expression::Identifier(expr)) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}

			match &expr.properties.get(0).unwrap().value {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
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

			match &expr.properties.get(0).unwrap().key {
				Some(ast::Expression::Identifier(expr)) => {
					assert_eq!(expr.name, "foo");
				}
				_ => panic!("Expected Identifier"),
			}

			match &expr.properties.get(0).unwrap().value {
				ast::Expression::Identifier(expr) => {
					assert_eq!(expr.name, "bar");
				}
				_ => panic!("Expected Identifier"),
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
