#[derive(Debug)]
pub enum Expression {
	Identifier(IdentifierExpression),
	Literal(Literal),
	Unary(UnaryExpression),
	Binary(BinaryExpression),
	Conditional(ConditionalExpression),
	Call(CallExpression),
	New(NewExpression),
	Member(MemberExpression),
	Function(FunctionExpression),
	Assignment(AssignmentExpression),
	Spread(SpreadExpression),
	Template(TemplateLiteral),
	TaggedTemplate(TaggedTemplateExpression),
	Object(ObjectExpression),
	Array(ArrayExpression),
	Parenthesized(ParenthesizedExpression),
	Empty(EmptyExpression),
}

impl Expression {
	pub fn start(&self) -> usize {
		match self {
			Expression::Literal(literal) => match literal {
				Literal::String(literal) => literal.start,
				Literal::Number(literal) => literal.start,
				Literal::Boolean(literal) => literal.start,
				Literal::Null(literal) => literal.start,
			},

			Expression::Identifier(expr) => expr.start,
			Expression::Unary(expr) => expr.start,
			Expression::Binary(expr) => expr.start,
			Expression::Conditional(expr) => expr.start,
			Expression::Call(expr) => expr.start,
			Expression::New(expr) => expr.start,
			Expression::Member(expr) => expr.start,
			Expression::Function(expr) => expr.start,
			Expression::Assignment(expr) => expr.start,
			Expression::Spread(expr) => expr.start,
			Expression::Template(expr) => expr.start,
			Expression::TaggedTemplate(expr) => expr.start,
			Expression::Object(expr) => expr.start,
			Expression::Array(expr) => expr.start,
			Expression::Parenthesized(expr) => expr.start,
			Expression::Empty(expr) => expr.start,
		}
	}

	pub fn end(&self) -> usize {
		match self {
			Expression::Literal(literal) => match literal {
				Literal::String(literal) => literal.end,
				Literal::Number(literal) => literal.end,
				Literal::Boolean(literal) => literal.end,
				Literal::Null(literal) => literal.end,
			},

			Expression::Identifier(expr) => expr.end,
			Expression::Unary(expr) => expr.end,
			Expression::Binary(expr) => expr.end,
			Expression::Conditional(expr) => expr.end,
			Expression::Call(expr) => expr.end,
			Expression::New(expr) => expr.end,
			Expression::Member(expr) => expr.end,
			Expression::Function(expr) => expr.end,
			Expression::Assignment(expr) => expr.end,
			Expression::Spread(expr) => expr.end,
			Expression::Template(expr) => expr.end,
			Expression::TaggedTemplate(expr) => expr.end,
			Expression::Object(expr) => expr.end,
			Expression::Array(expr) => expr.end,
			Expression::Parenthesized(expr) => expr.end,
			Expression::Empty(expr) => expr.end,
		}
	}
}

impl From<Literal> for Expression {
	fn from(expression: Literal) -> Self {
		Expression::Literal(expression)
	}
}

impl From<StringLiteral> for Literal {
	fn from(literal: StringLiteral) -> Self {
		Literal::String(literal)
	}
}

impl From<NumberLiteral> for Literal {
	fn from(literal: NumberLiteral) -> Self {
		Literal::Number(literal)
	}
}

impl From<BooleanLiteral> for Literal {
	fn from(literal: BooleanLiteral) -> Self {
		Literal::Boolean(literal)
	}
}

impl From<NullLiteral> for Literal {
	fn from(literal: NullLiteral) -> Self {
		Literal::Null(literal)
	}
}

impl From<StringLiteral> for Expression {
	fn from(literal: StringLiteral) -> Self {
		Literal::from(literal).into()
	}
}

impl From<NumberLiteral> for Expression {
	fn from(literal: NumberLiteral) -> Self {
		Literal::from(literal).into()
	}
}

impl From<BooleanLiteral> for Expression {
	fn from(literal: BooleanLiteral) -> Self {
		Literal::from(literal).into()
	}
}

impl From<NullLiteral> for Expression {
	fn from(literal: NullLiteral) -> Self {
		Literal::from(literal).into()
	}
}

impl From<TemplateLiteral> for Expression {
	fn from(literal: TemplateLiteral) -> Expression {
		Expression::Template(literal)
	}
}

impl From<IdentifierExpression> for Expression {
	fn from(expression: IdentifierExpression) -> Expression {
		Expression::Identifier(expression)
	}
}

impl From<UnaryExpression> for Expression {
	fn from(expression: UnaryExpression) -> Expression {
		Expression::Unary(expression)
	}
}

impl From<BinaryExpression> for Expression {
	fn from(expression: BinaryExpression) -> Expression {
		Expression::Binary(expression)
	}
}

