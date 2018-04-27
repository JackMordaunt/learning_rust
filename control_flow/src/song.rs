// sing produces the 12 DAYS of christmas.
pub fn sing() -> String {
    let mut song = String::new();
    for n in 0..12 {
        song.push_str(&format!("{}\n", stanza(n)));
    }
    return song;
}

fn stanza(nth: i32) -> String {
    let mut s = String::new();
    s.push_str(&format!("On the {} of Christmas my true love sent to me:\n", DAYS[nth as usize]));
    for n in (-1..nth).rev() {
        let n = n + 1;
        s.push_str(&format!("{}", GIFTS[n as usize]));
        if n != 0 {
            s.push(',');
        } else {
            s.push('.');
        }
        s.push('\n');
    }
    return s;
}

static DAYS: [&'static str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fith",
    "sixth",
    "seventh",
    "eighth",
    "nineth",
    "tenth",
    "eleventh",
    "twelth",
];

static GIFTS: [&'static str; 12] = [
    "A Partridge in a Pear Tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling/Collie Birds",
    "Five Golden Rings",
    "Six Geese-a-Laying",
    "Seven Swans-a-Swimming",
    "Eight Maids-a-Milking",
    "Nine Ladies Dancing",
    "Ten Lords-a-Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming",
];
