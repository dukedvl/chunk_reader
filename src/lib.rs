use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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

#[derive(Default)]
pub struct ChunkFile {
    pub file_id: String,
    pub file_size: u32,
    pub format: String,
    pub chunks: Vec<Chunk>,
}

impl ChunkFile {
    pub fn parse(&mut self, path: &str) {
        let file = File::open(path).unwrap();
        let mut stream = BufReader::new(file);

        stream.fill_buf().unwrap();

        let mut file_id = [0; 4];

        stream.read_exact(&mut file_id).unwrap();

        println!(
            "File Type: {:?}",
            String::from_utf8_lossy(&file_id).to_string()
        );

        self.file_id = String::from_utf8(file_id.to_vec()).unwrap();

        let mut file_size = [0; 4];
        stream.read_exact(&mut file_size).unwrap();

        self.file_size = u32::from_be_bytes(file_size);
        println!("File Size: {:?}", self.file_size);

        let mut file_format: [u8; 4] = [0; 4];
        stream.read_exact(&mut file_format).unwrap();

        self.format = String::from_utf8_lossy(&file_format).to_string();

        println!("Format: {:?}", self.format);

        loop {
            if stream.buffer().is_empty() {
                break;
            }

            let mut some_chunk = Chunk::default();

            some_chunk.chunk_id = String::from_utf8(read_n_bytes(&mut stream, 4)).unwrap();

            let mut size_val: [u8; 4] = [0; 4];

            stream.read_exact(&mut size_val).unwrap();

            some_chunk.chunk_size = u32::from_le_bytes(size_val);

            some_chunk.initialize();

            some_chunk.data = read_n_bytes(&mut stream, some_chunk.chunk_size.into());

            self.chunks.push(some_chunk);
        }
    }

    pub fn print_chunks(&self) {
        //print chunks
        for c in self.chunks.iter() {
            println!("ID: {:?}", c.chunk_id);
            println!("Size:{}", c.chunk_size);
            println!("Data Len({:?})", c.data.len());
        }
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
