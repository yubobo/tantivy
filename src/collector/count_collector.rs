use std::io;
use super::Collector;
use ScoredDoc;
use SegmentReader;
use SegmentLocalId;

pub struct CountCollector {
    count: usize,
}

impl CountCollector {
    pub fn new() -> CountCollector {
        CountCollector {
            count: 0,
        }
    }

    pub fn count(&self,) -> usize {
        self.count
    }
}

impl Collector for CountCollector {

    fn set_segment(&mut self, _: SegmentLocalId, _: &SegmentReader) -> io::Result<()> {
        Ok(())
    }

    fn collect(&mut self, _: ScoredDoc) {
        self.count += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;
    use ScoredDoc;
    use collector::Collector;

    #[bench]
    fn build_collector(b: &mut Bencher) {
        b.iter(|| {
            let mut count_collector = CountCollector::new();
            let docs: Vec<u32> = (0..1_000_000).collect();
            for doc in docs {
                count_collector.collect(ScoredDoc(1f32, doc));
            }
            count_collector.count()
        });
    }
}