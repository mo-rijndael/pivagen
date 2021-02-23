use std::io::{self, Read, Write, Error, ErrorKind};
use std::mem::size_of;

pub trait FromReader {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self>
        where Self: Sized;
}
pub trait ToWriter {
    fn to_writer<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}

impl FromReader for usize {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buffer = [0u8; size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(usize::from_le_bytes(buffer))
    }
}
impl FromReader for String {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let len = usize::from_reader(reader)?;
        let mut buffer = vec![0u8; len];
        reader.read_exact(&mut buffer)?;
        let string = String::from_utf8(buffer)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(string)
    }
}
impl FromReader for bool {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buffer = [0u8; size_of::<bool>()];
        reader.read_exact(&mut buffer)?;
        let [res] = buffer;
        Ok(res != 0)
    }
}
impl ToWriter for usize {
    fn to_writer<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let buffer = self.to_le_bytes();
        writer.write_all(&buffer)?;
        Ok(())
    }
}
impl ToWriter for String {
    fn to_writer<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.len().to_writer(writer)?;
        writer.write_all(self.as_bytes())
    }
}
impl ToWriter for bool {
    fn to_writer<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[*self as u8])?;
        Ok(())
    }
}
