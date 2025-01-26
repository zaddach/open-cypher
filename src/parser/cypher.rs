extern crate pest;
extern crate pest_derive;

use std::collections::HashMap;

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::ast::cypher::{AddOrSubtractExpression, PlusMinusOperator, AndExpression, Atom, CaseExpression, ComparisonExpression, ComparisonOperator, Create, Delete, ExplicitProcedureInvocation, Expression, FilterExpression, FunctionInvocation, IdInColl, ImplicitProcedureInvocation, InQueryCall, Limit, ListComprehension, ListOperatorExpression, Literal, Match, Merge, MergeAction, MultiPartQuery, MultiPartQueryPart, MultiplyDivideModuloExpression, MultiplyDivideModuloOperator, NodePattern, NotExpression, NullOperatorExpression, NumberLiteral, OrExpression, Order, Pattern, PatternComprehension, PatternElement, PatternPart, PowerOfExpression, ProcedureInvocation, ProjectionBody, ProjectionItem, ProjectionItems, Properties, PropertyExpression, PropertyOrLabelsExpression, Query, ReadingClause, RegularQuery, RelationshipDetail, RelationshipPattern, RelationshipsPattern, Remove, RemoveItem, Set, SetItem, SinglePartQuery, SingleQuery, Skip, SortItem, StandaloneCall, StarOrYieldItems, StringListNullOperatorExpression, StringListNullOperatorExpressionInner, StringOperator, StringOperatorExpression, UnaryAddOrSubtractExpression, Union, Unwind, UpdatingClause, With, XorExpression, YieldItem, YieldItems};

#[derive(Parser)]
#[grammar = "cypher.pest"]
pub struct CypherParser;

pub fn parse(code: &str) -> Result<Query, Error<Rule>> {
    Query::parse(code)
}

impl Query {
    pub fn parse(code: &str) -> Result<Query, Error<Rule>> {
        for pair in CypherParser::parse(Rule::Cypher, code)? {
            match pair.as_rule() {
                Rule::Cypher => for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::Statement => for pair in pair.into_inner() {
                            match pair.as_rule() {
                                Rule::Query => for pair in pair.into_inner() {
                                    match pair.as_rule() {
                                        Rule::RegularQuery => return Ok(Query::RegularQuery(RegularQuery::parse(pair)?)),
                                        Rule::StandaloneCall => return Ok(Query::StandaloneCall(StandaloneCall::parse(pair)?)),
                                        _ => unreachable!("Unexpected rule in Query: {:?}", pair.as_rule()),
                                    }
                                },
                                _ => unreachable!("Unexpected rule in Statement: {:?}", pair.as_rule()),
                            }
                        },
                        _ => unreachable!("Unexpected rule in Cypher: {:?}", pair.as_rule()),
                    }
                }
                _ => unreachable!("Unexpected top-level rule: {:?}", pair.as_rule()),
            }
        }
        unreachable!("Unexpected missing rule in Cypher")
    }
}


impl RegularQuery {
    fn parse(pair: Pair<'_, Rule>) -> Result<RegularQuery, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::RegularQuery));

        let mut query = None;
        let mut union = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SingleQuery => query = Some(SingleQuery::parse(pair)?),
                Rule::Union => union.push(Union::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!("Unexpected rule in RegularQuery: {:?}", pair.as_rule()),
            }
        }

        Ok(RegularQuery { query: query.unwrap(), union })
    }
}


impl SingleQuery {
    fn parse(pair: Pair<'_, Rule>) -> Result<SingleQuery, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::SingleQuery));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SinglePartQuery => return Ok(SingleQuery::SinglePartQuery(SinglePartQuery::parse(pair)?)),
                Rule::MultiPartQuery => return Ok(SingleQuery::MultiPartQuery(MultiPartQuery::parse(pair)?)),
                _ => unreachable!("Unexpected rule in SingleQuery: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Unexpected missing rule in SingleQuery")
    }
}

impl SinglePartQuery {
    fn parse(pair: Pair<'_, Rule>) -> Result<SinglePartQuery, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::SinglePartQuery));

        let mut reading_clauses = Vec::new();
        let mut updating_clauses = Vec::new();
        let mut return_ = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ReadingClause => reading_clauses.push(ReadingClause::parse(pair)?),
                Rule::UpdatingClause => updating_clauses.push(UpdatingClause::parse(pair)?),
                Rule::Return => return_ = Some(parse_return(pair)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(SinglePartQuery { reading_clauses, updating_clauses, return_ })
    }
}

fn parse_return(pair: Pair<'_, Rule>) -> Result<ProjectionBody, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::Return));

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::ProjectionBody => return ProjectionBody::parse(pair),
            Rule::SP | Rule::RETURN => (),
            _ => unreachable!(),
        }
    }

    unreachable!()
}

