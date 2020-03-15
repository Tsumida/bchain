
//! This is an impl for crypography algorithm SHA2-256.
//! SHA256 works correctly when the content contains only numbers or latters.
//! 

use byteorder::{LittleEndian, BigEndian, NativeEndian, ReadBytesExt, WriteBytesExt};

/// 参考: https://zhuanlan.zhihu.com/p/94619052
static init_hash: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];
pub struct SHA256{
    msg: Vec<u8>,
}

#[inline]
fn ch(x:u32, y:u32, z:u32) -> u32{
    (x & y) ^ (!x & z)
}

#[inline]
fn maj(x:u32, y:u32, z:u32) -> u32{
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline]
fn cigma_0(x:u32) -> u32{
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline]
fn cigma_1(x:u32) -> u32{
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline]
fn gama_0(x:u32) -> u32{
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline]
fn gama_1(x:u32) -> u32{
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

// 模2^32 加法。
macro_rules! modular_32_add {
    ($x:expr, $($y:expr),+) => {
        {
            let a:u32 = $x;
            let res= a.wrapping_add(modular_32_add!($($y), +));
            res
        }
    };
    ($x:expr) => {
        $x
    }
}

impl SHA256{
    pub fn new(msg: &[u8]) -> SHA256{
        // 448 >> 3 == 56 Byte
        let mut res = SHA256{
            msg: msg.to_vec(),
        };
        res.append_padding();
        res
    }

    pub fn cal_sha_256(&mut self) -> String{
        let mut H: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];
        self.compact_iteration(&mut H);
        // digital signature = H[0] append H[1] ...
        H.iter()
            .map(|e| format!("{:08x}", e))
            .collect::<String>()
    }

    fn cal_padding(&self) -> usize{
        let leng = self.msg.len() % 64;
        if leng > 56{
            64 - (leng - 56) // 57 --> 63 --> 64 + 56
        }else{
            56 - leng
        }
    }


    fn get_be_u32_chunks(&self) -> Vec<Vec<u32>>{
        let l = self.msg.len() / 64 ;
        let mut res = Vec::with_capacity(l);

        for i in 0..l{
            // 自然方式读取
            let st = 64*i;
            res.push(
                (0..16).map(|i| 
                    u32::from_be_bytes(
                        unsafe{
                            *(self.msg[st+4*i..st+4*i+4].as_ptr() as *const [u8; 4])
                        }
                )).collect::<Vec<u32>>()
            );
        }
        res
    }

    fn compact(&mut self, chunk: &Vec<u32>, H:&mut [u32; 8], w: &mut [u32; 64]){
        assert_eq!(0, chunk.len() % 16);
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (
            H[0], H[1], H[2], H[3], H[4], H[5], H[6], H[7]
        );
       
        // copy chunk to w[0..16]
        for i in 0..16{
            w[i] = chunk[i] // big-endian
        }

        for i in 16..64{
            let s0 = gama_0(w[i-15]);
            let s1 = gama_1(w[i-2]);
            w[i] = modular_32_add!(w[i-16], s0, w[i-7], s1);
        }
        
        // compression loop.
        for i in 0..64{
            let temp1   = modular_32_add!(
                h, cigma_1(e), ch(e, f, g), init_hash[i], w[i]
            );
            let temp2   = modular_32_add!(
                cigma_0(a), maj(a, b, c)
            );

            h = g;
            g = f;
            f = e;
            e = modular_32_add!(d, temp1);
            d = c;
            c = b;
            b = a;
            a = modular_32_add!(temp1, temp2);
        }

        H[0] = modular_32_add!(H[0], a);
        H[1] = modular_32_add!(H[1], b);
        H[2] = modular_32_add!(H[2], c);
        H[3] = modular_32_add!(H[3], d);
        H[4] = modular_32_add!(H[4], e);
        H[5] = modular_32_add!(H[5], f);
        H[6] = modular_32_add!(H[6], g);
        H[7] = modular_32_add!(H[7], h);
    }

    fn compact_iteration(&mut self, H: &mut [u32; 8]){
        let chunks = self.get_be_u32_chunks();
        // main loop
        // produce a 256bit digital signature for each chunk.
        let mut w: [u32; 64] = [0; 64];
        for chunk in chunks{
            self.compact(&chunk, H, &mut w);
        }
    }

    fn append_padding(&mut self){
        // append padding.
        let msg_len = (self.msg.len() as u64) << 3;     // bit
        self.msg.push(0b10000000);                      // at least 1 Byte
        let padding_len = self.cal_padding();
        self.msg.extend(
            std::iter::repeat(0u8).take(padding_len) 
        );
        self.msg.write_u64::<BigEndian>(msg_len).unwrap();  

        assert_eq!(0, self.msg.len() % 64);
    }

}



