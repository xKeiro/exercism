pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();

    if trimmed_message.is_empty() {
        return Bob::Empty.say();
    }
    let have_letters = trimmed_message
        .chars()
        .filter(|x| x.is_alphabetic())
        .count()
        > 0;
    let is_uppercase = trimmed_message.to_uppercase() == trimmed_message && have_letters;
    let is_question = trimmed_message.ends_with('?');

    match (is_uppercase, is_question) {
        (true, true) => Bob::YelledQuestion.say(),
        (true, false) => Bob::Yell.say(),
        (false, true) => Bob::Question.say(),
        _ => Bob::Default.say(),
    }
}

enum Bob {
    Question,
    Yell,
    YelledQuestion,
    Empty,
    Default,
}

impl Bob {
    fn say(&self) -> &str {
        match *self {
            Bob::Question => "Sure.",
            Bob::Yell => "Whoa, chill out!",
            Bob::YelledQuestion => "Calm down, I know what I'm doing!",
            Bob::Empty => "Fine. Be that way!",
            Bob::Default => "Whatever.",
        }
    }
}
