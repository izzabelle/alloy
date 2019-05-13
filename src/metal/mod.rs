pub mod consts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum MetalName {
    Bismuth = 0,
    Tin = 1,
    Zinc = 2,
    Copper = 3,
    Bronze = 4,
    BismuthBronze = 5,
    BlackBronze = 6,
    Brass = 7,
    Lead = 8,
    Gold = 9,
    RoseGold = 10,
    Silver = 11,
    SterlingSilver = 12,
    Platinum = 13,
    WroughtIron = 14,
    Nickel = 15,
    PigIron = 16,
    Steel = 17,
    BlackSteel = 18,
    BlueSteel = 19,
    RedSteel = 20,
    UnknownMetal = 21,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetalSource {
    Bismuthinite,
    Cassiterite,
    Sphalerite,
    Malachite,
    NativeCopper,
    Tetrahedrite,
    Galena,
    NativeGold,
    NativeSilver,
    NativePlatinum,
    Bloom,
    Garnierite,
    Hematite,
    Limonite,
    Magnetite,
    PigIron,
    Metal { name: MetalName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MetalUnits {
    OreSmall = 10,
    OrePoor = 15,
    OreNormal = 25,
    OreRich = 35,
    Ingot = 100,
}

#[derive(Debug)]
pub struct Component {
    pub name: MetalName,
    pub range: (f32, f32),
}

#[derive(Debug)]
pub enum MetalKind {
    Alloy {
        components: [Option<Component>; 4],
    },
    Metal {
        source: [Option<MetalSource>; 3],
        units: [MetalUnits; 5],
    },
    WroughtIron {
        source: MetalSource,
        units: u32,
    },
}

#[derive(Debug)]
pub struct Metal {
    pub name: MetalName,
    pub kind: MetalKind,
}

impl Metal {
    pub fn metals_iter() -> impl Iterator<Item = &'static Metal> {
        Metal::METALS.iter()
    }
}
