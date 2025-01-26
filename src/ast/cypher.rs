use std::collections::HashMap;

#[derive(Debug)]
pub enum Query {
    RegularQuery(RegularQuery),
    StandaloneCall(StandaloneCall),
}

#[derive(Debug)]
pub enum SingleQuery {
    SinglePartQuery(SinglePartQuery),
    MultiPartQuery(MultiPartQuery),
}

#[derive(Debug)]
pub struct RegularQuery {
    pub query: SingleQuery,
    pub union: Vec<Union>,
}

#[derive(Debug)]
pub struct Union {
    pub all: bool,
    pub query: SingleQuery,
}

#[derive(Debug)]
pub struct SinglePartQuery {
    pub reading_clauses: Vec<ReadingClause>,
    pub updating_clauses: Vec<UpdatingClause>,
    pub return_: Option<ProjectionBody>,
}

#[derive(Debug)]
pub struct MultiPartQuery {
    pub parts: Vec<MultiPartQueryPart>,
    pub query: SinglePartQuery,
}

#[derive(Debug)]
pub struct MultiPartQueryPart {
    pub reading_clauses: Vec<ReadingClause>,
    pub updating_clauses: Vec<UpdatingClause>,
    pub with: With,
}

#[derive(Debug)]
pub struct With {
    pub projection_body: ProjectionBody,
    pub where_: Option<Expression>,
}

#[derive(Debug)]
pub enum UpdatingClause {
    Create(Create),
    Merge(Merge),
    Delete(Delete),
    Set(Set),
    Remove(Remove),
}

#[derive(Debug)]
pub struct Merge {
    pub pattern_part: PatternPart,
    pub actions: Vec<MergeAction>,
}

#[derive(Debug)]
pub enum MergeAction {
    Match(Set),
    Create(Set),
}

#[derive(Debug)]
pub struct Create {
    pub pattern: Pattern,
}

#[derive(Debug)]
pub struct Pattern {
    pub parts: Vec<PatternPart>,
}

#[derive(Debug)]
pub struct PatternPart {
    pub variable: Option<String>,
    pub pattern_element: PatternElement,
}

#[derive(Debug)]
pub struct PatternElement {
    pub node_pattern: NodePattern,
    pub relationship_patterns: Vec<(RelationshipPattern, NodePattern)>,
}

#[derive(Debug)]
pub struct NodePattern {
    pub variable: Option<String>,
    pub node_labels: Vec<String>,
    pub properties: Option<Properties>,
}

#[derive(Debug)]
pub struct RelationshipsPattern {
    pub node: NodePattern,
    pub relationships: Vec<(RelationshipPattern, NodePattern)>,
}

#[derive(Debug)]
pub enum RelationshipPattern {
    LeftAndRight(Option<RelationshipDetail>),
    Left(Option<RelationshipDetail>),
    Right(Option<RelationshipDetail>),
    Undirected(Option<RelationshipDetail>),
}

#[derive(Debug)]
pub struct RelationshipDetail {
    pub variable: Option<String>,
    pub relationship_types: Vec<String>,
    pub properties: Option<Properties>,
}

#[derive(Debug)]
pub enum Properties {
    Parameter(String),
    MapLiteral(HashMap<String, Expression>),
}

#[derive(Debug)]
pub struct Set {
    pub items: Vec<SetItem>,
}

#[derive(Debug)]
pub enum SetItem {
    AssignPropertyExpression {
        property_expression: PropertyExpression,
        expression: Expression,
    },
    AssignVariable {
        variable: String,
        expression: Expression,
    },
    IncrementVariable {
        variable: String,
        expression: Expression,
    },
    VariableNodeLabels {
        variable: String,
        node_labels: Vec<String>,
    },
}

