use crate::constants::RIFF_ID;

use crate::FourCC;

/// TODO:
/// 1. Add lots and lots of error checking. Since `Vec` use `usize` internally, we have to limit it to `u32`.
///    In similar vein, we also need to check for overflow and stuff, especially during subtraction.
/// 2. Should we separate Type and NoType? We currently represent these types at the most raw level (literally as `Vec<u8>`) which is quite hard to deal with.
#[derive(Debug)]
pub struct ChunkBuilder {
    chunk_id: FourCC,
    payload_len: u32,
    chunk_type: Option<FourCC>,
    data: ChunkData,
}

impl ChunkBuilder {
    pub fn new_notype(id: FourCC, data: ChunkData) -> Self {
        ChunkBuilder {
            chunk_id: id,
            payload_len: ChunkBuilder::calculate_payload(&data),
            chunk_type: None,
            data: ChunkBuilder::fit_data(data),
        }
    }

    pub fn new_type(id: FourCC, chunk_type: FourCC, data: ChunkData) -> Self {
        ChunkBuilder {
            chunk_id: id,
            payload_len: ChunkBuilder::calculate_payload(&data),
            chunk_type: Some(chunk_type),
            data: ChunkBuilder::fit_data(data),
        }
    }

    fn calculate_payload(data: &ChunkData) -> u32 {
        match &data {
            ChunkData::ChunkData(data) => data
                .iter()
                .map(|x| {
                    if x.chunk_type.is_none() {
                        x.payload_len + 8
                    } else {
                        x.payload_len + 4 + 8
                    }
                })
                .sum(),
            ChunkData::RawData(data) => data.len() as u32,
        }
    }

    fn fit_data(data: ChunkData) -> ChunkData {
        match data {
            ChunkData::RawData(mut vec) => {
                if vec.len() % 2 == 1 {
                    vec.push(0);
                }
                ChunkData::RawData(vec)
            }
            chunks => chunks,
        }
    }

    fn to_bytes<'a>(&self, mut result: &'a mut Vec<u8>) -> &'a Vec<u8> {
        result.extend_from_slice(self.chunk_id.as_bytes());
        result.extend_from_slice(&self.payload_len.to_le_bytes());
        if self.chunk_type.is_some() {
            result.extend_from_slice(self.chunk_type.as_ref().unwrap().as_bytes());
        }
        match &self.data {
            ChunkData::RawData(raw) => result.extend_from_slice(&raw),
            ChunkData::ChunkData(chunks) => {
                for x in chunks {
                    x.to_bytes(&mut result);
                }
            }
        }
        result
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RiffBuilder {
    payload_len: u32,
    chunk_type: FourCC,
    data: Vec<ChunkBuilder>,
}

impl RiffBuilder {
    pub fn new(chunk_type: FourCC) -> Self {
        RiffBuilder {
            payload_len: 4,
            chunk_type,
            data: Vec::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(RIFF_ID.as_bytes());
        result.extend_from_slice(&self.payload_len.to_le_bytes());
        result.extend_from_slice(&self.chunk_type.data);
        for x in &self.data {
            x.to_bytes(&mut result);
        }
        result
    }

    pub fn add_chunk(mut self, chunk: ChunkBuilder) -> Self {
        self.payload_len += chunk.payload_len + 8;
        if chunk.chunk_type.is_some() {
            self.payload_len += 4;
        }
        if let ChunkData::RawData(ref vec) = chunk.data {
            // TODO: I am quite confident this can be abused somehow...
            if vec.len() as u32 - chunk.payload_len == 1 {
                self.payload_len += 1;
            }
        }
        self.data.push(chunk);
        self
    }
}

#[derive(Debug)]
pub enum ChunkData {
    RawData(Vec<u8>),
    ChunkData(Vec<ChunkBuilder>),
}
