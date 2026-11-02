use sparqlers::fluent::{New, Query, Service};
use sparqlers::namespace::{wd::*, wdt::*};
use sparqlers::{
    Blank, Entity, Literal, PathPrefix, PathSuffix, Property, Qualifiable, SparqlTerm, Tag, Var,
    Wikidata,
};

#[test]
fn fluent() {
    let stadium = Var::name("stadium");
    let stadium_label = stadium.label();

    let instance_of = Property(Wikidata(WDT, 31));

    let defunct_sports_team = Entity(WD, 15896028);

    let query = Query::<New>::select(&[&stadium, &stadium_label]);

    assert_eq!(query.sparql, "SELECT ?stadium ?stadiumLabel");

    let query = query.where_();

    assert_eq!(query.sparql, "SELECT ?stadium ?stadiumLabel WHERE {");

    let query = query.add((&stadium, &instance_of, &defunct_sports_team));

    assert_eq!(
        query.sparql,
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
        query.sparql,
        "SELECT ?stadium ?stadiumLabel WHERE { ?stadium wdt:P31 wd:Q15896028 . SERVICE wikibase:label { bd:serviceParam wikibase:language \"en\" . }"
    );

    let query = query.limit(10);

    assert_eq!(
        query.sparql,
        "SELECT ?stadium ?stadiumLabel WHERE { ?stadium wdt:P31 wd:Q15896028 . SERVICE wikibase:label { bd:serviceParam wikibase:language \"en\" . } } LIMIT 10"
    );
}

#[test]
fn literal() {
    let v = Var("v".to_string());
    let p = Property(Wikidata(WDT, 23));

    let query = Query::<New>::select(&[&v])
        .where_()
        .add((&v, &p, &Literal("cat")));

    assert_eq!(query.sparql, "SELECT ?v WHERE { ?v wdt:P23 \"cat\" .");

    let query = Query::<New>::select(&[&v])
        .where_()
        .add((&v, &p, &Literal("cat").lang("en")));

    assert_eq!(query.sparql, "SELECT ?v WHERE { ?v wdt:P23 \"cat\"@en .");

    let query = Query::<New>::select(&[&v]).where_().add((
        &v,
        &p,
        &Literal("2019-06-12").typed("xsd:dateTime"),
    ));

    assert_eq!(
        query.sparql,
        "SELECT ?v WHERE { ?v wdt:P23 \"2019-06-12\"^^xsd:dateTime ."
    );
}

#[test]
fn predicate_objects() {
    let player = Var("player".to_string());
    let foo = Property(Wikidata(WDT, 23));
    let bar = Property(Wikidata(WDT, 42));
    let object = Entity(WD, 123);
    let var_object = Var("object".to_string());

    let clause = player.properties(&[foo.is(&object), bar.set(&var_object)]);

    assert_eq!(
        clause.to_sparql(),
        "?player wdt:P23 wd:Q123 ; wdt:P42 ?object ."
    );
}

#[test]
fn prefix_suffix() {
    let player = Var("player".to_string());
    let prop = Property(Wikidata(WDT, 23)).suffixed(PathSuffix::OneOrMore);
    let object = Entity(WD, 123);

    let obj_pred = player.property(&prop);
    let clause = obj_pred.is(&object);

    assert_eq!(clause.to_sparql(), "?player wdt:P23+ wd:Q123 .");

    let inv_prop = Property(Wikidata(WDT, 42)).prefixed(PathPrefix::Inverse);

    let obj_pred = player.property(&inv_prop);
    let clause = obj_pred.is(&object);

    assert_eq!(clause.to_sparql(), "?player ^wdt:P42 wd:Q123 .");
}

#[test]
fn property_or() {
    let cast_member = Property(Wikidata(WDT, 161));
    let voice_actor = Property(Wikidata(WDT, 725));

    let clause = cast_member.or(&voice_actor);

    assert_eq!(clause.link_statement().to_sparql(), "p:P161|p:P725");

    assert_eq!(clause.to_statement().to_sparql(), "ps:P161|ps:P725");

    assert_eq!(clause.to_qualified().to_sparql(), "pq:P161|pq:P725");
}

#[test]
fn blank_expression() {
    let media = Var("media".to_string());
    let actor = Var("actor".to_string());
    let role = Var("role".to_string());
    let character_role = Property(Wikidata(WDT, 453));
    let cast_member = Property(Wikidata(WDT, 161));
    let voice_actor = Property(Wikidata(WDT, 725));

    let cast_link = cast_member.or(&voice_actor).link_statement();
    let cast_statement = media.property(&cast_link);
    let cast_link_statement = cast_member.or(&voice_actor).to_statement();
    let role_qualified = character_role.to_qualified();
    let blank = Blank(&[cast_link_statement.set(&actor), role_qualified.set(&role)]);

    let clause = cast_statement.is(&blank);

    assert_eq!(
        clause.to_sparql(),
        "?media p:P161|p:P725 [ ps:P161|ps:P725 ?actor ; pq:P453 ?role ] ."
    );
}
