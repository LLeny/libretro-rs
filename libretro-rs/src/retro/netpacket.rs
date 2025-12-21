
use crate::ffi::*;
use crate::prelude::*;
use core::ffi::c_int;

#[derive(Clone, Copy, Debug)]
pub struct NetpacketSender {
  send: non_null_retro_netpacket_send_t,
}

impl NetpacketSender {
  pub const fn new(send: non_null_retro_netpacket_send_t) -> Self {
    Self { send }
  }

  pub fn from_option(send: retro_netpacket_send_t) -> Option<Self> {
    send.map(Self::new)
  }

  pub unsafe fn send(&self, flags: u16, client_id: u16, data: &[u8]) -> Result<(), &'static str> {
    if data.len() > 65536 {
      return Err("netpacket payload too large");
    }
    (self.send)(flags as c_int, data.as_ptr() as *const _, data.len(), client_id);
    Ok(())
  }

  pub unsafe fn flush(&self, flags: u16, client_id: u16) {
    (self.send)(flags as c_int, core::ptr::null(), 0, client_id);
  }
}

#[derive(Clone, Copy, Debug)]
pub struct NetpacketPollReceive {
  poll_receive: non_null_retro_netpacket_poll_receive_t,
}

impl NetpacketPollReceive {
  pub const fn new(poll_receive: non_null_retro_netpacket_poll_receive_t) -> Self {
    Self { poll_receive }
  }

  pub fn from_option(poll_receive: retro_netpacket_poll_receive_t) -> Option<Self> {
    poll_receive.map(Self::new)
  }

  pub unsafe fn poll_receive(&self) {
    (self.poll_receive)()
  }
}
