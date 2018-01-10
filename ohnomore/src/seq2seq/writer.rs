use std::cmp::min;
use std::path::Path;

use hdf5;
use hdf5::IntoData;
use itertools::multizip;

use error::Result;
use lookup::Lookup;
use super::{InputVector, InputVectorizer};

/// Data types collects (and typically stores) vectorized sentences.
pub trait Collector<S, T> {
    fn collect(&mut self, token: S, tag: &String, lemma: T) -> Result<()>
    where
        S: AsRef<str>,
        T: AsRef<str>;
}

/// Collector that stores vectorized sentences in a HDF5 container.
pub struct HDF5Collector {
    writer: HDF5Writer,
    lookup: Box<Lookup<char>>,
    vectorizer: InputVectorizer,
}

impl HDF5Collector {
    pub fn new<P>(
        hdf5_path: P,
        char_lookup: Box<Lookup<char>>,
        vectorizer: InputVectorizer,
        batch_size: usize,
    ) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(HDF5Collector {
            writer: HDF5Writer::new(hdf5::File::new(hdf5_path)?, batch_size),
            lookup: char_lookup,
            vectorizer: vectorizer,
        })
    }
}

impl<S, T> Collector<S, T> for HDF5Collector
where
    S: AsRef<str>,
    T: AsRef<str>,
{
    fn collect(&mut self, token: S, tag: &String, lemma: T) -> Result<()> {
        let input = self.vectorizer.vectorize(token, tag);

        let lemma: Vec<usize> = lemma
            .as_ref()
            .chars()
            .map(|c| self.lookup.lookup(&c).unwrap_or(self.lookup.null()))
            .collect();

        self.writer.write(&lemma, input)?;

        Ok(())
    }
}

pub struct HDF5Writer {
    file: hdf5::File,
    batch: usize,
    batch_size: usize,
    batch_idx: usize,
    tokens: Vec<Vec<i32>>,
    tags: Vec<i32>,
    lemmas: Vec<Vec<i32>>,
}

impl HDF5Writer {
    pub fn new(hdf5_file: hdf5::File, batch_size: usize) -> Self {
        HDF5Writer {
            file: hdf5_file,
            batch: 0,
            batch_size,
            batch_idx: 0,
            tokens: Vec::new(),
            tags: Vec::new(),
            lemmas: Vec::new(),
        }
    }

    pub fn write(&mut self, lemma: &[usize], input: InputVector) -> Result<()> {
        self.tokens
            .push(input.chars().iter().map(|l| *l as i32).collect());
        self.tags.push(input.tag() as i32);
        self.lemmas.push(lemma.iter().map(|l| *l as i32).collect());

        self.batch_idx += 1;

        if self.batch_idx >= self.batch_size {
            self.write_batch()?;
            self.clear_batch();
        }

        Ok(())
    }

    fn clear_batch(&mut self) {
        self.batch_idx = 0;
        self.batch += 1;

        self.tokens.clear();
        self.tags.clear();
        self.lemmas.clear();
    }

    fn write_batch(&mut self) -> Result<()> {
        let token_time_steps: usize = self.tokens
            .iter()
            .map(Vec::len)
            .max()
            .expect("Attempting to write a batch with empty tokens");
        let lemma_time_steps: usize = self.lemmas
            .iter()
            .map(Vec::len)
            .max()
            .expect("Attempting to write a batch with empty tokens");

        let mut tokens_batch = vec![0; self.batch_size * token_time_steps];
        let mut token_lens_batch = vec![0; self.batch_size];
        let mut tags_batch = vec![0; self.batch_size];
        let mut lemmas_batch = vec![0; self.batch_size * lemma_time_steps];
        let mut lemma_lens_batch = vec![0; self.batch_size];

        for (idx, token, tag, lemma) in
            multizip((0..self.batch_size, &self.tokens, &self.tags, &self.lemmas))
        {
            tags_batch[idx] = *tag;

            let token_offset = idx * token_time_steps;
            let token_len = min(token_time_steps, token.len());
            tokens_batch[token_offset..token_offset + token_len]
                .copy_from_slice(&token[..token_len]);
            token_lens_batch[idx] = token_len as i32;

            let lemma_offset = idx * lemma_time_steps;
            let lemma_len = min(lemma_time_steps, lemma.len());
            lemmas_batch[lemma_offset..lemma_offset + lemma_len]
                .copy_from_slice(&lemma[..lemma_len]);
            lemma_lens_batch[idx] = lemma_len as i32;
        }

        self.write_batch_raw("tokens", &tokens_batch)?;
        self.write_batch_raw("token_lens", &token_lens_batch)?;
        self.write_batch_raw("tags", &tags_batch)?;
        self.write_batch_raw("lemmas", &lemmas_batch)?;
        self.write_batch_raw("lemma_lens", &lemma_lens_batch)?;

        Ok(())
    }

    fn write_batch_raw(&self, layer: &str, data: &[i32]) -> Result<()> {
        let mut writer = hdf5::Writer::new(
            &self.file,
            &format!("batch{}-{}", self.batch, layer),
            &[self.batch_size, data.len() / self.batch_size],
        );

        writer.write(
            data.into_data()?,
            &[0, 0],
            &[self.batch_size, data.len() / self.batch_size],
        )?;
        Ok(())
    }
}

impl Drop for HDF5Writer {
    fn drop(&mut self) {
        self.write_batch().expect("Cannot write last batch");
        self.file
            .write("batches", self.batch + 1)
            .expect("Cannot write last batch");
    }
}
