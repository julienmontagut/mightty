//! Terminal state management
//!
//! This module handles the terminal screen buffer, cursor position,
//! and terminal state operations.

#[derive(Default)]
pub struct TerminalState {
    screen_buffer: Vec<Vec<char>>,
    cursor_row: usize,
    cursor_col: usize,
    pub show_cursor: bool,
    rows: usize,
    cols: usize,
}

impl TerminalState {
    pub fn new() -> Self {
        Self::with_size(24, 80)
    }

    pub fn with_size(rows: usize, cols: usize) -> Self {
        Self {
            screen_buffer: vec![vec![' '; cols]; rows],
            cursor_row: 0,
            cursor_col: 0,
            show_cursor: true,
            rows,
            cols,
        }
    }

    pub fn print_char(&mut self, c: char) {
        if self.cursor_row < self.rows && self.cursor_col < self.cols {
            self.screen_buffer[self.cursor_row][self.cursor_col] = c;
            self.cursor_col += 1;

            if self.cursor_col >= self.cols {
                self.cursor_col = 0;
                self.cursor_row += 1;
            }
        }
    }

    pub fn execute_control(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.cursor_row += 1;
                if self.cursor_row >= self.rows {
                    self.scroll_up();
                    self.cursor_row = self.rows - 1;
                }
            }
            b'\r' => {
                self.cursor_col = 0;
            }
            b'\t' => {
                self.cursor_col = ((self.cursor_col / 8) + 1) * 8;
                if self.cursor_col >= self.cols {
                    self.cursor_col = self.cols - 1;
                }
            }
            b'\x08' => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
            }
            _ => {}
        }
    }

    fn scroll_up(&mut self) {
        for i in 1..self.rows {
            self.screen_buffer[i - 1] = self.screen_buffer[i].clone();
        }
        self.screen_buffer[self.rows - 1] = vec![' '; self.cols];
    }

    pub fn handle_csi(&mut self, params: Vec<i16>, intermediates: &[u8], ignore: bool, c: char) {
        let _ = ignore;
        let _ = intermediates;
        match c {
            'A' => {
                if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                }
            }
            'B' => {
                if self.cursor_row < self.rows - 1 {
                    self.cursor_row += 1;
                }
            }
            'C' => {
                if self.cursor_col < self.cols - 1 {
                    self.cursor_col += 1;
                }
            }
            'D' => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
            }
            'H' => {
                if let Some(&row) = params.first() {
                    self.cursor_row = ((row as usize).saturating_sub(1)).min(self.rows - 1);
                }
                if let Some(&col) = params.get(1) {
                    self.cursor_col = ((col as usize).saturating_sub(1)).min(self.cols - 1);
                }
            }
            'J' => {
                if let Some(&n) = params.first() {
                    match n {
                        0 => {
                            // Clear from cursor to end of screen
                            for col in self.cursor_col..self.cols {
                                self.screen_buffer[self.cursor_row][col] = ' ';
                            }
                            for row in (self.cursor_row + 1)..self.rows {
                                for col in 0..self.cols {
                                    self.screen_buffer[row][col] = ' ';
                                }
                            }
                        }
                        1 => {
                            // Clear from beginning to cursor
                            for row in 0..self.cursor_row {
                                for col in 0..self.cols {
                                    self.screen_buffer[row][col] = ' ';
                                }
                            }
                            for col in 0..=self.cursor_col {
                                self.screen_buffer[self.cursor_row][col] = ' ';
                            }
                        }
                        2 => {
                            // Clear entire screen
                            for row in 0..self.rows {
                                for col in 0..self.cols {
                                    self.screen_buffer[row][col] = ' ';
                                }
                            }
                            self.cursor_row = 0;
                            self.cursor_col = 0;
                        }
                        _ => {}
                    }
                }
            }
            'K' => {
                if let Some(&n) = params.first() {
                    match n {
                        0 => {
                            // Clear from cursor to end of line
                            for col in self.cursor_col..self.cols {
                                self.screen_buffer[self.cursor_row][col] = ' ';
                            }
                        }
                        1 => {
                            // Clear from beginning to cursor
                            for col in 0..=self.cursor_col {
                                self.screen_buffer[self.cursor_row][col] = ' ';
                            }
                        }
                        2 => {
                            // Clear entire line
                            for col in 0..self.cols {
                                self.screen_buffer[self.cursor_row][col] = ' ';
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_display_text(&self) -> String {
        self.screen_buffer
            .iter()
            .map(|line| line.iter().collect::<String>().trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
