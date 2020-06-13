use rayon::prelude::*;
use rayon::iter::plumbing::*;
use base_custom::BaseCustom;

pub struct BruteProducer {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub chars: Vec<char>,
    base: BaseCustom<u8>,
    remaining: usize
}

impl BruteProducer {
    pub fn new(from: Vec<u8>, to: Vec<u8>, chars: Vec<char>) -> Self {
        let char_idxs: Vec<_> = (0..chars.len() as u8).collect();

        let base = BaseCustom::<u8>::new(&char_idxs);

        let start = base.decimal(&from);
        let end = base.decimal(&to);

        Self {
            from,
            to,
            chars,
            base,
            remaining: ((end - start) + 1) as usize
        }
    }
}

impl ParallelIterator for BruteProducer {
    type Item = String;

    fn drive_unindexed<C>(self, consumer: C) -> <C as Consumer<Self::Item>>::Result where
        C: UnindexedConsumer<Self::Item> {
        bridge(self, consumer)
    }
}

impl IndexedParallelIterator for BruteProducer {
    fn len(&self) -> usize {
        self.remaining
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> <C as Consumer<Self::Item>>::Result {
        bridge(self, consumer)
    }

    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> <CB as ProducerCallback<Self::Item>>::Output {
        callback.callback(self)
    }
}

impl Producer for BruteProducer {
    type Item = String;
    type IntoIter = BruteIterator;

    fn into_iter(self) -> Self::IntoIter {
        BruteIterator::new(
            self.from,
            self.to,
            self.chars
        )
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let left_end = self.base.gen(self.base.decimal(&self.from) + (index - 1) as u64);
        let right_start = self.base.gen(self.base.decimal(&self.from) + index as u64);

        (BruteProducer::new(self.from, left_end, self.chars.clone()), BruteProducer::new(right_start, self.to, self.chars))
    }
}

pub struct BruteIterator {
    from: Vec<u8>,
    to: Vec<u8>,
    chars: Vec<char>,
    base: BaseCustom<u8>,
    remaining: usize
}

impl BruteIterator {
    fn new(from: Vec<u8>, to: Vec<u8>, chars: Vec<char>) -> Self {
        let char_idxs: Vec<_> = (0..chars.len() as u8).collect();

        let base = BaseCustom::<u8>::new(&char_idxs);

        let start = base.decimal(&from);
        let end = base.decimal(&to);

        Self {
            from,
            to,
            chars,
            base,
            remaining: ((end - start) + 1) as usize
        }
    }

    fn cycle_up(&mut self, index: usize) {
        let cur = &mut self.from[index];
        let max = (self.chars.len() - 1) as u8;

        if *cur == max {
            *cur = 0;

            if index > 0 {
                self.cycle_up(index - 1);
            }
        } else {
            *cur += 1;
        }
    }

    fn cycle_down(&mut self, index: usize) {
        let cur = &mut self.to[index];
        let max = (self.chars.len() - 1) as u8;

        if *cur == 0 {
            *cur = max;
            self.cycle_down(index - 1);
        } else {
            *cur -= 1;
        }
    }
}

impl Iterator for BruteIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let max = (self.chars.len() - 1) as u8;

        if self.remaining > 0 {
            // todo: this is super expensive, i wonder if theres a faster way than creating a new string via loop each time
            let curr = self.from.iter()
                .map(|&idx| self.chars[idx as usize])
                .collect();

            self.remaining -= 1;

            if self.remaining > 0 {
                self.cycle_up(self.from.len() - 1);
            }

            Some(curr)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining;

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for BruteIterator {}

impl DoubleEndedIterator for BruteIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let max = (self.chars.len() - 1) as u8;

        if self.remaining > 0 {
            // todo: this is super expensive, i wonder if theres a faster way than creating a new string via loop each time
            let curr = self.to.iter()
                .map(|&idx| self.chars[idx as usize])
                .collect();

            self.remaining -= 1;

            if self.remaining > 0 {
                self.cycle_down(self.to.len() - 1);
            }

            Some(curr)
        } else {
            None
        }
    }
}