use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use fastnbt::de::from_bytes;

use crate::{biome::Biome, Block, Chunk, HeightMode, JavaChunk, Palette, Rgba, TopShadeRenderer};

const CHUNK_1_17_0: &[u8] = include_bytes!("../../resources/1.17.0.chunk");
const CHUNK_1_17_1: &[u8] = include_bytes!("../../resources/1.17.1.chunk");
const CHUNK_CUSTOM_HEIGHTS_1_17_1: &[u8] =
    include_bytes!("../../resources/1.17.1-custom-heights.chunk");

/// A palette that colours blocks based on the hash of their full description.
/// Will produce gibberish looking maps but is great for testing rendering isn't
/// changing.
struct HashPalette;

impl Palette for HashPalette {
    fn pick(&self, block: &Block, _: Option<Biome>) -> Rgba {
        // Call methods just to exercise all the code.
        block.name();
        block.snowy();
        let hash = calculate_hash(block.encoded_description());
        let bytes = hash.to_be_bytes();
        [bytes[0], bytes[1], bytes[2], 255]
    }
}

fn exercise_render(chunk: &impl Chunk) -> [[u8; 4]; 256] {
    let palette = HashPalette;

    let renderer = TopShadeRenderer::new(&palette, HeightMode::Trust);
    renderer.render(chunk, None);

    let renderer = TopShadeRenderer::new(&palette, HeightMode::Calculate);
    renderer.render(chunk, None)
}

