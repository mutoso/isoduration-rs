/*
    Copyright © 2021 Alastair Feille

    Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
    http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
    <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
    option. This file may not be copied, modified, or distributed
    except according to those terms.

    SPDX-License-Identifier: MIT OR Apache-2.0
*/

use std::{collections::HashMap,
          fmt::Display,
          num::ParseIntError,
          str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Duration
{
    Weeks
    {
        weeks: u64
    },
    Full
    {
        years:   u64,
        months:  u8,
        days:    u8,
        hours:   u8,
        minutes: u8,
        seconds: u8,
    },
}

impl FromStr for Duration
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        lazy_static! {
            static ref WEEKS: Regex = Regex::new(r"^P(?P<weeks>[0-9]*)W$").unwrap();
            static ref FULL: Regex = Regex::new(r"^P((?P<years>\d+)Y)?((?P<months>\d+)M)?((?P<days>\d+)D)?(T((?P<hours>\d+)H)?((?P<minutes>\d+)M)?((?P<seconds>\d+)S)?)?$").unwrap();
        }
        if WEEKS.is_match(s)
        {
            let caps = WEEKS.captures(s).unwrap();
            return Ok(Duration::Weeks { weeks: caps.name("weeks")
                                                   .unwrap()
                                                   .as_str()
                                                   .parse()
                                                   .map_err(|e: ParseIntError| e.to_string())?, });
        }
        else if FULL.is_match(s)
        {
            let captures = FULL.captures(s).unwrap();
            let dict: HashMap<&str, &str> =
                FULL.capture_names()
                    .flatten()
                    .filter_map(|n| Some((n, captures.name(n)?.as_str())))
                    .collect();
            let d = Duration::Full { years:   dict.get("years")
                                                  .unwrap_or(&"0")
                                                  .parse()
                                                  .map_err(|e: ParseIntError| e.to_string())?,
                                     months:  dict.get("months")
                                                  .unwrap_or(&"0")
                                                  .parse()
                                                  .map_err(|e: ParseIntError| e.to_string())?,
                                     days:    dict.get("days")
                                                  .unwrap_or(&"0")
                                                  .parse()
                                                  .map_err(|e: ParseIntError| e.to_string())?,
                                     hours:   dict.get("hours")
                                                  .unwrap_or(&"0")
                                                  .parse()
                                                  .map_err(|e: ParseIntError| e.to_string())?,
                                     minutes: dict.get("minutes")
                                                  .unwrap_or(&"0")
                                                  .parse()
                                                  .map_err(|e: ParseIntError| e.to_string())?,
                                     seconds: dict.get("seconds")
                                                  .unwrap_or(&"0")
                                                  .parse()
                                                  .map_err(|e: ParseIntError| e.to_string())?, };
            return Ok(d);
        }
        else
        {
            return Err("Doesn't match format".to_string());
        }
    }
}

impl Display for Duration
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let mut s = "P".to_string();
        match *self
        {
            Duration::Weeks { weeks } => s.push_str(&format!("{}W", weeks)),
            Duration::Full { years,
                             months,
                             days,
                             hours,
                             minutes,
                             seconds, } =>
            {
                if years > 0
                {
                    s.push_str(&format!("{}Y", years));
                }
                if months > 0
                {
                    s.push_str(&format!("{}M", months));
                }
                if days > 0
                {
                    s.push_str(&format!("{}D", days));
                }
                if hours > 0 || minutes > 0 || seconds > 0
                {
                    s.push_str("T");
                }
                if hours > 0
                {
                    s.push_str(&format!("{}H", hours));
                }
                if minutes > 0
                {
                    s.push_str(&format!("{}M", minutes));
                }
                if seconds > 0
                {
                    s.push_str(&format!("{}S", seconds));
                }
            },
        }
        write!(f, "{}", s)
    }
}
