use std::collections::HashMap;
enum HidCharacter {
    Shift,
    CarriageReturn,
    Character(char),
    Unknown,
}

fn decode_char(char: u16, shift: bool) -> HidCharacter {
    if 0x28 == char {
        return HidCharacter::CarriageReturn;
    } else if 0x02 == char {
        return HidCharacter::Shift;
    }

    let hid: HashMap<u16, char> = [
        (4, 'a'),
        (5, 'b'),
        (6, 'c'),
        (7, 'd'),
        (8, 'e'),
        (9, 'f'),
        (10, 'g'),
        (11, 'h'),
        (12, 'i'),
        (13, 'j'),
        (14, 'k'),
        (15, 'l'),
        (16, 'm'),
        (17, 'n'),
        (18, 'o'),
        (19, 'p'),
        (20, 'q'),
        (21, 'r'),
        (22, 's'),
        (23, 't'),
        (24, 'u'),
        (25, 'v'),
        (26, 'w'),
        (27, 'x'),
        (28, 'y'),
        (29, 'z'),
        (30, '1'),
        (31, '2'),
        (32, '3'),
        (33, '4'),
        (34, '5'),
        (35, '6'),
        (36, '7'),
        (37, '8'),
        (38, '9'),
        (39, '0'),
        (44, ' '),
        (45, '-'),
        (46, '='),
        (47, '['),
        (48, ']'),
        (49, '\\'),
        (51, ';'),
        (52, '\''),
        (53, '~'),
        (54, ','),
        (55, '.'),
        (56, '/'),
    ]
    .iter()
    .cloned()
    .collect();

    let hid2: HashMap<u16, char> = [
        (4, 'A'),
        (5, 'B'),
        (6, 'C'),
        (7, 'D'),
        (8, 'E'),
        (9, 'F'),
        (10, 'G'),
        (11, 'H'),
        (12, 'I'),
        (13, 'J'),
        (14, 'K'),
        (15, 'L'),
        (16, 'M'),
        (17, 'N'),
        (18, 'O'),
        (19, 'P'),
        (20, 'Q'),
        (21, 'R'),
        (22, 'S'),
        (23, 'T'),
        (24, 'U'),
        (25, 'V'),
        (26, 'W'),
        (27, 'X'),
        (28, 'Y'),
        (29, 'Z'),
        (30, '!'),
        (31, '@'),
        (32, '#'),
        (33, '$'),
        (34, '%'),
        (35, '^'),
        (36, '&'),
        (37, '*'),
        (38, '('),
        (39, ')'),
        (44, ' '),
        (45, '_'),
        (46, '+'),
        (47, '{'),
        (48, '}'),
        (49, '|'),
        (51, ':'),
        (52, '"'),
        (53, '~'),
        (54, '<'),
        (55, '>'),
        (56, '?'),
    ]
    .iter()
    .cloned()
    .collect();

    let map = if shift { &hid2 } else { &hid };

    match map.get(&char) {
        Some(c) => HidCharacter::Character(*c),
        None => HidCharacter::Unknown,
    }
}

pub fn decode(card: &[u16]) -> Result<String, String> {
    let mut result = String::new();
    let mut shift = false;
    for character in card.iter() {
        let character = *character;
        if character != 0 {
            match decode_char(character, shift) {
                HidCharacter::Shift => {
                    shift = !shift;
                }
                HidCharacter::CarriageReturn => return Ok(result),
                HidCharacter::Character(c) => {
                    result.push_str(&format!("{}", c));
                }
                HidCharacter::Unknown => {
                    return Err(format!("Unknown character: {}", character));
                }
            }
        }
    }
    Err("No carriage return found".to_string())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_card() {
        const CARD: [u16; 88] = [
            0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0027, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x001f, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0022, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0020, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0024, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0025,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0022, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0028, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        ];
        let result = decode(&CARD);
        assert_eq!(result, Ok("0002530785".to_string()));
    }
    #[test]
    fn invalid_card() {
        const CARD: [u16; 88] = [
            0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0027, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x001f, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0022, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0020, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0003, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0025,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0022, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0028, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        ];
        let result = decode(&CARD);
        assert_eq!(result, Err("Unknown character: 3".to_string()));
    }
    #[test]
    fn no_carriage_return() {
        const CARD: [u16; 88] = [
            0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0027, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x001f, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0024,
            0x0000, 0x0000, 0x0000, 0x0022, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0024, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0025, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0024, 0x0000, 0x0000, 0x0027, 0x0000, 0x0000, 0x0000, 0x0000,
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0024, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        ];
        let result = decode(&CARD);
        assert_eq!(result, Err("No carriage return found".to_string()));
    }
}
