// rustywatch - Stopwatch for the command line
// Copyright (C) 2022 Ingo Ruhnke <grumbel@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io::{self, Write};

fn main()
{
    let millissec_to_hours    = 1000 * 60 * 60;
    let millissec_to_minutes  = 1000 * 60;
    let millissec_to_seconds  = 1000;

    let clock = Instant::now();
    loop {
        let elapsed = clock.elapsed();
        let mut remaining = elapsed.as_millis();

        let hours = remaining / millissec_to_hours; remaining -= hours * millissec_to_hours;
        let minutes = remaining / millissec_to_minutes; remaining -= minutes * millissec_to_minutes;
        let seconds = remaining / millissec_to_seconds; remaining -= seconds * millissec_to_seconds;
        let millisec = remaining;

        print!("Time: {:02}:{:02}:{:02}'{:03}\r", hours, minutes, seconds, millisec);
        io::stdout().flush().unwrap();

        sleep(Duration::from_millis(10));
    }
}

/* EOF */
