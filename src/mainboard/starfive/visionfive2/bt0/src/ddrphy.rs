use crate::init::{read32, write32};

// drivers/ram/starfive/ddrphy_train.c
pub const TRAIN_DATA: [u32; 363] = [
    0xb00,      //
    0x101,      //
    0x640000,   //
    0x1,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x1,        //
    0x7,        //
    0x10002,    //
    0x300080f,  //
    0x1,        //
    0x5,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x1010000,  //
    0x280a0000, //
    0x0,        //
    0x1,        //
    0x3200000f, //
    0x0,        //
    0x0,        //
    0x10102,    //
    0x1,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0xaa,       //
    0x55,       //
    0xb5,       //
    0x4a,       //
    0x56,       //
    0xa9,       //
    0xa9,       //
    0xb5,       //
    0x1000000,  //
    0x1000000,  //
    0x0,        //
    0xf0f0000,  //
    0x14,       //
    0x7d0,      //
    0x300,      //
    0x0,        //
    0x0,        //
    0x1000000,  //
    0x10101,    //
    0x0,        //
    0x30000,    //
    0x100,      //
    0x170f,     //
    0x0,        //
    0x0,        //
    0x0,        //
    0xa140a01,  //
    0x204010a,  //
    0x2080510,  //
    0x40400,    //
    0x1000101,  //
    0x10100,    //
    0x2040f00,  //
    0x34000000, //
    0x0,        //
    0x0,        //
    0x1000000,  //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x10100,    //
    0x80101,    //
    0x2000200,  //
    0x1000100,  //
    0x1000000,  //
    0x2000200,  //
    0x200,      //
    0x0,        //
    0x0,        //
    0x0,        //
    0xe000004,  //
    0xc0d100f,  //
    0xa09080b,  //
    0x2010000,  //
    0x80103,    //
    0x200,      //
    0x0,        //
    0xf000000,  //
    0x4,        //
    0xa,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x30100,    //
    0x1010001,  //
    0x10200,    //
    0x4000103,  //
    0x1050001,  //
    0x10600,    //
    0x107,      //
    0x0,        //
    0x0,        //
    0x10001,    //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x10000,    //
    0x4,        //
    0x0,        //
    0x10000,    //
    0x0,        //
    0x3c0003,   //
    0x80100a0,  //
    0x16,       //
    0x2c,       //
    0x33,       //
    0x20043,    //
    0x2000200,  //
    0x4,        //
    0x60c,      //
    0xa1400,    //
    0x280000,   //
    0x6,        //
    0x46,       //
    0x70,       //
    0x610,      //
    0x12b,      //
    0x4001035,  //
    0x1010404,  //
    0x1e01,     //
    0x1e001e,   //
    0x1000100,  //
    0x100,      //
    0x0,        //
    0x5060403,  //
    0x1011108,  //
    0x1010101,  //
    0xf0a0a,    //
    0x0,        //
    0x0,        //
    0x4000000,  //
    0x4021008,  //
    0x4020206,  //
    0xc0034,    //
    0x100038,   //
    0x17003f,   //
    0x10001,    //
    0x10001,    //
    0x10005,    //
    0x20064,    //
    0x100010b,  //
    0x60006,    //
    0x650100,   //
    0x1000065,  //
    0x10c010c,  //
    0x1e1a1e1a, //
    0x1011e1a,  //
    0xa070601,  //
    0xa07060d,  //
    0x100b080d, //
    0xc00f,     //
    0xc01000,   //
    0xc01000,   //
    0x21000,    //
    0x120005,   //
    0x190064,   //
    0x10b,      //
    0x1100,     //
    0x1e1a0056, //
    0x6000101,  //
    0x130204,   //
    0x1e1a0058, //
    0x1000101,  //
    0x230408,   //
    0x1e1a005e, //
    0x9000101,  //
    0x610,      //
    0x4040800,  //
    0x40100,    //
    0x3000277,  //
    0xa032001,  //
    0xa0a,      //
    0x80908,    //
    0x901,      //
    0x1100315c, //
    0xa062002,  //
    0xa0a,      //
    0x141708,   //
    0x150d,     //
    0x2d00838e, //
    0xf102004,  //
    0xf0b,      //
    0x8c,       //
    0x578,      //
    0xc20,      //
    0x7940,     //
    0x206a,     //
    0x14424,    //
    0x730006,   //
    0x3030133,  //
    0x4,        //
    0x0,        //
    0x4,        //
    0x1,        //
    0x5,        //
    0x2,        //
    0x6,        //
    0x50,       //
    0x1,        //
    0x5,        //
    0x28,       //
    0x73,       //
    0xd6,       //
    0x1,        //
    0x5,        //
    0x6b,       //
    0x1000133,  //
    0x140040,   //
    0x10001,    //
    0x1900040,  //
    0x1000c,    //
    0x42b0040,  //
    0x320,      //
    0x360014,   //
    0x1010101,  //
    0x2020101,  //
    0x8080404,  //
    0x67676767, //
    0x67676767, //
    0x67676767, //
    0x67676767, //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x5500,     //
    0x5a00,     //
    0x55003c,   //
    0x0,        //
    0x3c00005a, //
    0x5500,     //
    0x5a00,     //
    0x55003c,   //
    0x0,        //
    0x3c00005a, //
    0x18171615, //
    0x14131211, //
    0x7060504,  //
    0x3020100,  //
    0x0,        //
    0x0,        //
    0x0,        //
    0x1000000,  //
    0x4020201,  //
    0x80804,    //
    0x0,        //
    0x4,        //
    0x0,        //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x14,       //
    0x9,        //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x34,       //
    0x1b,       //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x4,        //
    0x0,        //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x14,       //
    0x9,        //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x34,       //
    0x1b,       //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x4,        //
    0x0,        //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x14,       //
    0x9,        //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x34,       //
    0x1b,       //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x4,        //
    0x0,        //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x14,       //
    0x9,        //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
    0x0,        //
    0x34,       //
    0x1b,       //
    0x31,       //
    0x31,       //
    0x0,        //
    0x0,        //
    0x4d4d,     //
];

