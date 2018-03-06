use std::collections::HashSet;

pub static AUXILIARY_MARKER: &str = "%aux";
pub static PASSIVE_MARKER: &str = "%passiv";
pub static REFLEXIVE_PERSONAL_PRONOUN_LEMMA: &str = "#refl";

pub static SEPARABLE_PARTICLE_POS: &str = "PTKVZ";

pub static AUXILIARY_PREFIX: &str = "VA";
pub static MODAL_PREFIX: &str = "VM";
pub static PUNCTUATION_PREFIX: &str = "$";
pub static VERB_PREFIX: &str = "V";

pub static ARTICLE_TAG: &str = "ART";
pub static ATTRIBUTIVE_POSSESIVE_PRONOUN_TAG: &str = "PPOSAT";
pub static SUBST_POSSESIVE_PRONOUN_TAG: &str = "PPOSS";
pub static FOREIGN_WORD_TAG: &str = "FM";
pub static NAMED_ENTITY_TAG: &str = "NE";
pub static NON_WORD_TAG: &str = "XY";
pub static NOUN_TAG: &str = "NN";
pub static PARTICIPLE_TAG: &str = "VVPP";
pub static PERSONAL_PRONOUN_TAG: &str = "PPER";
pub static REFLEXIVE_PERSONAL_PRONOUN_TAG: &str = "PRF";
pub static SUBST_REL_PRONOUN: &str = "PRELS";
pub static ATTR_REL_PRONOUN: &str = "PRELAT";
pub static TRUNCATED_TAG: &str = "TRUNC";
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
    pub static ref NO_LEMMA_TAGS: HashSet<&'static str> = hashset! {
        "PTKVZ"
    };

    pub static ref LEMMA_IS_FORM_TAGS: HashSet<&'static str> = hashset! {
        "$,",
        "$.",
        "$(",
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
        "PTKNEG",
    };

    /// Part-of-speech tags that have special (non-word) lemmas.
    pub static ref SPECIAL_LEMMA_TAGS: HashSet<&'static str> = hashset! {
        "ART",
        "PDS",
        "PPER",
        "PPOSAT",
        "PRF",
        "PRELS",
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
