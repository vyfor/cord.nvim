#![allow(clippy::upper_case_acronyms)]

pub mod client;
pub mod server;

pub type HANDLE = *mut std::ffi::c_void;
pub type DWORD = u32;
pub type BOOL = i32;
pub type LPCWSTR = *const u16;
pub type LPVOID = *mut std::ffi::c_void;

pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
pub const ERROR_PIPE_CONNECTED: DWORD = 535;
pub const ERROR_IO_PENDING: DWORD = 997;
pub const PIPE_ACCESS_DUPLEX: DWORD = 0x00000003;
pub const FILE_FLAG_OVERLAPPED: DWORD = 0x40000000;
pub const PIPE_TYPE_MESSAGE: DWORD = 0x00000004;
pub const PIPE_READMODE_MESSAGE: DWORD = 0x00000002;
pub const PIPE_UNLIMITED_INSTANCES: DWORD = 255;
pub const WAIT_OBJECT_0: DWORD = 0;
pub const INFINITE: DWORD = 0xFFFFFFFF;

#[repr(C)]
pub struct Overlapped {
    pub internal: usize,
    pub internal_high: usize,
    pub offset: DWORD,
    pub offset_high: DWORD,
    pub h_event: HANDLE,
}

extern "system" {
    pub fn CreateNamedPipeW(
        lpName: LPCWSTR,
        dwOpenMode: DWORD,
        dwPipeMode: DWORD,
        nMaxInstances: DWORD,
        nOutBufferSize: DWORD,
        nInBufferSize: DWORD,
        nDefaultTimeOut: DWORD,
        lpSecurityAttributes: LPVOID,
    ) -> HANDLE;

    pub fn ConnectNamedPipe(hNamedPipe: HANDLE, lpOverlapped: LPVOID) -> BOOL;
    pub fn GetLastError() -> DWORD;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
    pub fn CreateEventW(
        lpEventAttributes: LPVOID,
        bManualReset: BOOL,
        bInitialState: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn WaitForSingleObject(hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
    pub fn WriteFile(
        hFile: HANDLE,
        lpBuffer: *const u8,
        nNumberOfBytesToWrite: DWORD,
        lpNumberOfBytesWritten: *mut DWORD,
        lpOverlapped: *mut Overlapped,
    ) -> BOOL;
    pub fn ReadFile(
        hFile: HANDLE,
        lpBuffer: *mut u8,
        nNumberOfBytesToRead: DWORD,
        lpNumberOfBytesRead: *mut DWORD,
        lpOverlapped: *mut Overlapped,
    ) -> BOOL;
}
