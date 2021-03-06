extern crate libc;

use libc::c_char;
use libc::c_uint;

use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;
use std::collections::HashMap;

use std::mem::transmute;
use std::error::Error;
use std::str;

use std::c_str::CString;

#[allow(dead_code)]
pub const CASS_ERROR_LAST_ENTRY: ::libc::c_uint = 50331654;
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub const CASS_OK:u32 = 0;



#[allow(dead_code)]
pub enum CassErrorSourceType {
  NONE=0,
  LIB=1,
  SERVER=2,
  SSL=3,
  COMPRESSION=4,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[deriving(Eq)]
#[deriving(PartialEq)]
pub enum CassErrorLibType {
  BAD_PARAMS=16777217,
  NO_STREAMS=16777218,
  UNABLE_TO_INIT=16777219,
  MESSAGE_ENCODE=16777220,
  HOST_RESOLUTION=16777221,
  UNEXPECTED_RESPONSE=16777222,
  REQUEST_QUEUE_FULL=16777223,
  NO_AVAILABLE_IO_THREAD=16777224,
  WRITE_ERROR=16777225,
  NO_HOSTS_AVAILABLE=16777226,
  INDEX_OUT_OF_BOUNDS=16777227,
  INVALID_ITEM_COUNT=16777228,
  INVALID_VALUE_TYPE=16777229,
  REQUEST_TIMED_OUT=16777230,
  UNABLE_TO_SET_KEYSPACE=16777231,
  CALLBACK_ALREADY_SET=16777232,
  INVALID_STATEMENT_TYPE=16777233,
  NAME_DOES_NOT_EXIST=16777234,
  UNABLE_TO_DETERMINE_PROTOCOL=16777235,
  NULL_VALUE=16777236,
  NOT_IMPLEMENTED=16777237,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum CassErrorServerType {
  SERVER_ERROR=33554432,
  PROTOCOL_ERROR=33554442,
  BAD_CREDENTIALS=33554688,
  UNAVAILABLE=33558528,
  OVERLOADED=33558529,
  IS_BOOTSTRAPPING=33558530,
  TRUNCATE_ERROR=33558531,
  WRITE_TIMEOUT=33558784,
  READ_TIMEOUT=33559040,
  SYNTAX_ERROR=33562624,
  UNAUTHORIZED=33562880,
  INVALID_QUERY=33563136,
  CONFIG_ERROR=33563392,
  ALREADY_EXISTS=33563648,
  UNPREPARED=33563904,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum CassErrorSSLType {
  INVALID_CERT=50331649,
  INVALID_PRIVATE_KEY=50331650,
  NO_PEER_CERT=50331651,
  INVALID_PEER_CERT=50331652,
  IDENTITY_MISMATCH=50331653,
  NOT_IMPLEMENTED=50331654,
}


pub struct CassError {
  pub err:_CassError,
}

impl Show for CassError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {unsafe{
    let err = self.err;
    let err = cass_error_desc(err);
    let err = CString::new(err, false);
    write!(f, "Error:{}", err)
  }}
}

impl Error for CassError {
  fn description(&self) -> &str {unsafe{
    str::from_c_str(cass_error_desc(self.err))
  }}
}

impl Copy for CassError {}

impl CassError {
  pub fn new(err:u32) -> CassError {
    CassError{err:err}
  }

  pub fn is_error(&self) -> bool {
    if self.err != CASS_OK {true} else {false}
  }
}

  type CassErrorSource = c_uint;  
  pub type _CassError = c_uint;
  #[link(name = "cassandra")]
  extern "C" {
    pub fn cass_error_desc(error: _CassError) -> *const c_char;
  }

