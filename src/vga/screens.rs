use crate::vga::{
    ColorCode, GLOBAL_WRITER, ScreenChar, VGA_BUFFER_ADDRESS, VGA_BUFFER_HEIGHT, VGA_BUFFER_WIDTH,
};

static mut SCREENS: [VirtualScreen; 4] = [VirtualScreen::empty(); 4];
static mut ACTIVE_SCREEN_INDEX: usize = 0;

type VirtualBuffer = [[ScreenChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT];

#[derive(Clone, Copy)]
pub struct VirtualScreen {
    pub buffer: VirtualBuffer,
    pub column_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
}

impl VirtualScreen {
    pub const fn empty() -> Self {
        Self {
            buffer: [[ScreenChar::default(); VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::default(),
        }
    }
}

#[expect(static_mut_refs)]
pub fn switch_screen(
    new_index: usize,
    current_col_pos: usize,
    current_row_pos: usize,
    current_color_code: ColorCode
) -> Option<(usize, usize, ColorCode)> {
    unsafe {
        if new_index >= SCREENS.len() || new_index == ACTIVE_SCREEN_INDEX {
            return None;
        }

        let old_index = ACTIVE_SCREEN_INDEX;

        let vga_ptr: *mut VirtualBuffer = VGA_BUFFER_ADDRESS as *mut VirtualBuffer;
        let old_screen_ptr: *mut VirtualBuffer =
            &mut SCREENS[old_index].buffer as *mut VirtualBuffer;
        let new_screen_ptr: *mut VirtualBuffer =
            &mut SCREENS[new_index].buffer as *mut VirtualBuffer;

        core::ptr::copy_nonoverlapping(vga_ptr, old_screen_ptr, 1);
        SCREENS[old_index].column_position = current_col_pos;
        SCREENS[old_index].row_position = current_row_pos;
        SCREENS[old_index].color_code = current_color_code;

        core::ptr::copy_nonoverlapping(new_screen_ptr, vga_ptr, 1);
        ACTIVE_SCREEN_INDEX = new_index;

        Some((
            SCREENS[new_index].column_position,
            SCREENS[new_index].row_position,
            SCREENS[new_index].color_code,
        ))
    }
}

pub fn handle_shortcut_switch_screen(new_index: usize) {
    let mut writer = GLOBAL_WRITER.lock();

    let res = switch_screen(
        new_index,
        writer.column_position,
        writer.row_position,
        writer.color_code
    );

    if let Some((new_col, new_row, new_color_code)) = res {
        writer.move_cursor_pos(new_col, new_row);
        writer.color_code = new_color_code;
    }
}
