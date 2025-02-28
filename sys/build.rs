use bindgen::{
    Formatter,
    callbacks::{DeriveInfo, IntKind, ParseCallbacks},
};
use std::{cell::RefCell, env, error::Error, path::PathBuf, str::FromStr};

const DERIVE_COMMON: [&str; 4] = ["XC_KEY", "XC_COUNTRY_ID", "XC_EXCHANGE_ID", "XC_SYMBOL"];

#[derive(Debug, Default)]
struct Callbacks {
    current_file: RefCell<String>,
}

impl ParseCallbacks for Callbacks {
    fn include_file(&self, filename: &str) {
        self.current_file.replace(filename.to_owned());
    }

    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        let mut retval = Vec::default();
        if DERIVE_COMMON.contains(&info.name) {
            retval.push("Default");
            retval.push("Eq");
            retval.push("Hash");
            retval.push("Ord");
            retval.push("PartialEq");
            retval.push("PartialOrd");
        }

        retval.iter().map(ToString::to_string).collect::<Vec<_>>()
    }

    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        if name.ends_with("_XINT8") {
            Some(IntKind::I8)
        } else if name == "XPT_DEFAULT" || name.ends_with("_XUINT8") {
            Some(IntKind::U8)
        } else if name.ends_with("_XINT16") {
            Some(IntKind::I16)
        } else if name.ends_with("_XUINT16") {
            Some(IntKind::U16)
        } else if name.starts_with("XFW_") || name.ends_with("_XINT32") {
            Some(IntKind::I32)
        } else if self.current_file.borrow().as_str() == "xerr.h"
            || name.starts_with("XF_")
            || name.starts_with("XFENUM_")
            || name.starts_with("XFKEY_")
            || name.starts_with("XFNUM_")
            || name.starts_with("XFDT_")
            || name.starts_with("XFPRI_")
            || name.starts_with("XFEXCH_")
            || name.starts_with("XFCC_")
            || name.starts_with("XFCUR_")
            || name.starts_with("XFSYM_")
            || name.starts_with("XFSTR_")
            || name.starts_with("XFSTRUCT_")
            || name.starts_with("XCFMT_")
            || name.starts_with("XPRSOFT_")
            || name.ends_with("_XUINT32")
        {
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

    path.push("gen.inc.rs");
    let path = path;

    // If you want to re-generate rust files, delete the ones in src/gen
    if path.exists() {
        return Ok(());
    }

    bindgen::builder()
        .parse_callbacks(Box::new(Callbacks::default()))
        .header("wrapper.h")
        .clang_arg("-I/usr/local/exegy/include")
        .allowlist_recursively(false)
        .allowlist_file("/usr/local/exegy/include/.*\\.h")
        .no_debug("XC_GROUP_DERIVATIVE_REFERENCE")
        .no_debug("XC_GROUP_DERIVATIVE_REFERENCE_INSTRUMENT")
        .no_debug("XC_GROUP_DERIVATIVE_REFERENCE_PRODUCT")
        .no_debug("XC_GROUP_DERIVATIVE_REFERENCE_LEG_V3_8")
        .no_debug("XC_GROUP_DERIVATIVE_REFERENCE_LEG")
        .no_debug("XC_GROUP_FXFWD_REFRESH_ALL")
        .no_debug("XC_GROUP_FXFWD_QUOTE_ALL")
        .no_debug("XC_GROUP_FXSWAP_REFRESH_ALL")
        .no_debug("XC_GROUP_FXSWAP_QUOTE_ALL")
        .formatter(Formatter::Rustfmt)
        .generate_cstr(true)
        .impl_debug(true)
        .sort_semantically(true)
        .generate()?
        .write_to_file(&path)?;

    Ok(())
}
