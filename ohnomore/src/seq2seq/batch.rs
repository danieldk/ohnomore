use itertools::zip;
use tensorflow::Tensor;

use lookup::Lookup;

pub struct Batch<W, T> {
    tokens: W,
    tags: T,
}

impl<W, T> Batch<W, T> {
    pub fn tokens(&self) -> &W {
        &self.tokens
    }

    pub fn tags(&self) -> &T {
        &self.tags
    }
}

impl Batch<Vec<String>, Vec<String>> {
    pub fn vectorize(
        &self,
        char_lookup: &Lookup<char>,
        tag_lookup: &Lookup<String>,
    ) -> Option<Batch<Tensor<i32>, Tensor<i32>>> {
        assert_eq!(self.tokens.len(), self.tags.len());

        let batch_size = self.tokens.len();
        let max_seq_len = self.tokens.iter().map(|t| t.len()).max()?;

        let mut token_tensor = Tensor::new(&[batch_size as u64, max_seq_len as u64]);
        let mut tag_tensor = Tensor::new(&[batch_size as u64]);

        for (idx, (token, tag)) in zip(&self.tokens, &self.tags).enumerate() {
            tag_tensor[idx] = tag_lookup.lookup(tag).unwrap_or(tag_lookup.null()) as i32;

            let token_offset = idx * max_seq_len;
            for (char_idx, char) in token.chars().enumerate() {
                token_tensor[token_offset + char_idx] =
                    char_lookup.lookup(&char).unwrap_or(char_lookup.null()) as i32;
            }
        }

        Some(Batch {
            tokens: token_tensor,
            tags: tag_tensor,
        })
    }
}
