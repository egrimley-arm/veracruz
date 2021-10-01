use lazy_static::lazy_static;
use std::sync::Mutex;
use wasi_types::Inode;

lazy_static! {
    static ref PARSEC_INODE: Mutex<Option<Inode>> = Mutex::new(None);
    static ref BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

pub fn register_parsec_inode(inode: Inode) {
    *PARSEC_INODE.lock().unwrap() = Some(inode)
}

pub fn is_parsec_inode(inode: Inode) -> bool {
    match *PARSEC_INODE.lock().unwrap() {
        None => false,
        Some(x) => x == inode,
    }
}

pub fn write(buf: &[u8]) {
    (*BUFFER.lock().unwrap()).extend(buf);
}

pub fn read(len: usize) -> Vec<u8> {
    let mut buf = BUFFER.lock().unwrap();
    let size = buf.len();
    let retlen = if len < size { len } else { size };
    let mut ret: Vec<u8> = (*buf).splice(..retlen, vec![]).collect();

    // ROT13 implementation:
    for x in ret.iter_mut() {
        if b"A"[0] <= *x && *x <= b"Z"[0] {
            *x = b"A"[0] + (*x - b"A"[0] + 13) % 26
        }
        if b"a"[0] <= *x && *x <= b"z"[0] {
            *x = b"a"[0] + (*x - b"a"[0] + 13) % 26
        }
    }

    ret
}
