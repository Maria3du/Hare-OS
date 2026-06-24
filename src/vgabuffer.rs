use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
// O buffer de vídeo é uma área de memória que representa o conteúdo exibido na tela. Ele é usado para armazenar os caracteres e suas cores que serão exibidos no monitor. No modo texto, cada posição do buffer de vídeo corresponde a um caractere e suas informações de cor.
//modo de texto vga é uma maneira de imprimir texto na tela
//"allow(dead code)"uma anotação que permite que o código seja compilado mesmo que haja funções ou variáveis não utilizadas
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] 
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode(u8);

impl ColorCode {
   pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col] =Volatile::new(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }
//tipo escritor
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    pub fn new_line(&mut self) {    
        for row in (1..BUFFER_HEIGHT).rev() {
        for col in 0..BUFFER_WIDTH {
            let char_at_row = self.buffer.chars[row][col].read();
            self.buffer.chars[row - 1][col] =Volatile::new(char_at_row);
        }
    }

    self.clear_row(BUFFER_HEIGHT - 1);
    self.column_position = 0;
    }
   pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] =Volatile::new(blank);
        }
    }
}
//'static mut' é necessário para criar uma instância global do escritor, pois o buffer de vídeo é uma área de memória fixa e acessível em todo o sistema operacional. O 'static' garante que a instância do escritor tenha um tempo de vida estático, ou seja, ela estará disponível durante toda a execução do sistema operacional. O 'mut' é necessário porque o escritor precisa ser mutável para poder modificar o conteúdo do buffer de vídeo.
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vgabuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}