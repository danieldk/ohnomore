pub static AUXILIARY_MARKER: &str = "%aux";
pub static PASSIVE_MARKER: &str = "%passiv";

pub static SEPARABLE_PARTICLE_POS: &str = "PTKVZ";

pub static AUXILIARY_PREFIX: &str = "VA";
pub static MODAL_PREFIX: &str = "VM";
pub static VERB_PREFIX: &str = "V";

pub static PARTICIPLE_TAG: &str = "VVPP";
pub static ZU_INFINITIVE_VERB: &str = "VVIZU";

pub static AUXILIARY_RELATION: &str = "AUX";
pub static SEP_VERB_PREFIX_RELATION: &str = "AVZ";

pub static PASSIVE_VERB_LEMMA: &str = "werden";

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
