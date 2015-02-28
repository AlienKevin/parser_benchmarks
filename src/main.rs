#[macro_use]
extern crate nom;

extern crate test;

use nom::{HexDisplay,IResult,FlatMap,FlatMapOpt,Functor,Producer,ProducerState,FileProducer,be_u16,be_u32,be_u64,be_f32,be_f64};
use nom::{Consumer,ConsumerState};
use nom::IResult::*;

use std::str;
use std::io::SeekFrom;

fn mp4_box(input:&[u8]) -> IResult<&[u8], &[u8]> {
  match be_u32(input) {
    Done(i, offset) => {
      let sz: usize = offset as usize;
      if i.len() >= sz - 4 {
        return Done(&i[(sz-4)..], &i[0..(sz-4)])
      } else {
        return Incomplete(1234)
      }
    }
    Error(e)      => Error(e),
    Incomplete(e) => Incomplete(e)
  }
}

#[derive(PartialEq,Eq,Debug)]
struct FileType<'a> {
  major_brand:         &'a str,
  major_brand_version: &'a [u8],
  compatible_brands:   Vec<&'a str>
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Mvhd32 {
  version_flags: u32, // actually:
  // version: u8,
  // flags: u24       // 3 bytes
  created_date:  u32,
  modified_date: u32,
  scale:         u32,
  duration:      u32,
  speed:         f32,
  volume:        u16, // actually a 2 bytes decimal
  /* 10 bytes reserved */
  scaleA:        f32,
  rotateB:       f32,
  angleU:        f32,
  rotateC:       f32,
  scaleD:        f32,
  angleV:        f32,
  positionX:     f32,
  positionY:     f32,
  scaleW:        f32,
  preview:       u64,
  poster:        u32,
  selection:     u64,
  current_time:  u32,
  track_id:      u32
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Mvhd64 {
  version_flags: u32, // actually:
  // version: u8,
  // flags: u24       // 3 bytes
  created_date:  u64,
  modified_date: u64,
  scale:         u32,
  duration:      u64,
  speed:         f32,
  volume:        u16, // actually a 2 bytes decimal
  /* 10 bytes reserved */
  scaleA:        f32,
  rotateB:       f32,
  angleU:        f32,
  rotateC:       f32,
  scaleD:        f32,
  angleV:        f32,
  positionX:     f32,
  positionY:     f32,
  scaleW:        f32,
  preview:       u64,
  poster:        u32,
  selection:     u64,
  current_time:  u32,
  track_id:      u32
}
take!(ten_bytes 10);

#[allow(non_snake_case)]
chain!(mvhd32 <&[u8], MvhdBox>,
  version_flags: be_u32 ~
  created_date:  be_u32 ~
  modified_date: be_u32 ~
  scale:         be_u32 ~
  duration:      be_u32 ~
  speed:         be_f32 ~
  volume:        be_u16 ~ // actually a 2 bytes decimal
              ten_bytes ~
  scaleA:        be_f32 ~
  rotateB:       be_f32 ~
  angleU:        be_f32 ~
  rotateC:       be_f32 ~
  scaleD:        be_f32 ~
  angleV:        be_f32 ~
  positionX:     be_f32 ~
  positionY:     be_f32 ~
  scaleW:        be_f32 ~
  preview:       be_u64 ~
  poster:        be_u32 ~
  selection:     be_u64 ~
  current_time:  be_u32 ~
  track_id:      be_u32,
  ||{
    MvhdBox::M32(Mvhd32 {
      version_flags: version_flags,
      created_date:  created_date,
      modified_date: modified_date,
      scale:         scale,
      duration:      duration,
      speed:         speed,
      volume:        volume,
      scaleA:        scaleA,
      rotateB:       rotateB,
      angleU:        angleU,
      rotateC:       rotateC,
      scaleD:        scaleD,
      angleV:        angleV,
      positionX:     positionX,
      positionY:     positionY,
      scaleW:        scaleW,
      preview:       preview,
      poster:        poster,
      selection:     selection,
      current_time:  current_time,
      track_id:      track_id
    })
  }
);

#[allow(non_snake_case)]
chain!(mvhd64 <&[u8], MvhdBox>,
  version_flags: be_u32 ~
  created_date:  be_u64 ~
  modified_date: be_u64 ~
  scale:         be_u32 ~
  duration:      be_u64 ~
  speed:         be_f32 ~
  volume:        be_u16 ~ // actually a 2 bytes decimal
              ten_bytes ~
  scaleA:        be_f32 ~
  rotateB:       be_f32 ~
  angleU:        be_f32 ~
  rotateC:       be_f32 ~
  scaleD:        be_f32 ~
  angleV:        be_f32 ~
  positionX:     be_f32 ~
  positionY:     be_f32 ~
  scaleW:        be_f32 ~
  preview:       be_u64 ~
  poster:        be_u32 ~
  selection:     be_u64 ~
  current_time:  be_u32 ~
  track_id:      be_u32,
  ||{
    MvhdBox::M64(Mvhd64 {
      version_flags: version_flags,
      created_date:  created_date,
      modified_date: modified_date,
      scale:         scale,
      duration:      duration,
      speed:         speed,
      volume:        volume,
      scaleA:        scaleA,
      rotateB:       rotateB,
      angleU:        angleU,
      rotateC:       rotateC,
      scaleD:        scaleD,
      angleV:        angleV,
      positionX:     positionX,
      positionY:     positionY,
      scaleW:        scaleW,
      preview:       preview,
      poster:        poster,
      selection:     selection,
      current_time:  current_time,
      track_id:      track_id
    })
  }
);

#[derive(Debug)]
pub enum MvhdBox {
  M32(Mvhd32),
  M64(Mvhd64)
}

#[derive(Debug)]
pub enum MoovBox {
  Mdra,
  Dref,
  Cmov,
  Rmra,
  Iods,
  Mvhd(MvhdBox),
  Clip,
  Trak,
  Udta
}

#[derive(Debug)]
enum MP4Box<'a> {
  Ftyp(FileType<'a>),
  Moov(Vec<MoovBox>),
  Mdat,
  Free,
  Skip,
  Wide,
  Unknown
}

tag!(ftyp    "ftyp".as_bytes());

fn brand_name(input:&[u8]) -> IResult<&[u8],&str> {
  take!(major_brand_bytes 4);
  major_brand_bytes(input).map_res(str::from_utf8)
}
take!(major_brand_version 4);
many0!(compatible_brands<&[u8], &str> brand_name);

fn filetype_parser<'a>(input: &'a[u8]) -> IResult<&'a [u8], FileType<'a> > {
  chaining_parser!(input,
    m: brand_name          ~
    v: major_brand_version ~
    c: compatible_brands   ,
    ||{FileType{major_brand: m, major_brand_version:v, compatible_brands: c}})
}

