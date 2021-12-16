#[derive(PartialEq)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

struct Packet {
    version: u8,
    packet_type: PacketType,
    subpackets: Option<Vec<Packet>>,
    literal: Option<usize>,
}

fn to_binary(input: &str) -> String {
    let mut s = String::new();

    input.chars().for_each(|c| match c {
        '0' => s += "0000",
        '1' => s += "0001",
        '2' => s += "0010",
        '3' => s += "0011",
        '4' => s += "0100",
        '5' => s += "0101",
        '6' => s += "0110",
        '7' => s += "0111",
        '8' => s += "1000",
        '9' => s += "1001",
        'A' => s += "1010",
        'B' => s += "1011",
        'C' => s += "1100",
        'D' => s += "1101",
        'E' => s += "1110",
        'F' => s += "1111",
        _ => panic!("Bad input"),
    });

    s
}

fn get_bits(s: &mut String, bits: usize) -> usize {
    usize::from_str_radix(s.drain(..bits).as_str(), 2).unwrap()
}

fn get_next_packet(s: &mut String) -> Packet {
    let version = get_bits(s, 3) as u8;

    let packet_type = match get_bits(s, 3) {
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        4 => PacketType::Literal,
        5 => PacketType::GreaterThan,
        6 => PacketType::LessThan,
        7 => PacketType::EqualTo,
        _ => panic!("Bad packet type"),
    };

    if packet_type == PacketType::Literal {
        let mut groups = Vec::new();
        let mut done = false;
        while !done {
            let group_start = get_bits(s, 1);
            done = group_start == 0;
            groups.push(get_bits(s, 4) as u8);
        }

        let mut value = 0;
        for (i, v) in groups.iter().rev().enumerate() {
            value |= (*v as usize) << (i * 4);
        }

        Packet {
            version,
            packet_type,
            subpackets: None,
            literal: Some(value),
        }
    } else {
        let mut subpackets = Vec::new();
        let length_type = get_bits(s, 1) as u8;
        let length = match length_type {
            0 => get_bits(s, 15) as u16,
            1 => get_bits(s, 11) as u16,
            _ => panic!("Bad packet length type"),
        };

        if length_type == 1 {
            for _ in 0..length {
                subpackets.push(get_next_packet(s));
            }
        } else {
            let mut substr = s.drain(..length as usize).as_str().to_string();
            while !substr.is_empty() {
                subpackets.push(get_next_packet(&mut substr));
            }
        }

        Packet {
            version,
            packet_type,
            subpackets: Some(subpackets),
            literal: None,
        }
    }
}

fn sum_versions(p: &Packet) -> usize {
    if p.packet_type == PacketType::Literal {
        return p.version as usize;
    }

    let mut ret_value = p.version as usize;
    p.subpackets.as_ref().unwrap().iter().for_each(|sp| {
        ret_value += sum_versions(sp);
    });

    ret_value
}

fn eval_packet(p: &Packet) -> usize {
    if p.packet_type == PacketType::Literal {
        return p.literal.unwrap();
    }

    let mut value_arr = p
        .subpackets
        .as_ref()
        .unwrap()
        .iter()
        .map(|sp| eval_packet(sp));

    match p.packet_type {
        PacketType::Sum => value_arr.sum(),
        PacketType::Product => value_arr.product(),
        PacketType::Minimum => value_arr.min().unwrap(),
        PacketType::Maximum => value_arr.max().unwrap(),
        PacketType::Literal => p.literal.unwrap() as usize,
        PacketType::GreaterThan => (value_arr.next() > value_arr.next()) as usize,
        PacketType::LessThan => (value_arr.next() < value_arr.next()) as usize,
        PacketType::EqualTo => (value_arr.next() == value_arr.next()) as usize,
    }
}

fn part1(line: &str) -> usize {
    let mut packet_string = to_binary(line);
    let p = get_next_packet(&mut packet_string);
    sum_versions(&p)
}

fn part2(line: &str) -> usize {
    let mut packet_string = to_binary(line);
    let p = get_next_packet(&mut packet_string);
    eval_packet(&p)
}

pub fn run(lines: &[String]) {
    let input = lines[0].as_str();
    run_parts!(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const V6_LITERAL_2021: &str = "D2FE28";
    const OPERATOR_PACKET_0: &str = "38006F45291200";
    const OPERATOR_PACKET_1: &str = "EE00D40C823060";
    const OPERATOR_PACKET_2: &str = "8A004A801A8002F478";
    const OPERATOR_PACKET_3: &str = "620080001611562C8802118E34";
    const OPERATOR_PACKET_4: &str = "C0015000016115A2E0802F182340";
    const OPERATOR_PACKET_5: &str = "A0016C880162017C3686B18A3D4780";
    const SUM_1_2: &str = "C200B40A82";
    const PROD_7_8_9: &str = "04005AC33890";
    const MIN_7_8_9: &str = "880086C3E88112";
    const MAX_7_8_9: &str = "CE00C43D881120";
    const LESS_5_15: &str = "D8005AC2A8F0";
    const GREATER_5_15: &str = "F600BC2D8F";
    const EQUAL_5_15: &str = "9C005AC2F8F0";
    const EQUAL_SUM_1_3_PROD_2_2: &str = "9C0141080250320F1802104A08";

    #[test]
    fn test_part1() {
        assert_eq!(part1(V6_LITERAL_2021), 6);
        assert_eq!(part1(OPERATOR_PACKET_0), 9);
        assert_eq!(part1(OPERATOR_PACKET_1), 14);
        assert_eq!(part1(OPERATOR_PACKET_2), 16);
        assert_eq!(part1(OPERATOR_PACKET_3), 12);
        assert_eq!(part1(OPERATOR_PACKET_4), 23);
        assert_eq!(part1(OPERATOR_PACKET_5), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SUM_1_2), 3);
        assert_eq!(part2(PROD_7_8_9), 54);
        assert_eq!(part2(MIN_7_8_9), 7);
        assert_eq!(part2(MAX_7_8_9), 9);
        assert_eq!(part2(LESS_5_15), 1);
        assert_eq!(part2(GREATER_5_15), 0);
        assert_eq!(part2(EQUAL_5_15), 0);
        assert_eq!(part2(EQUAL_SUM_1_3_PROD_2_2), 1);
    }
}