impl ReadingClause {
    fn parse(pair: Pair<'_, Rule>) -> Result<ReadingClause, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ReadingClause));

        match pair.into_inner().next() {
            Some(pair) => match pair.as_rule() {
                Rule::Match => Ok(ReadingClause::Match(Match::parse(pair)?)),
                Rule::Unwind => Ok(ReadingClause::Unwind(Unwind::parse(pair)?)),
                Rule::InQueryCall => Ok(ReadingClause::InQueryCall(InQueryCall::parse(pair)?)),
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }
}

impl Match {
    fn parse(pair: Pair<'_, Rule>) -> Result<Match, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Match));

        let mut optional = false;
        let mut pattern = None;
        let mut where_ = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::MATCH | Rule::SP => (),
                Rule::OPTIONAL => optional = true,
                Rule::Pattern => pattern = Some(Pattern::parse(pair)?),
                Rule::Where => where_ = Some(parse_where(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(Match { optional, pattern: pattern.unwrap(), where_ })
    }
}

fn parse_where(pair: Pair<'_, Rule>) -> Result<Expression, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::Where));

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Expression => return Ok(Expression::parse(pair)?),
            Rule::WHERE | Rule::SP => (),
            _ => unreachable!(),
        }
    }

    unreachable!()
}

impl Pattern {
    fn parse(pair: Pair<'_, Rule>) -> Result<Pattern, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Pattern));

        let mut parts = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::PatternPart => parts.push(PatternPart::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!("Unexpected rule in Pattern: {:?}", pair.as_rule()),
            }
        }

        Ok(Pattern { parts })
    }
}

impl PatternPart {
    fn parse(pair: Pair<'_, Rule>) -> Result<PatternPart, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::PatternPart));

        let mut variable = None;
        let mut pattern_element = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Variable => variable = Some(parse_variable(pair)?),
                Rule::AnonymousPatternPart => match pair.into_inner().next() {
                    Some(pair) => match pair.as_rule() {
                        Rule::PatternElement => pattern_element = Some(PatternElement::parse(pair)?),
                        _ => unreachable!(),
                    },
                    None => unreachable!(),
                },
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(PatternPart { variable, pattern_element: pattern_element.unwrap() })
    }
}

fn parse_variable(pair: Pair<'_, Rule>) -> Result<String, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::Variable));
    Ok(pair.as_str().to_string())
}

fn parse_relationship_types(pair: Pair<'_, Rule>) -> Result<Vec<String>, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::RelationshipTypes));

    let mut relationship_types = Vec::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::RelTypeName => relationship_types.push(pair.to_string()),
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    Ok(relationship_types)
}

impl PatternElement {
    fn parse(pair: Pair<'_, Rule>) -> Result<PatternElement, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::PatternElement));

        let mut node_pattern = None;
        let mut relationship_patterns = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::PatternElement => return Ok(PatternElement::parse(pair)?),
                Rule::NodePattern => node_pattern = Some(NodePattern::parse(pair)?),
                Rule::PatternElementChain => relationship_patterns.push(parse_pattern_element_chain(pair)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(PatternElement { node_pattern: node_pattern.unwrap(), relationship_patterns })
    }
}

fn parse_pattern_element_chain(pair: Pair<'_, Rule>) -> Result<(RelationshipPattern, NodePattern), Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::PatternElementChain));

    let mut node_pattern = None;
    let mut relationship_pattern = None;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::NodePattern => node_pattern = Some(NodePattern::parse(pair)?),
            Rule::RelationshipPattern => relationship_pattern = Some(RelationshipPattern::parse(pair)?),
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    Ok((relationship_pattern.unwrap(), node_pattern.unwrap()))
}

impl RelationshipPattern {
    fn parse(pair: Pair<'_, Rule>) -> Result<RelationshipPattern, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::RelationshipPattern));

        let mut left_arrow_head = false;
        let mut right_arrow_head = false;
        let mut relationship_detail = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::LeftArrowHead => left_arrow_head = true,
                Rule::RightArrowHead => right_arrow_head = true,
                Rule::RelationshipDetail => relationship_detail = Some(RelationshipDetail::parse(pair)?),
                Rule::Dash | Rule::SP => (),
                _ => unreachable!(),
            }
        }

        match (left_arrow_head, right_arrow_head) {
            (true, true) => Ok(RelationshipPattern::LeftAndRight(relationship_detail)),
            (true, false) => Ok(RelationshipPattern::Left(relationship_detail)),
            (false, true) => Ok(RelationshipPattern::Right(relationship_detail)),
            (false, false) => Ok(RelationshipPattern::Undirected(relationship_detail)),
        }
    }
}

impl RelationshipDetail {
    fn parse(pair: Pair<'_, Rule>) -> Result<RelationshipDetail, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::RelationshipDetail));

        let mut variable = None;
        let mut relationship_types = Vec::new();
        let mut properties = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Variable => variable = Some(parse_variable(pair)?),
                Rule::RelationshipTypes => relationship_types = parse_relationship_types(pair)?,
                Rule::Properties => properties = Some(Properties::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(RelationshipDetail { variable, relationship_types, properties })
    }
}

impl NodePattern {
    fn parse(pair: Pair<'_, Rule>) -> Result<NodePattern, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::NodePattern));

        let mut variable = None;
        let mut node_labels = Vec::new();
        let mut properties = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Variable => variable = Some(parse_variable(pair)?),
                Rule::NodeLabels => node_labels = parse_node_labels(pair)?,
                Rule::Properties => properties = Some(Properties::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(NodePattern { variable, node_labels, properties })
    }
}

fn parse_node_labels(pair: Pair<'_, Rule>) -> Result<Vec<String>, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::NodeLabels));

    let mut node_labels = Vec::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::NodeLabel => node_labels.push(parse_node_label(pair)?),
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    Ok(node_labels)
}

fn parse_node_label(pair: Pair<'_, Rule>) -> Result<String, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::NodeLabel));

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::LabelName => return Ok(pair.to_string()),
            Rule::SP => (),
            _ => unreachable!("Unexpected rule in NodeLabel: {:?}", pair.as_rule()),
        }
    }

    unreachable!("Unexpected missing rule in NodeLabel")
}

impl Properties {
    fn parse(pair: Pair<'_, Rule>) -> Result<Properties, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Properties));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Parameter => return Ok(Properties::Parameter(pair.to_string())),
                Rule::MapLiteral => return Ok(Properties::MapLiteral(parse_map_literal(pair)?)),
                _ => unreachable!("Unexpected rule in Properties: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Unexpected missing rule in Properties")
    }
}

impl OrExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<OrExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::OrExpression) || matches!(pair.as_rule(), Rule::Expression));

        let mut xor_expressions = Vec::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::OrExpression => for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::XorExpression => xor_expressions.push(XorExpression::parse(pair)?),
                        Rule::SP | Rule::XOR | Rule::OR => (),
                        _ => unreachable!("Unexpected rule in OrExpression: {:?}", pair.as_rule()),
                    }
                },
                Rule::XorExpression => xor_expressions.push(XorExpression::parse(pair)?),
                Rule::SP | Rule::XOR => (),
                _ => unreachable!(),
            }
        }

        Ok(OrExpression(xor_expressions))
    }
}

