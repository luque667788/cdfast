#!/usr/bin/env run-cargo-script

use std::ffi::{OsStr, OsString};
use std::io;
use std::io::Write;
use termion::clear;
use termion::cursor::{DetectCursorPos, Goto};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

fn write_to_line(
    stdout: &mut RawTerminal<io::Stdout>,
    text: &OsStr,
    point: (u16, u16),
) -> Result<(), &'static str> {
    if point.0 < 1 || point.1 < 1 {
        Err("Goto is one-based (lol)")
    } else {
        write!(stdout, "{}", Goto(point.0, point.1)).unwrap(); // Reset the cursor to the top left corner
        write!(stdout, "{}", clear::CurrentLine).unwrap(); // Clear the line
        write!(stdout, "{:?}", text).unwrap(); // Write the text
                                               //stdout.flush().unwrap(); // maybe change this later
        Ok(())
    }
}

fn write_to_line_bg(
    stdout: &mut RawTerminal<io::Stdout>,
    text: &OsStr,
    point: (u16, u16),
) -> Result<(), &'static str> {
    if point.0 < 1 || point.1 < 1 {
        Err("Goto is one-based (lol)")
    } else {
        write!(stdout, "{}", Goto(point.0, point.1)).unwrap(); // Reset the cursor to the top left corner
        write!(stdout, "{}", clear::CurrentLine).unwrap(); // Clear the line
        write!(stdout, "{}", termion::color::Bg(termion::color::White)).unwrap();
        write!(stdout, "{:?}", text).unwrap(); // Write the text
        write!(stdout, "{}", termion::color::Bg(termion::color::Reset)).unwrap();
        //stdout.flush().unwrap(); // maybe change this later
        Ok(())
    }
}

fn write_to_line_bg_noclear(
    stdout: &mut RawTerminal<io::Stdout>,
    text: &OsStr,
    point: (u16, u16),
) -> Result<(), &'static str> {
    if point.0 < 1 || point.1 < 1 {
        Err("Goto is one-based (lol)")
    } else {
        write!(stdout, "{}", Goto(point.0, point.1)).unwrap(); // Reset the cursor to the top left corner
        write!(stdout, "{}", termion::color::Bg(termion::color::White)).unwrap();
        write!(stdout, "{:?}", text).unwrap(); // Write the text
        write!(stdout, "{}", termion::color::Bg(termion::color::Reset)).unwrap();
        //stdout.flush().unwrap(); // maybe change this later
        Ok(())
    }
}

fn printfiles(
    stdout: &mut RawTerminal<io::Stdout>,
    startline: u16,
) -> Result<(Vec<OsString>, u16), &'static str> {
    write!(stdout, "{}", Goto(1, 1)).unwrap();
    //print the current directory in the first line
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    write!(stdout, "{}", current_dir).unwrap();
    write!(stdout, "{}", clear::AfterCursor).unwrap(); // Clear the screen // Clear the screen
                                                       // set cursor to (1,1)

    // print all the files in the current directory
    let mut y = 1;
    let paths = std::fs::read_dir(".").unwrap();
    let paths2 = std::fs::read_dir(".").unwrap();
    let paths2 = paths2.map(|f| f.unwrap().path().as_mut_os_string().to_owned());
    // collect all the paths in an vector
    let paths2 = paths2.collect::<Vec<OsString>>();

    for path in paths {
        writeln!(
            stdout,
            "{}{:?}",
            Goto(1, y + startline),
            path.unwrap().path()
        )
        .unwrap();
        stdout.flush().unwrap();
        y += 1;
    }
    let num = y - 1;
    return Ok((paths2, num as u16));
}

