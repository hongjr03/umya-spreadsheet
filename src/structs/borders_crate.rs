// borders
use super::Borders;
use super::Style;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub(crate) struct BordersCrate {
    borders: ThinVec<Borders>,
}

impl BordersCrate {
    #[inline]
    pub(crate) fn get_borders(&self) -> &[Borders] {
        &self.borders
    }

    #[inline]
    pub(crate) fn get_borders_mut(&mut self) -> &mut ThinVec<Borders> {
        &mut self.borders
    }

    #[inline]
    pub(crate) fn set_borders(&mut self, value: Borders) -> &mut Self {
        self.borders.push(value);
        self
    }

    pub(crate) fn set_style(&mut self, style: &Style) -> u32 {
        match style.get_borders() {
            Some(v) => {
                let hash_code = v.get_hash_code();
                let mut id = 0;
                for borders in &self.borders {
                    if borders.get_hash_code() == hash_code {
                        return id;
                    }
                    id += 1;
                }
                self.set_borders(v.clone());
                id
            }
            None => 0,
        }
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"border" {
                    let obj = Borders::default();
                    self.set_borders(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"border" {
                    let mut obj = Borders::default();
                    obj.set_attributes(reader, e);
                    self.set_borders(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"borders" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "borders")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.borders.is_empty() {
            // borders
            write_start_tag(
                writer,
                "borders",
                vec![("count", &self.borders.len().to_string())],
                false,
            );

            // border
            for border in &self.borders {
                border.write_to(writer);
            }

            write_end_tag(writer, "borders");
        }
    }
}
