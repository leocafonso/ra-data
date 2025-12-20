use std::env;

enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple),
                None => Ok(res),
            },
        }
    }
}

fn main() {
    let chip_name = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_R7") || x.starts_with("CARGO_FEATURE_R8"))
        .get_one()
    {
        Ok(x) => Some(x),
        Err(GetOneError::None) => None,
        Err(GetOneError::Multiple) => panic!("Multiple RA chip Cargo features enabled"),
    };

    if let Some(chip_name) = chip_name {
        let chip_name = chip_name
            .strip_prefix("CARGO_FEATURE_")
            .unwrap()
            .to_ascii_lowercase()
            .replace('_', "-");

        println!("cargo:rustc-env=RA_METAPAC_PAC_PATH=chips/{}/pac.rs", chip_name);
        println!("cargo:rustc-env=RA_METAPAC_METADATA_PATH=chips/{}/metadata.rs", chip_name);

        let out_dir = env::var("OUT_DIR").unwrap();
        let out_path = std::path::Path::new(&out_dir);

        let device_x_path = std::path::Path::new("src/chips").join(&chip_name).join("device.x");
        if device_x_path.exists() {
            std::fs::copy(&device_x_path, out_path.join("device.x")).unwrap();
            println!("cargo:rustc-link-search={}", out_dir);
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
