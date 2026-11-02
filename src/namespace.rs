pub mod wd {
    use crate::Entity;

    pub const WD: &str = "wd";

    pub const STADIUM: Entity = Entity(WD, 483110);
    pub const SPORTS_CLUB: Entity = Entity(WD, 847017);
    pub const SPORTS_TEAM: Entity = Entity(WD, 12973014);
    pub const SPORTS_VENUE: Entity = Entity(WD, 1076486);
    pub const ASSOCIATION_FOOTBALL_VENUE: Entity = Entity(WD, 1154710);
    pub const ASSOCIATION_FOOTBALL_CLUB: Entity = Entity(WD, 476028);
    pub const MENS_ASSOCIATION_FOOTBALL_CLUB: Entity = Entity(WD, 103231176);
    pub const WOMENS_ASSOCIATION_FOOTBALL_CLUB: Entity = Entity(WD, 51481377);
    pub const COLLEGE_SPORTS_TEAM: Entity = Entity(WD, 18558301);
    pub const HIGH_SCHOOL_SPORTS_TEAM: Entity = Entity(WD, 117715247);
    pub const DEFUNCT_SPORTS_TEAM: Entity = Entity(WD, 15896028);

    pub const UNITED_KINGDOM: Entity = Entity(WD, 145);
    pub const UNITED_STATES: Entity = Entity(WD, 30);
    pub const MAJOR_LEAGUE_SOCCER: Entity = Entity(WD, 18543);
    pub const USL_CHAMPIONSHIP: Entity = Entity(WD, 1362411);

    pub const DOUGLAS_ADAMS: Entity = Entity(WD, 42); // Don't Panic
}

pub mod wdt {
    use crate::{Property, Wikidata};

    pub const WDT: &str = "wdt";

    pub const COUNTRY: Property<Wikidata> = Property(Wikidata(WDT, 17));
    pub const INSTANCE_OF: Property<Wikidata> = Property(Wikidata(WDT, 31));
    pub const SUBCLASS_OF: Property<Wikidata> = Property(Wikidata(WDT, 279));
    pub const OFFICIAL_WEBSITE: Property<Wikidata> = Property(Wikidata(WDT, 856));

    pub const LEAGUE_OR_COMPETITION: Property<Wikidata> = Property(Wikidata(WDT, 118));
    pub const LOCATED_IN_TERRITORIAL_ENTITY: Property<Wikidata> = Property(Wikidata(WDT, 31));
    pub const OCCUPANT: Property<Wikidata> = Property(Wikidata(WDT, 466));
    pub const COORDINATE_LOCATION: Property<Wikidata> = Property(Wikidata(WDT, 625));

    pub const END_TIME: Property<Wikidata> = Property(Wikidata(WDT, 582));
}