o!(filetype <&[u8], FileType>  ftyp ~ [ filetype_parser ]);

fn filetype_box(input:&[u8]) -> IResult<&[u8], MP4Box> {
  match filetype(input) {
    Error(a)      => Error(a),
    Incomplete(a) => Incomplete(a),
    Done(i, o)    => {
      Done(i, MP4Box::Ftyp(o))
    }
  }
}

tag!(moov_tag "moov".as_bytes());

tag!(mdra    "mdra".as_bytes());
fn moov_mdra(input:&[u8]) -> IResult<&[u8], MoovBox> {
  mdra(input).map(|_| MoovBox::Mdra)
}

tag!(dref    "dref".as_bytes());
fn moov_dref(input:&[u8]) -> IResult<&[u8], MoovBox> {
  dref(input).map(|_| MoovBox::Dref)
}

tag!(cmov    "cmov".as_bytes());
fn moov_cmov(input:&[u8]) -> IResult<&[u8], MoovBox> {
  cmov(input).map(|_| MoovBox::Cmov)
}

tag!(rmra    "rmra".as_bytes());
fn moov_rmra(input:&[u8]) -> IResult<&[u8], MoovBox> {
  rmra(input).map(|_| MoovBox::Rmra)
}

tag!(iods    "iods".as_bytes());
fn moov_iods(input:&[u8]) -> IResult<&[u8], MoovBox> {
  iods(input).map(|_| MoovBox::Iods)
}

fn mvhd_box(input:&[u8]) -> IResult<&[u8],MvhdBox> {
  if input.len() < 100 {
    Incomplete(0)
  } else if input.len() == 100 {
    mvhd32(input)
  } else if input.len() == 112 {
    mvhd64(input)
  } else {
    Error(0)
  }
}

tag!(mvhd    "mvhd".as_bytes());
chain!(moov_mvhd<&[u8],MoovBox>,
    mvhd              ~
    content: mvhd_box ,
    || { MoovBox::Mvhd(content) }
);

tag!(clip    "clip".as_bytes());
fn moov_clip(input:&[u8]) -> IResult<&[u8], MoovBox> {
  clip(input).map(|_| MoovBox::Clip)
}

