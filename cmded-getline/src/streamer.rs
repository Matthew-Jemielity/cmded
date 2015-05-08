use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use std::io::stdin;
use std::io::Stdin;

enum Stream {
    Live(Stdin),
    File(File)
}

pub struct InputStream {
    which: Stream,
}

impl InputStream {
    fn live(from: Stdin) -> InputStream {
        InputStream { which: Stream::Live(from) }
    }
    fn file(from: File) -> InputStream {
        InputStream { which: Stream::File(from) }
    }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        match self.which {
            Stream::Live(ref mut from) => from.read(buf),
            Stream::File(ref mut from) => from.read(buf)
        }
    }
}

pub fn from(path: &str) -> Result<BufReader<InputStream>, Error> {
    let input = try!(match path {
        "-" => Ok(InputStream::live(stdin())),
        _ => || -> Result<InputStream, Error> {
            let file = try!(File::open(path));
            Ok(InputStream::file(file))
        } ()
    });
    Ok(BufReader::new(input))
}

