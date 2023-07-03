use crate::BitBoard;

pub const WHITE_PAWN_ATTACKS: [BitBoard; 64] = [
    BitBoard(0x0000_0000_0000_0200),
    BitBoard(0x0000_0000_0000_0500),
    BitBoard(0x0000_0000_0000_0a00),
    BitBoard(0x0000_0000_0000_1400),
    BitBoard(0x0000_0000_0000_2800),
    BitBoard(0x0000_0000_0000_5000),
    BitBoard(0x0000_0000_0000_a000),
    BitBoard(0x0000_0000_0000_4000),
    BitBoard(0x0000_0000_0002_0000),
    BitBoard(0x0000_0000_0005_0000),
    BitBoard(0x0000_0000_000a_0000),
    BitBoard(0x0000_0000_0014_0000),
    BitBoard(0x0000_0000_0028_0000),
    BitBoard(0x0000_0000_0050_0000),
    BitBoard(0x0000_0000_00a0_0000),
    BitBoard(0x0000_0000_0040_0000),
    BitBoard(0x0000_0000_0200_0000),
    BitBoard(0x0000_0000_0500_0000),
    BitBoard(0x0000_0000_0a00_0000),
    BitBoard(0x0000_0000_1400_0000),
    BitBoard(0x0000_0000_2800_0000),
    BitBoard(0x0000_0000_5000_0000),
    BitBoard(0x0000_0000_a000_0000),
    BitBoard(0x0000_0000_4000_0000),
    BitBoard(0x0000_0002_0000_0000),
    BitBoard(0x0000_0005_0000_0000),
    BitBoard(0x0000_000a_0000_0000),
    BitBoard(0x0000_0014_0000_0000),
    BitBoard(0x0000_0028_0000_0000),
    BitBoard(0x0000_0050_0000_0000),
    BitBoard(0x0000_00a0_0000_0000),
    BitBoard(0x0000_0040_0000_0000),
    BitBoard(0x0000_0200_0000_0000),
    BitBoard(0x0000_0500_0000_0000),
    BitBoard(0x0000_0a00_0000_0000),
    BitBoard(0x0000_1400_0000_0000),
    BitBoard(0x0000_2800_0000_0000),
    BitBoard(0x0000_5000_0000_0000),
    BitBoard(0x0000_a000_0000_0000),
    BitBoard(0x0000_4000_0000_0000),
    BitBoard(0x0002_0000_0000_0000),
    BitBoard(0x0005_0000_0000_0000),
    BitBoard(0x000a_0000_0000_0000),
    BitBoard(0x0014_0000_0000_0000),
    BitBoard(0x0028_0000_0000_0000),
    BitBoard(0x0050_0000_0000_0000),
    BitBoard(0x00a0_0000_0000_0000),
    BitBoard(0x0040_0000_0000_0000),
    BitBoard(0x0200_0000_0000_0000),
    BitBoard(0x0500_0000_0000_0000),
    BitBoard(0x0a00_0000_0000_0000),
    BitBoard(0x1400_0000_0000_0000),
    BitBoard(0x2800_0000_0000_0000),
    BitBoard(0x5000_0000_0000_0000),
    BitBoard(0xa000_0000_0000_0000),
    BitBoard(0x4000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
];

pub const BLACK_PAWN_ATTACKS: [BitBoard; 64] = [
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0000),
    BitBoard(0x0000_0000_0000_0002),
    BitBoard(0x0000_0000_0000_0005),
    BitBoard(0x0000_0000_0000_000a),
    BitBoard(0x0000_0000_0000_0014),
    BitBoard(0x0000_0000_0000_0028),
    BitBoard(0x0000_0000_0000_0050),
    BitBoard(0x0000_0000_0000_00a0),
    BitBoard(0x0000_0000_0000_0040),
    BitBoard(0x0000_0000_0000_0200),
    BitBoard(0x0000_0000_0000_0500),
    BitBoard(0x0000_0000_0000_0a00),
    BitBoard(0x0000_0000_0000_1400),
    BitBoard(0x0000_0000_0000_2800),
    BitBoard(0x0000_0000_0000_5000),
    BitBoard(0x0000_0000_0000_a000),
    BitBoard(0x0000_0000_0000_4000),
    BitBoard(0x0000_0000_0002_0000),
    BitBoard(0x0000_0000_0005_0000),
    BitBoard(0x0000_0000_000a_0000),
    BitBoard(0x0000_0000_0014_0000),
    BitBoard(0x0000_0000_0028_0000),
    BitBoard(0x0000_0000_0050_0000),
    BitBoard(0x0000_0000_00a0_0000),
    BitBoard(0x0000_0000_0040_0000),
    BitBoard(0x0000_0000_0200_0000),
    BitBoard(0x0000_0000_0500_0000),
    BitBoard(0x0000_0000_0a00_0000),
    BitBoard(0x0000_0000_1400_0000),
    BitBoard(0x0000_0000_2800_0000),
    BitBoard(0x0000_0000_5000_0000),
    BitBoard(0x0000_0000_a000_0000),
    BitBoard(0x0000_0000_4000_0000),
    BitBoard(0x0000_0002_0000_0000),
    BitBoard(0x0000_0005_0000_0000),
    BitBoard(0x0000_000a_0000_0000),
    BitBoard(0x0000_0014_0000_0000),
    BitBoard(0x0000_0028_0000_0000),
    BitBoard(0x0000_0050_0000_0000),
    BitBoard(0x0000_00a0_0000_0000),
    BitBoard(0x0000_0040_0000_0000),
    BitBoard(0x0000_0200_0000_0000),
    BitBoard(0x0000_0500_0000_0000),
    BitBoard(0x0000_0a00_0000_0000),
    BitBoard(0x0000_1400_0000_0000),
    BitBoard(0x0000_2800_0000_0000),
    BitBoard(0x0000_5000_0000_0000),
    BitBoard(0x0000_a000_0000_0000),
    BitBoard(0x0000_4000_0000_0000),
    BitBoard(0x0002_0000_0000_0000),
    BitBoard(0x0005_0000_0000_0000),
    BitBoard(0x000a_0000_0000_0000),
    BitBoard(0x0014_0000_0000_0000),
    BitBoard(0x0028_0000_0000_0000),
    BitBoard(0x0050_0000_0000_0000),
    BitBoard(0x00a0_0000_0000_0000),
    BitBoard(0x0040_0000_0000_0000),
];