#[cfg(test)]
mod sha_256_test{
    use super::*;
    use byteorder::{LittleEndian, BigEndian, NativeEndian, ReadBytesExt, WriteBytesExt};

    #[test]
    fn test_proc() {
        let s = "abc";
        let mut sha = SHA256::new(s.to_ascii_lowercase().as_bytes());
        
        assert_eq!(0, sha.msg.len()%64);
        let cks = sha.get_be_u32_chunks()[0].to_owned();
        let mut cmp1 = vec![0x61626380u32];
        cmp1.append(&mut vec![0; 13]);
        cmp1.push(0);
        cmp1.push(24u32); // 0x00000018

        assert_eq!(
            cmp1, 
            cks,
        );
    }

    #[test]
    fn test_correctness() {
        assert_eq!(
            SHA256::new("abc".to_ascii_lowercase().as_bytes()).cal_sha_256(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        );
        assert_eq!(
            SHA256::new("abcdef".to_ascii_lowercase().as_bytes()).cal_sha_256(),
            "bef57ec7f53a6d40beb640a780a639c83bc29ac8a9816f1fc6c5c6dcd93c4721"
        );

        assert_eq!(
            SHA256::new("dddddddddddddddddddddddddddddd".to_ascii_lowercase().as_bytes()).cal_sha_256(),
            "31273282b1095ad8c0032d2c8dbe2e32b635f41411c67ac31f3d029cde22ade1",
        );

        assert_eq!(
            SHA256::new("1234567890abcedf".to_ascii_lowercase().as_bytes()).cal_sha_256(),
            "907e464a92ac7fa21c919ab718a37f494a2c560006f13147a25bf5056e7aeafd",
        );
    }

    #[test]
    #[should_panic]
    fn test_incorrect() {
        assert_eq!(
            SHA256::new("Hello, world!".to_ascii_lowercase().as_bytes()).cal_sha_256(),
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3",
        );
        // got 68e656b251e67e8358bef8483ab0d51c6619f3e7a1a9f0e75838d41ff368f728
    }

    #[test]
    fn test_modular_32_add() {
        assert_eq!(
            6,
            modular_32_add!(1, 2, 3),
        );

        assert_eq!(
            0x00000000u32,
            modular_32_add!(0x80000000u32, 0x80000000u32),
        );

        assert_eq!(
            0x80000000u32,
            modular_32_add!(0x80000000u32),
        );
    }

    #[test]
    fn test_padding() {
        let mut s1 = SHA256::new("abc".to_ascii_lowercase().as_bytes());
        let mut pad1 = vec![0x61626380u32];
        pad1.append(
            &mut vec![0; 14]
        );
        pad1.push(24);
        assert_eq!(
            s1.get_be_u32_chunks()[0].to_owned(),
            pad1
        );


        let mut s2 = SHA256::new("abcdef".to_ascii_lowercase().as_bytes());
        let mut pad2 = vec![0x61626364u32, 0x65668000u32];
        pad2.append(
            &mut vec![0; 13]
        );
        pad2.push(48);
        assert_eq!(
            s2.get_be_u32_chunks()[0].to_owned(),
            pad2
        );


    }

    #[test]
    fn test_endian() {
        let n = 0x61626380u32;
        println!("{:x}", n);
        let mut v1 = Vec::new();
        v1.write_u32::<BigEndian>(n).unwrap();
        println!("be:{:?}", &v1);
        
        let n1 = u32::from_be_bytes(
            unsafe{*(v1.as_ptr() as *const [u8;4])}
        );
        assert_eq!(
            n1, 
            0x61626380u32,
        );
    }

}

