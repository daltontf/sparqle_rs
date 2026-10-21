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
}

pub mod wdt {
    use crate::Property;

    pub const WDT: &str = "wdt";

    pub const COUNTRY: Property = Property(WDT, 17);
    pub const INSTANCE_OF: Property = Property(WDT, 31);
    pub const SUBCLASS_OF: Property = Property(WDT, 279);

    pub const LEAGUE_OR_COMPETITION: Property = Property(WDT, 118);
    pub const LOCATED_IN_TERRITORIAL_ENTITY: Property = Property(WDT, 131);
    pub const OCCUPANT: Property = Property(WDT, 466);
    pub const COORDINATE_LOCATION: Property = Property(WDT, 625);

    pub const END_TIME: Property = Property(WDT, 582);
}