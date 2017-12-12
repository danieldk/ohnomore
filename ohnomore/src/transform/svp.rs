use std::cmp::Ordering;
use std::collections::VecDeque;

use fst::{IntoStreamer, Set, Streamer};

use constants::*;

use automaton::PrefixAutomaton;

/// Candidate list of prefixes and the corresponding stripped form.
struct PrefixesCandidate<'a> {
    stripped_form: &'a str,
    prefixes: Vec<String>,
}

/// Look for all matches of (prefix)* in the given form. Ideally,
/// we'd construct a Kleene star automaton of the prefix automaton.
/// Unfortunately, this functionality is not (yet) provided by the
/// fst crate. Instead, we repeatedly search prefixes in the set.
fn prefix_star<'a>(prefix_set: &Set, s: &'a str) -> Vec<PrefixesCandidate<'a>> {
    let mut result = Vec::new();

    let mut q = VecDeque::new();
    q.push_back(PrefixesCandidate {
        stripped_form: s,
        prefixes: Vec::new(),
    });

    while let Some(PrefixesCandidate {
        stripped_form,
        prefixes,
    }) = q.pop_front()
    {
        result.push(PrefixesCandidate {
            stripped_form,
            prefixes: prefixes.clone(),
        });

        for prefix in find_prefixes(prefix_set, stripped_form) {
            let mut prefixes = prefixes.clone();
            let prefix_len = prefix.len();
            prefixes.push(prefix);
            q.push_back(PrefixesCandidate {
                stripped_form: &stripped_form[prefix_len..],
                prefixes,
            });
        }
    }

    result
}

fn find_prefixes<S>(prefix_set: &Set, s: S) -> Vec<String>
where
    S: AsRef<str>,
{
    let automaton = PrefixAutomaton::from(s.as_ref());

    let mut prefixes = Vec::new();

    let mut stream = prefix_set.search(&automaton).into_stream();
    while let Some(prefix) = stream.next() {
        prefixes.push(prefix.to_owned());
    }

    prefixes
        .into_iter()
        .map(|p| {
            String::from_utf8(p)
                .expect("Cannot decode prefix, PrefixAutomaton returned invalid prefix")
        })
        .collect()
}

pub fn longest_prefixes<F, L, T>(prefix_set: &Set, form: F, lemma: L, tag: T) -> Vec<String>
where
    F: AsRef<str>,
    L: AsRef<str>,
    T: AsRef<str>,
{
    let lemma = lemma.as_ref();
    let form = form.as_ref();
    let tag = tag.as_ref();

    let all_prefixes = prefix_star(prefix_set, form);

    let prefixes_candidates: Vec<_> = all_prefixes
        .into_iter()
        .filter(|candidate| {
            let prefixes = &candidate.prefixes;

            if prefixes.is_empty() {
                return true;
            }

            let last_prefix = prefixes.last().unwrap();

            // Avoid e.g. 'dazu' as a valid prefix for a zu-infinitive.
            if tag == ZU_INFINITIVE_VERB && last_prefix.ends_with("zu")
                && !candidate.stripped_form.starts_with("zu")
            {
                return false;
            }

            // 1. Do not start stripping parts of the lemma
            // 2. Prefix should not end with lemma. E.g.:
            //    abgefangen fangen -> ab#fangen, not: ab#gefangen#fangen
            !prefixes.iter().any(|p| lemma.starts_with(p)) && !last_prefix.ends_with(&lemma)
                && is_verb(candidate.stripped_form)
        })
        .collect();

    prefixes_candidates
        .into_iter()
        .max_by(|l, r| {
            match l.stripped_form.len().cmp(&r.stripped_form.len()) {
                Ordering::Less => return Ordering::Greater,
                Ordering::Greater => return Ordering::Less,
                Ordering::Equal => (),
            }

            l.prefixes.len().cmp(&r.prefixes.len()).reverse()
        })
        .map(|t| t.prefixes)
        .unwrap_or(Vec::new())
}

fn is_verb<S>(verb: S) -> bool
where
    S: AsRef<str>,
{
    // A separable verb with a length shorter than 3 is unlikely.
    verb.as_ref().len() > 2
}