impl XorExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<XorExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::XorExpression));

        let mut and_expressions = Vec::new();

        match pair.into_inner().next() {
            Some(pair) => match pair.as_rule() {
                Rule::AndExpression => and_expressions.push(AndExpression::parse(pair)?),
                Rule::SP | Rule::AND => (),
                _ => unreachable!(),
            },
            None => unreachable!(),
        }

        Ok(XorExpression(and_expressions))
    }
}

impl AndExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<AndExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::AndExpression));

        let mut not_expressions = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::NotExpression => not_expressions.push(NotExpression::parse(pair)?),
                Rule::SP | Rule::AND => (),
                _ => unreachable!(),
            }
        }

        Ok(AndExpression(not_expressions))
    }
}

impl NotExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<NotExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::NotExpression));

        let mut not_count = 0;
        let mut comparison_expression = None;

        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::NOT => not_count += 1,
                Rule::ComparisonExpression => comparison_expression = Some(ComparisonExpression::parse(item)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(NotExpression { not: not_count % 2 == 1, expression: comparison_expression.unwrap() })
    }
}

impl ComparisonExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<ComparisonExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ComparisonExpression));

        println!("Parsing comparison expression: {:?}", pair);

        let mut add_or_subtract_expression = None;
        let mut partial_comparisons = Vec::new();

        for pair in pair.into_inner() {
            println!("{:?}", pair.as_rule());
            match pair.as_rule() {
                Rule::SP => (),
                Rule::AddOrSubtractExpression => add_or_subtract_expression = Some(AddOrSubtractExpression::parse(pair)?),
                Rule::PartialComparisonExpression => {
                    let mut operator = None;
                    let mut expression = None;
                    for pair in pair.into_inner() {
                        println!("{:?}", pair.as_rule());
                        match pair.as_rule() {
                            Rule::GE => operator = Some(ComparisonOperator::GreaterEqual),
                            Rule::GT => operator = Some(ComparisonOperator::GreaterThan),
                            Rule::LE => operator = Some(ComparisonOperator::LessEqual),
                            Rule::LT => operator = Some(ComparisonOperator::LessThan),
                            Rule::EQ => operator = Some(ComparisonOperator::Equal),
                            Rule::NE => operator = Some(ComparisonOperator::NotEqual),
                            Rule::SP => (),
                            Rule::AddOrSubtractExpression => expression = Some(AddOrSubtractExpression::parse(pair)?),
                            _ => unreachable!(),
                        }
                    }

                    partial_comparisons.push((operator.unwrap(), expression.unwrap()));
                }
                _ => unreachable!(),
            }
        }

        Ok(ComparisonExpression { expression: add_or_subtract_expression.unwrap(), comparisons: partial_comparisons })
    }
}

impl AddOrSubtractExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<AddOrSubtractExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::AddOrSubtractExpression));

        let mut multiply_or_divide_expression = None;
        let mut partial_add_or_subtract_expressions = Vec::new();
        let mut operator = None;


        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::SP => (),
                Rule::MultiplyDivideModuloExpression => {
                    if let Some(operator) = operator.take() {
                        partial_add_or_subtract_expressions.push((operator, MultiplyDivideModuloExpression::parse(item)?));
                    }
                    else {
                        multiply_or_divide_expression = Some(MultiplyDivideModuloExpression::parse(item)?);
                    }
                },
                Rule::PLUS => operator = Some(PlusMinusOperator::Plus),
                Rule::MINUS => operator = Some(PlusMinusOperator::Minus),
                _ => unreachable!(),
            }
        }

        Ok(AddOrSubtractExpression { expression: multiply_or_divide_expression.unwrap(), operations: partial_add_or_subtract_expressions })
    }
}

impl MultiplyDivideModuloExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<MultiplyDivideModuloExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::MultiplyDivideModuloExpression));

        let mut power_of_expression = None;
        let mut operator = None;
        let mut operations = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SP => (),
                Rule::PowerOfExpression => {
                    if let Some(operator) = operator.take() {
                        operations.push((operator, PowerOfExpression::parse(pair)?));
                    }
                    else {
                        power_of_expression = Some(PowerOfExpression::parse(pair)?);
                    }
                },
                Rule::MULTIPLY => operator = Some(MultiplyDivideModuloOperator::Multiply),
                Rule::DIVIDE => operator = Some(MultiplyDivideModuloOperator::Divide),
                Rule::MODULO => operator = Some(MultiplyDivideModuloOperator::Modulo),
                _ => unreachable!(),
            }
        }

        Ok(MultiplyDivideModuloExpression { expression: power_of_expression.unwrap(), operations })
    }
}

impl PowerOfExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<PowerOfExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::PowerOfExpression));

        let mut expressions = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SP | Rule::POW => (),
                Rule::UnaryAddOrSubtractExpression => expressions.push(UnaryAddOrSubtractExpression::parse(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(PowerOfExpression(expressions))
    }
}

impl UnaryAddOrSubtractExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<UnaryAddOrSubtractExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::UnaryAddOrSubtractExpression));

        let mut subtract_count = 0;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SP => (),
                Rule::PLUS => (), // FIXME: I don't really understand what the unary plus is supposed to do.
                Rule::MINUS => subtract_count += 1,
                Rule::StringListNullOperatorExpression => return Ok(UnaryAddOrSubtractExpression {negate: subtract_count % 2 == 1, expression: StringListNullOperatorExpression::parse(pair)? }),
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

