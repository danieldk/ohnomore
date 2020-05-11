# Oh No! More Lemmas

ohnomore consists of two tools to incorporate TüBa-D/Z style lemmas
into language processing pipelines. The first tool, `ohnomore-preproc`
takes TüBa-D/Z lemmas and transforms them into lemmas that are more
fit for machine learning pipelines. For example:

* Alternative lemmatizations are removed.
* Separable prefix markers are removed.
* Separable prefixes are removed when they are separated.
* The special reflexive lemma *#refl* is replaced by the lowercased form.
* Lemmas of truncations are replaced by their forms.

The second tool, `ohnomore` performs the opposite transformation (as
much as is feasible).
