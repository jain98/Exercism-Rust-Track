use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    inner: R,
    num_reads: usize,
    num_bytes: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(inner: R) -> ReadStats<R> {
        Self {
            inner,
            num_reads: 0,
            num_bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.num_reads += 1;
        self.inner.read(buf).and_then(|n| {
            self.num_bytes += n;
            Ok(n)
        })
    }
}

pub struct WriteStats<W> {
    inner: W,
    num_writes: usize,
    num_bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(inner: W) -> WriteStats<W> {
        Self {
            inner,
            num_writes: 0,
            num_bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.num_writes += 1;
        self.inner.write(buf).and_then(|n| {
            self.num_bytes += n;
            Ok(n)
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