impl StringListNullOperatorExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<StringListNullOperatorExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::StringListNullOperatorExpression));

        let mut property_or_labels_expression = None;
        let mut operations = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::PropertyOrLabelsExpression => property_or_labels_expression = Some(PropertyOrLabelsExpression::parse(pair)?),
                Rule::StringOperatorExpression => operations.push(StringListNullOperatorExpressionInner::StringOperator(StringOperatorExpression::parse(pair)?)),
                Rule::ListOperatorExpression => operations.push(StringListNullOperatorExpressionInner::ListOperator(ListOperatorExpression::parse(pair)?)),
                Rule::NullOperatorExpression => operations.push(StringListNullOperatorExpressionInner::NullOperator(NullOperatorExpression::parse(pair)?)),
                _ => unreachable!(),
            }
        }

        Ok(StringListNullOperatorExpression { expression: property_or_labels_expression.unwrap(), operations })
    }
}

impl PropertyExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<PropertyExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::PropertyExpression));

        let mut atom = None;
        let mut property_lookup = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Atom => atom = Some(Atom::parse(pair)?),
                Rule::PropertyLookup => property_lookup = Some(parse_property_lookup(pair)?),
                Rule::SP => (),
                _ => unreachable!("Unexpected rule in PropertyExpression: {:?}", pair.as_rule()),
            }
        }

        Ok(PropertyExpression { atom: atom.unwrap(), property_path: property_lookup.unwrap() })
    }
}

impl PropertyOrLabelsExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<PropertyOrLabelsExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::PropertyOrLabelsExpression));

        let mut atom = None;
        let mut property_lookup = Vec::new();
        let mut node_labels = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Atom => atom = Some(Atom::parse(pair)?),
                Rule::PropertyLookup => property_lookup = parse_property_lookup(pair)?,
                Rule::NodeLabels => node_labels = parse_node_labels(pair)?,
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(PropertyOrLabelsExpression { atom: atom.unwrap(), property_lookup, node_labels })
    }
}

fn parse_property_lookup(pair: Pair<'_, Rule>) -> Result<Vec<String>, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::PropertyLookup));

    let mut property_lookup = Vec::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::PropertyKeyName => property_lookup.push(pair.as_str().to_string()),
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    Ok(property_lookup)
}

impl StringOperatorExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<StringOperatorExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::StringOperatorExpression));

        let mut operator = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SP | Rule::WITH => (),
                Rule::STARTS => operator = Some(StringOperator::StartsWith),
                Rule::ENDS => operator = Some(StringOperator::EndsWith),
                Rule::CONTAINS => operator = Some(StringOperator::Contains),
                Rule::PropertyOrLabelsExpression => return Ok(StringOperatorExpression { operator: operator.unwrap(), expression: PropertyOrLabelsExpression::parse(pair)? }),
                _ => unreachable!("Unexpected rule in StringOperatorExpression: {:?}", pair.as_rule()),
            }
        }

        unreachable!()
    }
}

impl ListOperatorExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<ListOperatorExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ListOperatorExpression));

        let mut is_range = false;
        let mut range_start = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SP | Rule::IN => (),
                Rule::DOT_DOT => is_range = true,
                Rule::PropertyOrLabelsExpression => return Ok(ListOperatorExpression::In(PropertyOrLabelsExpression::parse(pair)? )),
                Rule::Expression => if !is_range {
                    range_start = Some(Expression::parse(pair)?);
                }
                else {
                    return Ok(ListOperatorExpression::Range(range_start, Some(Expression::parse(pair)?)));
                },
                _ => unreachable!("Unexpected rule in ListOperatorExpression: {:?}", pair.as_rule()),
            }
        }

        if is_range {
            Ok(ListOperatorExpression::Range(range_start, None))
        }
        else {
            Ok(ListOperatorExpression::Index(range_start.unwrap()))
        }
    }
}

impl Atom {
    fn parse(pair: Pair<'_, Rule>) -> Result<Atom, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Atom));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Literal => return Ok(Atom::Literal(Literal::parse(pair)?)),
                Rule::Parameter => return Ok(Atom::Parameter(parse_parameter(pair)?)),
                Rule::CaseExpression => return Ok(Atom::CaseExpression(CaseExpression::parse(pair)?)),
                Rule::CountStarExpression => return Ok(Atom::CountStar),
                Rule::ListComprehension => return Ok(Atom::ListComprehension(ListComprehension::parse(pair)?)),
                Rule::PatternComprehension => return Ok(Atom::PatternComprehension(PatternComprehension::parse(pair)?)),
                Rule::AllFilterExpression => return Ok(Atom::All(parse_all_any_none_single_filter_expression(pair)?)),
                Rule::AnyFilterExpression => return Ok(Atom::Any(parse_all_any_none_single_filter_expression(pair)?)),
                Rule::NoneFilterExpression => return Ok(Atom::None(parse_all_any_none_single_filter_expression(pair)?)),
                Rule::SingleFilterExpression => return Ok(Atom::Single(parse_all_any_none_single_filter_expression(pair)?)),
                Rule::RelationshipsPattern => return Ok(Atom::RelationshipsPattern(RelationshipsPattern::parse(pair)?)),
                Rule::ParenthesizedExpression => return Ok(Atom::ParenthesizedExpression(parse_parenthesized_expression(pair)?)),
                Rule::FunctionInvocation => return Ok(Atom::FunctionInvocation(FunctionInvocation::parse(pair)?)),
                Rule::Variable => return Ok(Atom::Variable(parse_variable(pair)?)),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

fn parse_parameter(pair: Pair<'_, Rule>) -> Result<String, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::Parameter));
    Ok(pair.as_str().to_string())
}

fn parse_all_any_none_single_filter_expression(pair: Pair<'_, Rule>) -> Result<FilterExpression, Error<Rule>> {
    assert!(
        matches!(pair.as_rule(), Rule::AllFilterExpression) ||
        matches!(pair.as_rule(), Rule::AnyFilterExpression) ||
        matches!(pair.as_rule(), Rule::NoneFilterExpression) ||
        matches!(pair.as_rule(), Rule::SingleFilterExpression));

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::FilterExpression => return Ok(FilterExpression::parse(pair)?),
            Rule::SP | Rule::ANY_ | Rule::ALL | Rule::NONE | Rule::SINGLE => (),
            _ => unreachable!("Unexpected rule in parse_all_any_none_single_filter_expression: {:?}", pair.as_rule()),
        }
    }

