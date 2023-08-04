use std::vec;

use colored::Color;
use colored::Colorize;
use colored::ColoredString;

#[allow(dead_code)]
pub enum Palette {
    ROSEWATER,
    FLAMINGO,
    PINK,
    MAUVE,
    RED,
    MAROON,
    PEACH,
    YELLOW,
    GREEN,
    TEAL,
    SKY,
    SAPPHIRE,
    BLUE,
    LAVENDER,
    TEXT,
    SUBTEXT1,
    SUBTEXT0,
    OVERLAY2,
    OVERLAY1,
    OVERLAY0,
    SURFACE2,
    SURFACE1,
    SURFACE0,
    BASE,
    MANTLE,
    CRUST,
}

impl Palette {
    fn rgb(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        // Does nothing.
        (r, g, b)
    }

    pub fn to_color(&self) -> (u8, u8, u8) {
        match self {
            // Self::rgb only for VSCode visibility
            Palette::ROSEWATER => Self::rgb(245, 224, 220),
            Palette::FLAMINGO => Self::rgb(242, 205, 205),
            Palette::PINK => Self::rgb(245, 194, 231),
            Palette::MAUVE => Self::rgb(203, 166, 247),
            Palette::RED => Self::rgb(243, 139, 168),
            Palette::MAROON => Self::rgb(235, 160, 172),
            Palette::PEACH => Self::rgb(250, 179, 135),
            Palette::YELLOW => Self::rgb(255, 219, 140), // Modified; slightly more vibrant
            Palette::GREEN => Self::rgb(154, 231, 146), // Modified; slightly more vibrant
            Palette::TEAL => Self::rgb(129, 255, 234), // Modified; slightly more vibrant
            Palette::SKY => Self::rgb(137, 220, 235),
            Palette::SAPPHIRE => Self::rgb(116, 199, 236),
            Palette::BLUE => Self::rgb(137, 180, 250),
            Palette::LAVENDER => Self::rgb(180, 190, 254),
            Palette::TEXT => Self::rgb(205, 214, 244),
            Palette::SUBTEXT1 => Self::rgb(186, 194, 222),
            Palette::SUBTEXT0 => Self::rgb(166, 173, 200),
            Palette::OVERLAY2 => Self::rgb(147, 153, 178),
            Palette::OVERLAY1 => Self::rgb(127, 132, 156),
            Palette::OVERLAY0 => Self::rgb(108, 112, 134),
            Palette::SURFACE2 => Self::rgb(88, 91, 112),
            Palette::SURFACE1 => Self::rgb(69, 71, 90),
            Palette::SURFACE0 => Self::rgb(49, 50, 68),
            Palette::BASE => Self::rgb(30, 30, 46),
            Palette::MANTLE => Self::rgb(24, 24, 37),
            Palette::CRUST => Self::rgb(17, 17, 27),
        }
    }
}
// Shouldn't be required, as no fancy ANSI escape codes are being used, but some colors aren't included in the colored API.


const COLOR_MAP: [(char, Color); 16] = [
    ('0', Color::TrueColor { r: 0, g: 0, b: 0 }),             // Black
    ('1', Color::TrueColor { r: 0, g: 0, b: 170 }),           // Dark Blue
    ('2', Color::TrueColor { r: 0, g: 170, b: 0 }),           // Dark Green
    ('3', Color::TrueColor { r: 0, g: 170, b: 170 }),         // Dark Aqua
    ('4', Color::TrueColor { r: 170, g: 0, b: 0 }),           // Dark Red
    ('5', Color::TrueColor { r: 170, g: 0, b: 170 }),         // Dark Purple
    ('6', Color::TrueColor { r: 255, g: 170, b: 0 }),         // Gold
    ('7', Color::TrueColor { r: 170, g: 170, b: 170 }),       // Gray
    ('8', Color::TrueColor { r: 85, g: 85, b: 85 }),          // Dark Gray
    ('9', Color::TrueColor { r: 85, g: 85, b: 255 }),         // Blue
    ('a', Color::TrueColor { r: 85, g: 255, b: 85 }),         // Green
    ('b', Color::TrueColor { r: 85, g: 255, b: 255 }),        // Aqua
    ('c', Color::TrueColor { r: 255, g: 85, b: 85 }),         // Red
    ('d', Color::TrueColor { r: 255, g: 85, b: 255 }),        // Light Purple
    ('e', Color::TrueColor { r: 255, g: 255, b: 85 }),        // Yellow
    ('f', Color::TrueColor { r: 205, g: 214, b: 244 }),       // White
];

pub fn color(str: String, color: Palette) -> ColoredString {
    let rgb = color.to_color();
    str.truecolor(rgb.0, rgb.1, rgb.2)
}

pub fn mc_color(str: String) -> String {
    

    let lines = str.lines().peekable();
    let mut new_lines: Vec<String> = vec![];
    
    for line in lines {
        let mut trimmed: String;

        if line.starts_with("§") {
            let color_code = &line[0..3];
            trimmed = line.chars().skip(2).collect();
            trimmed = trimmed.trim_start().to_string();
            trimmed.insert_str(0, color_code);
        } else {
            trimmed = line.trim_start().to_string();
        }
        
        let mut chars = trimmed.chars().peekable();
        let mut result = String::new();
        let mut current_color: Option<Color> = None;
        let (mut obfuscated, mut bold, mut strikethrough, mut underline, mut italic) = (false, false, false, false, false);

        while let Some(c) = chars.next() {
            if c == '§' {
                if let Some(color_char) = chars.next() {
                    match color_char {
                        'r' => {
                            current_color = None;
                            obfuscated = false;
                            bold = false;
                            strikethrough = false;
                            underline = false;
                            italic = false;
                        }
                        'k' => {
                            obfuscated = true
                        }
                        'l' => {
                            bold = true
                        }
                        'm' => {
                            strikethrough = true
                        }
                        'n' => {
                            underline = true
                        }
                        'o' => {
                            italic = true
                        }
                        _ => {
                            if let Some(color) = COLOR_MAP.iter().find(|(ch, _)| *ch == color_char) {
                                // "In Java Edition, if a color code is used after a formatting code, the formatting code is disabled beyond the color code point. For example, §cX§nY displays as XY, whereas §nX§cY displays as XY. Therefore, when using a color code in tandem with a formatting code, ensure the color code is used first and reuse the formatting code when changing colors."
                                current_color = Some(color.1);
                                obfuscated = false;
                                bold = false;
                                strikethrough = false;
                                underline = false;
                                italic = false;
                            }
                        }
                    }
                }
            } else {
                if let Some(color) = current_color {
                    if obfuscated {
                        if c != ' ' {
                            result.push_str(&format!("{}", "?".to_string().color(color)));
                        } else {
                            result.push_str(" ");
                        }
                    } else if bold {
                        result.push_str(&format!("{}", c.to_string().color(color)).bold());
                    } else if strikethrough {
                        result.push_str(&format!("{}", c.to_string().color(color)).strikethrough());
                    } else if underline {
                        result.push_str(&format!("{}", c.to_string().color(color)).underline());
                    } else if italic {
                        result.push_str(&format!("{}", c.to_string().color(color)).italic());
                    } else {
                        result.push_str(&format!("{}", c.to_string().color(color)));
                    }
                } else {
                    result.push(c);
                }
            }
        }    

        new_lines.push(result);
    }
    
    new_lines.join("\n")
}

pub fn ping_color(ping: u64) -> ColoredString {
    if ping < 100 {
        ping.to_string().green()
    } else if ping < 300 {
        ping.to_string().yellow()
    } else {
        ping.to_string().red()
    }
}