#![no_std]

use heapless::Vec; // fixed capacity `std::Vec` like container // type level integer used to specify capacity

pub struct MathProblem<'a> {
    question: &'a str,
    answer: i32,
}
pub struct Lesson<'a> {
    pub title: &'a str,
    problems: Vec<MathProblem<'a>, 10>,
}

pub fn create_math_lessons() -> Vec<Lesson<'static>, 10> {
    let mut vec = Vec::new();
    vec.push(Lesson {
        title: "Addition",
        problems: Vec::new(),
    });
    vec.push(Lesson {
        title: "Subtraction",
        problems: Vec::new(),
    });
    vec.push(Lesson {
        title: "Multiplication",
        problems: Vec::new(),
    });
    vec.push(Lesson {
        title: "Divesion",
        problems: Vec::new(),
    });
    vec
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::any::Any;

    #[test]
    fn it_works() {
        // let l = create_math_lessons();
        // l.first().unwrap().lesson_type.type_id()
    }
}