    unreachable!()
}

impl FilterExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<FilterExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::FilterExpression));

        let mut id_in_coll = None;
        let mut where_ = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IdInColl => id_in_coll = Some(IdInColl::parse(pair)?),
                Rule::Where => where_ = Some(parse_where(pair)?),
                Rule::SP => (),
                _ => unreachable!("Unexpected rule in FilterExpression: {:?}", pair.as_rule()),
            }
        }

        Ok(FilterExpression { id_in_coll: id_in_coll.unwrap(), where_ })
    }
}

impl IdInColl {
    fn parse(pair: Pair<'_, Rule>) -> Result<IdInColl, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::IdInColl));

        let mut variable = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Variable => variable = Some(parse_variable(pair)?),
                Rule::Expression => return Ok(IdInColl {variable: variable.unwrap(), expression: Expression::parse(pair)?}),
                Rule::SP | Rule::IN => (),
                _ => unreachable!("Unexpected rule in IdInColl: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Unexpected missing rule in IdInColl")
    }
}

impl ListComprehension {
    fn parse(pair: Pair<'_, Rule>) -> Result<ListComprehension, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ListComprehension));

        let mut filter_expression = None;
        let mut expression = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::FilterExpression => filter_expression = Some(FilterExpression::parse(pair)?),
                Rule::Expression => expression = Some(Expression::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!("Unexpected rule in ListComprehension: {:?}", pair.as_rule()),
            }
        }

        Ok(ListComprehension { filter_expression: filter_expression.unwrap(), expression })
    }
}

impl PatternComprehension {
    fn parse(pair: Pair<'_, Rule>) -> Result<PatternComprehension, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::PatternComprehension));
        unimplemented!()
    }
}

impl RelationshipsPattern {
    fn parse(pair: Pair<'_, Rule>) -> Result<RelationshipsPattern, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::RelationshipsPattern));

        let mut node = None;
        let mut relationships = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::NodePattern => node = Some(NodePattern::parse(pair)?),
                Rule::PatternElementChain => relationships.push(parse_pattern_element_chain(pair)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(RelationshipsPattern { node: node.unwrap(), relationships })
    }
}

fn parse_parenthesized_expression(pair: Pair<'_, Rule>) -> Result<Expression, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::ParenthesizedExpression));

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Expression => return Ok(Expression::parse(pair)?),
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    unreachable!()
}

impl FunctionInvocation {
    fn parse(pair: Pair<'_, Rule>) -> Result<FunctionInvocation, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::FunctionInvocation));

        let mut function_name = None;
        let mut distinct = false;
        let mut arguments = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::FunctionName => function_name = Some(pair.as_str().to_string()),
                Rule::DISTINCT => distinct = true,
                Rule::Expression => arguments.push(Expression::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!("Unexpected rule in FunctionInvocation: {:?}", pair.as_rule()),
            }
        }

        Ok(FunctionInvocation { function_name: function_name.unwrap(), distinct, arguments })
    }
}

impl CaseExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<CaseExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::CaseExpression));

        let mut expression = None;
        let mut alternatives = Vec::new();
        let mut else_ = None;

        let mut in_else = false;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Expression => {
                    if in_else {
                        else_ = Some(Expression::parse(pair)?);
                    }
                    else {
                        expression = Some(Expression::parse(pair)?);
                    }
                },
                Rule::CaseAlternative => alternatives.push(parse_case_alternative(pair)?),
                Rule::ELSE => in_else = true,
                Rule::SP | Rule::CASE | Rule::END => (),
                _ => unreachable!("Unexpected rule in CaseExpression: {:?}", pair.as_rule()),
            }
        }

        Ok(CaseExpression { expression, alternatives, else_ })
    }
}

fn parse_case_alternative(pair: Pair<'_, Rule>) -> Result<(Expression, Expression), Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::CaseAlternative));

    let mut when = None;
    let mut in_when = false;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::WHEN => in_when = true,
            Rule::THEN => assert!(when.is_some()),
            Rule::Expression => {
                if in_when {
                    when = Some(Expression::parse(pair)?);
                    in_when = false;
                } else {
                    return Ok((when.unwrap(), Expression::parse(pair)?));
                }
            },
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    unreachable!()
}

impl Literal {
    fn parse(pair: Pair<'_, Rule>) -> Result<Literal, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Literal));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::StringLiteral => return Ok(Literal::String(parse_string_literal(pair)?)),
                Rule::NumberLiteral => return Ok(Literal::Number(NumberLiteral::parse(pair)?)),
                Rule::BooleanLiteral => return Ok(Literal::Boolean(pair.as_str().parse().unwrap())),
                Rule::NULL => return Ok(Literal::Null),
                Rule::MapLiteral => return Ok(Literal::MapLiteral(parse_map_literal(pair)?)),
                Rule::ListLiteral => return Ok(Literal::ListLiteral(parse_list_literal(pair)?)),
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

fn parse_string_literal(pair: Pair<'_, Rule>) -> Result<String, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::StringLiteral));

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::StringDoubleText => return Ok(parse_string_text(pair)?),
            Rule::StringSingleText => return Ok(parse_string_text(pair)?),
            _ => unreachable!(),
        }
    }
    
    unreachable!("Expected rule missing in StringLiteral")
}

fn parse_string_text(pair: Pair<'_, Rule>) -> Result<String, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::StringDoubleText) || matches!(pair.as_rule(), Rule::StringSingleText));

    let mut chars = Vec::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::StringDoubleTextChar => chars.push(parse_string_text_char(pair)?),
            Rule::StringSingleTextChar => chars.push(parse_string_text_char(pair)?),
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    Ok(chars.join(""))
}

