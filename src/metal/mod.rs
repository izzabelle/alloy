pub mod consts;

/// list of all metal names in TFC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// source of metal ores/ingots
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// defines the quantity of units of metal from the different sources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MetalUnits {
    OreSmall = 10,
    OrePoor = 15,
    OreNormal = 25,
    OreRich = 35,
    Ingot = 100,
}

/// a single component of an alloy and the range of the percentage that the alloy should be
#[derive(Debug, Clone)]
pub struct Component {
    pub name: MetalName,
    pub range: (f32, f32),
}

/// the type of metal and what it's comprised of
#[derive(Debug, Clone)]
pub enum MetalKind {
    Alloy { components: [Option<Component>; 4] },
    Metal { sources: [Option<MetalSource>; 3] },
    WroughtIron { source: MetalSource },
}

/// a metal
#[derive(Debug)]
pub struct Metal {
    pub name: MetalName,
    pub kind: MetalKind,
}

impl Metal {
    /// returns and Iterator of all metals in TFC
    pub fn all_iter() -> impl Iterator<Item = &'static Metal> {
        Metal::METALS.iter()
    }
}
