use sparql_client::SparqlClient;
use sparqlers::fluent::{Filter, New, Query, Service};
use sparqlers::namespace::{wd::*, wdt::*};
use sparqlers::{PropertyPath, Var, Tag, Constant};

#[tokio::main]
async fn main() {
    let client = SparqlClient::new("https://query.wikidata.org/sparql");

    let stadium = Var::name("stadium");
    let coordinates = Var::name("coordinates");
    let occupant = Var::name("occupant");

    let is_or_child_of = PropertyPath(vec![&INSTANCE_OF, &SUBCLASS_OF]);

    match Query::<New>::select(vec![
        &stadium,
        &stadium.label(),
        &coordinates,
        &coordinates.label(),
        &occupant,
        &occupant.label(),
    ])
    .where_()
    .add(&stadium, &COUNTRY, &UNITED_STATES)
    .add(&stadium, &is_or_child_of, &SPORTS_VENUE)
    .add(&stadium, &COORDINATE_LOCATION, &coordinates)
    .add(&stadium, &OCCUPANT, &occupant)
    .add(&occupant, &is_or_child_of, &ASSOCIATION_FOOTBALL_CLUB)
    .filter_not_exists(
        Filter::new()
            .values(
                Var::name("foo"),
                vec![
                    &COLLEGE_SPORTS_TEAM,
                    &HIGH_SCHOOL_SPORTS_TEAM,
                    &DEFUNCT_SPORTS_TEAM,
                ],
            )
            .add(&occupant, &is_or_child_of, &Var::name("foo")),
    )
    .service(&Tag("wikibase", "label".to_string()), Service::new()
        .add(&Tag("bd", "serviceParam".to_string()), &Tag("wikibase", "language".to_string()), &Constant("en")))
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

//                {{ ?occupant {LEAGUE_OR_COMPETITION} {MAJOR_LEAGUE_SOCCER} }}
//          UNION {{ ?occupant {LEAGUE_OR_COMPETITION} {USL_CHAMPIONSHIP} }}
