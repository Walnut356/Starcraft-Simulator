use crate::*;

// TODO get build times from editor

// pub const : Self = Self::new(
//         Base::,
//         Faction::,
//         Target::,
//         Flags::new_std(Flag::, Flag::),
//         ,
//         ,
//         (Weapon::, None, None),
//         Armor::new(, ),
//         Cost::new(, , , const_real!()),
//         const_real!(),
//         const_real!(),
//     );

impl Unit {
    // ------------------------------------------------------------------------------------------ //
    //                                           PROTOSS                                          //
    // ------------------------------------------------------------------------------------------ //

    // ------------------------------------------ units ----------------------------------------- //
    // pub const ZEALOT: Self = Self::new(
    //     Base::Zealot,
    //     Faction::Protoss,
    //     Target::Ground,
    //     Flags::new_std(Flag::Light, Flag::Biological),
    //     100,
    //     50,
    //     (Weapon::ZEALOT, None, None),
    //     Armor::new(1, 0),
    //     Cost::new(100, 0, 2, const_real!(27.0)),
    //     const_real!(0.5),
    //     const_real!(2.25 * GAME_SPEED),
    // );

    pub const STALKER: Self = Self::new(
        Base::Stalker,
        Faction::Protoss,
        Target::Ground,
        Flags::new_std(Flag::Armored, Flag::Mechanical),
        80,
        80,
        (Weapon::STALKER, None, None),
        Armor::new(1, 0),
        Cost::new(125, 50, 2, duration!(42.0)),
        const_real!(0.625),
        rate!(2.9531),
    );

    pub const ADEPT: Self = Self::new(
        Base::Adept,
        Faction::Protoss,
        Target::Ground,
        Flags::new_std(Flag::Light, Flag::Biological),
        70,
        70,
        (Weapon::ADEPT, None, None),
        Armor::new(1, 0),
        Cost::new(100, 25, 2, duration!(42)),
        const_real!(0.5),
        rate!(2.5),
    );

    // --------------------------------------- structures --------------------------------------- //

    // ------------------------------------------------------------------------------------------ //
    //                                           TERRAN                                           //
    // ------------------------------------------------------------------------------------------ //

    // ------------------------------------------ units ----------------------------------------- //

    pub const MARINE: Self = Self::new(
        Base::Marine,
        Faction::Terran,
        Target::Ground,
        Flags::new_std(Flag::Light, Flag::Biological),
        45,
        0,
        (Weapon::MARINE, None, None),
        Armor::new(0, 0),
        Cost::new(50, 0, 1, duration!(25)),
        const_real!(0.375),
        rate!(2.25),
    );

    pub const MARAUDER: Self = Self::new(
        Base::Marauder,
        Faction::Terran,
        Target::Ground,
        Flags::new_std(Flag::Armored, Flag::Biological),
        125,
        0,
        (Weapon::MARAUDER, None, None),
        Armor::new(1, 0),
        Cost::new(100, 25, 2, duration!(30)),
        const_real!(0.625),
        rate!(2.25),
    );

    // --------------------------------------- structures --------------------------------------- //

    // ------------------------------------------------------------------------------------------ //
    //                                            ZERG                                            //
    // ------------------------------------------------------------------------------------------ //

    // ------------------------------------------ units ----------------------------------------- //

    // pub const ZERGLING: Self = Self::new(
    //     Base::Zergling,
    //     Faction::Zerg,
    //     Target::Ground,
    //     Flags::new_std(Flag::Light, Flag::Biological),
    //     35,
    //     0,
    //     (Weapon::ZERGLING, None, None),
    //     Armor::new(0, 0),
    //     Cost {
    //         minerals: const_real!(25),
    //         gas: const_real!(0),
    //         supply: const_real!(0.5),
    //         build_time: const_real!(17),
    //     },
    //     const_real!(0.375),
    //     const_real!(2.9531 * GAME_SPEED),
    // );

    pub const ROACH: Self = Self::new(
        Base::Roach,
        Faction::Zerg,
        Target::Ground,
        Flags::new_std(Flag::Armored, Flag::Biological),
        145,
        0,
        (Weapon::ROACH, None, None),
        Armor::new(1, 0),
        Cost::new(75, 25, 2, duration!(27.0)),
        const_real!(0.625),
        rate!(2.25),
    )
    .with_health_regen(rate!(0.273));

    pub const HYDRA: Self = Self::new(
        Base::Hydralisk,
        Faction::Zerg,
        Target::Ground,
        Flags::new_std(Flag::Light, Flag::Biological),
        90,
        0,
        (Weapon::HYDRA, None, None),
        Armor::new(0, 0),
        Cost::new(100, 50, 2, duration!(33.0)),
        const_real!(0.625),
        rate!(2.25),
    )
    .with_health_regen(rate!(0.273));

    // --------------------------------------- structures --------------------------------------- //
}
