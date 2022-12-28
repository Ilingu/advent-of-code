#[derive(Debug)]
pub struct Packet {
    sub_packet: Vec<Box<(Packet, usize)>>, // (value, order_id)
    values: Vec<(usize, usize)>,           // (value, order_id)
}

impl Packet {
    pub fn new(sub_packet: Vec<Box<(Packet, usize)>>, values: Vec<(usize, usize)>) -> Self {
        Self { sub_packet, values }
    }

    pub fn parse(packet_string: &str) -> Self {
        if packet_string == "[]" {
            return Self {
                values: vec![],
                sub_packet: vec![],
            };
        }

        let unwrapped = &packet_string[1..packet_string.len() - 1];
        let mut elements: Vec<(&str, usize)> = vec![];

        let (mut inside_block, mut last_split) = (0, 0);
        for (i, ch) in unwrapped.chars().enumerate() {
            match ch {
                '[' => inside_block += 1,
                ']' => inside_block -= 1,
                ',' => {
                    if inside_block == 0 {
                        elements.push((unwrapped[last_split..i].trim_matches(','), elements.len()));
                        last_split = i
                    }
                }
                _ => (),
            };
        }
        elements.push((
            unwrapped[last_split..unwrapped.len()].trim_matches(','),
            elements.len(),
        ));

        let mut values: Vec<(usize, usize)> = vec![];
        let mut sub_packet: Vec<Box<(Packet, usize)>> = vec![];
        for (elem, order_id) in elements {
            match elem.parse::<usize>() {
                Ok(int) => values.push((int, order_id)),
                Err(_) => sub_packet.push(Box::new((Self::parse(elem), order_id))),
            }
        }

        Self { sub_packet, values }
    }

    pub fn len(&self) -> usize {
        self.values.len() + self.sub_packet.len()
    }

    pub fn get_value_by_order_id(&self, id: usize) -> Option<&(usize, usize)> {
        self.values.iter().find(|(_, odr_id)| odr_id == &id)
    }

    pub fn get_sub_packet_by_order_id(&self, id: usize) -> Option<&Box<(Packet, usize)>> {
        self.sub_packet.iter().find(|v| v.1 == id)
    }

    pub fn first(&self) -> Option<usize> {
        let (a_value, a_sub_packet) = (
            self.get_value_by_order_id(0),
            self.get_sub_packet_by_order_id(0),
        );

        if a_value.is_some() {
            return Some(a_value.unwrap().0);
        } else if a_sub_packet.is_some() {
            return Self::first(&a_sub_packet.unwrap().0);
        } else {
            return None;
        }
    }

    pub fn is_right_order(a: &Self, b: &Self) -> Option<bool> {
        let (a_len, b_len) = (a.len(), b.len());
        let biggest = *vec![a_len, b_len].iter().max().unwrap();

        if a_len == 0 && b_len > 0 {
            return Some(true);
        }
        if b_len == 0 && a_len > 0 {
            return Some(false);
        }

        for ord_id in 0..biggest {
            let (a_val, b_val) = (
                a.get_value_by_order_id(ord_id),
                b.get_value_by_order_id(ord_id),
            );
            let (a_packet, b_packet) = (
                a.get_sub_packet_by_order_id(ord_id),
                b.get_sub_packet_by_order_id(ord_id),
            );

            if a_val.is_none() && b_val.is_none() {
                if a_packet.is_none() && b_packet.is_none() {
                    continue;
                }
                if a_packet.is_none() {
                    return Some(true);
                }
                if b_packet.is_none() {
                    return Some(false);
                }

                return match Self::is_right_order(&a_packet.unwrap().0, &b_packet.unwrap().0) {
                    Some(res) => Some(res),
                    None => continue,
                };
            } else if a_val.is_some() && b_val.is_none() {
                let a_value = a_val.unwrap().0;
                if b_packet.is_none() {
                    return Some(false);
                }

                return match Self::is_right_order(
                    &Self::new(vec![], vec![(a_value, 0)]),
                    &b_packet.unwrap().0,
                ) {
                    Some(res) => Some(res),
                    None => continue,
                };
            } else if b_val.is_some() && a_val.is_none() {
                let b_value = b_val.unwrap().0;
                if a_packet.is_none() {
                    return Some(true);
                }

                return match Self::is_right_order(
                    &a_packet.unwrap().0,
                    &Self::new(vec![], vec![(b_value, 0)]),
                ) {
                    Some(res) => Some(res),
                    None => continue,
                };
            } else {
                /* a_val.is_some() && b_val.is_some() */
                let (a_value, b_value) = (a_val.unwrap().0, b_val.unwrap().0);
                if a_value == b_value {
                    continue;
                }
                return Some(a_value < b_value);
            }
        }
        return None;
    }
}
