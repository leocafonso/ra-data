use std::collections::BTreeMap;

use anyhow::Context;

#[derive(Debug)]
pub struct Rzones {
    pub rzones: BTreeMap<String, ParsedRzone>,
}

impl Rzones {
    pub fn parse() -> anyhow::Result<(Vec<String>, Self)> {
        let rzones = RzonesParsed::parse()?.0;
        let chips = rzones.keys().cloned().collect();
        Ok((
            chips,
            Self { rzones },
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Memory {
    pub name: String,
    pub kind: String,
    pub address: u64,
    pub size: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Peripheral {
    pub name: String,
    pub address: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedRzone {
    pub family: String,
    pub core: String,
    pub interrupt_count: u32,
    pub memories: Vec<Memory>,
    pub peripherals: Vec<Peripheral>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RzonesParsed(pub BTreeMap<String, ParsedRzone>);

impl RzonesParsed {
    pub fn parse() -> anyhow::Result<Self> {
        let files = glob::glob("sources/devices/zone/*.rzone").unwrap().map(Result::unwrap);

        let for_each_file = |f: std::path::PathBuf| {
            let ff = f.file_name().unwrap().to_string_lossy();
            let ff = ff.strip_suffix(".rzone").unwrap();
            let parsed_header = ParsedRzone::parse(&f).unwrap();
            (ff.to_string(), parsed_header)
        };

        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            Ok(Self(files.par_bridge().map(for_each_file).collect()))
        }
        #[cfg(not(feature = "rayon"))]
        {
            Ok(Self(files.map(for_each_file).collect()))
        }
    }
}

impl ParsedRzone {
    fn parse(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read rzone file {:?}", path.as_ref()))?;

        let doc = roxmltree::Document::parse(&content)
            .with_context(|| format!("Failed to parse XML in {:?}", path.as_ref()))?;

        let device = doc
            .descendants()
            .find(|n| n.has_tag_name("device"))
            .context("Could not find <device> tag")?;

        let family = device
            .attribute("Dname")
            .context("Could not find Dname attribute in <device> tag")?
            .to_string();

        let processor = device
            .descendants()
            .find(|n| n.has_tag_name("processor"))
            .context("Could not find <processor> tag")?;

        let core = processor
            .attribute("Dcore")
            .context("Could not find Dcore attribute in <processor> tag")?
            .to_string();

        let interrupt_count = processor
            .attribute("DnumInterrupts")
            .context("Could not find DnumInterrupts attribute in <processor> tag")?
            .parse()?;

        let mut memories = Vec::new();
        if let Some(memories_node) = doc.descendants().find(|n| n.has_tag_name("memories")) {
            for memory_node in memories_node.children().filter(|n| n.has_tag_name("memory")) {
                let name = memory_node.attribute("name").unwrap_or("").to_string();
                let kind = memory_node.attribute("type").unwrap_or("").to_string();
                let start_str = memory_node.attribute("start").unwrap_or("0");
                let size_str = memory_node.attribute("size").unwrap_or("0");

                let address = if start_str.starts_with("0x") {
                    u64::from_str_radix(&start_str[2..], 16)?
                } else {
                    start_str.parse()?
                };

                let size = if size_str.starts_with("0x") {
                    u64::from_str_radix(&size_str[2..], 16)?
                } else {
                    size_str.parse()?
                };

                memories.push(Memory {
                    name,
                    kind,
                    address,
                    size,
                });
            }
        }

        let mut peripherals = Vec::new();
        if let Some(peripherals_node) = doc.descendants().find(|n| n.has_tag_name("peripherals")) {
            for node in peripherals_node.descendants() {
                if node.has_tag_name("peripheral") || node.has_tag_name("group") {
                    let name = node.attribute("name").unwrap_or("").to_string();
                    let start_str = node.attribute("start").unwrap_or("0");

                    let address = if start_str.starts_with("0x") {
                        u64::from_str_radix(&start_str[2..], 16)?
                    } else {
                        start_str.parse()?
                    };

                    if !name.is_empty() {
                        peripherals.push(Peripheral { name, address });
                    }
                }
            }
        }

        Ok(Self {
            family,
            core,
            interrupt_count,
            memories,
            peripherals,
        })
    }
}