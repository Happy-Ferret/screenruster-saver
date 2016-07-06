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
//  0. You just DO WHAT THE FUCK YOU WANT TO.use std::io::{Read, BufRead, BufReader, Write};

use std::fmt;
use std::error;
use std::io;

use gl;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	ContextCreation(gl::GliumCreationError<Display>),
	SwapBuffers(gl::SwapBuffersError),
	Protocol,
}

#[derive(Debug)]
pub enum Display {
	NotFound,
	Visual,
	Context,
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::Io(value)
	}
}

impl From<Display> for Error {
	fn from(value: Display) -> Self {
		Error::ContextCreation(gl::GliumCreationError::BackendCreationError(value))
	}
}

impl From<gl::GliumCreationError<Display>> for Error {
	fn from(value: gl::GliumCreationError<Display>) -> Self {
		Error::ContextCreation(value)
	}
}

impl From<gl::SwapBuffersError> for Error {
	fn from(value: gl::SwapBuffersError) -> Self {
		Error::SwapBuffers(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Io(ref err) =>
				err.description(),

			Error::ContextCreation(..) =>
				"OpenGL error.",

			Error::SwapBuffers(ref err) =>
				err.description(),

			Error::Protocol =>
				"Protocol error.",
		}
	}
}
