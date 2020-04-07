use std::io::{self, Write};

pub struct Choice<T> {
    pub val: &'static str,
    pub extra: T,
}

pub struct Choices<'a, T, M> {
    pub choices: Vec<&'a Choice<T>>,
    pub flavor: &'static str,
    pub extra: M,
}

impl<'a, T, M> Choices<'a, T, M> {
    pub fn print(&self) {
        println!("");
        println!("{}", self.flavor);
        for (i, choice) in self.choices.iter().enumerate() {
            println!("{}) {}", i + 1, choice.val);
        }
    }

    pub fn get_input(&self) -> &Choice<T> {
        print!("What will you do? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let ind_res = input.trim().parse::<usize>();
        match ind_res {
            Err(_) => {
                println!("Your input must be a non-negative number.");
                return self.get_input();
            }
            Ok(ind) => {
                if ind > self.choices.len() {
                    println!("Your input is not in the range of the list.");
                    self.get_input();
                }
                return self.choices[ind - 1];
            }
        };
    }
}
