use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tsify::Tsify;

const POW_2_16: usize = 2_usize.pow(16);

#[derive(Serialize, Deserialize, Tsify)]
pub struct Memory {
    #[serde(
        serialize_with = "serialize_memory",
        deserialize_with = "deserialize_memory"
    )]
    inner: [u16; POW_2_16],
}

#[allow(dead_code)]
impl Memory {
    pub fn new() -> Memory {
        Memory {
            inner: [0; POW_2_16],
        }
    }

    pub fn load_file(&mut self, file: Vec<u16>) {
        let mut mem_i = file[0] as usize; // origin
        let mut vec_i = 1;

        while vec_i < file.len() {
            self.inner[mem_i] = file[vec_i];

            vec_i += 1;
            mem_i += 1;
        }
    }

    pub fn get(&self, loc: u16) -> u16 {
        return self.inner[loc as usize];
    }

    pub fn set(&mut self, loc: u16, val: u16) {
        self.inner[loc as usize] = val;
    }
}

fn serialize_memory<S>(data: &[u16; POW_2_16], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_seq(data.iter())
}

fn deserialize_memory<'de, D>(deserializer: D) -> Result<[u16; POW_2_16], D::Error>
where
    D: Deserializer<'de>,
{
    let vec = Vec::<u16>::deserialize(deserializer)?;

    if vec.len() != POW_2_16 {
        return Err(serde::de::Error::invalid_length(
            vec.len(),
            &"expected 65536 elements",
        ));
    }

    let mut array = [0u16; POW_2_16];
    array.copy_from_slice(&vec);
    Ok(array)
}
