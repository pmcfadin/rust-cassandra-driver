#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum CassErrorSourceType {
  PORT = 0,
  CQL_VERSION = 1,
  NUM_THREADS_IO = 2,
  QUEUE_SIZE_IO = 3,
  QUEUE_SIZE_EVENTS = 4,
  CONTACT_POINTS = 5,
  CORE_CONNECTIONS_PER_HOST = 6,
  MAX_CONNECTIONS_PER_HOST = 7,
  MAX_SIMULTANEOUS_CREATION = 8,
  WAIT_TIME = 9,
  CONNECT_TIMEOUT = 10,
  WRITE_TIMEOUT = 11,
  READ_TIMEOUT = 12,
  LOG_LEVEL = 13,
  LOG_DATA = 14,
  LOG_CALLBACK = 15
}
