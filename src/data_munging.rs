// http://codekata.com/kata/kata04-data-munging/
use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub fn read_weather() -> (u8, u8) {
    let path = Path::new("data/weather.dat");
    let content = fs::read_to_string(path).expect("Invalid path.");
    let split = content.split('\n');
    let mut min_spread = (0, u8::MAX);
    for line in split {
        let mut values = line.split_whitespace();
        let day: u8 = match values.next() {
            Some(d) => match d.parse::<u8>() {
                Ok(n) => n,
                Err(_) => 0,
            },
            None => 0,
        };
        let max: u8 = match values.next() {
            Some(d) => match d.parse::<u8>() {
                Ok(n) => n,
                Err(_) => u8::MAX,
            },
            None => u8::MAX,
        };
        let min: u8 = match values.next() {
            Some(d) => match d.parse::<u8>() {
                Ok(n) => n,
                Err(_) => 0,
            },
            None => 0,
        };

        let spread = max - min;
        if spread < min_spread.1 {
            min_spread = (day, spread);
        }
    }

    return min_spread;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_munging() {
        assert_eq!((14, 2), read_weather())
    }
}
