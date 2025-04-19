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

    use std::convert::TryFrom;
    use std::fmt::Debug;
    use std::{iter::FromIterator, simd::*};

    use bitvec::prelude::*;

    #[test]
    fn bitvec_basic_test() {
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

    #[test]
    fn bitvec_simd_test() {
        trait NativeType: Debug + Default + PartialEq {}
        impl NativeType for u8 {}

        #[derive(Debug, PartialEq, Eq)]
        struct BatchItem<T, const N: usize>
        where
            T: SimdElement + NativeType,
            LaneCount<N>: SupportedLaneCount,
        {
            data: Simd<T, N>,
            valid: usize,
            len: usize,
        }

        struct Array<T> {
            data: Vec<T>,
            valid: BitVec,
        }

        // Enable `collect()` an array from iterator of `Option<T>`.
        impl<T> FromIterator<Option<T>> for Array<T>
        where
            T: NativeType,
        {
            fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
                let iter = iter.into_iter();
                let mut data = Vec::with_capacity(iter.size_hint().0);
                let mut valid = BitVec::with_capacity(iter.size_hint().0);
                for e in iter {
                    valid.push(e.is_some());
                    data.push(e.unwrap_or_default());
                }
                Array { data, valid }
            }
        }

        // Returns a batch iterator for SIMD.
        impl<T> Array<T> {
            fn batch_iter<const N: usize>(&self) -> BatchIter<T, N> {
                BatchIter {
                    array: self,
                    idx: 0,
                }
            }
        }

        struct BatchIter<'a, T, const N: usize> {
            array: &'a Array<T>,
            idx: usize,
        }

        impl<T, const N: usize> Iterator for BatchIter<'_, T, N>
        where
            T: SimdElement + NativeType,
            LaneCount<N>: SupportedLaneCount,
        {
            type Item = BatchItem<T, N>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.idx >= self.array.valid.len() {
                    return None;
                }
                let len = (self.array.valid.len() - self.idx).min(N);
                let range = self.idx..self.idx + len;

                let mut valid = [0u8; std::mem::size_of::<usize>()];
                let bytes = (len + 7) >> 3;
                valid[..bytes].copy_from_slice(unsafe {
                    std::slice::from_raw_parts(
                        (self.array.valid.as_bitptr().pointer() as *const u8).add(self.idx >> 3),
                        bytes,
                    )
                });
                let valid = usize::from_le_bytes(valid);

                let data = if len == N {
                    <[T; N]>::try_from(&self.array.data[range]).unwrap().into()
                } else {
                    let mut data = Simd::<T, N>::default();
                    data.as_mut_array()[..len].copy_from_slice(&self.array.data[range]);
                    data
                };

                self.idx += N;
                Some(BatchItem { data, valid, len })
            }
        }

        let a = (0..12)
            .map(|i| if i % 2 == 0 { Some(i) } else { None })
            .collect::<Array<u8>>();
        let mut iter = a.batch_iter::<8>();
        assert_eq!(
            iter.next(),
            Some(BatchItem {
                valid: 0b_0101_0101, // 二进制
                data: [0, 0, 2, 0, 4, 0, 6, 0].into(),
                len: 8
            })
        );

        assert_eq!(
            iter.next(),
            Some(BatchItem {
                valid: 0b_0000_0101,
                data: [8, 0, 10, 0, 0, 0, 0, 0].into(),
                len: 4
            })
        );
        assert_eq!(iter.next(), None);
    }
}
