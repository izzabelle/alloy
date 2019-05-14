use crate::metal::{Component, Metal, MetalKind, MetalName, MetalSource, MetalUnits};

impl MetalUnits {
    const UNIT_TYPES: [MetalUnits; 5] = [
        MetalUnits::OreSmall,
        MetalUnits::OrePoor,
        MetalUnits::OreNormal,
        MetalUnits::OreRich,
        MetalUnits::Ingot,
    ];
}

impl Metal {
    /// const array of all TFC metals
    pub const METALS: [Metal; 22] = [
        Metal {
            name: MetalName::Bismuth,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::Bismuthinite), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::Tin,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::Cassiterite), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::Zinc,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::Sphalerite), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::Copper,
            kind: MetalKind::Metal {
                sources: [
                    Some(MetalSource::Malachite),
                    Some(MetalSource::NativeCopper),
                    Some(MetalSource::Tetrahedrite),
                ],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::Bronze,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::Copper,
                        range: (88.0, 92.0),
                    }),
                    Some(Component {
                        name: MetalName::Tin,
                        range: (8.0, 12.0),
                    }),
                    None,
                    None,
                ],
            },
        },
        Metal {
            name: MetalName::Bronze,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::Copper,
                        range: (50.0, 65.0),
                    }),
                    Some(Component {
                        name: MetalName::Zinc,
                        range: (20.0, 30.0),
                    }),
                    Some(Component {
                        name: MetalName::Bismuth,
                        range: (10.0, 20.0),
                    }),
                    None,
                ],
            },
        },
        Metal {
            name: MetalName::BlackBronze,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::Copper,
                        range: (50.0, 70.0),
                    }),
                    Some(Component {
                        name: MetalName::Silver,
                        range: (10.0, 25.0),
                    }),
                    Some(Component {
                        name: MetalName::Gold,
                        range: (10.0, 25.0),
                    }),
                    None,
                ],
            },
        },
        Metal {
            name: MetalName::Brass,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::Copper,
                        range: (88.0, 92.0),
                    }),
                    Some(Component {
                        name: MetalName::Zinc,
                        range: (8.0, 12.0),
                    }),
                    None,
                    None,
                ],
            },
        },
        Metal {
            name: MetalName::Lead,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::Galena), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::Gold,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::NativeGold), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::RoseGold,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::Gold,
                        range: (70.0, 85.0),
                    }),
                    Some(Component {
                        name: MetalName::Copper,
                        range: (15.0, 30.0),
                    }),
                    None,
                    None,
                ],
            },
        },
        Metal {
            name: MetalName::Silver,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::NativeSilver), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::SterlingSilver,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::Silver,
                        range: (60.0, 80.0),
                    }),
                    Some(Component {
                        name: MetalName::Copper,
                        range: (20.0, 40.0),
                    }),
                    None,
                    None,
                ],
            },
        },
        Metal {
            name: MetalName::Platinum,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::NativePlatinum), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::WroughtIron,
            kind: MetalKind::WroughtIron {
                source: MetalSource::Bloom,
                units: 100,
            },
        },
        Metal {
            name: MetalName::Nickel,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::Garnierite), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::PigIron,
            kind: MetalKind::Metal {
                sources: [
                    Some(MetalSource::Hematite),
                    Some(MetalSource::Limonite),
                    Some(MetalSource::Limonite),
                ],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::Steel,
            kind: MetalKind::Metal {
                sources: [Some(MetalSource::PigIron), None, None],
                units: MetalUnits::UNIT_TYPES,
            },
        },
        Metal {
            name: MetalName::BlackSteel,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::Steel,
                        range: (50.0, 70.0),
                    }),
                    Some(Component {
                        name: MetalName::Nickel,
                        range: (15.0, 25.0),
                    }),
                    Some(Component {
                        name: MetalName::BlackBronze,
                        range: (15.0, 25.0),
                    }),
                    None,
                ],
            },
        },
        Metal {
            name: MetalName::BlueSteel,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::BlackSteel,
                        range: (50.0, 55.0),
                    }),
                    Some(Component {
                        name: MetalName::Steel,
                        range: (20.0, 25.0),
                    }),
                    Some(Component {
                        name: MetalName::BismuthBronze,
                        range: (10.0, 15.0),
                    }),
                    Some(Component {
                        name: MetalName::SterlingSilver,
                        range: (10.0, 15.0),
                    }),
                ],
            },
        },
        Metal {
            name: MetalName::RedSteel,
            kind: MetalKind::Alloy {
                components: [
                    Some(Component {
                        name: MetalName::BlackSteel,
                        range: (50.0, 55.0),
                    }),
                    Some(Component {
                        name: MetalName::Steel,
                        range: (20.0, 25.0),
                    }),
                    Some(Component {
                        name: MetalName::RoseGold,
                        range: (10.0, 15.0),
                    }),
                    Some(Component {
                        name: MetalName::Brass,
                        range: (10.0, 15.0),
                    }),
                ],
            },
        },
        Metal {
            name: MetalName::UnknownMetal,
            kind: MetalKind::Alloy {
                components: [None, None, None, None],
            },
        },
    ];
}
