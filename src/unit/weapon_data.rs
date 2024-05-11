use crate::*;

impl Weapon {
    // ------------------------------------------------------------------------------------------ //
    //                                           PROTOSS                                          //
    // ------------------------------------------------------------------------------------------ //

    // ------------------------------------------ units ----------------------------------------- //
    // pub const ZEALOT: Self = Self::new(
    //     8,
    //     1,
    //     const_real!(1.2 / GAME_SPEED),
    //     0,
    //     None,
    //     Target::Ground,
    //     [1, 0],
    //     const_real!(0.1),
    //     ProjType::Hitscan,
    // );

    pub const STALKER: Self = Self::new(
        13,
        Multihit::Single,
        duration!(1.87),
        5,
        Some(Flag::Armored),
        Target::Both,
        [1, 1],
        const_real!(6),
        ProjType::Projectile(rate!(18.75)),
        duration!(0.5),
        duration!(0.167),
    );

    // pub const ADEPT: Self = Self::new(
    //     10,
    //     1,
    //     const_real!(2.25 / GAME_SPEED),
    //     12,
    //     Some(Flag::Light),
    //     Target::Ground,
    //     [1, 1],
    //     const_real!(4),
    // );

    // --------------------------------------- structures --------------------------------------- //

    // ------------------------------------------------------------------------------------------ //
    //                                           TERRAN                                           //
    // ------------------------------------------------------------------------------------------ //

    // ------------------------------------------ units ----------------------------------------- //

    pub const MARINE: Self = Self::new(
        6,
        Multihit::Single,
        duration!(0.8608),
        0,
        None,
        Target::Both,
        [1, 0],
        const_real!(5),
        ProjType::Hitscan,
        duration!(0.75),
        duration!(0.05),
    );

    // pub const MARAUDER: Self = Self::new(
    //     10,
    //     1,
    //     const_real!(1.5 / GAME_SPEED),
    //     10,
    //     Some(Flag::Armored),
    //     Target::Ground,
    //     [1, 1],
    //     const_real!(6),
    // );

    // --------------------------------------- structures --------------------------------------- //

    // ------------------------------------------------------------------------------------------ //
    //                                            ZERG                                            //
    // ------------------------------------------------------------------------------------------ //

    // ------------------------------------------ units ----------------------------------------- //

    // pub const ZERGLING: Self = Self::new(
    //     5,
    //     1,
    //     const_real!(0.696 / GAME_SPEED),
    //     0,
    //     None,
    //     Target::Ground,
    //     [1, 0],
    //     const_real!(0.1),
    // );

    pub const ROACH: Self = Self::new(
        16,
        Multihit::Single,
        duration!(2.0),
        0,
        None,
        Target::Ground,
        [2, 0],
        const_real!(4),
        ProjType::Projectile(rate!(18.75)),
        duration!(0.5),
        duration!(0.167),
    );

    // --------------------------------------- structures --------------------------------------- //
}
