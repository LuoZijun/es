use crate::vlq;
use crate::serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::lexer::span::{ Loc, Span, LineColumn, };


use std::io::{ Write, Cursor, };
use std::path::{Path, PathBuf};

// Source Map Revision 3 Proposal
//      https://docs.google.com/document/d/1U1RGAehQwRypUTovF1KRlpiOFze0b-_2gc6fAH0KY0k/edit#
//      http://www.ruanyifeng.com/blog/2013/01/javascript_source_map.html
// 
// Tail
// @ sourceMappingURL=/path/to/file.js.map
// 
// JSON Format:
// {
//     "version" : 3,
//     "file": "out.js",
//     "sourceRoot": "",
//     "sources": ["foo.js", "bar.js"],
//     "sourcesContent": [null, null],
//     "names": ["src", "maps", "are", "fun"],
//     "mappings": "A,AAAB;;ABCDE;"
// }
// 
// SourceMap example:
// http://ajax.googleapis.com/ajax/libs/jquery/1.9.0/jquery.min.js
// http://ajax.googleapis.com/ajax/libs/jquery/1.9.0/jquery.min.map

const VERSION: u8      = 3u8;
const COMMA: &[u8]     = b",";
const SEMICOLON: &[u8] = b";";


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub dst_column: usize,          // A
    pub src_file_index: usize,      // B
    pub src_line: usize,            // C
    pub src_column: usize,          // D
    pub ident_index: Option<usize>, // E
}


#[derive(Debug)]
pub struct SourceMap<'a> {
    file: PathBuf,
    source_root: Option<PathBuf>,
    sources: Vec<PathBuf>,
    sources_content: &'a [&'a str],
    names: &'a [&'a [char]],
    mappings: Cursor<Vec<u8>>,
}

impl<'a> SourceMap<'a> {
    

    pub fn new<T>(dst_filepath: T,
                  src_filepaths: Vec<PathBuf>,
                  src_contents: &'a [&'a str],
                  src_idents: &'a [&'a [char]],) -> Self
    where
        T: Into<PathBuf> 
    {
        Self {
            file: dst_filepath.into(),
            source_root: None,
            sources: src_filepaths,
            sources_content: src_contents,
            names: src_idents,
            mappings: Cursor::new(Vec::with_capacity(src_idents.len() * 5 )),
        }
    }

    pub fn add_line(&mut self) {
        self.mappings.write(SEMICOLON);
    }

    pub fn add_pos(&mut self, pos: Position) {
        let is_beginning = match self.mappings.get_ref().last() {
            Some(last) => last == &SEMICOLON[0],
            None => true,
        };

        if !is_beginning {
            self.mappings.write(COMMA);
        }
        
        vlq::encode(pos.dst_column as i64, &mut self.mappings).expect("Ooops ...");
        vlq::encode(pos.src_file_index as i64, &mut self.mappings).expect("Ooops ...");
        vlq::encode(pos.src_line as i64, &mut self.mappings).expect("Ooops ...");
        vlq::encode(pos.src_column as i64, &mut self.mappings).expect("Ooops ...");

        if let Some(ident_index) = pos.ident_index {
            vlq::encode(ident_index as i64, &mut self.mappings).expect("Ooops ...");
        }
    }

    pub fn tail(&self) -> String {
        format!("//@ sourceMappingURL=/path/to/file.js.map")
    }
}




impl<'a> Serialize for SourceMap<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("", 1)?;
        
        s.serialize_field("version", &VERSION)?;
        s.serialize_field("file", &self.file)?;
        
        match self.source_root {
            Some(ref root) => s.serialize_field("sourceRoot", &root)?,
            _ => s.serialize_field("sourceRoot", "")?,
        }

        s.serialize_field("sources", &self.sources)?;
        s.serialize_field("sourcesContent", &self.sources_content)?;
            
        let names = self.names.iter().map(|name| name.iter().collect::<String>()).collect::<Vec<String>>();
        s.serialize_field("names", &names)?;

        let mappings = unsafe {
            String::from_utf8_unchecked(self.mappings.get_ref().to_owned())
        };

        s.serialize_field("mappings", &mappings )?;

        s.end()
    }
}



#[test]
fn test_serialize() {
    use crate::toolshed::{ Arena, };
    use crate::serde_json;

    use std::io::{ Write, Cursor, };
    use std::path::{Path, PathBuf};


    let arena = Arena::new();

    let file = "dist/main.js".into();
    let source_root = None;
    let sources = vec![];
    let sources_content = arena.alloc_vec(vec![ arena.alloc_str("这是一份源代码:)") ]);

    let names = arena.alloc_vec(vec![
        arena.alloc_vec("let".chars().collect::<Vec<char>>()),
    ]);
    let mappings = Cursor::new(Vec::new());

    let source_map = SourceMap { file, source_root, sources, sources_content, names, mappings };

    let res = serde_json::to_string(&source_map);
    assert_eq!(res.is_ok(), true);

    let res = res.unwrap();
    let json = r#"{"version":3,"file":"dist/main.js","sourceRoot":"","sources":[],"sourcesContent":["这是一份源代码:)"],"names":["let"],"mappings":""}"#;
    assert_eq!(res, json);
}