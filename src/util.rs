//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::time::Duration;

pub trait DurationExt {
	fn as_msecs(&self) -> u64;
	fn as_nanosecs(&self) -> u64;
}

impl DurationExt for Duration {
	fn as_msecs(&self) -> u64 {
		self.as_secs() * 1_000 + (self.subsec_nanos() / 1_000_000) as u64
	}

	fn as_nanosecs(&self) -> u64 {
		self.as_secs() * 1_000_000_000 + self.subsec_nanos() as u64
	}
}