pub const PAWN_ATTACKS: [[BitBoard; 64]; 2] = [WHITE_PAWN_ATTACKS, BLACK_PAWN_ATTACKS];

pub const KNIGHT_ATTACKS: [BitBoard; 64] = [
    BitBoard(0x0000_0000_0002_0400),
    BitBoard(0x0000_0000_0005_0800),
    BitBoard(0x0000_0000_000a_1100),
    BitBoard(0x0000_0000_0014_2200),
    BitBoard(0x0000_0000_0028_4400),
    BitBoard(0x0000_0000_0050_8800),
    BitBoard(0x0000_0000_00a0_1000),
    BitBoard(0x0000_0000_0040_2000),
    BitBoard(0x0000_0000_0204_0004),
    BitBoard(0x0000_0000_0508_0008),
    BitBoard(0x0000_0000_0a11_0011),
    BitBoard(0x0000_0000_1422_0022),
    BitBoard(0x0000_0000_2844_0044),
    BitBoard(0x0000_0000_5088_0088),
    BitBoard(0x0000_0000_a010_0010),
    BitBoard(0x0000_0000_4020_0020),
    BitBoard(0x0000_0002_0400_0402),
    BitBoard(0x0000_0005_0800_0805),
    BitBoard(0x0000_000a_1100_110a),
    BitBoard(0x0000_0014_2200_2214),
    BitBoard(0x0000_0028_4400_4428),
    BitBoard(0x0000_0050_8800_8850),
    BitBoard(0x0000_00a0_1000_10a0),
    BitBoard(0x0000_0040_2000_2040),
    BitBoard(0x0000_0204_0004_0200),
    BitBoard(0x0000_0508_0008_0500),
    BitBoard(0x0000_0a11_0011_0a00),
    BitBoard(0x0000_1422_0022_1400),
    BitBoard(0x0000_2844_0044_2800),
    BitBoard(0x0000_5088_0088_5000),
    BitBoard(0x0000_a010_0010_a000),
    BitBoard(0x0000_4020_0020_4000),
    BitBoard(0x0002_0400_0402_0000),
    BitBoard(0x0005_0800_0805_0000),
    BitBoard(0x000a_1100_110a_0000),
    BitBoard(0x0014_2200_2214_0000),
    BitBoard(0x0028_4400_4428_0000),
    BitBoard(0x0050_8800_8850_0000),
    BitBoard(0x00a0_1000_10a0_0000),
    BitBoard(0x0040_2000_2040_0000),
    BitBoard(0x0204_0004_0200_0000),
    BitBoard(0x0508_0008_0500_0000),
    BitBoard(0x0a11_0011_0a00_0000),
    BitBoard(0x1422_0022_1400_0000),
    BitBoard(0x2844_0044_2800_0000),
    BitBoard(0x5088_0088_5000_0000),
    BitBoard(0xa010_0010_a000_0000),
    BitBoard(0x4020_0020_4000_0000),
    BitBoard(0x0400_0402_0000_0000),
    BitBoard(0x0800_0805_0000_0000),
    BitBoard(0x1100_110a_0000_0000),
    BitBoard(0x2200_2214_0000_0000),
    BitBoard(0x4400_4428_0000_0000),
    BitBoard(0x8800_8850_0000_0000),
    BitBoard(0x1000_10a0_0000_0000),
    BitBoard(0x2000_2040_0000_0000),
    BitBoard(0x0004_0200_0000_0000),
    BitBoard(0x0008_0500_0000_0000),
    BitBoard(0x0011_0a00_0000_0000),
    BitBoard(0x0022_1400_0000_0000),
    BitBoard(0x0044_2800_0000_0000),
    BitBoard(0x0088_5000_0000_0000),
    BitBoard(0x0010_a000_0000_0000),
    BitBoard(0x0020_4000_0000_0000),
];

