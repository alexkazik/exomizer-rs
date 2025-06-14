#[cfg(target_pointer_width = "16")]
type InputBuffer = u32;
#[cfg(not(target_pointer_width = "16"))]
pub(super) type InputBuffer = usize;

pub(super) struct Input<I>
where
    I: Iterator<Item = u8>,
{
    pub(super) src: I,
    pub(super) bit_buffer: InputBuffer,
    pub(super) bit_buffer_length: usize,
}

impl<I> Input<I>
where
    I: Iterator<Item = u8>,
{
    pub(super) fn read_bits(&mut self, bits: usize) -> Result<usize, DecrunchError> {
        if bits == 0 {
            return Ok(0);
        }
        #[allow(clippy::cast_possible_truncation)]
        while self.bit_buffer_length < bits {
            self.bit_buffer_length += 8;
            self.bit_buffer |= InputBuffer::from(self.src.next().ok_or(DecrunchError::EndOfInput)?)
                << (InputBuffer::BITS as InputBuffer - self.bit_buffer_length as InputBuffer);
        }
        #[allow(clippy::pedantic)]
        let r = self.bit_buffer >> (InputBuffer::BITS as InputBuffer - bits as InputBuffer);
        self.bit_buffer <<= bits;
        self.bit_buffer_length -= bits;

        #[allow(clippy::unnecessary_cast)]
        Ok(r as usize)
    }

    pub(super) fn read_n_bits<const N: usize>(&mut self) -> Result<usize, DecrunchError> {
        #[allow(clippy::cast_possible_truncation)]
        while self.bit_buffer_length < N {
            self.bit_buffer_length += 8;
            self.bit_buffer |= InputBuffer::from(self.src.next().ok_or(DecrunchError::EndOfInput)?)
                << (InputBuffer::BITS as InputBuffer - self.bit_buffer_length as InputBuffer);
        }
        #[allow(clippy::pedantic)]
        let r = self.bit_buffer >> (InputBuffer::BITS as InputBuffer - N as InputBuffer);
        self.bit_buffer <<= N;
        self.bit_buffer_length -= N;

        #[allow(clippy::unnecessary_cast)]
        Ok(r as usize)
    }

    #[inline]
    pub(super) fn read_bool(&mut self) -> Result<bool, DecrunchError> {
        Ok(self.read_n_bits::<1>()? != 0)
    }

    #[cfg(feature = "clz")]
    pub(super) fn read_next_one(&mut self) -> Result<usize, DecrunchError> {
        let mut result = 0;
        let mut leading;
        loop {
            leading = self.bit_buffer.leading_zeros() as usize;
            if leading < self.bit_buffer_length {
                break;
            }
            // all bits in the buffer are 0, add them to the result and refill it with 8 new bits
            result += self.bit_buffer_length;
            self.bit_buffer_length = 8;
            self.bit_buffer = InputBuffer::from(self.src.next().ok_or(DecrunchError::EndOfInput)?)
                << (InputBuffer::BITS as InputBuffer - 8);
        }
        result += leading;

        leading += 1; // also remove the '1' after (all the) '0's
        self.bit_buffer <<= leading;
        self.bit_buffer_length -= leading;

        Ok(result)
    }
}

pub(super) struct Tables {
    pub(super) base: [u16; 68],
    pub(super) bits: [u8; 68],
}

