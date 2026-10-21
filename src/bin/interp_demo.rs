use sparql_client::SparqlClient;
use sparqlers::namespace::{wd::*, wdt::*};
use sparqlers::{PropertyPath, Var};

#[tokio::main]
async fn main() {
    let client = SparqlClient::new("https://query.wikidata.org/sparql");

    let stadium = Var::name("stadium");
    let coordinates = Var::name("coordinates");
    let occupant = Var::name("occupant");

    let stadium_label = stadium.label();
    let coordinates_label = coordinates.label();
    let occupant_label = occupant.label();

    let occupant_to_stmt = OCCUPANT.to_stmt();
    let occupant_stmt = OCCUPANT.stmt();

    let end_time_qualifier = END_TIME.qualifier();

    let is_or_child_of = PropertyPath(vec![&INSTANCE_OF, &SUBCLASS_OF]);

    match client
        .sparql_query(
             &format!(r#"
            SELECT {stadium} {stadium_label} {coordinates} {coordinates_label} {occupant} {occupant_label} WHERE {{
              {stadium} {is_or_child_of} {STADIUM} .
              {stadium} {COUNTRY} {UNITED_STATES} .
              {stadium} {COORDINATE_LOCATION} {coordinates} .
              {stadium} {OCCUPANT} {occupant} .
              {occupant} {is_or_child_of} {ASSOCIATION_FOOTBALL_CLUB} .
    
              FILTER NOT EXISTS {{
                VALUES ?excludeTeam {{ {COLLEGE_SPORTS_TEAM} {HIGH_SCHOOL_SPORTS_TEAM} {DEFUNCT_SPORTS_TEAM} }}
                {occupant} {INSTANCE_OF}/{SUBCLASS_OF}* ?excludeTeam .                
              }}

              {stadium} {occupant_to_stmt} ?statement .
              ?statement {occupant_stmt} {occupant} .

              FILTER NOT EXISTS {{
               {occupant} {end_time_qualifier} ?endTime .
              }}
    
              SERVICE wikibase:label {{
                bd:serviceParam wikibase:language "en" .
              }}
            }} LIMIT 100
          "#)).await {
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