pub const KING_ATTACKS: [BitBoard; 64] = [
    BitBoard(0x0000_0000_0000_0302),
    BitBoard(0x0000_0000_0000_0705),
    BitBoard(0x0000_0000_0000_0e0a),
    BitBoard(0x0000_0000_0000_1c14),
    BitBoard(0x0000_0000_0000_3828),
    BitBoard(0x0000_0000_0000_7050),
    BitBoard(0x0000_0000_0000_e0a0),
    BitBoard(0x0000_0000_0000_c040),
    BitBoard(0x0000_0000_0003_0203),
    BitBoard(0x0000_0000_0007_0507),
    BitBoard(0x0000_0000_000e_0a0e),
    BitBoard(0x0000_0000_001c_141c),
    BitBoard(0x0000_0000_0038_2838),
    BitBoard(0x0000_0000_0070_5070),
    BitBoard(0x0000_0000_00e0_a0e0),
    BitBoard(0x0000_0000_00c0_40c0),
    BitBoard(0x0000_0000_0302_0300),
    BitBoard(0x0000_0000_0705_0700),
    BitBoard(0x0000_0000_0e0a_0e00),
    BitBoard(0x0000_0000_1c14_1c00),
    BitBoard(0x0000_0000_3828_3800),
    BitBoard(0x0000_0000_7050_7000),
    BitBoard(0x0000_0000_e0a0_e000),
    BitBoard(0x0000_0000_c040_c000),
    BitBoard(0x0000_0003_0203_0000),
    BitBoard(0x0000_0007_0507_0000),
    BitBoard(0x0000_000e_0a0e_0000),
    BitBoard(0x0000_001c_141c_0000),
    BitBoard(0x0000_0038_2838_0000),
    BitBoard(0x0000_0070_5070_0000),
    BitBoard(0x0000_00e0_a0e0_0000),
    BitBoard(0x0000_00c0_40c0_0000),
    BitBoard(0x0000_0302_0300_0000),
    BitBoard(0x0000_0705_0700_0000),
    BitBoard(0x0000_0e0a_0e00_0000),
    BitBoard(0x0000_1c14_1c00_0000),
    BitBoard(0x0000_3828_3800_0000),
    BitBoard(0x0000_7050_7000_0000),
    BitBoard(0x0000_e0a0_e000_0000),
    BitBoard(0x0000_c040_c000_0000),
    BitBoard(0x0003_0203_0000_0000),
    BitBoard(0x0007_0507_0000_0000),
    BitBoard(0x000e_0a0e_0000_0000),
    BitBoard(0x001c_141c_0000_0000),
    BitBoard(0x0038_2838_0000_0000),
    BitBoard(0x0070_5070_0000_0000),
    BitBoard(0x00e0_a0e0_0000_0000),
    BitBoard(0x00c0_40c0_0000_0000),
    BitBoard(0x0302_0300_0000_0000),
    BitBoard(0x0705_0700_0000_0000),
    BitBoard(0x0e0a_0e00_0000_0000),
    BitBoard(0x1c14_1c00_0000_0000),
    BitBoard(0x3828_3800_0000_0000),
    BitBoard(0x7050_7000_0000_0000),
    BitBoard(0xe0a0_e000_0000_0000),
    BitBoard(0xc040_c000_0000_0000),
    BitBoard(0x0203_0000_0000_0000),
    BitBoard(0x0507_0000_0000_0000),
    BitBoard(0x0a0e_0000_0000_0000),
    BitBoard(0x141c_0000_0000_0000),
    BitBoard(0x2838_0000_0000_0000),
    BitBoard(0x5070_0000_0000_0000),
    BitBoard(0xa0e0_0000_0000_0000),
    BitBoard(0x40c0_0000_0000_0000),
];

