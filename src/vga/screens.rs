use crate::vga::{
    ColorCode, GLOBAL_VGA_SCREEN, ScreenChar, VGA_BUFFER_ADDRESS, VGA_BUFFER_HEIGHT, VGA_BUFFER_WIDTH,
};

static mut SCREENS: [VirtualVgaScreen; 4] = [VirtualVgaScreen::empty(); 4];
static mut ACTIVE_SCREEN_INDEX: usize = 0;

type VirtualBuffer = [[ScreenChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT];

/// Virtual VGA screen used for tab switching
/// 
/// It saves [VgaScreen][crate::vga::VgaScreen] infos
#[derive(Clone, Copy)]
pub struct VirtualVgaScreen {
    pub column_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
    pub buffer: VirtualBuffer,
}

impl VirtualVgaScreen {
    /// Creates a [VirtualVgaScreen] with default zeroed contents
    pub const fn empty() -> Self {
		let default_char = ScreenChar::new(0, ColorCode::white_on_black());
        Self {
            buffer: [[default_char; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::white_on_black(),
        }
    }
}

/// Switches the active screen to `new_index` and saves the current cursor/color states
/// 
/// Returns the saved cursor/color states of the new screen, or None if parameters are invalid
#[expect(static_mut_refs)]
pub fn switch_screen(
    new_index: usize,
    current_col_pos: usize,
    current_row_pos: usize,
    current_color_code: ColorCode,
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

/// Handles keyboard shortcut to switch screens
pub fn handle_shortcut_switch_screen(new_index: usize) {
    let mut writer = GLOBAL_VGA_SCREEN.lock();

    let res = switch_screen(
        new_index,
        writer.column_position,
        writer.row_position,
        writer.color_code,
    );

    if let Some((new_col, new_row, new_color_code)) = res {
        writer.move_cursor_pos(new_col, new_row);
        writer.color_code = new_color_code;
    }
}
