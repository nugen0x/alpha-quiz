use console::Style;
use dialoguer::Input;
use edit_distance::edit_distance;
use progress_bar::ProgressBar;
use serde::{Deserialize, Serialize};
use style::generate_colored_text;
use std::collections::HashMap;

mod progress_bar;
mod style;

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    text: String,
    answer: String,
}


fn main() {
    let questions: HashMap<&str, &str> = HashMap::from([
        ("RUST YEAR OF BIRTH?", "2010"),
        ("2022 FIFA WORLDCUP WINNER?", "Argentina"),
        ("What year is it?", "2025"),
        ("Best lang?", "Rust"),
        ("Capital of Turkiye?", "Ankara"),
    ]);

    let mut score = 0;
    let mut progress_bar = ProgressBar::new(questions.len() as u64);

    for (i, (question, correct_answer)) in questions.iter().enumerate() {
        println!("Question {}: {}", i + 1, question);
    
        let answer:String = Input::new()
            .with_prompt("Your answer")
            .interact_text()
            .unwrap();
        
        // if user answer can be parsed as i32
        if let (Ok(correct_answer), Ok(answer)) = (correct_answer.parse::<i32>(), answer.parse::<i32>()) {

            // if answer is close (1 to 3)
            if (1..=3).contains(&answer.abs_diff(correct_answer)) {
                let close_answer = generate_colored_text(&answer, "yellow");
                println!("{} is wrong but you are really close!", close_answer);
            }

            // if answer is exactly correct
            else if answer == correct_answer {
                score += 1;
                let right_answer = generate_colored_text(&answer, "green");
                println!("{} is correct!", right_answer);
            } else {
                let false_answer = generate_colored_text(&answer, "red");
                println!("{} is wrong", false_answer);
            }
        } 

        // If answer is not a number, do string-based comparison
        else {
            if answer.trim().eq_ignore_ascii_case(correct_answer) {
                score += 1;
                let right_answer = generate_colored_text(&answer, "green");
                println!("{} is correct!", right_answer);
            } else if matches!(edit_distance(&answer, correct_answer), 1..=3) {
                let close_answer = generate_colored_text(&answer, "yellow");
                println!("{} is wrong but you are really close!", close_answer);
            } else {
                let false_answer = generate_colored_text(&answer, "red");
                println!("{} is wrong", false_answer);
            }
        }
    }
    progress_bar.set_step((score) as u64);
    println!("Quiz is complete. Your score: {}/{}", score, questions.len());
}
