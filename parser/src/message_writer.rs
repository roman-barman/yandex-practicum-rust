use std::io::Write;

pub trait MessageWriter {
    type Error: std::error::Error;
    fn write_to<T: Write>(&self, writer: &mut T) -> Result<(), Self::Error>;
}
