// Taken from https://bitbucket.org/magli143/exomizer/wiki/Home
pub const DEMO_UNCOMPRESSED: &[u8] = br##"Exomizer is a program that compresses files in a way that tries to be as efficient as possible but still allows them to be decompressed in environments where CPU speed and RAM are limited. For some popular 8-bit computers using 6502 compatible CPUs it can also generate executable files that decompress themselves in memory when run.

The exomizer program itself is a console application written in ANSI-C.

Exomizer operates in two major modes. It has a raw mode that compresses plain files and produces plain files. This mode is used by the raw subcommand. All other subcommands use the second mode. It requires all source files to be targeted to be decrunched at specific addresses in the 16-bit address space of the target computer. In this mode, no file can be larger than 64kB since all data must be addressable using 16-bits. The targeted mode has the following features:

It reads plain or .prg files to any given address.
It can produce stand-alone self-decrunching files for the following targets:
Commodore VIC20, C64, C65, C16/plus4, C128 and PET 4032
Atari 400/800 XL/XE
Apple ][+ and //e
Oric-1 and Oric Atmos
BBC Micro B
It can produce files for both "in memory" and "from disk" decrunching.
It handles RLE-sequences well, no packer is necessary.
It is able to link/combine more than one source file into the same crunched target file.
Included in the downloadable zip file are the source code and a precompiled binary for Win32. It also includes a makefile for Gnu make and gcc so it should be easy to build on any system where these tools are available.

Any suggestions, comments and/or bug reports can be sent to me, the author.
"##;

pub const DEMO_COMPRESSED_P61_FAST: &[u8] = &[
    16, 17, 1, 2, 0, 0, 0, 0, 68, 69, 85, 85, 86, 118, 103, 137, 83, 52, 52, 69, 69, 117, 88, 122,
    50, 18, 17, 67, 53, 102, 87, 55, 33, 34, 10, 255, 46, 114, 111, 104, 116, 117, 97, 32, 176,
    101, 168, 232, 44, 169, 109, 43, 216, 100, 52, 110, 155, 115, 27, 98, 73, 81, 97, 168, 99, 84,
    185, 180, 211, 112, 81, 169, 217, 103, 117, 172, 93, 47, 100, 164, 132, 146, 110, 80, 45, 180,
    99, 5, 106, 147, 167, 105, 36, 186, 103, 204, 206, 13, 121, 55, 65, 10, 115, 101, 189, 108, 98,
    97, 21, 105, 100, 118, 175, 114, 34, 43, 36, 111, 172, 230, 197, 140, 131, 216, 90, 119, 209,
    109, 56, 50, 121, 40, 83, 57, 42, 139, 100, 60, 43, 196, 83, 70, 97, 101, 37, 11, 58, 117, 213,
    186, 215, 85, 89, 105, 76, 45, 99, 2, 103, 199, 18, 26, 101, 107, 55, 96, 82, 117, 164, 71,
    247, 177, 102, 28, 68, 134, 32, 89, 84, 235, 100, 117, 241, 99, 110, 21, 168, 94, 226, 210, 73,
    123, 46, 50, 51, 44, 87, 98, 107, 121, 84, 236, 98, 174, 75, 74, 112, 100, 234, 100, 220, 32,
    143, 137, 50, 176, 136, 178, 114, 117, 208, 19, 25, 164, 140, 112, 232, 112, 117, 122, 19, 182,
    100, 131, 111, 71, 110, 163, 119, 80, 49, 210, 200, 168, 36, 172, 73, 72, 89, 29, 116, 108,
    187, 134, 147, 91, 103, 105, 165, 117, 149, 42, 109, 120, 207, 6, 134, 146, 131, 34, 5, 26,
    241, 70, 210, 52, 146, 171, 109, 20, 195, 24, 13, 47, 107, 135, 145, 108, 49, 166, 157, 214,
    225, 171, 44, 6, 104, 173, 3, 142, 219, 4, 32, 181, 225, 213, 237, 97, 112, 177, 147, 44, 108,
    64, 181, 235, 178, 24, 123, 227, 117, 113, 77, 79, 45, 69, 76, 178, 82, 185, 12, 100, 151, 6,
    102, 180, 103, 208, 37, 163, 178, 251, 32, 34, 107, 146, 48, 174, 138, 93, 102, 75, 28, 89, 57,
    121, 59, 14, 170, 182, 198, 107, 171, 22, 70, 98, 153, 33, 115, 153, 219, 117, 100, 115, 112,
    26, 237, 41, 168, 66, 153, 21, 105, 105, 77, 210, 67, 144, 45, 180, 115, 93, 116, 65, 91, 46,
    114, 79, 51, 166, 49, 45, 32, 174, 10, 101, 47, 128, 97, 220, 43, 91, 93, 207, 168, 112, 15,
    244, 250, 69, 88, 47, 76, 37, 180, 48, 10, 56, 67, 7, 52, 73, 149, 105, 88, 105, 125, 72, 43,
    84, 69, 140, 80, 170, 56, 159, 49, 187, 67, 167, 52, 115, 184, 117, 24, 47, 54, 130, 168, 53,
    196, 30, 32, 77, 48, 50, 44, 73, 86, 83, 27, 171, 142, 230, 45, 225, 10, 58, 115, 210, 35, 69,
    159, 7, 156, 50, 243, 118, 1, 111, 128, 197, 169, 45, 239, 77, 142, 148, 63, 226, 176, 183, 83,
    116, 64, 23, 228, 46, 173, 235, 232, 100, 7, 121, 20, 113, 118, 105, 103, 107, 99, 147, 4, 238,
    220, 73, 46, 120, 213, 171, 97, 87, 237, 45, 185, 187, 17, 136, 190, 24, 149, 117, 101, 34, 64,
    86, 36, 212, 227, 148, 153, 97, 215, 48, 218, 45, 116, 84, 55, 69, 169, 127, 42, 45, 215, 232,
    139, 30, 32, 118, 176, 42, 51, 209, 236, 116, 131, 200, 92, 217, 122, 78, 96, 145, 146, 17,
    105, 239, 66, 107, 64, 133, 140, 73, 114, 51, 169, 65, 234, 145, 49, 7, 123, 13, 40, 254, 0,
    181, 241, 233, 109, 185, 76, 117, 78, 2, 15, 92, 141, 117, 102, 176, 111, 199, 103, 28, 212,
    20, 208, 75, 48, 119, 147, 80, 34, 246, 99, 158, 33, 122, 196, 144, 117, 97, 159, 1, 101, 168,
    135, 214, 128, 189, 195, 14, 128, 115, 183, 224, 219, 197, 28, 105, 183, 193, 33, 244, 208, 82,
    75, 222, 110, 80, 31, 59, 154, 104, 136, 230, 197, 228, 138, 98, 99, 75, 180, 14, 239, 19, 107,
    65, 60, 1, 21, 165, 119, 185, 57, 42, 249, 11, 178, 137, 137, 234, 224, 195, 77, 136, 106, 16,
    106, 120, 118, 72, 243, 224, 17, 128, 223, 129, 74, 38, 190, 12, 222, 52, 57, 28, 62, 72, 41,
    198, 38, 254, 100, 106, 189, 14, 74, 119, 26, 82, 80, 219, 30, 130, 94, 99, 233, 122, 238, 75,
    120, 159, 69, 100, 255, 67, 45, 73, 83, 78, 65, 16, 234, 38, 206, 129, 61, 114, 119, 72, 189,
    252, 249, 163, 61, 44, 94, 68, 122, 0, 210, 98, 188, 101, 130, 212, 231, 215, 203, 109, 75,
    101, 103, 22, 9, 209, 49, 96, 37, 64, 219, 74, 35, 238, 40, 125, 72, 52, 252, 118, 140, 153,
    12, 160, 22, 241, 199, 141, 192, 55, 224, 242, 135, 242, 154, 138, 65, 178, 173, 110, 208, 126,
    247, 11, 124, 148, 28, 85, 80, 67, 68, 134, 32, 113, 191, 50, 48, 53, 54, 15, 81, 162, 7, 18,
    29, 237, 56, 70, 118, 237, 152, 189, 11, 126, 208, 115, 232, 229, 70, 153, 109, 78, 201, 119,
    128, 254, 25, 77, 199, 65, 82, 68, 218, 69, 131, 149, 208, 255, 114, 63, 250, 188, 83, 250,
    150, 118, 14, 23, 197, 192, 219, 1, 226, 33, 59, 143, 122, 49, 204, 27, 172, 206, 223, 31, 165,
    132, 196, 111, 11, 157, 38, 178, 141, 105, 221, 180, 9, 136, 48, 29, 12, 246, 154, 97, 70, 104,
    121, 157, 32, 164, 177, 135, 33, 0, 54, 72, 52, 116, 116, 196, 26, 206, 0, 0, 128,
];