fn parse_string_text_char(pair: Pair<'_, Rule>) -> Result<String, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::StringDoubleTextChar) || matches!(pair.as_rule(), Rule::StringSingleTextChar));

    let text = pair.as_str().to_string();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::EscapedChar => return Ok(parse_escaped_char(pair)?),
            _ => unreachable!("Unexpected rule in StringDoubleTextChar or StringSingleTextChar: {:?}", pair.as_rule()),
        }
    }
    
    Ok(text)
}

fn parse_escaped_char(pair: Pair<'_, Rule>) -> Result<String, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::EscapedChar));

    match pair.as_str() {
        "\\'" => Ok("'".to_string()),
        "\\\"" => Ok("\"".to_string()),
        "\\b" | "\\B" => Ok("\u{0008}".to_string()),
        "\\f" | "\\F" => Ok("\u{000C}".to_string()),
        "\\n" | "\\N" => Ok("\n".to_string()),
        "\\r" | "\\R" => Ok("\r".to_string()),
        "\\t" | "\\T" => Ok("\t".to_string()),
        unicode if (unicode.starts_with("\\u") || unicode.starts_with("\\U")) && [6usize, 10usize].contains(&unicode.len()) => {
            let codepoint = u32::from_str_radix(&unicode[2..], 16).unwrap();
            Ok(std::char::from_u32(codepoint).unwrap().to_string())
        }
    _ => unreachable!("Unexpected escaped character: {:?}", pair.as_str()),
    }
}

impl NumberLiteral {
    fn parse(pair: Pair<'_, Rule>) -> Result<NumberLiteral, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::NumberLiteral));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IntegerLiteral => return Ok(NumberLiteral::Integer(pair.as_str().parse().unwrap())),
                Rule::DoubleLiteral => return Ok(NumberLiteral::Double(pair.as_str().parse().unwrap())),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

fn parse_map_literal(pair: Pair<'_, Rule>) -> Result<HashMap<String, Expression>, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::MapLiteral));

    let mut key = None;
    let mut items =HashMap::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::PropertyKeyName => key = Some(pair.to_string()),
            Rule::Expression => {
                items.insert(key.take().unwrap(), Expression::parse(pair)?);
            },
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    assert!(key.is_none());
    Ok(items)
}

fn parse_list_literal(pair: Pair<'_, Rule>) -> Result<Vec<Expression>, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::ListLiteral));

    let mut expressions = Vec::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Expression => expressions.push(Expression::parse(pair)?),
            Rule::SP => (),
            _ => unreachable!(),
        }
    }

    Ok(expressions)
}


impl NullOperatorExpression {
    fn parse(pair: Pair<'_, Rule>) -> Result<NullOperatorExpression, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::NullOperatorExpression));

        let mut not = false;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::NOT => not = true,
                Rule::SP | Rule::IS | Rule::NULL => (),
                _ => unreachable!("Unexpected rule in NullOperatorExpression: {:?}", pair.as_rule()),
            }
        }

        Ok(NullOperatorExpression { not })
    }
}



impl Unwind {
    fn parse(pair: Pair<'_, Rule>) -> Result<Unwind, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Unwind));

        let mut expression = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Expression => expression = Some(Expression::parse(pair)?),
                Rule::Variable => return Ok(Unwind { expression: expression.unwrap(), variable: parse_variable(pair)? }),
                Rule::SP | Rule::UNWIND | Rule::AS => (),
                _ => unreachable!("Unexpected rule in Unwind: {:?}", pair.as_rule()),
            }
        }
        unimplemented!("Expected rule missing in Unwind")
    }
}

impl InQueryCall {
    fn parse(pair: Pair<'_, Rule>) -> Result<InQueryCall, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::InQueryCall));

        let mut invocation = None;
        let mut yield_items = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ExplicitProcedureInvocation => invocation = Some(ExplicitProcedureInvocation::parse(pair)?),
                Rule::YieldItems => yield_items = Some(YieldItems::parse(pair)?),
                Rule::SP | Rule::CALL | Rule::YIELD => (),
                _ => unreachable!(),
            }
        }

        Ok(InQueryCall { invocation: invocation.unwrap(), yield_items })
    }
}

impl YieldItems {
    fn parse(pair: Pair<'_, Rule>) -> Result<YieldItems, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::YieldItems));

        let mut items = Vec::new();
        let mut where_ = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::YieldItem => items.push(YieldItem::parse(pair)?),
                Rule::Where => where_ = Some(parse_where(pair)?),
                Rule::STAR => return Ok(YieldItems::Star),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(YieldItems::Items { items, where_ })
    }
}

impl ExplicitProcedureInvocation {
    fn parse(pair: Pair<'_, Rule>) -> Result<ExplicitProcedureInvocation, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ExplicitProcedureInvocation));

        let mut name = None;
        let mut arguments = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcedureName => name = Some(pair.as_str().to_string()),
                Rule::Expression => arguments.push(Expression::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }
        
        Ok(ExplicitProcedureInvocation { name: name.unwrap(), arguments })
    }
}

impl UpdatingClause {
    fn parse(pair: Pair<'_, Rule>) -> Result<UpdatingClause, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::UpdatingClause));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Create => return Ok(UpdatingClause::Create(Create::parse(pair)?)),
                Rule::Merge => return Ok(UpdatingClause::Merge(Merge::parse(pair)?)),
                Rule::Delete => return Ok(UpdatingClause::Delete(Delete::parse(pair)?)),
                Rule::Set => return Ok(UpdatingClause::Set(Set::parse(pair)?)),
                Rule::Remove => return Ok(UpdatingClause::Remove(Remove::parse(pair)?)),
                _ => unreachable!("Unexpected rule in UpdatingClause: {:?}", pair.as_rule()),
            }
        }

        unreachable!()
    }
}

impl Create {
    fn parse(pair: Pair<'_, Rule>) -> Result<Create, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Create));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Pattern => return Ok(Create {pattern: Pattern::parse(pair)?}),
                Rule::SP | Rule::CREATE => (),
                _ => unreachable!("Unexpected rule in Create: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Expected rule in Create not found")
    }
}

