pub mod cypher;


#[derive(Debug)]
pub struct GqlProgram {
    pub program_activity: ProgramActivity,
    pub session_close: bool,
    
}

#[derive(Debug)]
pub enum ProgramActivity {
    Session(SessionActivity),
    Transaction(TransactionActivity),
}

#[derive(Debug)]
pub struct SessionActivity {
    pub session_set_commands: Vec<SessionSetCommand>,
    pub session_reset_commands: Vec<SessionResetCommand>,
}

#[derive(Debug)]
pub struct TransactionActivity {
    pub start_transaction: Option<TransactionCharacteristics>,
    pub procedure_specification: Option<ProcedureSpecification>,
    pub end_translation: Option<EndTransactionCommand>,
}

#[derive(Debug)]
pub enum EndTransactionCommand {
    Commit,
    Rollback,
}

#[derive(Debug)]
pub struct TransactionCharacteristics {
    pub access_mode: Vec<TransactionAccessMode>,
}

#[derive(Debug)]
pub enum TransactionAccessMode {
    ReadWrite,
    ReadOnly,
}

#[derive(Debug)]
pub enum SessionSetCommand {
    Schema(SchemaReference),
    Graph(GraphExpression),
    TimeZone(String),
    Paramater(SessionSetParameterClause),
}

#[derive(Debug)]
pub enum SchemaReference {
    CatalogSchemaReference(CatalogSchemaReference),
    ReferenceParameterSpecification(String),
}

#[derive(Debug)]
pub enum GraphExpression {
    ObjectExpressionPrimary(ObjectExpressionPrimary),
    GraphReference(GraphReference),
    BindingTableReference(BindingTableReference),
    ObjectNameOrBindingVariable(ObjectNameOrBindingVariable),
}

#[derive(Debug)]
pub struct  SessionSetGraphParameterClause {
    pub parameter_name: String,
    pub initializer: OptTypedGraphInitializer,
}

#[derive(Debug)]
pub struct SessionSetParameterName {
    pub if_not_exists: bool,
    pub name: String,
}

#[derive(Debug)]
pub enum SessionSetParameterClause {
    GraphParameter(SessionSetGraphParameterClause),
    BindingTableParameter(GraphType),
    ValueParameter(ValueExpression),

}

#[derive(Debug)]
pub struct OptTypedGraphInitializer {
    pub typed: bool,
    pub reference_value_type: Option<GraphReferenceValueType>,
    pub initializer: GraphExpression,
}

#[derive(Debug)]
pub enum GraphReferenceValueType {
    Open(OpenGraphReferenceValueType),
    Closed(ClosedGraphReferenceValueType),
}

#[derive(Debug)]
pub struct OpenGraphReferenceValueType {
    //TODO
}

#[derive(Debug)]
pub struct ClosedGraphReferenceValueType {
    //TODO
}

#[derive(Debug)]
pub enum SessionResetArgument {
    //TODO: ALL important?
    Parameters,
    //TODO: ALL important?
    Characteristics,
    Schema,
    //TODO: PROPERTY important?
    Graph,
    TimeZone,
    Parameter(String),
}

#[derive(Debug)]
pub struct SessionResetCommand {
    pub arguments: Vec<SessionResetArgument>,
}

#[derive(Debug)]
pub enum CatalogSchemaReference {
    Absolute(AbsoluteCatalogSchemaReference),
    Relative(RelativeCatalogSchemaReference),
}

#[derive(Debug)]
pub enum AbsoluteCatalogSchemaReference {
    //TODO
}

#[derive(Debug)]
pub enum RelativeCatalogSchemaReference {
    //TODO
}

#[derive(Debug)]
pub enum ObjectExpressionPrimary {
    //TODO
}

#[derive(Debug)]
pub enum GraphReference {
    //TODO
}

#[derive(Debug)]
pub enum BindingTableReference {
    //TODO
}

#[derive(Debug)]
pub enum ObjectNameOrBindingVariable {
    //TODO
}

#[derive(Debug)]
pub enum GraphType {
    //TODO
}

#[derive(Debug)]
pub enum ValueExpression {
    //TODO
}


#[derive(Debug)]
pub enum ProcedureSpecification {
    CatalogModifying(CatalogModifyingProcedureSpecification),
    DataModifying(DataModifyingProcedureSpecification),
    Query(ProcedureBody),
}

#[derive(Debug)]
pub struct CatalogModifyingProcedureSpecification {
    //TODO
}

#[derive(Debug)]
pub struct DataModifyingProcedureSpecification {
    //TODO
}

#[derive(Debug)]
pub struct ProcedureBody {
    pub at_schema: Option<SchemaReference>,
    pub binding_variable_definition_block: Option<BindingVariableDefinitionBlock>,
    pub statement_block: StatementBlock,
}

#[derive(Debug)]
pub struct BindingVariableDefinitionBlock {
    //TODO
}

#[derive(Debug)]
pub struct StatementBlock {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    CompositeQuery(CompositeQueryExpression),
    LinearDataModifying(LinearDataModifyingStatement),
    LinearCatalogModifying(LinearCatalogModifyingStatement),
}

#[derive(Debug)]
pub enum SetOperator {
    Union(SetQuantifier),
    Intersect(SetQuantifier),
    Except(SetQuantifier),
    Otherwise,
}

#[derive(Debug)]
pub enum SetQuantifier {
    Distinct,
    All,
}

#[derive(Debug)]
pub struct CompositeQueryExpression {
    pub query: LinearQueryStatement,
    pub partial_conjunctions: Vec<(SetOperator, LinearQueryStatement)>,
}

#[derive(Debug)]
pub enum LinearQueryStatement {
    Focused(FocusedLinearQueryStatement),
    Ambient(AmbientLinearQueryStatement),
}

#[derive(Debug)]
pub enum FocusedLinearQueryStatement {
    //TODO
}

#[derive(Debug)]
pub enum AmbientLinearQueryStatement {
    //TODO
}

#[derive(Debug)]
pub enum LinearDataModifyingStatement {
    //TODO
}

#[derive(Debug)]
pub enum LinearCatalogModifyingStatement {
    //TODO
}



