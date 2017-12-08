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
