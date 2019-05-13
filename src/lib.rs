// namespacing
use crate::metal::{Metal, MetalName};

/// tfc metal structs and constants
#[allow(dead_code)]
pub mod metal;

struct Component {
    name: MetalName,
    quantity: u32,
}

/// working alloy
pub struct WorkingAlloy {
    units: u32,
    added: [Option<Vec<Component>>; 4],
    percents: [Option<f32>; 4],
}

impl WorkingAlloy {
    /// initialize the working alloy
    pub fn init() -> Self {
        let units: u32 = 0;
        let added: [Option<Vec<Component>>; 4] = [None, None, None, None];
        let percents: [Option<f32>; 4] = [None, None, None, None];
        WorkingAlloy {
            units,
            added,
            percents,
        }
    }

    pub fn determine_metal(&self) -> &'static Metal {
        let mut metal: &'static Metal = &Metal::METALS[MetalName::UnknownMetal as usize];
        // if metal is completely empty just return unknown metal
        if self.percents == [None, None, None, None] {
            metal
        } else {
            Metal::metals_iter().enumerate().for_each(|m| {
                // if metal only has one type of metal added return,that metal type
                if let [Some(_), None, None, None] = self.percents {
                    match &self.added[0] {
                        Some(c) => {
                            if m.1.name == c[0].name {
                                metal = &m.1;
                            }
                        }
                        None => {}
                    }
                }
            });
            metal
        }
    }
}
