# Design document

## Introduction

We have discovered serious shortcomings in the SepVerb
lemmatizer. Since lemmatization is one of the cornerstones (dare I say
'crucial') in the third phase of the SFB A3 project, we need a better
lemmatizer.

Unfortunately, we cannot use an off-the-shell lemmatizer as-is,
because the lemmatization scheme of TüBa requires various extra
information such as part-of-speech and syntactic annotation.

The goal of this document is to describe TüBa-D/Z specifics and how
they could materialize in a lemmatizer.

## TüBa-D/Z guidelines

These are largely taken from the TüBa-D/Z stylebook.

* pronouns, nouns, determinar: base form nominative/singular
* verbs: base form infinitive
* adjective: base form predicate
* conjunction, punctuation marks: invariant

### Special phenomena

| Phenomenon                                                         | Rule                                                                  | Examples                                                                                                                        |
| ------------------------------------------------------------------ | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Abbreviations/acronyms                                             | Invariant                                                             | *z. B.*, *usw.*, *Dr.*, *TSV*, *FDP*                                                                                            |
| Spelling errors                                                    | Mapped correct spelling                                               | *wolte → wollte*                                                                                                                |
| Multiword term                                                     | lemma for each token                                                  | *New York → New York*                                                                                                           |
| Dialect                                                            | corresponding German word with underscore appended                    | *es jütt → es geben_*, *Dag → Tag_*                                                                                             |
| Contractions                                                       | mapping to a complex lemma with underscore between lemmas             | *Glaubense → glauben_Sie*                                                                                                       |
| Contractions (APPRART)                                             | reduced to preposition                                                | *zur → zu*                                                                                                                      |
| Non-standard use upper/lower-case                                  | mapping to correct lemma according to German orthography              | *seele → Seele*, *KOMMENTAR → Kommentar*                                                                                        |
| Spelling variations                                                | annotated as **distinct** lemmas                                      | *fantastische → fantastisch*, *phantastische → phantastisch*                                                                    |
| Ambiguous plural forms                                             | plurals unmarked for gender, all lemmata are listed separated by '\|' | *Jugendliche → Jugendlicher\|Jungendliche\|Jugendliches*, *die (PDS np\*) → der\|die\|das*, *denen (PRELS dp*) → der\|die\|das* |
| Auxiliaries (*sein*, *haben*, *werden*)                            | The lemma is suffixed with the tag *%aux* when used as an auxiliary   | *ist → sein%aux*                                                                                                                |
| Modals (*müssen*, *sollen*, *können*, *wollen*, *dürfen*, *mögen*) | The lemma is suffixed with the tag *%aux* when used as an auxiliary   | *darf → dürfen%aux*                                                                                                             |
| Auxilies/modals used as main verbs                                 | Infinitive without *%aux suffix*                                      | *ist → sein*                                                                                                                    |
| Passive *werden*                                                   | The lemma is suffixed with the tag *%passiv*                          | *wird (geehrt) → werden%passiv*                                                                                                 |
| Verbs with a separable prefix                                      | *prefix#lemma* regardless of separation of the prefix                 | *stellen ... ein → ein#stellen*, *eingestellt → ein#stellen*                                                                    |
| Reflexives (*PRF*)                                                 | *#refl*                                                               | *sich → #refl*                                                                                                                  |

# Handling of special phenomena

## Articles

Since articles are ambiguous, articles are lemmatized as follows by
the SepVerb lemmatizer:

* Definite article: *d*
* Indefinite article: *e*

For the training set, all article lemmas need to be replaced by these
shortened lemmas.

## Contractions (*APPRART*)

SepVerb does not have special handling for *APPRART* contractions and
lets the lemmatizer do the lemmatization. However, since this is a
closed class (?) it may make more sense to define a mapping for this
category.

## Non-standard use upper/lower-case

SepVerb does not rectify this? It would be interesting to see if
sequence-to-sequence models can automatically lowercase words.

SepVerb does lowercase the first letter of words that start with an
upper case and are not in the category *NN*, *NE*, or *FM*. These
are typically sentence-initial words.

Does it make sense to completely lowercase words which are not in
these categories?

## Ambiguous plural forms

This only affect preparation of training data. SepVerb always uses
the first lemma from the disjunction.

## Auxiliaries/modals/passive *werden*

During the preparation of training data, the *%aux*/*%passiv* tags are
removed. During lemmatization:

* *%passiv* when:
  - The lemma is *werden*.
  - The lemma governs a token with the *AUX* relation.
  - This governed token has the tag *VVPP*.
* Otherwise *%aux* is added, when the lemma governs a token with the
  *AUX* relation.
  
Since the auxiliary and modal verbs are a closed-class, we might want
to use a simple mapping for lemmatization. Also, the rules could be
more fine-grained, checking presence of a *VA* or *VM* tag.

## Separable verb prefixes

During the preparation of training data, the separable prefix
*particle#* is removed.

During lemmatization separable prefixes are added as follows:

- The tag is checked to start with *VV*.
- The token/lemma are lowercased.
- Then:
  - For infinitives (or null?), such as *ausgehen* the corrected lemma
    is looked up (*aus#gehen*). If the lookup does not succeed, then
	it will greedily try to split the verb, such that the result
	is a valid particle/verb combination.
  - For other inflections, the separable verb particle dependency relation
    and separable verb particle tag are used to find the particle for
	a verb and combine them.
  
