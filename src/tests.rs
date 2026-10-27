use sparqlers::fluent::{New, Query, Service};
use sparqlers::namespace::{wd::*, wdt::*};
use sparqlers::{Entity, Literal, Plain, Property, Tag, Var};

#[test]
fn fluent() {
    let stadium = Var::name("stadium");
    let stadium_label = stadium.label();

    let instance_of = Property(Plain(WDT, 31));

    let defunct_sports_team = Entity(WD, 15896028);

    let query = Query::<New>::select(vec![&stadium, &stadium_label]);

    assert_eq!(query.statement, "SELECT ?stadium ?stadiumLabel");

    let query = query.where_();

    assert_eq!(query.statement, "SELECT ?stadium ?stadiumLabel WHERE {");

    let query = query.add((&stadium, &instance_of, &defunct_sports_team));

    assert_eq!(
        query.statement,
        "SELECT ?stadium ?stadiumLabel WHERE { ?stadium wdt:P31 wd:Q15896028 ."
    );

    let query = query.service(
        &Tag("wikibase", "label".to_string()),
        Service::new().add(
            &Tag("bd", "serviceParam".to_string()),
            &Tag("wikibase", "language".to_string()),
            &Literal("en"),
        ),
    );

    assert_eq!(
        query.statement,
        "SELECT ?stadium ?stadiumLabel WHERE { ?stadium wdt:P31 wd:Q15896028 . SERVICE wikibase:label { bd:serviceParam wikibase:language \"en\" . }"
    );

    let query = query.limit(10);

    assert_eq!(
        query.statement,
        "SELECT ?stadium ?stadiumLabel WHERE { ?stadium wdt:P31 wd:Q15896028 . SERVICE wikibase:label { bd:serviceParam wikibase:language \"en\" . } } LIMIT 10"
    );
}

#[test]
fn literal() {
    let v = Var("v".to_string());
    let p = Var("p".to_string());

    let query = Query::<New>::select(vec![&v])
        .where_()
        .add((&v, &p, &Literal("cat")));

    assert_eq!(query.statement, "SELECT ?v WHERE { ?v ?p \"cat\" .");

    let query = Query::<New>::select(vec![&v])
        .where_()
        .add((&v, &p, &Literal("cat").lang("en")));

    assert_eq!(query.statement, "SELECT ?v WHERE { ?v ?p \"cat\"@en .");

    let query = Query::<New>::select(vec![&v]).where_().add((
        &v,
        &p,
        &Literal("2019-06-12").typed("xsd:dateTime"),
    ));

    assert_eq!(
        query.statement,
        "SELECT ?v WHERE { ?v ?p \"2019-06-12\"^^xsd:dateTime ."
    );
}
