use sparql_client::SparqlClient;
use sparqlers::fluent::{Filter, New, Query, Service};
use sparqlers::namespace::{wd::*, wdt::*};
use sparqlers::{Blank, Literal, PathRepeat, Tag, Var};

#[tokio::main]
async fn main() {
    let client = SparqlClient::new("https://query.wikidata.org/sparql");

    let stadium = Var::name("stadium");
    let coordinates = Var::name("coordinates");
    let occupant = Var::name("occupant");
    let website = Var::name("website");

    let is_or_child_of = INSTANCE_OF.append(&SUBCLASS_OF.repeated(PathRepeat::ZeroOrMore));

    match Query::<New>::select_distinct(vec![
        &stadium,
        &stadium.label(),
        &coordinates,
        &coordinates.label(),
        &occupant,
        &occupant.label(),
        &website.label(),
    ])
    .where_()
    .add(stadium.property(&COUNTRY).is(&UNITED_STATES))
    .add(stadium.property_path(&is_or_child_of).is(&SPORTS_VENUE))
    .add(stadium.property(&COORDINATE_LOCATION).set(&coordinates))
    .add(
        stadium
            .property(&OCCUPANT.to_stmt())
            .set(&Var::name("stmt")),
    )
    .add(Var::name("stmt").property(&OCCUPANT.stmt()).set(&occupant))
    .add(
        occupant
            .property_path(&is_or_child_of)
            .is(&ASSOCIATION_FOOTBALL_CLUB),
    )
    .optional(occupant.property(&OFFICIAL_WEBSITE).set(&website))
    .union(vec![
        occupant
            .property(&LEAGUE_OR_COMPETITION)
            .is(&MAJOR_LEAGUE_SOCCER),
        occupant
            .property(&LEAGUE_OR_COMPETITION)
            .is(&USL_CHAMPIONSHIP),
    ])
    .filter_not_exists(
        Filter::new()
            .values(
                Var::name("values"),
                vec![
                    // &COLLEGE_SPORTS_TEAM,
                    // &HIGH_SCHOOL_SPORTS_TEAM,
                    &DEFUNCT_SPORTS_TEAM,
                ],
            )
            .add(
                occupant
                    .property_path(&is_or_child_of)
                    .set(&Var::name("values")),
            ),
    )
    .filter_not_exists(Filter::new().add((&occupant, &END_TIME.qualifier(), &Blank)))
    .service(
        &Tag("wikibase", "label".to_string()),
        Service::new().add(
            &Tag("bd", "serviceParam".to_string()),
            &Tag("wikibase", "language".to_string()),
            &Literal("en"),
        ),
    )
    .order_by_asc(&occupant.label())
    .limit(100)
    .query(&client)
    .await
    {
        Ok(rows) => {
            for row in rows {
                println!(
                    "{} @ {} - {}",
                    row.get("stadiumLabel")
                        .map(|it| &it.value)
                        .unwrap_or(&"".to_string()),
                    row.get("coordinates")
                        .map(|it| &it.value)
                        .unwrap_or(&"".to_string()),
                    row.get("occupantLabel")
                        .map(|it| &it.value)
                        .unwrap_or(&"".to_string()),
                );
            }
        }
        Err(e) => eprintln!("query failed: {e:?}"),
    };
}
