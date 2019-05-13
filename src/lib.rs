// namespacing
use crate::metal::{MetalSource, MetalUnits};

/// tfc metal structs and constants
#[allow(dead_code)]
pub mod metal;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Source {
    source: MetalSource,
    quantity: MetalUnits,
}

/// working alloy
#[derive(Debug, Eq, PartialEq)]
pub struct WorkingAlloy {
    total_units: u32,
    added: Vec<Source>,
}

impl std::ops::AddAssign<(Source)> for WorkingAlloy {
    fn add_assign(&mut self, source: Source) {
        self.added.push(source);
        self.total_units += source.quantity as u32;
    }
}

impl WorkingAlloy {
    pub fn init() -> Self {
        let total_units: u32 = 0;
        let added: Vec<Source> = Vec::new();
        WorkingAlloy { total_units, added }
    }
}

#[cfg(test)]
use crate::metal::MetalName;

#[test]
fn test_add() {
    let mut working = WorkingAlloy::init();
    working += Source {
        source: MetalSource::Metal {
            name: MetalName::Copper,
        },
        quantity: MetalUnits::Ingot,
    };
    assert_eq!(
        working,
        WorkingAlloy {
            total_units: 100,
            added: vec![Source {
                source: MetalSource::Metal {
                    name: MetalName::Copper
                },
                quantity: MetalUnits::Ingot
            }]
        }
    );
}
