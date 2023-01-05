pub fn print_report(count: usize, total: usize) {
    println!();
    println!(
        "Imported {} records (total records in DB: {})",
        count, total
    )
}