impl From<ConditionalExpression> for Expression {
	fn from(expression: ConditionalExpression) -> Expression {
		Expression::Conditional(expression)
	}
}

impl From<CallExpression> for Expression {
	fn from(expression: CallExpression) -> Expression {
		Expression::Call(expression)
	}
}

impl From<NewExpression> for Expression {
	fn from(expression: NewExpression) -> Expression {
		Expression::New(expression)
	}
}

impl From<MemberExpression> for Expression {
	fn from(expression: MemberExpression) -> Expression {
		Expression::Member(expression)
	}
}

impl From<FunctionExpression> for Expression {
	fn from(expression: FunctionExpression) -> Expression {
		Expression::Function(expression)
	}
}

impl From<AssignmentExpression> for Expression {
	fn from(expression: AssignmentExpression) -> Expression {
		Expression::Assignment(expression)
	}
}

impl From<SpreadExpression> for Expression {
	fn from(expression: SpreadExpression) -> Expression {
		Expression::Spread(expression)
	}
}

impl From<TaggedTemplateExpression> for Expression {
	fn from(expression: TaggedTemplateExpression) -> Expression {
		Expression::TaggedTemplate(expression)
	}
}

impl From<ObjectExpression> for Expression {
	fn from(expression: ObjectExpression) -> Expression {
		Expression::Object(expression)
	}
}

impl From<ArrayExpression> for Expression {
	fn from(expression: ArrayExpression) -> Expression {
		Expression::Array(expression)
	}
}

impl From<ParenthesizedExpression> for Expression {
	fn from(expression: ParenthesizedExpression) -> Expression {
		Expression::Parenthesized(expression)
	}
}

#[derive(Debug)]
pub enum Literal {
	String(StringLiteral),
	Number(NumberLiteral),
	Boolean(BooleanLiteral),
	Null(NullLiteral),
}

impl Literal {
	pub fn start(&self) -> usize {
		match self {
			Literal::String(literal) => literal.start,
			Literal::Number(literal) => literal.start,
			Literal::Boolean(literal) => literal.start,
			Literal::Null(literal) => literal.start,
		}
	}

	pub fn end(&self) -> usize {
		match self {
			Literal::String(literal) => literal.end,
			Literal::Number(literal) => literal.end,
			Literal::Boolean(literal) => literal.end,
			Literal::Null(literal) => literal.end,
		}
	}

	pub fn raw(&self) -> &str {
		match self {
			Literal::String(literal) => &literal.raw,
			Literal::Number(literal) => &literal.raw,
			Literal::Boolean(literal) => &literal.raw,
			Literal::Null(literal) => &literal.raw,
		}
	}
}

#[derive(Debug)]
pub struct StringLiteral {
	pub start: usize,
	pub end: usize,
	pub value: String,
	pub quote: char,
	pub raw: String,
}

#[derive(Debug)]
pub struct NumberLiteral {
	pub start: usize,
	pub end: usize,
	pub value: f64,
	pub raw: String,
}

#[derive(Debug)]
pub struct BooleanLiteral {
	pub start: usize,
	pub end: usize,
	pub value: bool,
	pub raw: String,
}

#[derive(Debug)]
pub struct NullLiteral {
	pub start: usize,
	pub end: usize,
	pub raw: String,
}

#[derive(Debug)]
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

impl ToString for UnaryOperator {
	fn to_string(&self) -> String {
		match self {
			UnaryOperator::Minus => "-",
			UnaryOperator::Plus => "+",
			UnaryOperator::Increment => "++",
			UnaryOperator::Decrement => "--",
			UnaryOperator::Not => "!",
			UnaryOperator::BitwiseNot => "~",
			UnaryOperator::Typeof => "typeof",
			UnaryOperator::Void => "void",
			UnaryOperator::Delete => "delete",
		}
		.to_owned()
	}
}

#[derive(Debug)]
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

impl ToString for BinaryOperator {
	fn to_string(&self) -> String {
		match self {
			BinaryOperator::Plus => "+",
			BinaryOperator::Minus => "-",
			BinaryOperator::Multiply => "*",
			BinaryOperator::Divide => "/",
			BinaryOperator::Modulo => "%",
			BinaryOperator::Exponent => "**",
			BinaryOperator::LeftShift => "<<",
			BinaryOperator::RightShift => ">>",
			BinaryOperator::UnsignedRightShift => ">>>",
			BinaryOperator::LessThan => "<",
			BinaryOperator::GreaterThan => ">",
			BinaryOperator::LessThanOrEqual => "<=",
			BinaryOperator::GreaterThanOrEqual => ">=",
			BinaryOperator::Equal => "==",
			BinaryOperator::NotEqual => "!=",
			BinaryOperator::StrictEqual => "===",
			BinaryOperator::StrictNotEqual => "!==",
			BinaryOperator::BitwiseAnd => "&",
			BinaryOperator::BitwiseOr => "|",
			BinaryOperator::BitwiseXor => "^",
			BinaryOperator::LogicalAnd => "&&",
			BinaryOperator::LogicalOr => "||",
			BinaryOperator::InstanceOf => "instanceof",
			BinaryOperator::In => "in",
		}
		.to_owned()
	}
}

