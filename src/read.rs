use error::{Error, Result};

pub trait Read {
    fn skip_bytes(&mut self, len: usize) -> Result<()>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16<B: byteorder::ByteOrder>(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(B::read_u16(&buf))
    }
}

impl<'a> Read for &'a [u8] {
    fn skip_bytes(&mut self, len: usize) -> Result<()> {
        if len > self.len() {
            return Err(Error::UnexpectedEof);
        }

        *self = &self[len..];
        Ok(())
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        if buf.len() > self.len() {
            return Err(Error::UnexpectedEof);
        }

        let (head, tail) = self.split_at(buf.len());
        buf.copy_from_slice(head);
        *self = tail;
        Ok(())
    }
}