impl Merge {
    fn parse(pair: Pair<'_, Rule>) -> Result<Merge, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Merge));

        let mut pattern_part = None;
        let mut actions = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::PatternPart => pattern_part = Some(PatternPart::parse(pair)?),
                Rule::MergeAction => actions.push(MergeAction::parse(pair)?),
                Rule::SP | Rule::MERGE => (),
                _ => unreachable!("Unexpected rule in Merge: {:?}", pair.as_rule()),
            }
        }

        Ok(Merge { pattern_part: pattern_part.unwrap(), actions })
    }
}

impl Delete {
    fn parse(pair: Pair<'_, Rule>) -> Result<Delete, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Delete));

        let mut detach = false;
        let mut expressions = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::DETACH => detach = true,
                Rule::Expression => expressions.push(Expression::parse(pair)?),
                Rule::SP | Rule::DELETE => (),
                _ => unreachable!("Unexpected rule in Delete: {:?}", pair.as_rule()),
            }
        }

        Ok(Delete { detach, expressions })
    }
}

impl Set {
    fn parse(pair: Pair<'_, Rule>) -> Result<Set, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Set));

        let mut items = Vec::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SetItem => items.push(SetItem::parse(pair)?),
                Rule::SP | Rule::SET => (),
                _ => unreachable!("Unexpected rule in Set: {:?}", pair.as_rule()),
            }
        }

        Ok(Set { items })
    }
}

impl SetItem {
    fn parse(pair: Pair<'_, Rule>) -> Result<SetItem, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::SetItem));

        let mut property_expression = None;

        let mut variable = None;
        let mut eq = false;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Variable => variable = Some(parse_variable(pair)?),
                Rule::PropertyExpression => property_expression = Some(PropertyExpression::parse(pair)?),
                Rule::Expression => if let Some(property_expression) = property_expression.take() {
                    return Ok(SetItem::AssignPropertyExpression { property_expression, expression: Expression::parse(pair)? });
                }
                else if eq {
                    return Ok(SetItem::AssignVariable { variable: variable.unwrap(), expression: Expression::parse(pair)? });
                }
                else {
                    return Ok(SetItem::IncrementVariable { variable: variable.unwrap(), expression: Expression::parse(pair)? });
                },
                Rule::NodeLabels => return Ok(SetItem::VariableNodeLabels { variable: variable.unwrap(), node_labels: parse_node_labels(pair)? }),
                Rule::EQ => eq = true,
                Rule::SP | Rule::SET | Rule::INCREMENT => (),
                _ => unreachable!("Unexpected rule in SetItem: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Expected rule in SetItem not found")
    }
}

impl Remove {
    fn parse(pair: Pair<'_, Rule>) -> Result<Remove, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Remove));

        let mut items = Vec::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::RemoveItem => items.push(RemoveItem::parse(pair)?),
                Rule::SP | Rule::REMOVE => (),
                _ => unreachable!("Unexpected rule in Remove: {:?}", pair.as_rule()),
            }
        }

        Ok(Remove { items })
    }
}

impl RemoveItem {
    fn parse(pair: Pair<'_, Rule>) -> Result<RemoveItem, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::RemoveItem));

        let mut variable = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Variable => variable = Some(parse_variable(pair)?),
                Rule::NodeLabels => return Ok(RemoveItem::VariableNodeLabels { variable: variable.unwrap(), node_labels: parse_node_labels(pair)? }),
                Rule::PropertyExpression => return Ok(RemoveItem::PropertyExpression(PropertyExpression::parse(pair)? )),
                Rule::SP => (),
                _ => unreachable!("Unexpected rule in RemoveItem: {:?}", pair.as_rule()),
            }
        }
        todo!()
    }
}

impl MergeAction {
    fn parse(pair: Pair<'_, Rule>) -> Result<MergeAction, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::MergeAction));

        let mut create = false;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::CREATE => create = true,
                Rule::Set => if create {
                    return Ok(MergeAction::Create(Set::parse(pair)?));
                }
                else {
                    return Ok(MergeAction::Match(Set::parse(pair)?));
                },
                Rule::SP | Rule::ON | Rule::MATCH => (),
                _ => unreachable!("Unexpected rule in MergeAction: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Expected rule in MergeAction not found")
    }
}

impl ProjectionBody {
    fn parse(pair: Pair<'_, Rule>) -> Result<ProjectionBody, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ProjectionBody));

        let mut distinct = false;
        let mut items = None;
        let mut order = None;
        let mut skip = None;
        let mut limit = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::DISTINCT => distinct = true,
                Rule::ProjectionItems => items = Some(ProjectionItems::parse(pair)?),
                Rule::Order => order = Some(Order::parse(pair)?),
                Rule::Skip => skip = Some(Skip::parse(pair)?),
                Rule::Limit => limit = Some(Limit::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(ProjectionBody { distinct, items: items.unwrap(), order, skip, limit })
    }
}

impl ProjectionItems {
    fn parse(pair: Pair<'_, Rule>) -> Result<ProjectionItems, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ProjectionItems));

        let mut items = Vec::new();
        let mut star = false;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::STAR => star = true,
                Rule::ProjectionItem => items.push(ProjectionItem::parse(pair)?),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        Ok(ProjectionItems { star, items })
    }
}

impl ProjectionItem {
    fn parse(pair: Pair<'_, Rule>) -> Result<ProjectionItem, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ProjectionItem));

        let mut expression = None;
        let mut variable = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Expression => expression = Some(Expression::parse(pair)?),
                Rule::Variable => variable = Some(parse_variable(pair)?),
                Rule::SP | Rule::AS => (),
                _ => unreachable!("Unexpected rule in ProjectionItem: {:?}", pair.as_rule()),
            }
        }

        if let Some(variable) = variable {
            Ok(ProjectionItem::AsVariable { expression: expression.unwrap(), variable })
        }
        else {
            Ok(ProjectionItem::Expression(expression.unwrap()))
        }
    }
}

