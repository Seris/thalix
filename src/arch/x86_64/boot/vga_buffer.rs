use core;
use core::fmt;
use core::ptr::Unique;
use spin::Mutex;

const COLUMN_COUNT: usize = 80;
const LINE_COUNT: usize = 25;

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
	Black      = 0,
	Blue       = 1,
	Green      = 2,
	Cyan       = 3,
	Red        = 4,
	Magenta    = 5,
	Brown      = 6,
	LightGray  = 7,
	DarkGray   = 8,
	LightBlue  = 9,
	LightGreen = 10,
	LightCyan  = 11,
	LightRed   = 12,
	Pink       = 13,
	Yellow     = 14,
	White      = 15,
}

#[derive(Debug, Clone, Copy)]
struct ColorByte(u8);

impl ColorByte {
	const fn new(foreground: Color, background: Color) -> ColorByte {
		ColorByte((foreground as u8 ) | (background as u8) << 4)
	}
}

type Buffer = [[BufferChar; COLUMN_COUNT]; LINE_COUNT];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct BufferChar {
	ascii: u8,
	color: ColorByte
}

#[derive(Clone, Copy)]
pub struct Writer {
	raw_buffer: Unique<Buffer>,
	cursor_column: usize,
	color: ColorByte
}

impl fmt::Write for Writer {
	fn write_str(&mut self, string: &str) -> fmt::Result {
		self.write_bytes(&string.as_bytes());
		Ok(())
	}
}

#[allow(dead_code)]
impl Writer {
	const fn new() -> Writer {
		Writer {
			raw_buffer: unsafe { Unique::new(0xb8000 as *mut _) },
			cursor_column: 0,
			color: ColorByte::new(Color::White, Color::Black)
		}
	}

	fn write_bytes(&mut self, bytes: &[u8]){
		for &ascii in bytes.iter(){
			match ascii {
				b'\n' => self.new_line(),
				b'\t' => self.write_bytes(b"    "),
				_ => self.write_byte(ascii)
			}
		}
	}

	fn write_byte(&mut self, byte: u8){
		if self.cursor_column >= COLUMN_COUNT {
			self.new_line();
		}

		unsafe {
			core::ptr::write_volatile(
				&mut self.buffer()[LINE_COUNT - 1][self.cursor_column],
				BufferChar { ascii: byte, color: self.color }
			);
		}

		self.cursor_column += 1;
	}

	pub fn set_color(&mut self, foreground: Color, background: Color){
		self.color = ColorByte::new(foreground, background);
	}

	pub fn clear_screen(&mut self){
		self.cursor_column = 0;
		for line in 0..LINE_COUNT {
			self.clear_line(line);
		}
	}

	unsafe fn buffer(&self) -> &mut Buffer {
		&mut *self.raw_buffer.as_ptr()
	}

	fn move_lines_up(&mut self){
		let line = unsafe { self.buffer().iter().skip(1) };
		let prev_line = unsafe { self.buffer().iter_mut() };
		for (current, prev) in line.zip(prev_line) {
			for (char_current, char_prev) in current.iter().zip(prev.iter_mut()) {
				*char_prev = *char_current;
			}
		}
	}

	fn new_line(&mut self){
		{
			let last_line = unsafe {
				self.buffer()[LINE_COUNT-1].iter_mut().skip(self.cursor_column)
			};

			for buffer_char in last_line {
				buffer_char.color = self.color;
			}
		}

		self.move_lines_up();
		self.clear_line(LINE_COUNT-1);
		self.cursor_column = 0;
	}

	fn clear_line(&mut self, line: usize){
		let buffer = unsafe { self.buffer() };

		if line >= LINE_COUNT {
			panic!("[vga_buffer] can't clear line outside of vga buffer");
		}

		for buffer_char in buffer[line].iter_mut() {
			*buffer_char = BufferChar { ascii: b' ', color: self.color };
		}
	}
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer::new());

pub fn early_kprint(args: fmt::Arguments){
	use core::fmt::Write;
	let mut guard = WRITER.lock();
	guard.write_fmt(args).unwrap();
}

pub fn clear_screen(){
	let mut guard = WRITER.lock();
	guard.clear_screen();
}

pub fn set_color(foreground: Color, background: Color){
	let mut guard = WRITER.lock();
	guard.set_color(foreground, background);
}

macro_rules! early_kprint {
	($($arg:tt)*) => {
		$crate::arch::x86_64::vga_buffer::early_kprint(format_args!($($arg)*));
	}
}

macro_rules! early_kprintln {
	($fmt:expr) => { early_kprint!(concat!($fmt, "\n")) };
	($fmt:expr, $($arg:tt)*) => { early_kprint!(concat!($fmt, "\n"), $($arg)*) };
}
