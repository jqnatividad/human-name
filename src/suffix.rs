use std::collections::HashSet;

lazy_static! {
    static ref SUFFIXES: HashSet<&'static str> = {
        let s: HashSet<&'static str> = [
            "esq",
            "esquire",
            "attorney-at-law",
            "jr",
            "jnr",
            "sr",
            "snr",
            "2",
            "i",
            "ii",
            "iii",
            "iv",
            "v",
            "ae",
            "afc",
            "afm",
            "arrc",
            "bart",
            "bem",
            "bt",
            "cb",
            "cbe",
            "cfp",
            "cgc",
            "cgm",
            "ch",
            "chfc",
            "cie",
            "clu",
            "cmg",
            "cpa",
            "cpm",
            "csi",
            "csm",
            "cvo",
            "dbe",
            "dcb",
            "dcm",
            "dcmg",
            "dcvo",
            "dds",
            "dfc",
            "dfm",
            "dmd",
            "do",
            "dpm",
            "dsc",
            "dsm",
            "dso",
            "dvm",
            "ed",
            "erd",
            "gbe",
            "gc",
            "gcb",
            "gcie",
            "gcmg",
            "gcsi",
            "gcvo",
            "gm",
            "idsm",
            "iom",
            "iso",
            "kbe",
            "kcb",
            "kcie",
            "kcmg",
            "kcsi",
            "kcvo",
            "kg",
            "kp",
            "kt",
            "lg",
            "lt",
            "lvo",
            "ma",
            "mba",
            "mbe",
            "mc",
            "md",
            "mm",
            "mp",
            "msm",
            "mvo",
            "obe",
            "obi",
            "om",
            "phd",
            "phr",
            "pmp",
            "qam",
            "qc",
            "qfsm",
            "qgm",
            "qpm",
            "rd",
            "rrc",
            "rvm",
            "sgm",
            "td",
            "ud",
            "vc",
            "vd",
            "vrd",
        ].iter().cloned().collect();
        s
    };
}

pub fn is_suffix(part: &str) -> bool {
    let key: String = part.chars().filter_map( |c: char|
        if c == '.' || c.is_whitespace() {
            None
        } else if c.is_uppercase() {
            Some(c.to_lowercase().next().unwrap())
        } else {
            Some(c)
        }
    ).collect();

    SUFFIXES.contains(&*key)
}