// drivers/ram/starfive/ddrphy_utils.c
// This comes right after UTIL_DATA1, but needs to be written firts.
const UTIL_DATA2: [u32; 139] = [
    0x0,        //
    0x100,      //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x50000,    //
    0x4000000,  //
    0x55,       //
    0x0,        //
    0x0,        //
    0x0,        //
    0xf0001,    //
    0x280040,   //
    0x5002,     //
    0x10101,    //
    0x8008,     //
    0x81020,    //
    0x0,        //
    0x0,        //
    0x1000000,  //
    0x1,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x64,       //
    0x0,        //
    0x0,        //
    0x1010000,  //
    0x2020101,  //
    0x4040202,  //
    0x8080404,  //
    0xf0f0808,  //
    0xf0f0f0f,  //
    0x20200f0f, //
    0x1b428000, //
    0x4,        //
    0x1010000,  //
    0x1070501,  //
    0x54,       //
    0x4410,     //
    0x4410,     //
    0x4410,     //
    0x4410,     //
    0x4410,     //
    0x4410,     //
    0x4410,     //
    0x4410,     //
    0x4410,     //
    0x4410,     //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x64,       //
    0x0,        //
    0x108,      //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x3000000,  //
    0x0,        //
    0x0,        //
    0x0,        //
    0x4102035,  //
    0x41020,    //
    0x1c98c98,  //
    0x3f400000, //
    0x3f3f1f3f, //
    0x1f3f3f1f, //
    0x1f3f3f,   //
    0x0,        //
    0x0,        //
    0x1,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x76543210, //
    0x6010198,  //
    0x0,        //
    0x0,        //
    0x0,        //
    0x40700,    //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x2,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x0,        //
    0x1142,     //
    0x3020100,  //
    0x3000300,  //
    0x3000300,  //
    0x3000300,  //
    0x3000300,  //
    0x3000300,  //
    0x3000300,  //
    0x3000300,  //
    0x3000300,  //
    0x3000300,  //
    0x3000300,  //
    0x300,      //
    0x300,      //
    0x300,      //
    0x300,      //
    0x2,        //
    0x4011,     //
    0x4011,     //
    0x40,       //
    0x40,       //
    0x4011,     //
    0x1fff00,   //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x4011,     //
    0x1004011,  //
    0x200400,   //
];

pub fn train(base_addr: usize) {
    TRAIN_DATA.iter().enumerate().for_each(|(reg_nr, value)| {
        let addr = (base_addr + (reg_nr << 2));
        write32(addr, *value);
    });
}

