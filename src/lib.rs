#![feature(test)]

extern crate test;

use {
    test::{
        Bencher,
        black_box
    },
    serde_derive::{
        Serialize,
        Deserialize
    },
    speedy_derive::{
        Writable,
        Readable
    }
};

mod prost_bench {
    use {
        serde_derive::{
            Serialize,
            Deserialize
        },
        prost_derive::{
            Enumeration,
            Message
        }
    };

    #[derive(PartialEq, Debug, Serialize, Deserialize, Enumeration)]
    pub enum Enum {
        A = 0,
        B = 1,
        C = 2,
        D = 3
    }

    #[derive(PartialEq, Serialize, Deserialize, Message)]
    pub struct Foo {
        #[prost(uint32, tag="1")]
        a: u32,
        #[prost(uint64, tag="2")]
        b: u64,
        #[prost(string, tag="3")]
        c: String,
        #[prost(bytes, tag="4")]
        d: Vec< u8 >,
        #[prost(float, tag="5")]
        e: f32,
        #[prost(double, tag="6")]
        f: f64,
        #[prost(enumeration="Enum", tag="7")]
        g: i32
    }

    pub fn default() -> Foo {
        Foo {
            a: 77,
            b: 0x12345678ABCDEF00,
            c: "A very long and totally pointless string".to_owned(),
            d: vec![ 0, 1, 2, 3, 4, 5, 10, 15, 20, 100, 255 ],
            e: 3.1415,
            f: 2.7182,
            g: Enum::C as _
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize, Readable, Writable)]
enum Enum {
    A = 0,
    B = 1,
    C = 2,
    D = 3
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Readable, Writable)]
struct Foo {
    a: u8,
    b: u64,
    c: String,
    d: Vec< u8 >,
    e: f32,
    f: f64,
    g: Enum
}

impl Default for Foo {
    fn default() -> Self {
        Foo {
            a: 77,
            b: 0x12345678ABCDEF00,
            c: "A very long and totally pointless string".to_owned(),
            d: vec![ 0, 1, 2, 3, 4, 5, 10, 15, 20, 100, 255 ],
            e: 3.1415,
            f: 2.7182,
            g: Enum::C
        }
    }
}

#[inline]
fn empty_vec() -> Vec< u8 > {
    Vec::with_capacity( 256 )
}

#[inline(never)]
fn default_value() -> Foo {
    black_box( Foo::default() )
}

#[inline]
fn serialize_manual_with_endianness< B: byteorder::ByteOrder >( value: &Foo ) -> Vec< u8 > {
    use byteorder::WriteBytesExt;

    let mut buffer = empty_vec();
    buffer.write_u8( value.a ).unwrap();
    buffer.write_u64::< B >( value.b ).unwrap();
    buffer.write_u32::< B >( value.c.len() as u32 ).unwrap();
    buffer.extend_from_slice( value.c.as_bytes() );
    buffer.write_u32::< B >( value.d.len() as u32 ).unwrap();
    buffer.extend_from_slice( &value.d );
    buffer.write_f32::< B >( value.e ).unwrap();
    buffer.write_f64::< B >( value.f ).unwrap();
    buffer.write_i32::< B >( value.g as _ ).unwrap();
    buffer
}

#[inline]
fn bench_serialize_manual_with_endianness< B: byteorder::ByteOrder >( b: &mut Bencher ) {
    let value = default_value();

    b.iter( || {
        serialize_manual_with_endianness::< B >( &value )
    });
}

#[inline]
fn bench_deserialize_manual_with_endianness< B: byteorder::ByteOrder >( b: &mut Bencher ) {
    use std::io::Read;
    use byteorder::ReadBytesExt;

    let value = default_value();
    let serialized = serialize_manual_with_endianness::< B >( &value );

    b.iter( || {
        let mut buffer = &serialized[..];
        let a = buffer.read_u8().unwrap();
        let b = buffer.read_u64::< B >().unwrap();

        let c_len = buffer.read_u32::< B >().unwrap() as usize;
        let mut c = Vec::with_capacity( c_len );
        unsafe { c.set_len( c_len ); }
        buffer.read_exact( &mut c[..] ).unwrap();
        let c = String::from_utf8( c ).unwrap();

        let d_len = buffer.read_u32::< B >().unwrap() as usize;
        let mut d = Vec::with_capacity( d_len );
        unsafe { d.set_len( d_len ); }
        buffer.read_exact( &mut d[..] ).unwrap();

        let e = buffer.read_f32::< B >().unwrap();
        let f = buffer.read_f64::< B >().unwrap();
        let g = match buffer.read_i32::< B >().unwrap() {
            0 => Enum::A,
            1 => Enum::B,
            2 => Enum::C,
            3 => Enum::D,
            _ => panic!()
        };

        Foo { a, b, c, d, e, f, g }
    });
}

#[bench]
fn serialize_manual( b: &mut Bencher ) {
    bench_serialize_manual_with_endianness::< byteorder::NativeEndian >( b );
}

#[cfg(target_endian = "little")]
#[bench]
fn serialize_manual_foreign_endianness( b: &mut Bencher ) {
    bench_serialize_manual_with_endianness::< byteorder::BigEndian >( b );
}

