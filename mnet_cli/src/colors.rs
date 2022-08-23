pub enum Color {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Purple = 35,
    Cyan = 36,
    White = 37,

    StrongBlack = 90,
    StrongRed = 91,
    StrongGreen = 92,
    StrongYellow = 93,
    StrongBlue = 94,
    StrongPurple = 95,
    StrongCyan = 96,
    StrongWhite = 97,
}

pub enum Effect {
    Normal,
    Bold,
    Dim,
    Underline,
    Blinking,
    Reversed,
}

impl Effect {
    fn value(&self) -> &str {
        match self {
            Effect::Normal => "0;",
            Effect::Bold => "1;",
            Effect::Dim => "2;",
            Effect::Underline => "4;",
            Effect::Blinking => "5;",
            Effect::Reversed => "7;",
        }
    }
}

pub fn ansi_string(s: &str, color: Color, effect: Effect) -> String {
    let color_val = color as isize;
    let effect_val = effect.value();
    format!("\x1b[{}{}m{}\x1b[0m", effect_val, color_val, s)
}
