use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::RawHandle;
use std::{io, ptr};

use crate::ipc::client::{Connection, RichClient};
use crate::ipc::utils;
use crate::rpc::packet::Packet;

const FILE_GENERIC_READ: u32 = 0x80000000;
const FILE_GENERIC_WRITE: u32 = 0x40000000;
const OPEN_EXISTING: u32 = 3;
const FILE_FLAG_OVERLAPPED: u32 = 0x40000000;
const INVALID_HANDLE_VALUE: RawHandle = -1isize as RawHandle;

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
struct OVERLAPPED {
    Internal: usize,
    InternalHigh: usize,
    Offset: u32,
    OffsetHigh: u32,
    hEvent: RawHandle,
}

impl Default for OVERLAPPED {
    fn default() -> Self {
        Self {
            Internal: 0,
            InternalHigh: 0,
            Offset: 0,
            OffsetHigh: 0,
            hEvent: ptr::null_mut(),
        }
    }
}

extern "system" {
    fn CreateFileW(
        lpFileName: *const u16,
        dwDesiredAccess: u32,
        dwShareMode: u32,
        lpSecurityAttributes: *mut u8,
        dwCreationDisposition: u32,
        dwFlagsAndAttributes: u32,
        hTemplateFile: RawHandle,
    ) -> RawHandle;
    fn ReadFile(
        hFile: RawHandle,
        lpBuffer: *mut u8,
        nNumberOfBytesToRead: u32,
        lpNumberOfBytesRead: *mut u32,
        lpOverlapped: *mut OVERLAPPED,
    ) -> i32;
    fn WriteFile(
        hFile: RawHandle,
        lpBuffer: *const u8,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: *mut u32,
        lpOverlapped: *mut OVERLAPPED,
    ) -> i32;
    fn CloseHandle(hObject: RawHandle) -> i32;
}

impl Connection for RichClient {
    fn connect(client_id: u64) -> Result<Self, Box<dyn std::error::Error>> {
        for i in 0..10 {
            let pipe_name = format!(r"\\.\pipe\discord-ipc-{}", i);
            let pipe_name_wide: Vec<u16> = OsStr::new(&pipe_name)
                .encode_wide()
                .chain(Some(0))
                .collect();

            unsafe {
                let pipe = CreateFileW(
                    pipe_name_wide.as_ptr(),
                    FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                    0,
                    ptr::null_mut(),
                    OPEN_EXISTING,
                    FILE_FLAG_OVERLAPPED,
                    ptr::null_mut(),
                );

                if pipe != INVALID_HANDLE_VALUE {
                    return Ok(RichClient {
                        client_id,
                        pipe: Some(pipe),
                        last_activity: None,
                    });
                } else {
                    let err = io::Error::last_os_error();
                    if err.kind() == io::ErrorKind::NotFound {
                        continue;
                    } else {
                        return Err(err.into());
                    }
                }
            }
        }

        Err("Pipe not found".into())
    }

    fn write(&mut self, opcode: u32, data: Option<&[u8]>) -> io::Result<()> {
        if let Some(pipe) = self.pipe {
            let payload = match data {
                Some(packet) => {
                    let mut payload =
                        utils::encode(opcode, packet.len() as u32);
                    payload.extend_from_slice(packet);
                    payload
                }
                None => utils::encode(opcode, 0),
            };

            let mut bytes_written: u32 = 0;
            let mut overlapped = OVERLAPPED::default();

            unsafe {
                let result = WriteFile(
                    pipe,
                    payload.as_ptr(),
                    payload.len() as u32,
                    &mut bytes_written,
                    &mut overlapped,
                );

                if result == 0 {
                    let err = io::Error::last_os_error();
                    if err.raw_os_error() != Some(997) {
                        return Err(err);
                    }
                }
            }
        }
        Ok(())
    }

    fn read(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if let Some(pipe) = self.pipe {
            let mut header = [0; 8];
            let mut bytes_read: u32 = 0;
            let mut overlapped = OVERLAPPED::default();

            unsafe {
                let result = ReadFile(
                    pipe,
                    header.as_mut_ptr(),
                    header.len() as u32,
                    &mut bytes_read,
                    &mut overlapped,
                );

                if result == 0 {
                    let err = io::Error::last_os_error();
                    if err.raw_os_error() != Some(997) {
                        return Err(err.into());
                    }
                }

                let size = utils::decode(&header) as usize;
                let mut buffer = vec![0u8; size];

                let result = ReadFile(
                    pipe,
                    buffer.as_mut_ptr(),
                    size as u32,
                    &mut bytes_read,
                    &mut overlapped,
                );

                if result == 0 {
                    let err = io::Error::last_os_error();
                    if err.raw_os_error() != Some(997) {
                        return Err(err.into());
                    }
                }
                Ok(buffer)
            }
        } else {
            Err("Pipe not found".into())
        }
    }

    fn close(&mut self) {
        if let Some(pipe) = self.pipe.take() {
            unsafe {
                CloseHandle(pipe);
            }
        }
    }

    fn handshake(&mut self) -> io::Result<()> {
        self.write(
            0,
            Some(
                format!("{{\"v\": 1,\"client_id\":\"{}\"}}", self.client_id)
                    .as_bytes(),
            ),
        )
    }

    fn update(
        &mut self,
        packet: &crate::rpc::packet::Packet,
    ) -> io::Result<()> {
        if packet.activity != self.last_activity {
            return self.write(1, Some(packet.to_json().unwrap().as_bytes()));
        }

        Ok(())
    }

    fn clear(&mut self) -> io::Result<()> {
        self.write(
            1,
            Some(
                Packet {
                    pid: std::process::id(),
                    activity: None,
                }
                .to_json()
                .unwrap()
                .as_bytes(),
            ),
        )
    }
}
