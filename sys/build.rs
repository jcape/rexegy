use bindgen::{
    Formatter,
    callbacks::{DeriveInfo, IntKind, ParseCallbacks},
};
use std::{env, error::Error, path::PathBuf, str::FromStr};

#[derive(Debug)]
struct Callbacks;

const DERIVE_COMMON: [&str; 4] = ["XC_KEY", "XC_COUNTRY_ID", "XC_EXCHANGE_ID", "XC_SYMBOL"];

impl ParseCallbacks for Callbacks {
    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        let mut retval = Vec::default();
        if DERIVE_COMMON.contains(&info.name) {
            retval.push("Eq");
            retval.push("Hash");
            retval.push("PartialEq");
            retval.push("PartialOrd");
            retval.push("Ord");
        }

        retval.iter().map(ToString::to_string).collect::<Vec<_>>()
    }

    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        if name.ends_with("_XINT8") {
            Some(IntKind::I8)
        } else if name.ends_with("_XUINT8") {
            Some(IntKind::U8)
        } else if name.ends_with("_XINT16") {
            Some(IntKind::I16)
        } else if name.ends_with("_XUINT16") {
            Some(IntKind::U16)
        } else if name.ends_with("_XINT32") {
            Some(IntKind::I32)
        } else if name.ends_with("_XUINT32") {
            Some(IntKind::U32)
        } else if name.ends_with("_XINT64") {
            Some(IntKind::I64)
        } else if name.ends_with("_XUINT64") {
            Some(IntKind::U64)
        } else if name.starts_with("XOBJ_") {
            Some(IntKind::U16)
        } else if name.ends_with("_MAXLEN") {
            Some(IntKind::Custom {
                name: "usize",
                is_signed: false,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct XerrCallbacks;

impl ParseCallbacks for XerrCallbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        if name.starts_with("X") {
            Some(IntKind::U32)
        } else {
            None
        }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    match env::var("CARGO_CFG_TARGET_ARCH")
        .expect("Missing build architecture")
        .as_str()
    {
        "i386" => cargo_emit::rustc_link_lib!("xcapi32"),
        "x86_64" => cargo_emit::rustc_link_lib!("xcapi64"),
        _ => panic!("Invalid build architecture"),
    }

    cargo_emit::rustc_link_search!("/usr/local/exegy/lib");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Could not retrieve manifest dir.");
    let mut path = PathBuf::from_str(&manifest_dir)?;
    path.push("src");

    let mut xerr_path = path.clone();
    xerr_path.push("xerr.inc.rs");
    let xerr_path = xerr_path;

    path.push("gen.inc.rs");
    let path = path;

    // If you want to re-generate rust files, delete the ones in src/gen
    if xerr_path.exists() && path.exists() {
        return Ok(());
    }

    bindgen::builder()
        .parse_callbacks(Box::new(XerrCallbacks))
        .header("errors.h")
        .clang_arg("-I/usr/local/exegy/include")
        .allowlist_recursively(false)
        .allowlist_file(".*/xerr.h")
        .formatter(Formatter::Rustfmt)
        .generate_cstr(true)
        .impl_debug(true)
        .sort_semantically(true)
        .generate()?
        .write_to_file(&xerr_path)?;

    bindgen::builder()
        .parse_callbacks(Box::new(Callbacks))
        .header("wrapper.h")
        .clang_arg("-I/usr/local/exegy/include")
        .allowlist_recursively(false)
        .blocklist_file(".*/xerr.h")
        .formatter(Formatter::Rustfmt)
        .generate_cstr(true)
        .sort_semantically(true)
        .generate()?
        .write_to_file(&path)?;

    Ok(())
}
