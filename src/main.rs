use std::io;
/* This is a quiz program, created to learn the basics of rust. */
fn main() {
    println!("Quiz Game");

    // Declaring program variables
    let mut score: u32 = 0;
    let mut question_index:usize = 0;
    
    // Declaring the question bank
    let questions = vec!["What is the capital of france", "What is the capital of Gemany", "What is the capital of Spain"];
    let answers = vec!["Paris","Berlin", "Madrid"];
    
    // Main body of the program
    loop{
        let mut answer: String = String::new();
        println!("{}", questions[question_index]);
        io::stdin().read_line(&mut answer).expect("Please enter a correct value");
        if answer.trim().to_lowercase() == answers[question_index].to_lowercase() {
            println!("Welldone, this is the correct answer");
            score+=1;
        }else{
            println!("Sorry, try again")
        }
        question_index += 1;
        if question_index >= questions.len() {
            question_index = 0;
            answer = String::new();
            println!("Quiz finished, results: {}/{}", score, questions.len());
            println!("Would you like to quit?(yes or y)");
            io::stdin().read_line(&mut answer).expect("Please enter a correct value");
            match answer.trim().to_lowercase().as_str() {
                "yes" | "y" => break,
                _ => println!("You choose to continue")
            }
            
            score = 0;
        }
    }
}
