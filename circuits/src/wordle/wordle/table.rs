use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, Value},
    plonk::{ConstraintSystem, Error, TableColumn},
};

use serde::{Deserialize, Serialize};

use std::error::Error as StdError;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::utils::*;

#[derive(Serialize, Deserialize)]
struct Dict {
    words: Vec<String>,
}

fn read_words_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn StdError>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: Dict = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u.words)
}


/// A lookup table of values from dictionary.
#[derive(Debug, Clone)]
pub(super) struct DictTableConfig<F: FieldExt> {
    pub(super) value: TableColumn,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> DictTableConfig<F> {
    pub(super) fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let value = meta.lookup_table_column();

        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub(super) fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        let mut words = read_words_from_file("src/wordle/wordle/dict.json").unwrap().into_iter().map(|word| {
            word_to_polyhash(&word)
        }).collect::<Vec<_>>();
        words.push(0);

        layouter.assign_table(
            || "load dictionary-check table",
            |mut table| {
                let mut offset = 0;
                for word in words.iter() {
                    table.assign_cell(
                        || "num_bits",
                        self.value,
                        offset,
                        || Value::known(F::from(word.clone())),
                    )?;
                    offset += 1;
                }

                Ok(())
            },
        )
    }
}