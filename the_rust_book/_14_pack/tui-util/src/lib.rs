extern crate term_size;
use std::io::{self,Write};

#[derive(Debug,Clone,Copy)]
pub struct Screen {
    pub width: u32,
    pub height: u32,
}

pub fn esc(code: &str) -> String {
    format!("\x1B[{}", code)
}

pub fn write_raw(text: &str){
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(text.as_bytes()).unwrap();
    handle.flush().unwrap();
}

pub fn screen_size() -> Option<Screen> {
    match term_size::dimensions() {
        Some((width, height)) => Some(Screen {
            width: width as u32,
            height: height as u32,
        }),
        None => None,
    }
}

pub fn cursor_show(){
    write_raw(&esc("?25h"));
}

pub fn cursor_hide(){
    write_raw(&esc("?25l"));
}

pub fn cursor_set_pos(x: u32, y: u32){
    write_raw(&esc(&format!("{};{}H", y, x)));
}

pub fn cursor_set_x(x: u32){
    write_raw(&esc(&format!("{}G", x)));
}

pub fn cursor_move_up(amount: u32){
    write_raw(&esc(&format!("{}A", amount)));
}

pub fn cursor_move_down(amount: u32){
    write_raw(&esc(&format!("{}B", amount)));
}

pub fn cursor_move_right(amount: u32){
    write_raw(&esc(&format!("{}C", amount)));
}
pub fn cursor_move_left(amount: u32){

    write_raw(&esc(&format!("{}D", amount)));
}

pub fn line_erase_start(){
    write_raw("\x1B[0K"); 
}

pub fn line_erase_end(){
    write_raw("\x1B[1K");
}

pub fn line_erase(){
    write_raw("\x1B[2K");
}

pub fn mode_underline(){
    write_raw(&esc("4m"));
}

pub fn mode_crossout(){
    write_raw(&esc("29m"));
}

pub fn mode_italic(){
    write_raw(&esc("1m"));
}

pub fn mode_faint(){
    write_raw(&esc("2m"));
}

pub fn mode_normal(){
    write_raw(&esc("22"));
}

pub fn mode_bold(){
    write_raw(&esc("1m"));
}

pub fn mode_reset(){
    write_raw(&esc("0m"));
}

pub fn color_fg(color: u8) {
    write_raw(&esc(&format!("38;5;{}m", color)));
}

pub fn color_bg(color: u8) {
    write_raw(&esc(&format!("48;5;{}m", color)));
}

pub fn screen_clear_down(){
    write_raw(&esc("0J"));
}

pub fn screen_clear_up(){
    write_raw(&esc("1J"));
}

pub fn screen_clear(){
    write_raw(&esc("2J"));
}

pub fn screen_reset(){
    mode_reset();
    cursor_set_pos(0, 0);
    screen_clear();
}

