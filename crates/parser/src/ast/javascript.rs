use crate::ast::*;

pub enum Expression {
	Identifier(IdentifierExpression),
	Literal(Literal),
	Unary(UnaryExpression),
	Binary(BinaryExpression),
	Conditional(ConditionalExpression),
	Call(CallExpression),
	New(NewExpression),
	Member(MemberExpression),
	Sequence(SequenceExpression),
	Function(FunctionExpression),
	Assignment(AssignmentExpression),
	Spread(SpreadExpression),
	Template(TemplateLiteral),
	TaggedTemplate(TaggedTemplateExpression),
	Object(ObjectExpression),
	Array(ArrayExpression),
	Parenthesized(ParenthesizedExpression),
}

pub enum Literal {
	String(StringLiteral),
	Number(NumberLiteral),
	Boolean(BooleanLiteral),
	Null(NullLiteral),
}

pub struct StringLiteral {
	pub location: Location,
	pub value: String,
	pub quote: char,
}

pub struct NumberLiteral {
	pub location: Location,
	pub value: f64,
}

pub struct BooleanLiteral {
	pub location: Location,
	pub value: bool,
}

pub struct NullLiteral {
	pub location: Location,
}

pub enum UnaryOperator {
	Minus,
	Plus,
	Increment,
	Decrement,
	Not,
	BitwiseNot,
	Typeof,
	Void,
	Delete,
}

pub enum BinaryOperator {
	Plus,
	Minus,
	Multiply,
	Divide,
	Modulo,
	Exponent,
	LeftShift,
	RightShift,
	UnsignedRightShift,
	LessThan,
	GreaterThan,
	LessThanOrEqual,
	GreaterThanOrEqual,
	Equal,
	NotEqual,
	StrictEqual,
	StrictNotEqual,
	BitwiseAnd,
	BitwiseOr,
	BitwiseXor,
	LogicalAnd,
	LogicalOr,
	InstanceOf,
	In,
}

pub enum AssignmentOperator {
	Equal,
	PlusEqual,
	MinusEqual,
	MultiplyEqual,
	DivideEqual,
	ModuloEqual,
	ExponentEqual,
	LeftShiftEqual,
	RightShiftEqual,
	UnsignedRightShiftEqual,
	BitwiseAndEqual,
	BitwiseOrEqual,
	BitwiseXorEqual,
}

pub struct IdentifierExpression {
	pub location: Location,
	pub name: String,
}

pub struct UnaryExpression {
	pub location: Location,
	pub operator: UnaryOperator,
	pub operand: Box<Expression>,
}

pub struct BinaryExpression {
	pub location: Location,
	pub operator: BinaryOperator,
	pub left: Box<Expression>,
	pub right: Box<Expression>,
}

pub struct ConditionalExpression {
	pub location: Location,
	pub condition: Box<Expression>,
	pub consequent: Box<Expression>,
	pub alternate: Box<Expression>,
}

pub struct CallExpression {
	pub location: Location,
	pub callee: Box<Expression>,
	pub arguments: Vec<Expression>,
}

pub struct NewExpression {
	pub location: Location,
	pub callee: Box<Expression>,
	pub arguments: Vec<Expression>,
}

pub struct MemberExpression {
	pub location: Location,
	pub object: Box<Expression>,
	pub property: Box<Expression>,
	pub computed: bool,
	pub optional: bool
}

pub struct SequenceExpression {
	pub location: Location,
	pub expressions: Vec<Expression>,
}

pub struct FunctionExpression {
	pub location: Location,
	pub parameters: Vec<Identifier>,
	pub body: Box<Expression>,
}

pub struct AssignmentExpression {
	pub location: Location,
	pub operator: AssignmentOperator,
	pub left: Box<Expression>,
	pub right: Box<Expression>,
}

pub struct SpreadExpression {
	pub location: Location,
	pub argument: Box<Expression>,
}

pub struct TemplateLiteral {
	pub location: Location,
	pub raw: String,
}

pub struct TaggedTemplateExpression {
	pub location: Location,
	pub tag: Box<Expression>,
	pub quasi: TemplateLiteral,
}

pub struct ObjectExpression {
	pub location: Location,
	pub properties: Vec<ObjectProperty>,
}

pub struct ArrayExpression {
	pub location: Location,
	pub elements: Vec<Expression>,
}

pub struct TemplateElement {
	pub location: Location,
	pub value: String,
	pub tail: bool,
}

pub struct ObjectProperty {
	pub location: Location,
	pub key: Option<Expression>,
	pub value: Expression,
}

pub struct ParenthesizedExpression {
	pub location: Location,
	pub expression: Box<Expression>,
}
