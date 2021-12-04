use std::borrow::Borrow;
use std::io;
use std::io::{Read, Write};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    key: &'a [u8],
    curr_index: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: ?Sized + AsRef<[u8]>,
    {
        Xorcism {
            key: std::convert::AsRef::as_ref(key),
            curr_index: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for x in data {
            *x ^= self.get_next_key_char();
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator,
        Data::Item: Borrow<u8>,
    {
        data.into_iter()
            .map(|x| *x.borrow() ^ self.get_next_key_char())
            .collect::<Vec<u8>>()
            .into_iter()
    }

    pub fn reader<'i, Input>(self, input: Input) -> Reader<'a, Input>
    where
        Input: Read +'i
    {
        Reader::new(self, input)
    }

    pub fn writer<'i, Input>(self, input: Input) -> Writer<'a, Input>
        where
            Input: Write +'i
    {
        Writer::new(self, input)
    }

    // Helpers
    fn get_next_key_char(&mut self) -> u8 {
        self.curr_index %= self.key.len();
        let result = self.key[self.curr_index];
        self.curr_index += 1;
        result
    }
}

pub struct Reader<'key, Input> {
    xorcism: Xorcism<'key>,
    buf: Input,
}

impl<'key, Input> Reader<'key, Input>
where Input: Read,
{
    pub fn new(xorcism: Xorcism<'key>, buf: Input) -> Self {
        Reader { xorcism, buf }
    }
}

impl<Input> Read for Reader<'_, Input>
where Input: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = self.buf.read(buf);
        self.xorcism.munge_in_place(buf);
        result
    }
}

pub struct Writer<'key, Input> {
    xorcism: Xorcism<'key>,
    buf: Input,
}

impl<'key, Input> Writer<'key, Input>
    where Input: Write,
{
    pub fn new(xorcism: Xorcism<'key>, buf: Input) -> Self {
        Writer { xorcism, buf }
    }
}

impl<Input> Write for Writer<'_, Input>
    where Input: Write
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut new_buf = buf.to_vec();
        self.xorcism.munge_in_place(&mut new_buf);
        self.buf.write(&new_buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.flush()
    }
}