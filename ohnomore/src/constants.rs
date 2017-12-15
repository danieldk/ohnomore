use std::collections::HashSet;

pub static AUXILIARY_MARKER: &str = "%aux";
pub static PASSIVE_MARKER: &str = "%passiv";

pub static SEPARABLE_PARTICLE_POS: &str = "PTKVZ";

pub static AUXILIARY_PREFIX: &str = "VA";
pub static MODAL_PREFIX: &str = "VM";
pub static VERB_PREFIX: &str = "V";

pub static PARTICIPLE_TAG: &str = "VVPP";
pub static ZU_INFINITIVE_VERB: &str = "VVIZU";
pub static INFINITIVE_VERB_TAG: &str = "VVINF";

pub static ADVERBIAL_RELATION: &str = "ADV";
pub static AUXILIARY_RELATION: &str = "AUX";
pub static CONJ_COMPLEMENT_RELATION: &str = "CJ";
pub static COORDINATION_RELATION: &str = "KON";
pub static PUNCTUATION_RELATION: &str = "-PUNCT-";
pub static SEP_VERB_PREFIX_RELATION: &str = "AVZ";

pub static PASSIVE_VERB_LEMMA: &str = "werden";

lazy_static! {
    pub static ref LEMMA_IS_FORM_TAGS: HashSet<&'static str> = hashset! {
        "ADV",
        "APPR",
        "APPO",
        "APZR",
        "FM",
        "ITJ",
        "KOUI",
        "KOUS",
        "KON",
        "KOKOM",
        "ADJD",
        "CARD",
        "PTKZU",
        "PTKA",
        "PTKNEG"
    };
}

pub fn is_verb<S>(tag: S) -> bool
where
    S: AsRef<str>,
{
    tag.as_ref().starts_with("V")
}

pub fn is_finite_verb<S>(tag: S) -> bool
where
    S: AsRef<str>,
{
    tag.as_ref().starts_with("V") && tag.as_ref().ends_with("FIN")
}

pub fn is_separable_verb<S>(tag: S) -> bool
where
    S: AsRef<str>,
{
    let tag = tag.as_ref();
    tag == "VVFIN" || tag == "VVPP" || tag == "VVIMP" || tag == "VMFIN" || tag == "VAFIN"
}