#[derive(Debug)]
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

impl ToString for AssignmentOperator {
	fn to_string(&self) -> String {
		match self {
			AssignmentOperator::Equal => "=",
			AssignmentOperator::PlusEqual => "+=",
			AssignmentOperator::MinusEqual => "-+",
			AssignmentOperator::MultiplyEqual => "*=",
			AssignmentOperator::DivideEqual => "/=",
			AssignmentOperator::ModuloEqual => "%=",
			AssignmentOperator::ExponentEqual => "**=",
			AssignmentOperator::LeftShiftEqual => "<<=",
			AssignmentOperator::RightShiftEqual => ">>=",
			AssignmentOperator::UnsignedRightShiftEqual => ">>>=",
			AssignmentOperator::BitwiseAndEqual => "&=",
			AssignmentOperator::BitwiseOrEqual => "|=",
			AssignmentOperator::BitwiseXorEqual => "^=",
		}
		.to_owned()
	}
}

#[derive(Debug)]
pub struct IdentifierExpression {
	pub start: usize,
	pub end: usize,
	pub name: String,
}

#[derive(Debug)]
pub struct UnaryExpression {
	pub start: usize,
	pub end: usize,
	pub operator: UnaryOperator,
	pub operand: Box<Expression>,
}

#[derive(Debug)]
pub struct BinaryExpression {
	pub start: usize,
	pub end: usize,
	pub operator: BinaryOperator,
	pub left: Box<Expression>,
	pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct ConditionalExpression {
	pub start: usize,
	pub end: usize,
	pub condition: Box<Expression>,
	pub consequent: Box<Expression>,
	pub alternate: Box<Expression>,
}

#[derive(Debug)]
pub struct CallExpression {
	pub start: usize,
	pub end: usize,
	pub callee: Box<Expression>,
	pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct NewExpression {
	pub start: usize,
	pub end: usize,
	pub callee: Box<Expression>,
	pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct MemberExpression {
	pub start: usize,
	pub end: usize,
	pub object: Box<Expression>,
	pub property: Box<Expression>,
	pub computed: bool,
	pub optional: bool,
}

#[derive(Debug)]
pub struct SequenceExpression {
	pub start: usize,
	pub end: usize,
	pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct FunctionExpression {
	pub start: usize,
	pub end: usize,
	pub parameters: Vec<Expression>,
	pub body: Box<Expression>,
}

#[derive(Debug)]
pub struct AssignmentExpression {
	pub start: usize,
	pub end: usize,
	pub operator: AssignmentOperator,
	pub left: Box<Expression>,
	pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct SpreadExpression {
	pub start: usize,
	pub end: usize,
	pub argument: Box<Expression>,
}

#[derive(Debug)]
pub struct TemplateLiteral {
	pub start: usize,
	pub end: usize,
	pub raw: String,
}

#[derive(Debug)]
pub struct TaggedTemplateExpression {
	pub start: usize,
	pub end: usize,
	pub tag: Box<Expression>,
	pub quasi: Box<TemplateLiteral>,
}

#[derive(Debug)]
pub struct ObjectExpression {
	pub start: usize,
	pub end: usize,
	pub properties: Vec<ObjectProperty>,
}

#[derive(Debug)]
pub enum ObjectProperty {
	Keyed(ObjectKeyedProperty),
	Computed(ObjectComputedProperty),
	Spread(SpreadExpression),
}

#[derive(Debug)]
pub struct ObjectKeyedProperty {
	pub start: usize,
	pub end: usize,
	pub key: Box<IdentifierExpression>,
	pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct ObjectComputedProperty {
	pub start: usize,
	pub end: usize,
	pub key: Box<Expression>,
	pub value: Box<Expression>,
}

#[derive(Debug)]
pub struct ArrayExpression {
	pub start: usize,
	pub end: usize,
	pub elements: Vec<Expression>,
}

#[derive(Debug)]
pub struct ParenthesizedExpression {
	pub start: usize,
	pub end: usize,
	pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct EmptyExpression {
	pub start: usize,
	pub end: usize,
}