#[derive(Debug)]
pub struct Delete {
    pub detach: bool,
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct Remove {
    pub items: Vec<RemoveItem>,
}

#[derive(Debug)]
pub enum RemoveItem {
    VariableNodeLabels {
        variable: String,
        node_labels: Vec<String>,
    },
    PropertyExpression(PropertyExpression),
}

#[derive(Debug)]
pub struct PropertyExpression {
    pub atom: Atom,
    pub property_path: Vec<String>,
}

#[derive(Debug)]
pub enum Atom {
    Literal(Literal),
    Parameter(String),
    CaseExpression(CaseExpression),
    CountStar,
    ListComprehension(ListComprehension),
    PatternComprehension(PatternComprehension),
    All(FilterExpression),
    Any(FilterExpression),
    None(FilterExpression),
    Single(FilterExpression),
    RelationshipsPattern(RelationshipsPattern),
    ParenthesizedExpression(Expression),
    FunctionInvocation(FunctionInvocation),
    ExistentialSubquery(ExistentialSubquery),
    Variable(String),
}

#[derive(Debug)]
pub struct PatternComprehension {
    pub variable: Option<String>,
    pub relationship_pattern: RelationshipPattern,
    pub where_: Option<Expression>,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct ListComprehension {
    pub filter_expression: FilterExpression,
    pub expression: Option<Expression>,
}

#[derive(Debug)]
pub struct IdInColl {
    pub variable: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct FilterExpression {
    pub id_in_coll: IdInColl,
    pub where_: Option<Expression>,
}

#[derive(Debug)]
pub struct CaseExpression {
    pub expression: Option<Expression>,
    pub alternatives: Vec<(Expression, Expression)>,
    pub else_: Option<Expression>,
}

#[derive(Debug)]
pub enum ReadingClause {
    Match(Match),
    Unwind(Unwind),
    InQueryCall(InQueryCall),
}

#[derive(Debug)]
pub struct Match {
    pub optional: bool,
    pub pattern: Pattern,
    pub where_: Option<Expression>,
}

#[derive(Debug)]
pub struct Unwind {
    pub expression: Expression,
    pub variable: String,
}

#[derive(Debug)]
pub struct InQueryCall {
    pub invocation: ExplicitProcedureInvocation,
    pub yield_items: Option<YieldItems>,
}

#[derive(Debug)]
pub struct ExplicitProcedureInvocation {
    pub name: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct ImplicitProcedureInvocation {
    pub name: String,
}

#[derive(Debug)]
pub enum ProcedureInvocation {
    Explicit(ExplicitProcedureInvocation),
    Implicit(ImplicitProcedureInvocation),
}

#[derive(Debug)]
pub struct ProjectionBody {
    pub distinct: bool,
    pub items: ProjectionItems,
    pub order: Option<Order>,
    pub skip: Option<Skip>,
    pub limit: Option<Limit>
}

#[derive(Debug)]
pub struct Order {
    pub items: Vec<SortItem>,
}

#[derive(Debug)]
pub enum SortItem {
    Ascending(Expression),
    Descending(Expression),
}

#[derive(Debug)]
pub struct Skip {
    pub expression: Expression,
}

#[derive(Debug)]
pub struct Limit {
    pub expression: Expression,
}

#[derive(Debug)]
pub struct ProjectionItems {
    pub star: bool,
    pub items: Vec<ProjectionItem>,
}

#[derive(Debug)]
pub enum ProjectionItem {
    Expression(Expression),
    AsVariable {
        expression: Expression,
        variable: String,
    },
}

#[derive(Debug)]
pub struct StandaloneCall {
    pub invocation: ProcedureInvocation,
    pub yield_: Option<StarOrYieldItems>,
}

#[derive(Debug)]
pub enum StarOrYieldItems {
    Star,
    YieldItems(YieldItems),
}

#[derive(Debug)]
pub enum YieldItems {
    Star,
    Items {
        items: Vec<YieldItem>,
        where_: Option<Expression>,
    },
}

#[derive(Debug)]
pub struct YieldItem {
    pub procedure_result_field: Option<String>,
    pub variable: String,
}

#[derive(Debug)]
pub struct ProcedureResultField {
    pub name: String,
}

#[derive(Debug)]
pub struct OrExpression(pub Vec<XorExpression>);
pub type Expression = OrExpression;

#[derive(Debug)]
pub struct XorExpression(pub Vec<AndExpression>);

#[derive(Debug)]
pub struct AndExpression(pub Vec<NotExpression>);

#[derive(Debug)]
pub struct NotExpression {
    pub not: bool,
    pub expression: ComparisonExpression,
}

#[derive(Debug)]
pub struct ComparisonExpression {
    pub expression: AddOrSubtractExpression,
    pub comparisons: Vec<(ComparisonOperator, AddOrSubtractExpression)>,
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug)]
pub struct AddOrSubtractExpression {
    pub expression: MultiplyDivideModuloExpression,
    pub operations: Vec<(PlusMinusOperator, MultiplyDivideModuloExpression)>,
}

#[derive(Debug)]
pub enum PlusMinusOperator {
    Plus,
    Minus,
}

#[derive(Debug)]
pub struct MultiplyDivideModuloExpression {
    pub expression: PowerOfExpression,
    pub operations: Vec<(MultiplyDivideModuloOperator, PowerOfExpression)>,
}

#[derive(Debug)]
pub enum MultiplyDivideModuloOperator {
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub struct PowerOfExpression(pub Vec<UnaryAddOrSubtractExpression>);

#[derive(Debug)]
pub struct UnaryAddOrSubtractExpression {
    pub negate: bool,
    pub expression: StringListNullOperatorExpression,
}

#[derive(Debug)]
pub struct StringListNullOperatorExpression {
    pub expression: PropertyOrLabelsExpression,
    pub operations: Vec<StringListNullOperatorExpressionInner>,
}

#[derive(Debug)]
pub enum StringListNullOperatorExpressionInner {
    StringOperator(StringOperatorExpression),
    ListOperator(ListOperatorExpression),
    NullOperator(NullOperatorExpression),
}

#[derive(Debug)]
pub struct StringOperatorExpression {
    pub operator: StringOperator,
    pub expression: PropertyOrLabelsExpression,
}

#[derive(Debug)]
pub enum ListOperatorExpression {
    In(PropertyOrLabelsExpression),
    Index(Expression),
    Range(Option<Expression>, Option<Expression>),
}

#[derive(Debug)]
pub enum StringOperator {
    StartsWith,
    EndsWith,
    Contains,
}

#[derive(Debug)]
pub struct NullOperatorExpression {
    pub not: bool,
}

#[derive(Debug)]
pub struct PropertyOrLabelsExpression {
    pub atom: Atom,
    pub property_lookup: Vec<String>,
    pub node_labels: Vec<String>,
}

#[derive(Debug)]
pub struct FunctionInvocation {
    pub function_name: String,
    pub distinct: bool,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub enum ExistentialSubquery {
    RegularQuery(RegularQuery),
    PatternWhere {
        pattern: Pattern,
        where_: Option<Expression>,
    },
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(NumberLiteral),
    Boolean(bool),
    Null,
    MapLiteral(HashMap<String, Expression>),
    ListLiteral(Vec<Expression>),
}

#[derive(Debug)]
pub enum NumberLiteral {
    Integer(i64),
    Double(f64),
}