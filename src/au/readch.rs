use super::prelude::*;
use nix::{
  errno::Errno,
  sys::termios::{self, InputFlags, LocalFlags, SetArg, Termios},
  unistd,
};
use std::{os::unix::io::RawFd, str};

pub struct Readch {
  fd: RawFd,
  config: Termios,
}

impl Readch {
  pub fn new(fd: RawFd) -> Result<Self> {
    let mut config = termios::tcgetattr(fd)?;

    config
      .input_flags
      .remove(InputFlags::IGNBRK | InputFlags::BRKINT | InputFlags::IXON);

    config.input_flags.insert(InputFlags::IUTF8);

    config
      .local_flags
      .remove(LocalFlags::ECHO | LocalFlags::ICANON | LocalFlags::ISIG);

    Ok(Self { fd, config })
  }

  pub fn read(&self) -> Result<char> {
    let prev_attr = termios::tcgetattr(self.fd)?;

    termios::tcsetattr(self.fd, SetArg::TCSANOW, &self.config)?;

    let mut buf = vec![0 as u8];

    let ret = loop {
      let end = buf.len();

      // TODO: catch escape sequences
      match unistd::read(self.fd, &mut buf[end - 1..end]) {
        Ok(0) => break '\x04',
        Ok(1) => match str::from_utf8(&buf[..]) {
          Ok(s) => break s.chars().next().unwrap(),
          Err(_) => {},
        },
        Ok(_) => unreachable!(),
        Err(nix::Error::Sys(Errno::EINTR)) => {},
        Err(e) => return Err(e.into()),
      };
    };

    termios::tcsetattr(self.fd, SetArg::TCSANOW, &prev_attr)?;

    Ok(ret)
  }
}
