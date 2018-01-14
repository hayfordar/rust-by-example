#[derive(Debug)]
struct Signal<'a, T : 'a + Copy + PartialOrd> {
    samples : &'a mut [T]
}

impl<'a, T : Copy + PartialOrd> Signal<'a, T> {
    fn get_n_samples(&self) -> usize {
        self.samples.len()
    }

    fn mutate_sample_n(&mut self, n : usize, val : T) -> Option<bool> {
        match self.samples.get_mut(n) {
            Some(exist) => {
                *exist = val;
                Some(true)
            },
            None => None
        }
    }
}

fn main() {
    let mut samples : Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 0.0];
    let mut mysignal = Signal { samples : &mut samples };
    println!("{:?}", mysignal);

    mysignal.mutate_sample_n(1, 6.0);
    println!("{:?}", mysignal);

    println!("{:?}", mysignal.get_n_samples());

}
