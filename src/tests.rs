use sparqlers::fluent::{New, Query, Service};
use sparqlers::namespace::{wd::*, wdt::*};
use sparqlers::{Constant, Entity, Property, Tag, Var};

#[test]
fn fluent() {
    let stadium = Var::name("stadium");
    let stadium_label = stadium.label();

    let instance_of = Property(WDT, 31);

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
            &Constant("en"),
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