const UTIL_DATA1_A1: [u32; 44] = [
    0x4f0, 0x0, 0x1030200, 0x0, 0x0, 0x3000000, 0x1000001, 0x3000400, 0x1000001, 0x0, 0x0,
    0x1000001, 0x0, 0xc00004, 0xcc0008, 0x660601, 0x3, 0x0, 0x1, 0xaaaa, 0x5555, 0xb5b5, 0x4a4a,
    0x5656, 0xa9a9, 0xa9a9, 0xb5b5, 0x0, 0x0, 0x8000000, 0x4000008, 0x408, 0xe4e400, 0x71020,
    0xc0020, 0x620, 0x100, 0x55555555, 0xaaaaaaaa, 0x55555555, 0xaaaaaaaa, 0x5555, 0x1000100,
    0x800180,
];

const UTIL_DATA1_A2: [u32; 86] = [
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x4, 0x20, 0x0, 0x0, 0x0, 0x0, 0x7ff0000, 0x20008008, 0x810, 0x40100, 0x0, 0x1880c01,
    0x2003880c, 0x20000125, 0x7ff0200, 0x101, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20000,
    0x51515052, 0x31c06000, 0x11f0004, 0xc0c001, 0x3000000, 0x30202, 0x42100010, 0x10c053e,
    0xf0c20, 0x1000140, 0xa30120, 0xc00, 0x210, 0x200, 0x2800000, 0x80800101, 0x3, 0x76543210, 0x8,
    0x2800280, 0x2800280, 0x2800280, 0x2800280, 0x280, 0x8000, 0x800080, 0x800080, 0x800080,
    0x800080, 0x800080, 0x800080, 0x800080, 0x800080, 0x6e0080, 0x1a00003, 0x0, 0x30000, 0x80200,
    0x0, 0x20202020, 0x20202020, 0x2020,
];

fn util_data1_a(base: usize, v1: u32) {
    UTIL_DATA1_A1
        .iter()
        .enumerate()
        .for_each(|(reg_nr, value)| {
            let addr = (base + (reg_nr << 2));
            write32(addr, *value);
        });
    write32(base + (44 << 2), v1);
    UTIL_DATA1_A2
        .iter()
        .enumerate()
        .for_each(|(reg_nr, value)| {
            let addr = (base + ((45 + reg_nr) << 2));
            write32(addr, *value);
        });
    for r in 131..256 {
        let addr = (base + (r << 2));
        write32(addr, 0);
    }
}

const UTIL_DATA1_B1: [u32; 30] = [
    0x0, 0x0, 0x0, 0x0, 0x0, 0x100, 0x200, 0x0, 0x0, 0x0, 0x0, 0x400000, 0x80, 0xdcba98, 0x3000000,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2a, 0x15, 0x15, 0x2a, 0x33, 0xc, 0xc, 0x33,
];

const UTIL_DATA1_B2: [u32; 16] = [
    0x20202000, 0x202020, 0x20008008, 0x810, 0x0, 0x255, 0x30000, 0x300, 0x300, 0x300, 0x300,
    0x300, 0x42080010, 0x33e, 0x1010002, 0x80,
];

fn util_data1_b(base: usize, v1: u32, v2: u32, v3: u32) {
    UTIL_DATA1_B1
        .iter()
        .enumerate()
        .for_each(|(reg_nr, value)| {
            let addr = (base + (reg_nr << 2));
            write32(addr, *value);
        });
    write32(base + (30 << 2), v1);
    write32(base + (31 << 2), v2);
    write32(base + (32 << 2), v3);
    UTIL_DATA1_B2
        .iter()
        .enumerate()
        .for_each(|(reg_nr, value)| {
            let addr = (base + ((33 + reg_nr) << 2));
            write32(addr, *value);
        });
    for r in 49..256 {
        let addr = (base + (r << 2));
        write32(addr, 0);
    }
}

// First, we write everything from register 1792 on, then everything up to 1792.
// We do not know why, just copied it from the reference implementation.
pub fn util(base_addr: usize) {
    UTIL_DATA2.iter().enumerate().for_each(|(reg_nr, value)| {
        let addr = (base_addr + ((1792 + reg_nr) << 2));
        write32(addr, *value);
    });
    // 4 blocks of 256 each
    util_data1_a(base_addr, 0x1);
    util_data1_a(base_addr + (256 << 2), 0x0);
    util_data1_a(base_addr + ((256 * 2) << 2), 0x1);
    util_data1_a(base_addr + ((256 * 3) << 2), 0x0);
    // 3 blocks of 256 each
    util_data1_b(base_addr + ((256 * 4) << 2), 0xa418820, 0x3f0000, 0x13f);
    util_data1_b(base_addr + ((256 * 5) << 2), 0x0, 0x0, 0x0);
    util_data1_b(base_addr + ((256 * 6) << 2), 0x0, 0x10000000, 0x0);
}
