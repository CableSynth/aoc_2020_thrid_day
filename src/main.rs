use std::fs;

fn main() {

    let filename = "/home/caleb/codestuff/advent2020/third_day/src/input.txt";
    let data =  fs::read_to_string(filename).expect("error");
    let data_split_on_newline: Vec<&str> = data.split("\n").collect();
    let mut col_num = 0;
    let mut tree_count_d1_r1 = 0_i64;
    let mut tree_count_r3_d1 = 0_i64;
    let mut tree_count_r5_d1 = 0_i64;
    let mut tree_count_r7_d1 = 0_i64;
    let mut tree_count_r1_d2 = 0_i64;
    for (i, &line) in data_split_on_newline.iter().enumerate() {
        let mut current_line = String::from(line);
        //Check and extend string if needed
        if (col_num + (6*i)) >= line.len() {
            for _j in 0..(((col_num + (6*i))/line.len())){
                current_line.push_str(line);
            }
        }
        let mut current_line_chars = current_line.chars();
        let current_line_char = current_line_chars.nth(col_num).expect("out of bounds");
        if current_line_char == '#' {
            tree_count_d1_r1 += 1; 
            
        }
        if i%2 == 0 && i != 0{
            let mut current_line_chars = current_line.chars();
            let current_line_char = current_line_chars.nth(col_num/2).expect("out of bounds");
            if current_line_char == '#' {
                tree_count_r1_d2 += 1; 
            }
        }
        let mut current_line_chars = current_line.chars();
        let current_line_char1 = current_line_chars.nth(col_num + (2*i)).expect("out of bounds");
        if current_line_char1 == '#' {
            tree_count_r3_d1 += 1;
        }

        let mut current_line_chars = current_line.chars();
        let current_line_char2 = current_line_chars.nth(col_num + (4*i)).expect("out of bounds");
        if current_line_char2 == '#' {
            tree_count_r5_d1 += 1;
        }

        let mut current_line_chars = current_line.chars();
        let current_line_char3 = current_line_chars.nth(col_num + (6*i)).expect("out of bounds");
        if current_line_char3 == '#' {
            tree_count_r7_d1 += 1;
        }

        col_num += 1;
    }
    println!("{}", tree_count_r3_d1);
    println!("{}", tree_count_r5_d1);
    println!("{}", tree_count_r7_d1);
    println!("{}", tree_count_r1_d2);
    println!("{}", tree_count_d1_r1);
    println!("{}", tree_count_r3_d1 * tree_count_d1_r1 * tree_count_r5_d1 * tree_count_r7_d1 * tree_count_r1_d2);
}
