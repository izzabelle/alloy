// namespacing
use crate::metal::{Metal, MetalKind, MetalName, MetalSource, MetalUnits};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

/// TFC metal structs and constants
#[allow(dead_code)]
pub mod metal;

/// source and quantity
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Source {
    pub source: MetalSource,
    pub quantity: MetalUnits,
}

impl Source {
    pub fn metal_name(&self) -> MetalName {
        match self.source {
            MetalSource::Metal { name } => return name,
            _ => {
                for metal in Metal::all_iter() {
                    match metal.kind.clone() {
                        MetalKind::Metal { sources } => {
                            for source in sources.iter() {
                                match source {
                                    Some(metal_source) => {
                                        if metal_source == &self.source {
                                            return metal.name;
                                        }
                                    }
                                    None => continue,
                                }
                            }
                        }
                        MetalKind::WroughtIron { source: _ } => return MetalName::WroughtIron,
                        MetalKind::Alloy { components: _ } => continue,
                    }
                }
            }
        }
        panic!("yeah this should not happen");
    }
}

/// working alloy
#[derive(Debug, Eq, PartialEq)]
pub struct WorkingAlloy {
    pub total_units: u32,
    pub added: HashMap<Source, u8>,
}

impl WorkingAlloy {
    /// initialize a working alloy
    pub fn init() -> Self {
        let total_units: u32 = 0;
        let added: HashMap<Source, u8> = HashMap::new();
        WorkingAlloy { total_units, added }
    }

    /// adds source to the working alloy
    pub fn add(&mut self, source: &Source) {
        if self.added.contains_key(source) {
            self.added
                .entry(*source)
                .and_modify(|quantity| *quantity += 1);
        } else {
            self.added.insert(*source, 1);
        }

        self.total_units += source.quantity as u32;
    }

    /// removes a source from the working alloy, throws error if the source was not there
    pub fn remove(&mut self, source: &Source) -> Result<(), Error> {
        let rm = self.added.remove_entry(source);
        match rm {
            Some(_) => {
                self.total_units -= source.quantity as u32;
                Ok(())
            }
            None => Err(Error::new(
                ErrorKind::NotFound,
                "has not been entered in map",
            )),
        }
    }

    /// returns a vec of metal names and their percentage of the alloy
    pub fn metal_percents(&self) -> Vec<(MetalName, f32)> {
        let mut percents: Vec<(MetalName, f32)> = Vec::new();
        for (source, quantity) in self.added.iter() {
            let name = source.metal_name();
            let percent = (source.quantity as u32 * *quantity as u32) as f32
                / self.total_units as f32
                * 100.0;
            percents.push((name, percent));
        }
        percents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOURCE: Source = Source {
        source: MetalSource::NativeCopper,
        quantity: MetalUnits::OreSmall,
    };

    #[test]
    fn test_source_to_metal() {
        let mut source = TEST_SOURCE.clone();
        assert_eq!(MetalName::Copper, source.metal_name());
        source.source = MetalSource::Metal {
            name: MetalName::Gold,
        };
        assert_eq!(MetalName::Gold, source.metal_name());
        source.source = MetalSource::Bloom;
        assert_eq!(MetalName::WroughtIron, source.metal_name());
    }

}
