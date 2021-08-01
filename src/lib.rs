use std::io::prelude::*;

#[derive(Default)]
pub struct ChunkFile {
    pub file_id: String,
    pub file_size: u32,
    pub format: String,
    pub chunks: Vec<Chunk>,
}

#[derive(Default)]
pub struct Chunk {
    pub chunk_id: String,
    pub chunk_size: u32,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn get_size(&self) -> usize {
        (8 + self.chunk_size) as usize
    }

    pub fn initialize(&mut self) {
        self.data = Vec::with_capacity(self.chunk_size as usize);
    }
}

pub fn read_n_bytes<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
where
    R: BufRead,
{
    let mut buf = vec![];
    let mut raw_data = reader.take(bytes_to_read);

    raw_data.read_to_end(&mut buf).unwrap();

    buf
}

pub fn print_chunks(chunks: Vec<Chunk>) {
    //print chunks
    for c in chunks.iter() {
        println!("ID: {:?}", c.chunk_id);
        println!("Size:{}", c.chunk_size);
        println!("Data Len({:?})", c.data.len());
    }
}
