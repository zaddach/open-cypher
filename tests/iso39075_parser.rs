use pest::Parser;

#[rstest::rstest]

#[case("CREATE GRAPH mySocialNetwork ::socialNetworkGraphType")]
#[case("CREATE GRAPH mySocialNetwork TYPED socialNetworkGraphType")]
//#[case("CREATE GRAPH mySocialNetwork ::{(City :City {name STRING, state STRING, country STRING})}")]
// #[case("CREATE GRAPH mygraph ANY

// CREATE GRAPH mygraph {
//   (Person :Person {lastname STRING, firstname STRING,joined DATE})
// }

// CREATE GRAPH mygraph mygraphtype

// CREATE GRAPH /mygraph LIKE /mysrcgraph

// CREATE GRAPH mygraph ANY AS COPY OF mysrcgraph

// CREATE GRAPH mygraph {
//   (Person :Person {lastname STRING, firstname STRING,joined DATE})
// } AS COPY OF mysrcgraph")]
#[case("CREATE SCHEMA /myschema

CREATE SCHEMA /foo/myschema

CREATE SCHEMA /foo
NEXT CREATE SCHEMA /fee")]
#[case("INSERT (:Person { firstname: 'Firstname', lastname: 'Lastname', joined: DATE '2023-01-01' })-[:MEMBER_SINCE { since: \"2023-03-20\" }]->(:Team { name: 'Teamname' })")]
#[case("MATCH (a { firstname: 'Robert' }), (b { lastname: 'Kowalski' }) INSERT (a)-[:GRADUATED]->(b)")]
#[case("MATCH (p:Person)-[r:IS_FRIENDS_WITH]->(friend:Person) WHERE EXISTS (MATCH (p)-[:WORKS_FOR]->(:Company {name: \"GQL, Inc.\"})) RETURN p, r, friend")]
// #[case("MATCH (p:Person)-[r:IS_FRIENDS_WITH]->(friend:Person) WHERE EXISTS { MATCH (p)-[:WORKS_FOR]->(:Company { name: \"GQL, Inc.\" }) RETURN p } RETURN p, r, friend")]
#[case("SESSION SET GRAPH CURRENT_GRAPH")]
#[case("SESSION SET GRAPH CURRENT_PROPERTY_GRAPH")]
//#[case("SESSION SET VALUE IF NOT EXISTS $exampleProperty = DATE '2022-10-10'")]
#[case("SESSION SET TIME ZONE \"utc\"")]
fn parser_test_success(#[case] query: &str) {
    let _ = open_cypher::parser::iso_39075::GqlParser::parse(open_cypher::parser::iso_39075::Rule::GqlProgram, query).unwrap();
}
