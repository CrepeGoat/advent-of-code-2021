use aoc_lib::nom_utils::take_rem;
use core::fmt::Debug;
use core::str::FromStr;
use nom::bits::complete::take as take_bits;
use nom::combinator::map;
use std::io::BufRead;

use nom::sequence::tuple;
use std::convert::TryInto;

#[derive(Debug)]
pub enum ParseInputError<T: FromStr>
where
    <T as FromStr>::Err: Debug,
{
    InnerType(<T as FromStr>::Err),
    IO(std::io::Error),
}

fn parse_input<R: BufRead, T: FromStr>(mut reader: R) -> Result<T, ParseInputError<T>>
where
    <T as FromStr>::Err: Debug,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).map_err(ParseInputError::IO)?;

    T::from_str(&buffer).map_err(ParseInputError::InnerType)
}

fn iter_pairs<I: Iterator>(
    iter: I,
) -> impl Iterator<Item = (<I as Iterator>::Item, <I as Iterator>::Item)> {
    let mut item_buffer = None;

    iter.filter_map(move |item| match item_buffer.take() {
        Some(item1) => Some((item1, item)),
        None => {
            item_buffer = Some(item);
            None
        }
    })
}

#[derive(Debug, PartialEq, Eq)]
struct HexBits(Vec<u8>);

impl FromStr for HexBits {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        iter_pairs(s.chars().map(|c| c.to_digit(16).ok_or("invalid digit")))
            .map(|(r1, r2)| -> Result<_, _> { Ok(((r1? << 4) | r2?).try_into().unwrap()) })
            .collect::<Result<Vec<_>, _>>()
            .map(HexBits)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LengthType {
    TotalLength(u16),
    Subpackets(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PacketType {
    Literal(u32),
    Operator(LengthType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Packet(u8, PacketType);

impl Packet {
    fn read((stream, bit_offset): (&[u8], usize)) -> nom::IResult<(&[u8], usize), Self> {
        use LengthType::*;
        use PacketType::*;

        if let ((&[], 0), (0, _)) = take_rem()((stream, bit_offset))? {
            return Err(nom::Err::Failure(nom::error::Error {
                input: (stream, bit_offset),
                code: nom::error::ErrorKind::Eof,
            }));
        }

        let ((stream, bit_offset), (version_number, packet_id)) =
            tuple((take_bits(3_usize), take_bits(3_usize)))((stream, bit_offset))?;

        let ((stream, bit_offset), parsed) = match packet_id {
            4 => {
                let mut result: u32 = 0;

                let (mut stream, mut bit_offset) = (stream, bit_offset);
                loop {
                    let ((s, bo), (bit_continue, bits)): (_, (u32, u32)) =
                        tuple((take_bits(1_usize), take_bits(4_usize)))((stream, bit_offset))?;

                    result = (result << 4) | bits;
                    stream = s;
                    bit_offset = bo;
                    match bit_continue {
                        0 => break,
                        1 => (),
                        _ => unreachable!(),
                    }
                }
                ((stream, bit_offset), Literal(result))
            }
            _ => {
                let ((stream, bit_offset), len_type_id): (_, u8) =
                    take_bits(1_usize)((stream, bit_offset))?;
                match len_type_id {
                    0 => map(&take_bits(15_usize), |len| Operator(TotalLength(len)))((
                        stream, bit_offset,
                    ))?,
                    1 => map(&take_bits(11_usize), |len| Operator(Subpackets(len)))((
                        stream, bit_offset,
                    ))?,
                    _ => unreachable!(),
                }
            }
        };

        Ok(((stream, bit_offset), Self(version_number, parsed)))
    }
}

fn iter_packets<'a>(hbits: &'a HexBits) -> impl 'a + Iterator<Item = Packet> {
    itertools::unfold((&hbits.0[..], 0_usize), |input| -> Option<_> {
        let (new_input, packet) = Packet::read(*input).ok()?;
        *input = new_input;
        Some(packet)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(input, expt_result,
        case(vec![1, 2, 3, 4], vec![(1, 2), (3, 4)])
    )]
    fn test_iter_pairs(input: Vec<i32>, expt_result: Vec<(i32, i32)>) {
        assert_eq!(
            iter_pairs(input.into_iter()).collect::<Vec<_>>(),
            expt_result
        );
    }

    #[rstest(input, expt_result,
        case("D2FE28", Ok(HexBits(vec![0xD2, 0xFE, 0x28]))),
        case("38006F45291200", Ok(HexBits(vec![0x38, 0x00, 0x6F, 0x45, 0x29, 0x12, 0x00]))),
        case("EE00D40C823060", Ok(HexBits(vec![0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60]))),
        case("8A004A801A8002F478", Ok(HexBits(vec![0x8A, 0x00, 0x4A, 0x80, 0x1A, 0x80, 0x02, 0xF4, 0x78]))),
        case("620080001611562C8802118E34", Ok(HexBits(vec![0x62, 0x00, 0x80, 0x00, 0x16, 0x11, 0x56, 0x2C, 0x88, 0x02, 0x11, 0x8E, 0x34]))),
        case("C0015000016115A2E0802F182340", Ok(HexBits(vec![0xC0, 0x01, 0x50, 0x00, 0x01, 0x61, 0x15, 0xA2, 0xE0, 0x80, 0x2F, 0x18, 0x23, 0x40]))),
        case("A0016C880162017C3686B18A3D4780", Ok(HexBits(vec![0xA0, 0x01, 0x6C, 0x88, 0x01, 0x62, 0x01, 0x7C, 0x36, 0x86, 0xB1, 0x8A, 0x3D, 0x47, 0x80]))),

        case("D2F", Ok(HexBits(vec![0xD2]))),
        case("DG", Err("invalid digit")),
    )]
    fn test_hex_decoder_egs(input: &'static str, expt_result: Result<HexBits, &'static str>) {
        assert_eq!(HexBits::from_str(input), expt_result);
    }

    #[rstest(
        input,
        expt_result,
        case("D2FE28", 6),
        case("38006F45291200", 9),
        case("EE00D40C823060", 14),
        case("8A004A801A8002F478", 16),
        case("620080001611562C8802118E34", 12),
        case("C0015000016115A2E0802F182340", 23),
        case("A0016C880162017C3686B18A3D4780", 31)
    )]
    fn test_packet_decoder_version_sum_egs(input: &'static str, expt_result: usize) {
        let bytes = HexBits::from_str(input).unwrap();

        let calc_result: usize = iter_packets(&bytes).map(|p| p.0 as usize).sum();
        assert_eq!(calc_result, expt_result);
    }