macro_rules! decr {
    ($src:ident, $dst:ident, $IMPL_1LITERAL:expr, $FOUR_OFFSET_TABLES:expr, $REUSE_OFFSET:expr) => {
        let mut input = Input {
            src: $src.into_iter(),
            bit_buffer: 0,
            bit_buffer_length: 0,
        };

        // Init-Table
        let mut tables = Tables {
            base: [0; 68],
            bits: [0; 68],
        };

        let max_i: usize = if $FOUR_OFFSET_TABLES { 68 } else { 52 };
        let mut b2 = 1;
        for i in 0..max_i {
            if (i & 15) == 0 {
                b2 = 1;
            }
            tables.base[i] = b2;

            let b1 = input.read_n_bits::<4>()?;
            #[allow(clippy::cast_possible_truncation)]
            {
                tables.bits[i] = b1 as u8;
            }

            b2 = b2
                .checked_add(1 << b1)
                .ok_or(DecrunchError::DecodingError)?;
        }

        // output pointer (from the back)
        // Satefy: the pointer is at the end, and thus valid
        let mut out = unsafe { $dst.as_mut_ptr().add($dst.len()) };
        let mut out_free_space = $dst.len();
        let mut max_offset = 0;

        let mut impl_1literal = $IMPL_1LITERAL;
        let mut reuse_offset_state: u8 = 0;
        let mut literal = true;
        let mut offset = 0;
        loop {
            reuse_offset_state <<= 1;
            reuse_offset_state |= u8::from(literal);

            literal = if impl_1literal {
                impl_1literal = false;
                true
            } else {
                input.read_bool()?
            };
            let length;
            if literal {
                length = 1;
            } else {
                let index;
                #[cfg(feature = "clz")]
                {
                    index = input.read_next_one()?;
                }
                #[cfg(not(feature = "clz"))]
                {
                    let mut index_read = 0;
                    while !input.read_bool()? {
                        index_read += 1;
                    }
                    index = index_read;
                }
                if index == 16 {
                    return if input.src.next().is_some() {
                        // it is an error if there are still bytes left in the input buffer
                        Err(DecrunchError::UnusedInput)
                    } else {
                        // Safety: out is part of dst (always aligned to the end) and thus valid
                        #[allow(clippy::cast_sign_loss)]
                        Ok(unsafe {
                            &mut *slice_from_raw_parts_mut(
                                out,
                                $dst.len() - (out.offset_from($dst.as_ptr()) as usize),
                            )
                        })
                    };
                } else if index == 17 {
                    literal = true;
                    length = input.read_n_bits::<16>()?;
                } else {
                    length = (tables.base[index] as usize)
                        + input.read_bits(tables.bits[index] as usize)?;
                    let mut reuse_offset = false;
                    if $REUSE_OFFSET && (reuse_offset_state & 3) == 1 {
                        reuse_offset = input.read_bool()?;
                    }
                    if !reuse_offset {
                        let index;
                        if $FOUR_OFFSET_TABLES {
                            match length {
                                1 => {
                                    index = input.read_n_bits::<2>()? + 64;
                                }
                                2 => {
                                    index = input.read_n_bits::<4>()? + 48;
                                }
                                3 => {
                                    index = input.read_n_bits::<4>()? + 32;
                                }
                                _ => {
                                    index = input.read_n_bits::<4>()? + 16;
                                }
                            }
                        } else {
                            match length {
                                1 => {
                                    index = input.read_n_bits::<2>()? + 48;
                                }
                                2 => {
                                    index = input.read_n_bits::<4>()? + 32;
                                }
                                _ => {
                                    index = input.read_n_bits::<4>()? + 16;
                                }
                            }
                        }
                        offset = (tables.base[index] as usize)
                            + input.read_bits(tables.bits[index] as usize)?;
                    }
                }
            }
            // check output buffer overrun
            if out_free_space < length {
                return Err(DecrunchError::BufferOverflow);
            }
            out_free_space -= length;

            // check offset
            if !literal && offset > max_offset {
                return Err(DecrunchError::DecodingError);
            }

            // output data
            for _ in 0..length {
                out = unsafe { out.sub(1) };
                if literal {
                    // Safety: out is a pointer within dst
                    unsafe { out.write(input.src.next().ok_or(DecrunchError::EndOfInput)?) };
                } else {
                    // Safety: out is a pointer within dst and offset is also valid
                    unsafe { out.write(out.add(offset).read()) };
                }
            }

            // increase max_offset by the copied data
            max_offset += length;
        }
    };
}

use crate::simple::DecrunchError;
pub(super) use decr;
