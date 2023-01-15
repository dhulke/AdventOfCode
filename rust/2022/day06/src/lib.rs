use std::collections::{HashSet, VecDeque};

pub mod input;


pub fn get_start_of_packet_position(characters: impl Iterator<Item=char>, window_size: usize) -> isize {
    let mut window = VecDeque::with_capacity(window_size);
    for (index, character) in characters.enumerate() {
        window.push_back(character);

        if window.len() < window_size {
            continue;
        } else if window.len() > window_size {
            window.pop_front();
        }

        let hash_set: HashSet<&char> = HashSet::from_iter(window.iter());

        if hash_set.len() == window_size {
            return (index + 1) as isize;
        }
    }
    -1
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_start_of_packet_position() {
        assert_eq!(get_start_of_packet_position("bvwbjplbgvbhsrlpgdmjqwftvncz".chars(), 4), 5);
        assert_eq!(get_start_of_packet_position("nppdvjthqldpwncqszvftbrmjlhg".chars(), 4), 6);
        assert_eq!(get_start_of_packet_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars(), 4), 10);
        assert_eq!(get_start_of_packet_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars(), 4), 11);
    }

    #[test]
    fn test_get_start_of_packet_position_message() {
        assert_eq!(get_start_of_packet_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars(), 14), 19);
        assert_eq!(get_start_of_packet_position("bvwbjplbgvbhsrlpgdmjqwftvncz".chars(), 14), 23);
        assert_eq!(get_start_of_packet_position("nppdvjthqldpwncqszvftbrmjlhg".chars(), 14), 23);
        assert_eq!(get_start_of_packet_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars(), 14), 29);
        assert_eq!(get_start_of_packet_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars(), 14), 26);
    }
}