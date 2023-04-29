// https://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step
// I think this is way more elegant for negative / counting down ranges

/**
Start, Stop, Step
*/
pub struct SimpleStepRange(pub isize, pub isize, pub isize);

impl Iterator for SimpleStepRange {
    type Item = isize;

    #[inline]
    fn next(&mut self) -> Option<isize> {
        if self.0 < self.1 {
            let v = self.0;
            self.0 = v + self.2;
            Some(v)
        } else {
            None
        }
    }
}
