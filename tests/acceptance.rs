use assert_matches::assert_matches;
use std::fs::read_to_string;
use gerber::gerber;

fn test_file(filename: &str) {
    let data = read_to_string(filename).unwrap();
    assert_matches!(gerber(&data), Ok(_));
}

#[test]
fn two_square_boxes() {
    test_file("tests/data/2-13-1_Two_square_boxes.gbr")
}

#[test]
fn polarities_and_apertures() {
    test_file("tests/data/2-13-2_Polarities_and_Apertures.gbr");
}

#[test]
fn nested_blocks() {
    test_file("tests/data/4-6-4_Nested_blocks.gbr");
}

#[test]
fn block_with_different_orientations() {
    test_file("tests/data/4-11-6_Block_with_different_orientations.gbr");
}

#[test]
fn a_drill_file() {
    test_file("tests/data/6-1-6-2_A_drill_file.gbr");
}

#[test]
fn sample_macro_x1() {
    test_file("tests/data/sample_macro_X1.gbr");
}

#[test]
fn sample_macro() {
    test_file("tests/data/sample_macro.gbr");
}

#[test]
fn smd_prim_20_x1() {
    test_file("tests/data/SMD_prim_20_X1.gbr");
}

#[test]
fn smd_prim_20() {
    test_file("tests/data/SMD_prim_20.gbr");
}

#[test]
fn smd_prim_21_x1() {
    test_file("tests/data/SMD_prim_21_X1.gbr");
}

#[test]
fn smd_prim_21() {
    test_file("tests/data/SMD_prim_21.gbr");
}