impl Order {
    fn parse(pair: Pair<'_, Rule>) -> Result<Order, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Order));

        let mut items = Vec::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SortItem => items.push(SortItem::parse(pair)?),
                Rule::SP | Rule::ORDER | Rule::BY => (),
                _ => unreachable!(),
            }
        }

        Ok(Order { items })
    }
}

impl SortItem {
    fn parse(pair: Pair<'_, Rule>) -> Result<SortItem, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::SortItem));

        let mut expression = None;
        let mut descending = false;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Expression => expression = Some(Expression::parse(pair)?),
                Rule::DESCENDING | Rule::DESC => descending = true,
                Rule::ASCENDING| Rule::ASC => descending = false,
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        if descending {
            Ok(SortItem::Descending(expression.unwrap()))
        }
        else {
            Ok(SortItem::Ascending(expression.unwrap()))
        }
        
    }
}

impl Skip {
    fn parse(pair: Pair<'_, Rule>) -> Result<Skip, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Skip));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Expression => return Ok(Skip {expression: Expression::parse(pair)?}),
                Rule::SP | Rule::SKIP => (),
                _ => unreachable!("Unexpected rule in Skip: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Expected rule in Skip not found")
    }
}

impl Limit {
    fn parse(pair: Pair<'_, Rule>) -> Result<Limit, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Limit));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Expression => return Ok(Limit {expression: Expression::parse(pair)?}),
                Rule::SP | Rule::LIMIT => (),
                _ => unreachable!("Unexpected rule in Limit: {:?}", pair.as_rule()),
            }
        }
        
        unreachable!("Expected rule in Limit not found")
    }
}

impl MultiPartQuery {
    fn parse(pair: Pair<'_, Rule>) -> Result<MultiPartQuery, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::MultiPartQuery));

        let mut parts = Vec::new();
        let mut reading_clauses = Vec::new();
        let mut updating_clauses = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ReadingClause => reading_clauses.push(ReadingClause::parse(pair)?),
                Rule::UpdatingClause => updating_clauses.push(UpdatingClause::parse(pair)?),
                Rule::With => {
                    parts.push(MultiPartQueryPart { reading_clauses, updating_clauses, with: With::parse(pair)? });
                    reading_clauses = Vec::new();
                    updating_clauses = Vec::new();
                },
                Rule::SP => (),
                Rule::SinglePartQuery => return Ok(MultiPartQuery { parts, query: SinglePartQuery::parse(pair)? }),
                _ => unreachable!("Unexpected rule in MultiPartQuery: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Expected rule in MultiPartQuery not found")
    }
}

impl With {
    fn parse(pair: Pair<'_, Rule>) -> Result<With, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::With));

        let mut projection_body = None;
        let mut where_ = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProjectionBody => projection_body = Some(ProjectionBody::parse(pair)?),
                Rule::Where => where_ = Some(parse_where(pair)?),
                Rule::SP | Rule::WITH => (),
                _ => unreachable!(),
            }
        }

        Ok(With { projection_body: projection_body.unwrap(), where_ })
    }
}

impl Union {
    fn parse(pair: Pair<'_, Rule>) -> Result<Union, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::Union));

        let mut all = false;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ALL => all = true,
                Rule::SingleQuery => return Ok(Union { all, query: SingleQuery::parse(pair)? }),
                Rule::SP | Rule::UNION => (),
                _ => unreachable!("Unexpected rule in Union: {:?}", pair.as_rule()),
            }
        }
        
        unreachable!("Expected rule in Union not found")
    }
}

impl StandaloneCall {
    fn parse(pair: Pair<'_, Rule>) -> Result<StandaloneCall, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::StandaloneCall));

        let mut invocation = None;
        let mut yield_items: Option<YieldItems> = None;
        let mut star = false;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ExplicitProcedureInvocation => invocation = Some(ProcedureInvocation::Explicit(ExplicitProcedureInvocation::parse(pair)?)),
                Rule::ImplicitProcedureInvocation => invocation = Some(ProcedureInvocation::Implicit(ImplicitProcedureInvocation::parse(pair)?)),
                Rule::STAR => star = true,
                Rule::YieldItems => yield_items = Some(YieldItems::parse(pair)?),
                Rule::SP | Rule::CALL | Rule::YIELD => (),
                _ => unreachable!("Unexpected rule in StandaloneCall: {:?}", pair.as_rule()),
            }
        }

        let yield_ = if star {
            Some(StarOrYieldItems::Star)
        } else if let Some(items) = yield_items {
            Some(StarOrYieldItems::YieldItems(items))
        } else {
            None
        };

        Ok(StandaloneCall {
            invocation: invocation.unwrap(),
            yield_,
        })
    }
}

impl ImplicitProcedureInvocation {
    fn parse(pair: Pair<'_, Rule>) -> Result<ImplicitProcedureInvocation, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::ImplicitProcedureInvocation));

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcedureName => return Ok(ImplicitProcedureInvocation { name: pair.as_str().to_string() }),
                Rule::SP => (),
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

impl YieldItem {
    fn parse(pair: Pair<'_, Rule>) -> Result<YieldItem, Error<Rule>> {
        assert!(matches!(pair.as_rule(), Rule::YieldItem));

        let mut procedure_result_field = None;
        let mut variable = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcedureResultField => procedure_result_field = Some(parse_procedure_result_field(pair)?),
                Rule::Variable => variable = Some(parse_variable(pair)?),
                Rule::SP | Rule::AS => (),
                _ => unreachable!("Unexpected rule in YieldItem: {:?}", pair.as_rule()),
            }
        }

        Ok(YieldItem { procedure_result_field, variable: variable.unwrap() })
    }
}

fn parse_procedure_result_field(pair: Pair<'_, Rule>) -> Result<String, Error<Rule>> {
    assert!(matches!(pair.as_rule(), Rule::ProcedureResultField));

    Ok(pair.as_str().to_string())
}