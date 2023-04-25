pub mod packet;

use std::{fs, vec};

use std::cmp::Ordering;

use crate::packet::Packet;

fn main() {
    let signal = fs::read_to_string("./Assets/input.txt").unwrap();
    let pairs = signal.split("\n\n");

    let mut packets_pairs: Vec<Packet> = vec![];
    let mut p1 = 0;

    for (pair_id, pair) in pairs.enumerate() {
        let raw_packets_pair = pair.lines();
        assert_eq!(raw_packets_pair.clone().count(), 2);

        let mut packets_pair: Vec<Packet> = vec![];
        for packet_string in raw_packets_pair {
            packets_pair.push(Packet::parse(packet_string));
        }

        match Packet::is_right_order(&packets_pair[0], &packets_pair[1]) {
            Some(res) => {
                if res {
                    p1 += pair_id + 1;
                }
            }
            None => (),
        };
        packets_pairs.append(&mut packets_pair);
    }

    /* For part 2
    - Sort by first order_id value
        - sort by sum of value (but we don't care about this because we just can the decoder's indexes)
    */

    // Part2 sorting
    packets_pairs.sort_by(|a, b| -> Ordering {
        let (a_first_value, b_first_value) = (a.first(), b.first());
        if a_first_value.is_none() {
            return Ordering::Less;
        }
        if b_first_value.is_none() {
            return Ordering::Greater;
        }
        if a_first_value.unwrap() == b_first_value.unwrap() {
            return Ordering::Equal;
        }
        return if a_first_value.unwrap() > b_first_value.unwrap() {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    });

    let (mut divider2_idx, mut divider6_idx) = (0, 0);

    let mut last_first_value = 0;
    for (index, packet) in packets_pairs.iter().enumerate() {
        match packet.first() {
            Some(first) => {
                if last_first_value <= 1 && first >= 2 && divider2_idx == 0 {
                    divider2_idx = index + 1
                }
                if last_first_value <= 5 && first >= 6 && divider6_idx == 0 {
                    divider6_idx = index + 2 // +1 because of the way it count index and another +1 because divider2 count as one item more in the list
                }
                last_first_value = first
            }
            None => continue,
        }
    }

    println!("Part1: {p1}");
    println!("Part2: {}", divider2_idx * divider6_idx);
}
