fn main() {
    // let val = 244140625;
    // println!("{:b}", (val ^ -1) / 4096)
    for i in 0..4096 {
        println!(
            "{}",
            format!("{:.12},", i as f64 / 4096.0).get(2..).unwrap()
        );
    }
}
