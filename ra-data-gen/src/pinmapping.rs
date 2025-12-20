use anyhow::Context;
use glob::glob;
use regex::Regex;
use roxmltree::Document;

#[derive(Debug, Clone)]
pub struct Pin {
    pub position: String,
    pub signals: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub pins: Vec<Pin>,
}

pub struct PinMappings {
    mappings: Vec<(Regex, Package)>,
}

impl PinMappings {
    pub fn parse() -> anyhow::Result<Self> {
        let pattern = "/home/lafonso/study/rust/ra-data/sources/pinmapping/PinCfg*.xml";
        let files: Vec<_> = glob(pattern)
            .context("Failed to read pinmapping glob pattern")?
            .map(Result::unwrap)
            .collect();

        let parse_file = |path: std::path::PathBuf| -> Option<(Regex, Package)> {
            let filename = path.file_name().unwrap().to_str().unwrap();

            // Extract the pattern from filename: PinCfg(PATTERN).xml
            let pattern_str = filename
                .strip_prefix("PinCfg")
                .unwrap()
                .strip_suffix(".xml")
                .unwrap();

            // Convert pattern to regex: replace 'x' with '.'
            let regex_str = format!("^{}$", pattern_str.replace('x', "."));
            let regex = Regex::new(&regex_str).ok()?;

            let content = std::fs::read_to_string(&path).ok()?;
            let doc = Document::parse(&content).ok()?;

            // Find the package
            let device_node = doc.descendants().find(|n| n.has_tag_name("device"))?;
            let package_node = device_node.children().find(|n| n.has_tag_name("package"))?;
            let package_name = package_node.attribute("name").unwrap_or("Unknown").to_string();

            let mut pins = Vec::new();
            if let Some(layout_node) = package_node.children().find(|n| n.has_tag_name("pinLayout")) {
                for pin_node in layout_node.children().filter(|n| n.has_tag_name("pin")) {
                    let position = pin_node.attribute("name").unwrap_or("").to_string();
                    let signal = pin_node.attribute("ref").unwrap_or("").to_string();

                    if !position.is_empty() && !signal.is_empty() {
                        pins.push(Pin {
                            position,
                            signals: vec![signal],
                        });
                    }
                }
            }

            Some((
                regex,
                Package {
                    name: package_name,
                    pins,
                },
            ))
        };

        #[cfg(feature = "rayon")]
        let mappings = {
            use rayon::prelude::*;
            files.into_par_iter().filter_map(parse_file).collect()
        };

        #[cfg(not(feature = "rayon"))]
        let mappings = files.into_iter().filter_map(parse_file).collect();

        Ok(Self { mappings })
    }

    pub fn get_for_chip(&self, pn: &str) -> Option<&Package> {
        for (regex, package) in &self.mappings {
            if regex.is_match(pn) {
                return Some(package);
            }
        }
        None
    }
}