#[cfg(target_endian = "big")]
#[bench]
fn serialize_manual_foreign_endianness( b: &mut Bencher ) {
    bench_serialize_manual_with_endianness::< byteorder::LittleEndian >( b );
}

#[bench]
fn deserialize_manual( b: &mut Bencher ) {
    bench_deserialize_manual_with_endianness::< byteorder::NativeEndian >( b );
}

#[cfg(target_endian = "little")]
#[bench]
fn deserialize_manual_foreign_endianness( b: &mut Bencher ) {
    bench_deserialize_manual_with_endianness::< byteorder::BigEndian >( b );
}

#[cfg(target_endian = "big")]
#[bench]
fn deserialize_manual_foreign_endianness( b: &mut Bencher ) {
    bench_dedeserialize_manual_with_endianness::< byteorder::LittleEndian >( b );
}

#[bench]
fn serialize_serde_rmp( b: &mut Bencher ) {
    use rmp_serde::Serializer;
    use serde::Serialize;

    let value = default_value();

    b.iter( || {
        let mut buffer = empty_vec();
        value.serialize( &mut Serializer::new( &mut buffer ) ).unwrap();
        buffer
    });
}

#[bench]
fn serialize_rmp( b: &mut Bencher ) {
    use rmp::encode::*;
    let value = default_value();

    b.iter( || {
        let mut buffer = empty_vec();
        write_u8( &mut buffer, value.a ).unwrap();
        write_u64( &mut buffer, value.b ).unwrap();
        write_str( &mut buffer, &value.c ).unwrap();
        write_bin( &mut buffer, &value.d ).unwrap();
        write_f32( &mut buffer, value.e ).unwrap();
        write_f64( &mut buffer, value.f ).unwrap();
        write_i32( &mut buffer, value.g as _ ).unwrap();

        buffer
    });
}

#[bench]
fn serialize_prost( b: &mut Bencher ) {
    use prost::Message;

    let value = prost_bench::default();
    b.iter( || {
        let mut buffer = empty_vec();
        value.encode( &mut buffer ).unwrap()
    });
}

macro_rules! speedy_benches {
    ($serialize_fn_name:ident, $deserialize_fn_name:ident, $endianness:expr) => {
        #[bench]
        fn $serialize_fn_name( b: &mut Bencher ) {
            use speedy::Writable;

            let value = default_value();
            b.iter( || {
                let mut buffer = empty_vec();
                value.write_to_stream( $endianness, &mut buffer ).unwrap();
                buffer
            });
        }

        #[bench]
        fn $deserialize_fn_name( b: &mut Bencher ) {
            use speedy::{Readable, Writable};

            let value = default_value();
            let mut buffer = empty_vec();
            value.write_to_stream( $endianness, &mut buffer ).unwrap();

            b.iter( || {
                let deserialized: Foo = Readable::read_from_buffer( $endianness, &buffer ).unwrap();
                deserialized
            });
        }
    }
}

fn foreign_endianness() -> speedy::Endianness {
    match speedy::Endianness::NATIVE {
        speedy::Endianness::LittleEndian => speedy::Endianness::BigEndian,
        speedy::Endianness::BigEndian => speedy::Endianness::LittleEndian
    }
}

speedy_benches!( serialize_speedy, deserialize_speedy, speedy::Endianness::NATIVE );
speedy_benches!( serialize_speedy_foreign_endianness, deserialize_speedy_foreign_endianness, foreign_endianness() );

#[bench]
fn serialize_serde_bincode( b: &mut Bencher ) {
    use bincode::serialize_into;

    let value = default_value();

    b.iter( || {
        let mut buffer = empty_vec();
        serialize_into( &mut buffer, &value ).unwrap();
        buffer
    });
}

#[bench]
fn deserialize_serde_bincode( b: &mut Bencher ) {
    use bincode::{serialize_into, deserialize};

    let value = default_value();
    let mut buffer = Vec::new();
    serialize_into( &mut buffer, &value ).unwrap();

    b.iter( || {
        let deserialized: Foo = deserialize( &buffer ).unwrap();
        deserialized
    });
}

#[bench]
fn serialize_serde_json( b: &mut Bencher ) {
    let value = default_value();

    b.iter( || {
        let mut buffer = empty_vec();
        serde_json::to_writer( &mut buffer, &value ).unwrap();
        buffer
    });
}

#[bench]
fn serialize_serde_pickle( b: &mut Bencher ) {
    let value = default_value();

    b.iter( || {
        let mut buffer = empty_vec();
        serde_pickle::to_writer( &mut buffer, &value, true ).unwrap();
        buffer
    });
}

#[bench]
fn serialize_serde_xdr( b: &mut Bencher ) {
    let value = default_value();

    b.iter( || {
        let mut buffer = empty_vec();
        serde_xdr::to_writer( &mut buffer, &value ).unwrap();
        buffer
    });
}

#[bench]
fn serialize_serde_cbor( b: &mut Bencher ) {
    let value = default_value();

    b.iter( || {
        let mut buffer = empty_vec();
        serde_cbor::ser::to_writer( &mut buffer, &value ).unwrap();
        buffer
    });
}
