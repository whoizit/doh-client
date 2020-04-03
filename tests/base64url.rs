use data_encoding::BASE64URL_NOPAD;

#[test]
fn www_example_com() {
    let msg: [u8; 33] = [
        0x00, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x77, 0x77,
        0x77, 0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00,
        0x01, 0x00, 0x01,
    ];
    let result = "AAABAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB";

    assert_eq!(result, BASE64URL_NOPAD.encode(&msg));
}

#[test]
fn a_62characterlabel_makes_base64url_distinct_from_standard_base64_example_com() {
    let msg: [u8; 94] = [
        0x00, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x61, 0x3e,
        0x36, 0x32, 0x63, 0x68, 0x61, 0x72, 0x61, 0x63, 0x74, 0x65, 0x72, 0x6c, 0x61, 0x62, 0x65,
        0x6c, 0x2d, 0x6d, 0x61, 0x6b, 0x65, 0x73, 0x2d, 0x62, 0x61, 0x73, 0x65, 0x36, 0x34, 0x75,
        0x72, 0x6c, 0x2d, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x63, 0x74, 0x2d, 0x66, 0x72, 0x6f,
        0x6d, 0x2d, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x61, 0x72, 0x64, 0x2d, 0x62, 0x61, 0x73, 0x65,
        0x36, 0x34, 0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00,
        0x00, 0x01, 0x00, 0x01,
    ];
    let result = "AAABAAABAAAAAAAAAWE-NjJjaGFyYWN0ZXJsYWJlbC1tYWtlcy1iYXNlNjR1cmwtZGlzdGluY3Q\
    tZnJvbS1zdGFuZGFyZC1iYXNlNjQHZXhhbXBsZQNjb20AAAEAAQ";

    assert_eq!(result, BASE64URL_NOPAD.encode(&msg));
}
