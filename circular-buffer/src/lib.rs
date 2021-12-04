pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    head: usize,
    tail: isize,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T:Clone + Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![T::default(); capacity],
            head: 0,
            tail: -1,
            size: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        match self.size == self.buffer.capacity() {
            true => Err(Error::FullBuffer),
            false => {
                self.tail = (self.tail + 1).rem_euclid(self.buffer.capacity() as isize);
                self.buffer[self.tail as usize] = element;
                self.size += 1;
                Ok(())
            },
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.size == 0 {
            true => Err(Error::EmptyBuffer),
            false => {
                let result = self.buffer[self.head].clone();
                self.head = (self.head + 1).rem_euclid(self.buffer.capacity());
                self.size -= 1;
                Ok(result)
            },
        }
    }

    pub fn clear(&mut self) {
        while self.size != 0 {
            let _ = std::mem::replace(&mut self.buffer[self.head], T::default());
            self.head = (self.head + 1).rem_euclid(self.buffer.capacity());
            self.size -= 1;
        }
    }

    pub fn overwrite(&mut self, element: T) {
        match self.size == self.buffer.capacity() {
            true => {
                self.buffer[self.head] = element;
                self.head = (self.head + 1).rem_euclid(self.buffer.capacity());
            },
            false => {
                self.tail = (self.tail + 1).rem_euclid(self.buffer.capacity() as isize);
                self.buffer[self.tail as usize] = element;
                self.size += 1;
            }
        }
    }
}