fn main() {
    let mut emptydir = false;
    let startline: u16 = 2;
    let stdin = io::stdin();
    let mut stdout: RawTerminal<io::Stdout> = io::stdout().into_raw_mode().unwrap();
    //cursor to (1,1)
    write!(stdout, "{}", Goto(1, 1)).unwrap();
    write!(stdout, "{}", clear::All).unwrap(); // Clear the screen // Clear the screen

    //print the current directory in the first line
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    write!(stdout, "{}", current_dir).unwrap();
    write!(stdout, "{}", clear::AfterCursor).unwrap(); // Clear the screen

    let (mut paths2, mut num) = printfiles(&mut stdout, startline).unwrap();
    // change the emptydir variable to true if there are no files in the directory
    if paths2.len() == 0 {
        emptydir = true;
        write!(stdout, "{}", Goto(1, startline)).unwrap();
        write!(stdout, "{}", clear::AfterCursor).unwrap(); // Clear the screen
        writeln!(stdout, "No files in this directory").unwrap();
        paths2 = vec![OsString::from("No files in this directory")];
        num = 1;
        write!(stdout, "{}", Goto(1, 1)).unwrap();
    }
    let mut selected_line = 1;
    write_to_line_bg(
        &mut stdout,
        &paths2[(selected_line - 1) as usize],
        (1, selected_line + startline),
    )
    .unwrap();
    stdout.flush().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => {
                write_to_line(
                    &mut stdout,
                    &paths2[(selected_line - 1) as usize],
                    (1, startline + selected_line),
                )
                .unwrap();
                if selected_line > 1 {
                    selected_line -= 1;
                } else {
                    selected_line = num;
                }
                write_to_line_bg(
                    &mut stdout,
                    &paths2[(selected_line - 1) as usize],
                    (1, selected_line + startline),
                )
                .unwrap();
            }
            Key::Char('\n') => {
                if emptydir {
                    continue;
                }
                let current_dir: String =
                    String::from(std::env::current_dir().unwrap().to_str().unwrap());
                //clear the whole screen
                write!(stdout, "{}", Goto(1, 1)).unwrap();
                write!(stdout, "{}", clear::All).unwrap(); // Clear the screen
                                                           //print the current directory in the first line

                // save the carvable current dir in a .txt file named selected_directory.txt
                let mut file = std::fs::File::create(
                    "/home/luque/prog/rust/cdtree/target/debug/selected_directory.txt",
                ).unwrap();

                write!(file, "{}", current_dir).unwrap();

                // close the file
                drop(file);
                //print the current directory in the first line

                

                
            }
            Key::Down => {
                write_to_line(
                    &mut stdout,
                    &paths2[(selected_line - 1) as usize],
                    (1, selected_line + startline),
                )
                .unwrap();
                if selected_line < num {
                    selected_line += 1;
                } else {
                    selected_line = 1;
                }
                write_to_line_bg(
                    &mut stdout,
                    &paths2[(selected_line - 1) as usize],
                    (1, selected_line + startline),
                )
                .unwrap();
            }
            Key::Char('q') => {
                write!(stdout, "{}", clear::All).unwrap(); // Clear the screen
                break;
            }
            Key::Right => {
                let path = paths2[(selected_line - 1) as usize].clone();
                // move to the selected directory
                match std::env::set_current_dir(path) {
                    Ok(_) => {
                        (paths2, num) = printfiles(&mut stdout, startline).unwrap();
                        selected_line = 1;
                        if paths2.len() == 0 {
                            emptydir = true;
                            write!(stdout, "{}", Goto(1, startline)).unwrap();
                            write!(stdout, "{}", clear::AfterCursor).unwrap(); // Clear the screen
                            writeln!(stdout, "No files in this directory").unwrap();
                            paths2 = vec![OsString::from("No files in this directory")];
                            num = 1;
                            write!(stdout, "{}", Goto(1, 1)).unwrap();
                        } else {
                            emptydir = false;
                            write_to_line_bg(
                                &mut stdout,
                                &paths2[(selected_line - 1) as usize],
                                (1, selected_line + startline),
                            )
                            .unwrap();
                        }
                    }
                    Err(_) => {
                        let error_message = OsStr::new(" Error: Could not find directory");
                        write_to_line_bg_noclear(
                            &mut stdout,
                            error_message,
                            (20, selected_line + startline),
                        )
                        .unwrap();
                    }
                }
            }
            Key::Left => {
                // move to the selected directory
                let current_dir = std::env::current_dir().unwrap();
                if current_dir.to_str().unwrap() == "/" {
                    continue;
                }
                let parent_dir = current_dir.parent().unwrap().to_owned();
                match std::env::set_current_dir(parent_dir) {
                    Ok(_) => {
                        (paths2, num) = printfiles(&mut stdout, startline).unwrap();
                        selected_line = 1;
                        if paths2.len() == 0 {
                            write!(stdout, "{}", Goto(1, 1)).unwrap();
                            write!(stdout, "{}", clear::All).unwrap(); // Clear the screen
                            writeln!(stdout, "No files in this directory").unwrap();
                            paths2 = vec![OsString::from("No files in this directory")];
                            num = 1;
                            write!(stdout, "{}", Goto(1, 1)).unwrap();
                        } else {
                            write_to_line_bg(
                                &mut stdout,
                                &paths2[(selected_line - 1) as usize],
                                (1, selected_line + startline),
                            )
                            .unwrap();
                        }
                    }
                    Err(_) => {
                        let error_message = OsStr::new(" Error: Could not find directory");
                        write_to_line_bg_noclear(
                            &mut stdout,
                            error_message,
                            (20, selected_line + startline),
                        )
                        .unwrap();
                    }
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    //write!(stdout, "{}", Goto(1, 1)).unwrap();
    //write!(stdout, "{}", clear::All).unwrap(); // Clear the screen
}
