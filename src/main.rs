use std::io;
use std::io::Write;
//use std::cmp::Ordering;


fn main() {
    println!("Aye chum, enter your ability scores here.");
    let ab_score_name: [&str; 6] = ["Str", "Dex", "Con", "Int", "Wis", "Cha"];
    let mut ab_score: [i32; 6] = [0; 6];
    for i in 0..6 {
        print!("Gimme yer {}: ", ab_score_name[i]);
        io::stdout()
            .flush()
            .unwrap();
        
        let mut entry = String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read line");

        ab_score[i] = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
    println!("Here is your abilities with modifiers: ");
    for i in 0..6 {
        let ab_modifier = ((ab_score[i])/2)-5;
        print!("{} {}({}{})", ab_score_name[i].to_uppercase(), ab_score[i],
        if ab_modifier < 0 { "" } else { "+" }, ab_modifier);
        if i < 5 {
            print!(" ");
        }
    }
    println!()
}
