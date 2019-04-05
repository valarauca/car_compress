
//! Compression/Decompression
//! ---
//!
//! This library consistents of a collection of compression
//! and decompression algorithms. It very likely contains
//! what you need in order to handle what you need.
//!
//! ## How to use this library
//!
//! This library exposes the native compression functions,
//! as well general encapsulating `Comp` and `Decomp` types
//! which allow for dynamic, and wide ranging compression
//! without code changes.
//! This goes so far as to the support automatic compression
//! type detection for decompresion.
//!
//! There is also a collection of `lib$ALGORITHM` named
//! modules which publically expose some of the implemnation
//! details of the underlying algorith.
//!
//! ## What Algorithm do I want?
//!
//! [This may help](https://quixdb.github.io/squash-benchmark/).
//! Set the data type to approximate yours, set the transfer speed
//! to approximate  yours, then compare the compression (write)
//! times to the decompression (read) times and try to see if you
//! save time either decompressing every read, or save time
//! compression / decompressing because what you are moving
//! is big, or slow.
//!
//! ## Decompression
extern crate lz4;
pub mod liblz4 {
    //! LZ4 - fast but dumb
    //!
    //! Very fast, not an amazing compression ratio.
    //! Fast enough to use over highspeed PCI-e links
    //! and still get a speed up.
    //!
    //! Internal library is a mild work in progress.
    pub use super::lz4::{BlockSize as BSize, BlockMode as BMode, ContentChecksum as Checksum,
                         Decoder as Decode, Encoder as Encode, EncoderBuilder as Builder};
}

extern crate zstd;
pub mod libzstd {
    //! ZSTD - [tune-able lz4](https://facebook.github.io/zstd/)
    //!
    //! Tune-able compression ratio and speed. It can beat
    //! `zlib`, and `xz2` in eithers usecase depending on settings.
    pub use super::zstd::{Decoder as Decode, Encoder as Encode};
}

extern crate snap;
pub mod libsnap {
    //! SNAPPY - google's disappointing answer to lz4
    //!
    //! snappy is built to be a lot like lz4, very quick
    //! but very dumb, good for snappy in memory stuff
    //! over quick data center links.
    pub use super::snap::{Reader as Decode, Writer as Encode};
}

extern crate xz2;
pub mod libxz {
    //! XZ (LZMA2) - 7zip (kind of)
    //!
    //! LZMA is a nice compression algorithm on paper
    //! but the author of LZMA (and 7zip) wrote some
    //! extremely accursed c code nobody on the internet
    //! could make heads nor tails of.
    //!
    //! There was only one LZMA tool in town `7zip`.
    //! It didn't follow the unix cli conventions because
    //! why would a win32 program do that?
    //!
    //! BUUUUT LZMA is actually really good for
    //! archival compression, and the spice must flow!
    //!
    //! So `xz` arrivied, which speaks `LZMA2`, and
    //! can decompress `LZMA`.
    pub use super::xz2::read::XzDecoder as Decode;
    pub use super::xz2::write::XzEncoder as Encode;
}


extern crate flate2;
pub mod libflate {
    //! GZIP - young gz always there for you
    pub use super::flate2::Compression as GzQuality;
    pub use super::flate2::write::GzEncoder as Encode;
    pub use super::flate2::read::GzDecoder as Decode;
}

extern crate brotli2;
pub mod libbrotli {
    //! Brotli - Modern (2010's) Tunable Compression
    //!
    //! Tune-able compression ratio and speed. It can beat
    //! `zlib`, and `xz2` in eithers usecase depending on settings.
    //!
    //! The highest levels aren't great.
    pub use super::brotli2::write::BrotliEncoder as Encode;
    pub use super::brotli2::read::BrotliDecoder as Decode;
    pub use super::brotli2::stream::{CompressParams as Builder, CompressMode as Mode};
}

extern crate bzip2;
pub mod libbzip {
    //! BZIP2 - RUNNING THROUGH THE 90'S
    //!
    //! BZIP2 was better then `gzip`, in the way that `xz` is now.
    //! It was good archival compression, slow to write, but
    //! you'll read it a few times and that'll get made up for.
    //!
    //! `xz` was a lot better, it also compressed _at the same rate_
    //! as `bzip2`, and got a better ratio which is said for `bzip2`.
    //!
    //! Presently `brotli` and `zstd` are beating out `xz`
    //! so `bzip2` mostly hangs around because it got
    //! cc'd on a lot of weird 90's standards that are still
    //! in use today like `zip` files (and `jar` files).
    //!
    //! Also a lot of `perl` and `php` packages use it,
    //! and functionally those langauges control >30%
    //! of the internet's content (`facebook.com`, `pornhub.com`)
    //! they are _still relevant today_.
    pub use super::bzip2::write::BzEncoder as Encode;
    pub use super::bzip2::read::BzDecoder as Decode;
    pub use super::bzip2::Compression as BzQuality;
}

mod header;
mod comp;

pub use self::header::{Quality, Format};

pub use self::comp::{Decomp, Comp};
