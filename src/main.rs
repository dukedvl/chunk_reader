use chunk_reader::ChunkFile;

fn main() {
    let mut chunk_file = ChunkFile::default();

    chunk_file.parse("source.wav");

    chunk_file.print_chunks();
}