tag!(trak    "trak".as_bytes());
fn moov_trak(input:&[u8]) -> IResult<&[u8], MoovBox> {
  trak(input).map(|_| MoovBox::Trak)
}

tag!(udta   "udta".as_bytes());
fn moov_udta(input:&[u8]) -> IResult<&[u8], MoovBox> {
  udta(input).map(|_| MoovBox::Udta)
}

alt!(moov_internal<&[u8], MoovBox>, moov_mdra | moov_dref | moov_cmov |
     moov_rmra | moov_iods | moov_mvhd | moov_clip | moov_trak | moov_udta);


fn moov(input:&[u8]) -> IResult<&[u8], MoovBox> {
  match mp4_box(input) {
    Error(a)      => Error(a),
    Incomplete(a) => Incomplete(a),
    Done(i, o)    => {
      match moov_internal(o) {
        Error(a)      => Error(a),
        Incomplete(a) => Incomplete(a),
        Done(i2, o2)  => {
          Done(i, o2)
        }

      }
    }
  }
}

many0!(moov_many<&[u8],MoovBox> moov);

o!(moov_box_internal <&[u8], Vec<MoovBox> >  moov_tag ~ [ moov_many ]);

fn moov_box(input:&[u8]) -> IResult<&[u8], MP4Box> {
  match moov_box_internal(input) {
    Error(a)      => Error(a),
    Incomplete(a) => Incomplete(a),
    Done(i, o)  => {
      Done(i, MP4Box::Moov(o))
    }
  }
}

tag!(mdat    "mdat".as_bytes());
fn mdat_box(input:&[u8]) -> IResult<&[u8], MP4Box> {
  mdat(input).map(|_| MP4Box::Mdat)
}
tag!(free    "free".as_bytes());
fn free_box(input:&[u8]) -> IResult<&[u8], MP4Box> {
  free(input).map(|_| MP4Box::Free)
}

tag!(skip    "skip".as_bytes());
fn skip_box(input:&[u8]) -> IResult<&[u8], MP4Box> {
  skip(input).map(|_| MP4Box::Skip)
}

tag!(wide    "wide".as_bytes());
fn wide_box(input:&[u8]) -> IResult<&[u8], MP4Box> {
  wide(input).map(|_| MP4Box::Wide)
}

fn unknown_box(input:&[u8]) -> IResult<&[u8], MP4Box> {
  Done(input, MP4Box::Unknown)
}

alt!(box_parser_internal<&[u8], MP4Box>, filetype_box | moov_box | mdat_box | free_box | skip_box | wide_box | unknown_box);
fn box_parser(input:&[u8]) -> IResult<&[u8], MP4Box> {
  mp4_box(input).flat_map(box_parser_internal)
}

fn data_interpreter(bytes:&[u8]) -> IResult<&[u8], ()> {
  //println!("bytes:\n{}", bytes.to_hex(8));
  //println!("bytes length: {}", bytes.len());
  match box_parser(bytes) {
    Done(i, o) => {
      /*match o {
        MP4Box::Ftyp(f) => println!("-> FTYP: {:?}", f),
        MP4Box::Moov(m) => println!("-> MOOV: {:?}", m),
        MP4Box::Mdat    => println!("-> MDAT"),
        MP4Box::Free    => println!("-> FREE"),
        MP4Box::Skip    => println!("-> SKIP"),
        MP4Box::Wide    => println!("-> WIDE"),
        MP4Box::Unknown => println!("-> UNKNOWN")
      }*/
      //println!("remaining:\n{}", i.to_hex(8));
      //println!("got o");
      Done(i,())
    },
    Error(a) => {
      println!("mp4 parsing error: {:?}", a);
      assert!(false);
      Error(a)
    },
    Incomplete(a) => {
      println!("mp4 incomplete: {:?}", a);
      Incomplete(a)
    }
  }
}

many0!(full_data_interpreter<&[u8],()> data_interpreter);

use test::Bencher;
#[bench]
fn mp4_test(b: &mut Bencher) {
  //let data = include_bytes!("../small.mp4");
  let data = include_bytes!("../bigbuckbunny.mp4");
  b.iter(||{
    full_data_interpreter(data)
  });
}

fn main() {
  println!("Hello, world!");
  let data = include_bytes!("../small.mp4");
  full_data_interpreter(data);
  //parse_mp4_file("./small.mp4");
  //parse_mp4_file("./bigbuckbunny.mp4");
}
