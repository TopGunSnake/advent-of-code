use std::fs;

use bitvec::prelude::*;
use itertools::Itertools;

fn main() {
    let filename = "day16_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

struct Packet {
    size: usize, // How many bits long the packet is (included the sub packets).
    packet_type: PacketType,
    internal_packets: Option<Vec<Packet>>,
    data: Option<u128>, // Data from literal.
}

impl Packet {
    fn new(packet: &BitSlice<Msb0, u8>) -> Self {
        let packet_type_id = PacketType::from_bytes(&packet[3..6]);

        let (size, data, internal_packets) = match packet_type_id {
            PacketType::Literal => {
                // This packet is only containing literal groups, building a number.
                let mut index = 6; // Index of first bit of the group.
                let mut number = 0u128;
                loop {
                    let data = packet[index + 1..=index + 4]
                        .iter()
                        .enumerate()
                        .map(|(i, b)| (*b as usize) << (3 - i))
                        .sum::<usize>();
                    number <<= 4;
                    number += data as u128;
                    if packet[index] {
                        // We have another group.
                        index += 5;
                    } else {
                        break;
                    }
                }
                (index + 5, Some(number), None)
            }
            PacketType::Operator(_) => {
                // An operator contains multiple internal packets.
                let length_type_id = packet[6];
                let packets = if length_type_id {
                    const SIZE_FIELD_SIZE: usize = 11;
                    let internal_packets_count = packet[7..7 + SIZE_FIELD_SIZE]
                        .iter()
                        .enumerate()
                        .map(|(i, b)| (*b as usize) << (SIZE_FIELD_SIZE - 1 - i))
                        .sum::<usize>();
                    let mut start_of_next_packet = 7 + SIZE_FIELD_SIZE;
                    let mut packets = Vec::new();

                    for _ in 0..internal_packets_count {
                        let next_packet = Packet::new(&packet[start_of_next_packet..]);
                        start_of_next_packet += next_packet.size;
                        packets.push(next_packet);
                    }
                    packets
                } else {
                    const SIZE_FIELD_SIZE: usize = 15;
                    let internal_packets_total_size = packet[7..7 + SIZE_FIELD_SIZE]
                        .iter()
                        .enumerate()
                        .map(|(i, b)| (*b as usize) << (SIZE_FIELD_SIZE - 1 - i))
                        .sum::<usize>();
                    let mut start_of_next_packet = 7 + SIZE_FIELD_SIZE;
                    let mut packets = Vec::new();
                    while start_of_next_packet
                        < (7 + SIZE_FIELD_SIZE + internal_packets_total_size)
                    {
                        let next_packet = Packet::new(&packet[start_of_next_packet..]);
                        start_of_next_packet += next_packet.size;
                        packets.push(next_packet);
                    }

                    packets
                };

                (
                    packets.iter().map(|packet| packet.size).sum::<usize>()
                        + if length_type_id { 7 + 11 } else { 7 + 15 },
                    None,
                    Some(packets),
                )
            }
            PacketType::Invalid => panic!("Error parsing"),
        };

        Self {
            size,
            internal_packets,
            packet_type: packet_type_id,
            data,
        }
    }

    fn execute(&self) -> u128 {
        match &self.packet_type {
            PacketType::Literal => self.data.unwrap(),

            PacketType::Operator(OperatorType::Sum) => self
                .internal_packets
                .as_ref()
                .unwrap()
                .iter()
                .map(|packet| packet.execute())
                .sum(),
            PacketType::Operator(OperatorType::Product) => self
                .internal_packets
                .as_ref()
                .unwrap()
                .iter()
                .map(|packet| packet.execute())
                .product(),

            PacketType::Operator(OperatorType::Minimum) => self
                .internal_packets
                .as_ref()
                .unwrap()
                .iter()
                .map(|packet| packet.execute())
                .min()
                .unwrap(),
            PacketType::Operator(OperatorType::Maximum) => self
                .internal_packets
                .as_ref()
                .unwrap()
                .iter()
                .map(|packet| packet.execute())
                .max()
                .unwrap(),

            PacketType::Operator(OperatorType::GreaterThan) => {
                let (left, right) = self
                    .internal_packets
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|packet| packet.execute())
                    .collect_tuple()
                    .unwrap();
                if left > right {
                    1
                } else {
                    0
                }
            }
            PacketType::Operator(OperatorType::LessThan) => {
                let (left, right) = self
                    .internal_packets
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|packet| packet.execute())
                    .collect_tuple()
                    .unwrap();
                if left < right {
                    1
                } else {
                    0
                }
            }
            PacketType::Operator(OperatorType::Equal) => {
                let (left, right) = self
                    .internal_packets
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|packet| packet.execute())
                    .collect_tuple()
                    .unwrap();
                if left == right {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Error parsing"),
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Literal,
    Operator(OperatorType),
    Invalid,
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl PacketType {
    fn from_bytes(input: &BitSlice<Msb0, u8>) -> Self {
        let value = input
            .iter()
            .enumerate()
            .map(|(i, b)| (*b as usize) << (2 - i))
            .sum::<usize>();

        match value {
            4 => Self::Literal,
            0 => Self::Operator(OperatorType::Sum),
            1 => Self::Operator(OperatorType::Product),
            2 => Self::Operator(OperatorType::Minimum),
            3 => Self::Operator(OperatorType::Maximum),
            5 => Self::Operator(OperatorType::GreaterThan),
            6 => Self::Operator(OperatorType::LessThan),
            7 => Self::Operator(OperatorType::Equal),
            _ => Self::Invalid,
        }
    }
}

fn do_the_thing(input: &str) -> u128 {
    let bytes = hex::decode(input).expect("Input was invalid hex");
    let bytes = BitVec::<Msb0, u8>::from_slice(&bytes).unwrap();

    let packet = Packet::new(&bytes);

    packet.execute()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_1_and_2() {
        let input = "C200B40A82";

        let result = do_the_thing(input);

        assert_eq!(3, result);
    }

    #[test]
    fn test_product_6_and_9() {
        let input = "04005AC33890";

        let result = do_the_thing(input);

        assert_eq!(54, result);
    }
    #[test]
    fn test_min_7_8_and_9() {
        let input = "880086C3E88112";

        let result = do_the_thing(input);

        assert_eq!(7, result);
    }
    #[test]
    fn test_max_7_8_and_9() {
        let input = "CE00C43D881120";

        let result = do_the_thing(input);

        assert_eq!(9, result);
    }

    #[test]
    fn test_less_than_5_15() {
        let input = "D8005AC2A8F0";

        let result = do_the_thing(input);

        assert_eq!(1, result);
    }
    #[test]
    fn test_greater_than_5_15() {
        let input = "F600BC2D8F";

        let result = do_the_thing(input);

        assert_eq!(0, result);
    }
    #[test]
    fn test_equal_5_15() {
        let input = "9C005AC2F8F0";

        let result = do_the_thing(input);

        assert_eq!(0, result);
    }
    #[test]
    fn test_complicated_1_plus_3_equal_2_times_2() {
        let input = "9C0141080250320F1802104A08";

        let result = do_the_thing(input);

        assert_eq!(1, result);
    }
}
