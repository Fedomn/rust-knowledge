/// The memory bus is byte-addressed, and processors operate on register words, which are typically four to eight bytes, or even wider.
/// This means that when programmers wish to operate on individual bits within a byte of memory or a word of register, they have to do so manually,
/// using shift and mask operations that are likely familiar to anyone who has done this before.
///
/// bitvec enables you to write code for bit-addressed memory that is simple, easy, and fast. It compiles to the same,
/// or even better, object code than you would get from writing shift/mask instructions manually.
///
/// Uses of bitvec generally fall into three major genres.
/// 1. compact, fast, usize => bit collections
/// 2. truncated integer storage
/// 3. precise control of memory layout
///

#[cfg(test)]
pub mod btivec_test {

    use bitvec::prelude::*;

    #[test]
    fn btivec_basic_test() {
        // [bool; N] becomes BitArray
        // bitvec provides two orderings: Lsb0 and Msb0. These each refer to which numeric bit in a register element is considered to be the zero-index.
        //
        // lsb0: sets the zero index at the least significant bit of a register
        // bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0
        //
        // msb0: sets the zero index at the most significant bit of a register
        // lit0 bit1 bit2 bit3 bit4 bit5 bit6 bit7
        let arr = bitarr![u8, Lsb0; 0, 1, 1, 0]; // Lsb0; 后面就是正常的array声明语法
        println!("{:?}", arr);

        let mut data = [1u8, 1, 1];
        let bits = data.view_bits_mut::<Lsb0>();
        println!("{:?}", bits);

        let b = BitVec::<u8, Lsb0>::repeat(false, 8);
        println!("{:?}", b);
    }
}
