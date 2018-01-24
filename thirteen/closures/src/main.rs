use std::thread;
use std::time::Duration;

struct Cacher<T, U>
    where T : Fn(&U) -> U,
    U : Copy + PartialOrd
{
    calculation : T,
    value : Option<U>
}

impl<T, U> Cacher<T, U>
    where T : Fn(&U) -> U,
    U : Copy + PartialOrd
{
    fn new(calculation : T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value : None
        }
    }

    fn eval(&mut self, arg1 : &U) -> U {
        match self.value {
            Some(val) => val,
            None => {
                let val = (self.calculation)(&arg1);
                self.value = Some(val);
                val
            }
        }
    }

}


// fn expensive_calculation(m : &f64, c : &f64) -> f64 {
//     println!("Slowly computing results . . .");
//     thread::sleep(Duration::from_secs(2));
//     m * c * c
// }

fn more_expensive_calculation(m : &f64, p : &f64, c : &f64) -> f64 {
    println!("Very slowly computing results . . .");
    thread::sleep(Duration::from_secs(4));
    let mcc : f64 = m * c * c;
    let pc : f64 = p * c;

    (mcc * mcc + pc * pc).sqrt()
}

fn do_some_maths(mass : &f64, momentum : &f64) -> f64 {
    let c : f64 = 300000000.0;

    // This would be a god closure implementation, but we can improve it further by using
    // the cacher class we need to reduce redundancy in consecutive calls
    // let expensive_calculation = |m : &f64, c : &f64| {
    //     println!("Slowly computing results . . .");
    //     thread::sleep(Duration::from_secs(2));
    //     m * c * c
    // }

    let mut cacher = Cacher::new(|m : &f64| {
        println!("Slowly computing results . . .");
        println!("{:?} {:?}", c, m);
        thread::sleep(Duration::from_secs(2));
        m * c * c
    });

    // We can improve this block here by using a closure in place of the expensive_calculation()
    // function, whose body is effectively executed regardless of whether or not we include momentnum.
    match momentum.is_normal() {
        // false => {
       //     println!("Momentum is not normal, calling an expensive calculation:");
        //     expensive_calculation(&mass, &c)
        // },
        true => {
            println!("Momentum is normal, calling a very expensive calculation:");
            more_expensive_calculation(&mass, &momentum, &c)
        },
        false => {
            println!("Momentum is not normal, calling an expensive calculation:");
            cacher.eval(&mass)
        }
    }
}

fn main() {
    let mass : f64 = 100.0;
    let momentum : f64 = 181.8;

    println!("{:?}", do_some_maths(&mass, &momentum));

    let momentum : f64 = 0.0f64;

    println!("{:?}", do_some_maths(&mass, &momentum));
}
