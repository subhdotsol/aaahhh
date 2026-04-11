use rodio::{buffer::SamplesBuffer, Sink, OutputStream};
fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let samples = vec![0.0f32; 44100];
    let buf = SamplesBuffer::new(1, 44100, samples);
    sink.append(buf);
    match sink.try_seek(std::time::Duration::from_millis(100)) {
        Ok(_) => println!("Seek worked!"),
        Err(e) => println!("Seek failed: {:?}", e),
    }
}
