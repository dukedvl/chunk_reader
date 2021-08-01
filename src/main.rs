use chunk_reader::read_n_bytes;
use chunk_reader::{print_chunks, Chunk, ChunkFile};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("source.wav").unwrap();
    let mut stream = BufReader::new(file);

    stream.fill_buf().unwrap();
    let mut chunk_file = ChunkFile::default();

    let mut file_id = [0; 4];

    stream.read_exact(&mut file_id).unwrap();

    println!(
        "File Type: {:?}",
        String::from_utf8_lossy(&file_id).to_string()
    );

    chunk_file.file_id = String::from_utf8(file_id.to_vec()).unwrap();

    let mut file_size = [0; 4];
    stream.read_exact(&mut file_size).unwrap();

    chunk_file.file_size = u32::from_be_bytes(file_size);
    println!("File Size: {:?}", chunk_file.file_size);

    let mut file_format: [u8; 4] = [0; 4];
    stream.read_exact(&mut file_format).unwrap();

    chunk_file.format = String::from_utf8_lossy(&file_format).to_string();

    println!("Format: {:?}", chunk_file.format);

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

        chunk_file.chunks.push(some_chunk);
    }

    print_chunks(chunk_file.chunks);
}
