use std::fs;

use bitvec::prelude::*;

fn main() {
    let filename = "day16_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

struct Packet {
    version_number: u8,
    size: usize, // How many bits long the packet is (included the sub packets).
    internal_packets: Option<Vec<Packet>>,
}

impl Packet {
    fn new(packet: &BitSlice<Msb0, u8>) -> Self {
        // dbg!(packet);
        dbg!(packet.len());
        // let version_number =
        //     ((packet.get(0).unwrap()) << 2) + ((packet[1] as u8) << 1) + (packet[2] as u8);
        let version_number = packet[0..3]
            .iter()
            .inspect(|b| println!("Version bits: {:?}", b))
            .enumerate()
            .map(|(i, b)| (*b as u8) << (2 - i))
            .sum::<u8>();
        dbg!(&version_number);
        let packet_type_id = PacketType::from_bytes(&packet[3..6]);
        dbg!(&packet_type_id);

        let (size, internal_packets) = match packet_type_id {
            PacketType::Literal => {
                // This packet is only containing literal groups, building a number.
                let mut index = 6; // Index of first bit of the group.
                let mut number = 0u128;
                loop {
                    let data = packet[index + 1..=index + 4]
                        .iter()
                        .inspect(|b| println!("Literal Bits: {:?}", b))
                        .enumerate()
                        .map(|(i, b)| (*b as usize) << (3 - i))
                        .sum::<usize>();
                    dbg!(data);
                    number <<= 4;
                    number += data as u128;
                    dbg!(number);
                    if packet[index] {
                        // We have another group.
                        dbg!(index += 5);
                    } else {
                        break;
                    }
                }
                (index + 5, None)
            }
            PacketType::Operator => {
                // An operator contains multiple internal packets.
                let length_type_id = packet[6];
                let packets = match length_type_id {
                    true => {
                        const SIZE_FIELD_SIZE: usize = 11;
                        let internal_packets_count = packet[7..7 + SIZE_FIELD_SIZE]
                            .iter()
                            .inspect(|b| println!("Length Id 1 Bits: {:?}", b))
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
                    }
                    false => {
                        const SIZE_FIELD_SIZE: usize = 15;
                        let internal_packets_total_size = packet[7..7 + SIZE_FIELD_SIZE]
                            .iter()
                            .inspect(|b| println!("Length Id 0 Bits: {:?}", b))
                            .enumerate()
                            .map(|(i, b)| (*b as usize) << (SIZE_FIELD_SIZE - 1 - i))
                            .sum::<usize>();
                        let mut start_of_next_packet = 7 + SIZE_FIELD_SIZE;
                        let mut packets = Vec::new();
                        while dbg!(start_of_next_packet)
                            < dbg!(7 + SIZE_FIELD_SIZE + internal_packets_total_size)
                        {
                            let next_packet = Packet::new(&packet[start_of_next_packet..]);
                            start_of_next_packet += next_packet.size;
                            packets.push(next_packet);
                        }

                        packets
                    }
                };

                (
                    packets.iter().map(|packet| packet.size).sum::<usize>()
                        + if length_type_id { 7 + 11 } else { 7 + 15 },
                    Some(packets),
                )
            }
        };

        Self {
            version_number,
            size,
            internal_packets,
        }
    }

    fn get_version_number_sum(&self) -> u128 {
        self.version_number as u128
            + self
                .internal_packets
                .as_ref()
                .map_or(0, |internal_packets| {
                    internal_packets
                        .iter()
                        .map(|ip| ip.get_version_number_sum())
                        .sum()
                })
    }
}

#[derive(Debug)]
enum PacketType {
    Literal,
    Operator,
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
            _ => Self::Operator,
        }
    }
}

fn do_the_thing(input: &str) -> u128 {
    let bytes = hex::decode(input).expect("Input was invalid hex");
    let bytes = BitVec::<Msb0, u8>::from_slice(&bytes).unwrap();

    let packet = Packet::new(&bytes);

    packet.get_version_number_sum().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn test_example_1() {
        let input = "8A004A801A8002F478";

        let result = do_the_thing(input);

        assert_eq!(16, result);
    }

    #[test]
    fn test_example_2() {
        let input = "620080001611562C8802118E34";

        let result = do_the_thing(input);

        assert_eq!(12, result);
    }
    #[test]
    fn test_example_3() {
        let input = "C0015000016115A2E0802F182340";

        let result = do_the_thing(input);

        assert_eq!(23, result);
    }
    #[test]
    fn test_example_4() {
        let input = "A0016C880162017C3686B18A3D4780";

        let result = do_the_thing(input);

        assert_eq!(31, result);
    }

    #[test]
    fn test_single_literal() {
        let input = "D2FE28";

        let result = do_the_thing(input);

        assert_eq!(6, result);
    }

    #[test]
    fn test_operator_with_two_subpackets() {
        let input = "38006F45291200";

        let result = do_the_thing(input);
    }
}
