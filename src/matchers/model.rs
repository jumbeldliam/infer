///Returns whether a buffer is a glb.
pub fn is_glb(buf: &[u8]) -> bool {
    //https://github.com/KhronosGroup/glTF/tree/main/specification/2.0
    buf.len() > 3 && buf[0] == b'g' && buf[1] == b'l' && buf[2] == b'T' && buf[3] == b'F'
}

///Returns whether a buffer is a universal scene descriptor (usd).
pub fn is_usd(buf: &[u8]) -> bool {
    //https://www.iana.org/assignments/media-types/model/vnd.usda
    buf.len() > 17
        && ((buf[0] == b'#'
            && buf[1] == b'u'
            && buf[2] == b's'
            && buf[3] == b'd'
            && buf[4] == b'a'
            && buf[5] == b' '
            && buf[6] == b'M'
            && buf[7] == b'A'
            && buf[8] == b'J'
            && buf[9] == b'O'
            && buf[10] == b'R'
            && buf[12] == b'.'
            && buf[13] == b'M'
            && buf[14] == b'I'
            && buf[15] == b'N'
            && buf[16] == b'O'
            && buf[17] == b'R')
            || (buf[0] == b'P'
                && buf[1] == b'X'
                && buf[2] == b'R'
                && buf[3] == b'-'
                && buf[4] == b'U'
                && buf[5] == b'S'
                && buf[6] == b'D'
                && buf[7] == b'C'))
}

///Returns whether a buffer is an fbx.
pub fn is_fbx(buf: &[u8]) -> bool {
    //http://justsolve.archiveteam.org/wiki/FBX
    buf.len() > 20
        && buf[0] == b'K'
        && buf[1] == b'a'
        && buf[2] == b'y'
        && buf[3] == b'd'
        && buf[4] == b'a'
        && buf[5] == b'r'
        && buf[6] == b'a'
        && buf[7] == b' '
        && buf[8] == b'F'
        && buf[9] == b'B'
        && buf[10] == b'X'
}

///Returns whether a buffer is a 3ds.
pub fn is_3ds(buf: &[u8]) -> bool {
    //https://en.wikipedia.org/wiki/.3ds
    buf.len() > 1 && buf[0] == 0x4D && buf[1] == 0x4D
}

///Returns whether a buffer is an ascii stl.
pub fn is_stl(buf: &[u8]) -> bool {
    //https://www.garykessler.net/library/file_sigs.html
    buf.len() > 80
        && buf[0] == b's'
        && buf[1] == b'o'
        && buf[2] == b'l'
        && buf[3] == b'i'
        && buf[4] == b'd'
}

///Returns whether a buffer is an ISO 10303 (step).
pub fn is_step(buf: &[u8]) -> bool {
    //https://en.wikipedia.org/wiki/ISO_10303-21
    buf.len() > 11
        && buf[0] == b'I'
        && buf[1] == b'S'
        && buf[2] == b'O'
        && buf[3] == b'-'
        && buf[4] == b'1'
        && buf[5] == b'0'
        && buf[6] == b'3'
        && buf[7] == b'0'
        && buf[8] == b'3'
        && buf[9] == b'-'
        && buf[10] == b'2'
        && buf[11] == b'1'
}

///Returns whether a buffer is a draco compressed 3d model (drc).
pub fn is_drc(buf: &[u8]) -> bool {
    //https://google.github.io/draco/spec/
    buf.len() > 4
        && buf[0] == b'D'
        && buf[1] == b'R'
        && buf[2] == b'A'
        && buf[3] == b'C'
        && buf[4] == b'O'
}