#[cfg(test)]
// For generating constants
mod gen {
    // use crate::BitBoard;
    // use std::array;
    //
    // #[test]
    // fn gen_white_pawns() {
    //     let white_pawns = array::from_fn::<_, 64, _>(|index| {
    //         let pos = 1 << index;
    //         let board = BitBoard(pos);
    //
    //         board.north_west() | board.north_east()
    //     });
    //
    //     // for board in white_pawns {
    //     //     println!("{board:#}");
    //     // }
    //
    //     panic!("{white_pawns:#016x?}");
    // }
    //
    // #[test]
    // fn gen_black_pawns() {
    //     let black_pawns = array::from_fn::<_, 64, _>(|index| {
    //         let pos = 1 << index;
    //         let board = BitBoard(pos);
    //
    //         board.south_west() | board.south_east()
    //     });
    //
    //     // for board in black_pawns {
    //     //     println!("{board:#}");
    //     // }
    //
    //     panic!("{black_pawns:#016x?}");
    // }
    //
    // fn knight_attacks(board: BitBoard) -> BitBoard {
    //     let top_left = board.north().north_west() | board.west().north_west();
    //     let top_right = board.north().north_east() | board.east().north_east();
    //     let bottom_left = board.south().south_west() | board.west().south_west();
    //     let bottom_right = board.south().south_east() | board.east().south_east();
    //
    //     top_left | top_right | bottom_left | bottom_right
    // }
    //
    // #[test]
    // fn gen_knights() {
    //     let knights = array::from_fn::<_, 64, _>(|index| {
    //         let pos = 1 << index;
    //         let board = BitBoard(pos);
    //         knight_attacks(board)
    //     });
    //
    //     // for knight in knights {
    //     //     println!("{knight:#}");
    //     // }
    //
    //     panic!("{knights:#016x?}")
    // }
    //
    // #[test]
    // fn gen_kings() {
    //     let kings = array::from_fn::<_, 64, _>(|index| {
    //         let pos = 1 << index;
    //         let board = BitBoard(pos);
    //         board.north()
    //             | board.south()
    //             | board.west()
    //             | board.east()
    //             | board.north_west()
    //             | board.north_east()
    //             | board.south_west()
    //             | board.south_east()
    //     });
    //
    //     // for king in kings {
    //     //     println!("{king:#}");
    //     // }
    //
    //     panic!("{kings:#016x?}")
    // }
}