    #[rstest(
        input,
        expt_result,
        case("D2FE28", vec![Packet(6, PacketType::Literal(2021))]),
        case("38006F45291200", vec![
            Packet(1, PacketType::Operator(LengthType::TotalLength(27))),
            Packet(6, PacketType::Literal(10)),
            Packet(2, PacketType::Literal(20)),
        ]),
        case("EE00D40C823060", vec![
            Packet(7, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(2, PacketType::Literal(1)),
            Packet(4, PacketType::Literal(2)),
            Packet(1, PacketType::Literal(3)),
        ]),
        /*
        case("8A004A801A8002F478", vec![
            Packet(4, PacketType::Operator(LengthType::Subpackets(1))),
            Packet(1, PacketType::Operator(LengthType::Subpackets(1))),
            Packet(5, PacketType::Operator(LengthType::TotalLength(23))),
            Packet(6, PacketType::Literal(3)),
        ]),
        case("620080001611562C8802118E34", vec![
            Packet(3, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(1, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(6, PacketType::Literal(3)),
            Packet(6, PacketType::Literal(3)),
            Packet(5, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(6, PacketType::Literal(3)),
            Packet(6, PacketType::Literal(3)),
        ]),
        case("C0015000016115A2E0802F182340", vec![
            Packet(3, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(1, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(6, PacketType::Literal(3)),
            Packet(6, PacketType::Literal(3)),
            Packet(5, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(6, PacketType::Literal(3)),
            Packet(6, PacketType::Literal(3)),
        ]),
        case("A0016C880162017C3686B18A3D4780", vec![
            Packet(3, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(1, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(1, PacketType::Operator(LengthType::Subpackets(3))),
            Packet(6, PacketType::Literal(3)),
            Packet(6, PacketType::Literal(3)),
            Packet(6, PacketType::Literal(3)),
            Packet(6, PacketType::Literal(3)),
            Packet(6, PacketType::Literal(3)),
        ])
        */
    )]
    fn test_packet_decoder_egs(input: &'static str, expt_result: Vec<Packet>) {
        let bytes = HexBits::from_str(input).unwrap();
        for x in &bytes.0 {
            println!("{:#010b}", x);
        }

        let calc_result = iter_packets(&bytes).collect::<Vec<_>>();
        assert_eq!(calc_result, expt_result);
    }
}

fn main() {
    println!("Enter input:");
    let stdin = std::io::stdin();

    let bytes = parse_input::<_, HexBits>(stdin.lock()).unwrap();
    let version_sum: usize = iter_packets(&bytes).map(|p| p.0 as usize).sum();
    println!("version sum = {:?}", version_sum);
}
