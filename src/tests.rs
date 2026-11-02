use sparqlers::fluent::{New, Query, Service};
use sparqlers::namespace::{wd::*, wdt::*};
use sparqlers::{Entity, Literal, PathPrefix, PathSuffix, Plain, Property, SparqlTerm, Statement, Tag, Var};

#[test]
fn fluent() {
    let stadium = Var::name("stadium");
    let stadium_label = stadium.label();

    let instance_of = Property(Plain(WDT, 31));

    let defunct_sports_team = Entity(WD, 15896028);

    let query = Query::<New>::select(&[&stadium, &stadium_label]);

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
    let p = Property(Plain(WDT, 23));

    let query = Query::<New>::select(&[&v])
        .where_()
        .add((&v, &p, &Literal("cat")));

    assert_eq!(query.statement, "SELECT ?v WHERE { ?v wdt:P23 \"cat\" .");

    let query = Query::<New>::select(&[&v])
        .where_()
        .add((&v, &p, &Literal("cat").lang("en")));

    assert_eq!(query.statement, "SELECT ?v WHERE { ?v wdt:P23 \"cat\"@en .");

    let query = Query::<New>::select(&[&v]).where_().add((
        &v,
        &p,
        &Literal("2019-06-12").typed("xsd:dateTime"),
    ));

    assert_eq!(
        query.statement,
        "SELECT ?v WHERE { ?v wdt:P23 \"2019-06-12\"^^xsd:dateTime ."
    );
}

#[test]
fn predicate_objects() {
    let player = Var("player".to_string());
    let foo = Property(Plain(WDT, 23));
    let bar = Property(Plain(WDT, 42));
    let object = Entity(WD, 123);
    let var_object = Var("object".to_string());

    let clause = player.properties(&[foo.is(&object), bar.set(&var_object)]);

    assert_eq!(
        clause.to_statement(),
        "?player wdt:P23 wd:Q123 ; wdt:P42 ?object ."
    );
}

#[test]
fn prefix_suffix() {
    let player = Var("player".to_string());
    let prop = Property(Plain(WDT, 23)).suffixed(PathSuffix::OneOrMore);
    let object = Entity(WD, 123); 

    let obj_pred = player.property(&prop);
    let clause = obj_pred.is(&object);

    assert_eq!(
        clause.to_statement(),
        "?player wdt:P23+ wd:Q123 ."
    );

    let inv_prop = Property(Plain(WDT, 42)).prefixed(PathPrefix::Inverse);
    
    let obj_pred = player.property(&inv_prop);
    let clause = obj_pred.is(&object);

    assert_eq!(
        clause.to_statement(),
        "?player ^wdt:P42 wd:Q123 ."
    );
}

#[test]
fn statement() {
    let media = Var("media".to_string());
    let actor = Var("actor".to_string());
    let role = Var("role".to_string());
    let character_role = Property(Plain(WDT, 453));
    let cast_member = Property(Plain(WDT, 161));
    let voice_actor = Property(Plain(WDT, 725));

    let clause = Statement(&media, &cast_member, &actor, &character_role, &role);
    // TODO support multiple properties in the statement, e.g. (ps:P161|ps:P725)
    
    // assert_eq!(
    //     clause.to_statement(),
    //     "?media p:P161 [ ps:P161 ?actor ; pq:P453 ?role ] ."
    // );

}