// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::util::{decode, element_text, skip};
use crate::Error;

/// Represents a torrent in an RSS item.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Torrent {
    /// The URL of the torrent.
    pub link: Option<String>,
    /// The length of the torrent.
    pub content_length: Option<String>,
    /// The publication date of the torrent.
    pub pub_date: Option<String>,
}

impl Torrent {
    /// Builds an Torrent from source XML
    pub fn from_xml<R: BufRead>(reader: &mut Reader<R>) -> Result<Self, Error> {
        let mut torrent = Torrent::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(element) => match decode(element.name().as_ref(), reader)?.as_ref() {
                    "link" => torrent.link = element_text(reader)?,
                    "contentLength" => torrent.content_length = element_text(reader)?,
                    "pubDate" => torrent.pub_date = element_text(reader)?,
                    _ => skip(element.name(), reader)?,
                },
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }
            buf.clear();
        }
        Ok(torrent)
    }
}
