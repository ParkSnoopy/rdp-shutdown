use chrono::{ Local, NaiveTime };
use std::process::{ Command };

const HRS: u32 = 12+9;
const MIN: u32 = 0;
const SEC: u32 = 0;


fn main() {
    let now = Local::now();
    let shutdown = now.clone().with_time(NaiveTime::from_hms_opt(HRS, MIN, SEC).unwrap()).unwrap();

    println!();
    println!("NOW: {}", now);
    println!("END: {}", shutdown);

    let time: i64 = shutdown.signed_duration_since(now).num_seconds();

    println!();
    println!("ETA: {} seconds", time);

    Command::new("shutdown")
        .arg("/s")
        .arg("/t")
        .arg(format!("{}", time).as_str())
        .arg("/c")
        .arg(format!("RDP Shutdown at `{}:{}:{}`", HRS, MIN, SEC))
        .spawn()
        .unwrap();
}