#[test]
fn chunk_1_17_0() {
    let expected = [
        [140, 81, 38, 255],
        [0, 127, 175, 255],
        [142, 206, 84, 255],
        [200, 27, 88, 255],
        [142, 206, 84, 255],
        [0, 127, 175, 255],
        [140, 81, 38, 255],
        [140, 81, 38, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [0, 127, 175, 255],
        [122, 177, 72, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [122, 177, 72, 255],
        [0, 127, 175, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [141, 19, 62, 255],
        [197, 140, 31, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [122, 177, 72, 255],
        [200, 27, 88, 255],
        [197, 140, 31, 255],
        [142, 206, 84, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [122, 177, 72, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [197, 140, 31, 255],
        [229, 163, 36, 255],
        [0, 133, 24, 255],
        [229, 163, 36, 255],
        [172, 23, 75, 255],
        [197, 140, 31, 255],
        [142, 206, 84, 255],
        [172, 23, 75, 255],
        [142, 206, 84, 255],
        [197, 140, 31, 255],
        [172, 23, 75, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [197, 140, 31, 255],
        [197, 140, 31, 255],
        [122, 177, 72, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [122, 177, 72, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [122, 177, 72, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [0, 109, 150, 255],
        [100, 145, 59, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [100, 145, 59, 255],
        [0, 109, 150, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [0, 109, 150, 255],
        [100, 145, 59, 255],
        [141, 19, 62, 255],
        [100, 145, 59, 255],
        [0, 109, 150, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [197, 140, 31, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [197, 140, 31, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [229, 163, 36, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [161, 115, 25, 255],
        [161, 115, 25, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [161, 115, 25, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [120, 69, 32, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [141, 19, 62, 255],
        [197, 140, 31, 255],
        [0, 109, 19, 255],
        [197, 140, 31, 255],
        [141, 19, 62, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [197, 140, 31, 255],
        [0, 133, 24, 255],
        [0, 155, 28, 255],
        [0, 133, 24, 255],
        [197, 140, 31, 255],
        [140, 81, 38, 255],
        [120, 69, 32, 255],
        [172, 23, 75, 255],
        [197, 140, 31, 255],
        [142, 206, 84, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [0, 109, 19, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [197, 140, 31, 255],
        [142, 206, 84, 255],
        [172, 23, 75, 255],
        [142, 206, 84, 255],
        [197, 140, 31, 255],
        [120, 69, 32, 255],
        [140, 81, 38, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [122, 177, 72, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [120, 69, 32, 255],
    ];
    let chunk: JavaChunk = from_bytes(CHUNK_1_17_0).unwrap();
    assert_eq!(expected, exercise_render(&chunk));
}

#[test]
fn chunk_1_17_1() {
    let expected = [
        [132, 66, 119, 255],
        [159, 109, 90, 255],
        [133, 65, 125, 255],
        [145, 66, 119, 255],
        [133, 65, 128, 255],
        [146, 66, 122, 255],
        [147, 66, 125, 255],
        [147, 66, 125, 255],
        [149, 66, 127, 255],
        [149, 66, 127, 255],
        [149, 66, 127, 255],
        [150, 66, 130, 255],
        [150, 66, 130, 255],
        [150, 66, 130, 255],
        [150, 66, 130, 255],
        [150, 66, 130, 255],
        [133, 90, 74, 255],
        [136, 93, 76, 255],
        [136, 93, 76, 255],
        [124, 56, 99, 255],
        [124, 56, 99, 255],
        [125, 56, 102, 255],
        [125, 56, 105, 255],
        [125, 56, 105, 255],
        [126, 56, 107, 255],
        [126, 56, 107, 255],
        [128, 56, 109, 255],
        [128, 56, 109, 255],
        [128, 56, 109, 255],
        [128, 56, 109, 255],
        [129, 56, 112, 255],
        [144, 57, 43, 255],
        [113, 56, 100, 255],
        [133, 90, 74, 255],
        [113, 56, 100, 255],
        [136, 93, 76, 255],
        [123, 57, 98, 255],
        [113, 56, 102, 255],
        [124, 56, 99, 255],
        [125, 56, 102, 255],
        [125, 56, 105, 255],
        [141, 98, 81, 255],
        [142, 100, 83, 255],
        [128, 56, 109, 255],
        [128, 56, 109, 255],
        [128, 56, 109, 255],
        [128, 56, 109, 255],
        [144, 57, 42, 255],
        [113, 56, 96, 255],
        [132, 88, 71, 255],
        [132, 88, 71, 255],
        [133, 90, 74, 255],
        [113, 56, 100, 255],
        [113, 56, 100, 255],
        [123, 57, 98, 255],
        [124, 56, 99, 255],
        [125, 56, 102, 255],
        [125, 56, 105, 255],
        [141, 98, 81, 255],
        [142, 100, 83, 255],
        [126, 56, 107, 255],
        [128, 56, 109, 255],
        [128, 56, 109, 255],
        [144, 57, 42, 255],
        [118, 57, 84, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [113, 56, 96, 255],
        [120, 57, 90, 255],
        [121, 57, 94, 255],
        [123, 57, 98, 255],
        [123, 57, 98, 255],
        [137, 94, 77, 255],
        [139, 95, 79, 255],
        [141, 98, 81, 255],
        [142, 100, 83, 255],
        [126, 56, 107, 255],
        [126, 56, 107, 255],
        [115, 56, 115, 255],
        [118, 57, 84, 255],
        [118, 57, 84, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [120, 57, 90, 255],
        [121, 57, 94, 255],
        [113, 56, 100, 255],
        [113, 56, 102, 255],
        [123, 57, 98, 255],
        [137, 94, 77, 255],
        [114, 56, 107, 255],
        [141, 98, 81, 255],
        [141, 98, 81, 255],
        [142, 100, 83, 255],
        [142, 57, 42, 255],
        [117, 57, 81, 255],
        [117, 57, 81, 255],
        [118, 57, 84, 255],
        [112, 56, 90, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [120, 57, 90, 255],
        [121, 57, 94, 255],
        [121, 57, 94, 255],
        [123, 57, 98, 255],
        [123, 57, 98, 255],
        [137, 94, 77, 255],
        [139, 95, 79, 255],
        [141, 98, 81, 255],
        [141, 98, 81, 255],
        [142, 57, 42, 255],
        [106, 59, 56, 255],
        [106, 59, 58, 255],
        [117, 57, 81, 255],
        [118, 57, 84, 255],
        [112, 56, 90, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [120, 57, 90, 255],
        [120, 57, 90, 255],
        [121, 57, 94, 255],
        [113, 56, 100, 255],
        [123, 57, 98, 255],
        [124, 56, 99, 255],
        [139, 95, 79, 255],
        [141, 98, 81, 255],
        [140, 57, 42, 255],
        [106, 59, 54, 255],
        [106, 59, 56, 255],
        [106, 59, 58, 255],
        [117, 57, 81, 255],
        [112, 56, 90, 255],
        [112, 56, 90, 255],
        [119, 57, 88, 255],
        [112, 56, 93, 255],
        [120, 57, 90, 255],
        [113, 56, 96, 255],
        [113, 56, 100, 255],
        [121, 57, 94, 255],
        [123, 57, 98, 255],
        [124, 56, 99, 255],
        [139, 95, 79, 255],
        [141, 98, 81, 255],
        [106, 59, 54, 255],
        [110, 57, 79, 255],
        [111, 57, 82, 255],
        [116, 57, 76, 255],
        [117, 57, 81, 255],
        [118, 57, 84, 255],
        [118, 57, 84, 255],
        [118, 57, 84, 255],
        [119, 57, 88, 255],
        [120, 57, 90, 255],
        [113, 56, 96, 255],
        [121, 57, 94, 255],
        [121, 57, 94, 255],
        [123, 57, 98, 255],
        [123, 57, 98, 255],
        [139, 95, 79, 255],
        [106, 59, 54, 255],
        [106, 59, 54, 255],
        [106, 59, 56, 255],
        [111, 57, 82, 255],
        [116, 57, 76, 255],
        [117, 57, 81, 255],
        [117, 57, 81, 255],
        [118, 57, 84, 255],
        [118, 57, 84, 255],
        [112, 56, 93, 255],
        [119, 57, 88, 255],
        [120, 57, 90, 255],
        [121, 57, 94, 255],
        [121, 57, 94, 255],
        [123, 57, 98, 255],
        [123, 57, 98, 255],
        [119, 75, 56, 255],
        [106, 59, 54, 255],
        [109, 57, 75, 255],
        [106, 59, 56, 255],
        [114, 58, 73, 255],
        [116, 57, 76, 255],
        [116, 57, 76, 255],
        [116, 57, 76, 255],
        [117, 57, 81, 255],
        [118, 57, 84, 255],
        [119, 57, 88, 255],
        [112, 56, 93, 255],
        [120, 57, 90, 255],
        [120, 57, 90, 255],
        [121, 57, 94, 255],
        [121, 57, 94, 255],
        [109, 57, 75, 255],
        [119, 75, 56, 255],
        [106, 59, 54, 255],
        [106, 59, 54, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [114, 58, 73, 255],
        [110, 57, 79, 255],
        [114, 58, 73, 255],
        [117, 57, 81, 255],
        [112, 56, 90, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [120, 57, 90, 255],
        [120, 57, 90, 255],
        [121, 57, 94, 255],
        [104, 70, 52, 255],
        [119, 75, 56, 255],
        [119, 75, 56, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [109, 57, 75, 255],
        [114, 58, 73, 255],
        [114, 58, 73, 255],
        [116, 57, 76, 255],
        [111, 57, 87, 255],
        [118, 57, 84, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [120, 57, 90, 255],
        [120, 57, 90, 255],
        [104, 70, 52, 255],
        [119, 75, 56, 255],
        [119, 75, 56, 255],
        [109, 57, 75, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [114, 58, 73, 255],
        [116, 57, 76, 255],
        [117, 57, 81, 255],
        [112, 56, 90, 255],
        [118, 57, 84, 255],
        [119, 57, 88, 255],
        [119, 57, 88, 255],
        [120, 57, 90, 255],
        [109, 57, 75, 255],
        [119, 75, 56, 255],
        [109, 57, 75, 255],
        [119, 75, 56, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [113, 58, 69, 255],
        [110, 57, 79, 255],
        [116, 57, 76, 255],
        [117, 57, 81, 255],
        [118, 57, 84, 255],
        [118, 57, 84, 255],
        [106, 59, 65, 255],
        [106, 59, 65, 255],
    ];
    let chunk: JavaChunk = from_bytes(CHUNK_1_17_1).unwrap();
    exercise_render(&chunk);
    assert_eq!(expected, exercise_render(&chunk));
}

#[test]
fn chunk_custom_heights_1_17_1() {
    let expected = [
        [200, 27, 88, 255],
        [142, 206, 84, 255],
        [0, 127, 175, 255],
        [140, 81, 38, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [140, 81, 38, 255],
        [140, 81, 38, 255],
        [140, 81, 38, 255],
        [140, 81, 38, 255],
        [140, 81, 38, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [140, 81, 38, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [122, 177, 72, 255],
        [200, 27, 88, 255],
        [197, 140, 31, 255],
        [142, 206, 84, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [172, 23, 75, 255],
        [197, 140, 31, 255],
        [142, 206, 84, 255],
        [172, 23, 75, 255],
        [142, 206, 84, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [122, 177, 72, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [122, 177, 72, 255],
        [172, 23, 75, 255],
        [197, 140, 31, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [172, 23, 75, 255],
        [100, 145, 59, 255],
        [0, 109, 150, 255],
        [98, 57, 26, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [197, 140, 31, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [100, 145, 59, 255],
        [0, 109, 150, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [161, 115, 25, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [172, 23, 75, 255],
        [98, 57, 26, 255],
        [140, 81, 38, 255],
        [140, 81, 38, 255],
        [140, 81, 38, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [142, 206, 84, 255],
        [200, 27, 88, 255],
        [142, 206, 84, 255],
        [0, 127, 175, 255],
        [120, 69, 32, 255],
        [140, 81, 38, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [122, 177, 72, 255],
        [0, 127, 175, 255],
        [120, 69, 32, 255],
        [140, 81, 38, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [197, 140, 31, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [122, 177, 72, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [200, 27, 88, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [229, 163, 36, 255],
        [197, 140, 31, 255],
        [172, 23, 75, 255],
        [120, 69, 32, 255],
        [200, 27, 88, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [161, 115, 25, 255],
        [161, 115, 25, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [122, 177, 72, 255],
        [98, 57, 26, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [200, 27, 88, 255],
        [200, 27, 88, 255],
        [197, 140, 31, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [85, 145, 67, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [100, 145, 59, 255],
        [0, 109, 150, 255],
        [200, 27, 88, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [141, 19, 62, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [85, 145, 67, 255],
        [100, 145, 59, 255],
        [141, 19, 62, 255],
        [100, 145, 59, 255],
        [0, 109, 150, 255],
        [141, 19, 62, 255],
        [229, 163, 36, 255],
        [200, 27, 88, 255],
        [172, 23, 75, 255],
        [161, 115, 25, 255],
        [172, 23, 75, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [85, 145, 67, 255],
        [85, 145, 67, 255],
        [98, 57, 26, 255],
        [98, 57, 26, 255],
        [100, 145, 59, 255],
        [141, 19, 62, 255],
        [197, 140, 31, 255],
        [142, 206, 84, 255],
        [197, 140, 31, 255],
        [200, 27, 88, 255],
        [141, 19, 62, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [98, 57, 26, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [85, 145, 67, 255],
        [85, 144, 66, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [172, 23, 75, 255],
        [197, 140, 31, 255],
        [142, 206, 84, 255],
        [172, 23, 75, 255],
        [142, 206, 84, 255],
        [197, 140, 31, 255],
        [172, 23, 75, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [120, 69, 32, 255],
        [85, 145, 67, 255],
        [85, 144, 66, 255],
    ];

    let chunk: JavaChunk = from_bytes(CHUNK_CUSTOM_HEIGHTS_1_17_1).unwrap();
    assert_eq!(expected, exercise_render(&chunk));
}

fn calculate_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
