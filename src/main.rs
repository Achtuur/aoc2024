use days::{Day, DayNumber};

mod days;

fn main() {
    let mut day = Day::new(DayNumber::Day4);
    day.read_input();

    let timer = std::time::Instant::now();
    let res_a = day.exec_a();
    let t_a = timer.elapsed();

    let timer = std::time::Instant::now();
    let res_b = day.exec_b();
    let t_b = timer.elapsed();

    println!("resA: {:?} ({:?})", res_a, t_a);
    println!("resB: {:?} ({:?})", res_b, t_b);
}
