fn main() {
    let cf_flag : bool = true;
    let cf_assignment = if cf_flag {
        5
    } else {
        6
    };

    println!("{:?}", cf_assignment);

    let mut x = 0;
    println!("A lot of people ain't gon' be able to take that");
    loop {
        if x < 10 {
            println!("over and");
        } else {
            println!("over again.");
            break;
        };
        x = x + 1;
    }

    let mut x = 10;
    while x > 0 {
        println!("yuh");
        x = x - 1;
    }

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"
        , "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    for month in months.iter() {
        println!("{:?}", month);
    }

    /* use of ranges */
    for n in 0..10 {
        println!("{:?}", n);
    }


}
