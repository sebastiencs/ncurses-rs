#![allow(dead_code)]
#![allow(unused_imports)]

use form::ll;
use ll::WINDOW;
use constants::TRUE;
use std::ptr;
use std::cmp::PartialEq;

pub type FORM = ll::FORM;
pub type FIELD = ll::FIELD;
pub type FIELDTYPE = ll::FIELDTYPE;
pub type FieldOptions = ll::FieldOptions;

#[derive(Debug)]
pub enum FormCode {
    Ok = 0,
    SystemError = -1,
    BadArgument = -2,
    Posted = -3,
    Connected = -4,
    BadState = -5,
    NoRoom = -6,
    NotPosted = -7,
    UnknownCommand = -8,
    NoMatch = -9,
    NotSelectable = -10,
    NotConnected = -11,
    RequestDenied = -12,
    InvalidField = -13,
    Current = -14,
    Unknown = -15
}

trait Form_Result {
    fn from_code(_: i32) -> FormCode;
    fn result(_: i32) -> FormResult;
    fn result_ptr<T>(_: T) -> Option<T> where T: PartialEq;
}

impl Form_Result for FormCode {
    fn from_code(code: i32) -> FormCode {
        match code {
            0 => FormCode::Ok,
            -1 => FormCode::SystemError,
            -2 => FormCode::BadArgument,
            -3 => FormCode::Posted,
            -4 => FormCode::Connected,
            -5 => FormCode::BadState,
            -6 => FormCode::NoRoom,
            -7 => FormCode::NotPosted,
            -8 => FormCode::UnknownCommand,
            -9 => FormCode::NoMatch,
            -10 => FormCode::NotSelectable,
            -11 => FormCode::NotConnected,
            -12 => FormCode::RequestDenied,
            -13 => FormCode::InvalidField,
            -14 => FormCode::Current,
            _ => FormCode::Unknown,
        }
    }

    fn result(code: i32) -> FormResult {
        match code {
            0 => Ok(()),
            _ => Err(FormCode::from_code(code))
        }
    }

    fn result_ptr<T>(ptr: T) -> Option<T> where T: PartialEq {
        if ptr == ptr::null::<T>() { Some(ptr) }
        else { None }
    }
}

pub type FormResult = Result<(), FormCode>;

pub fn set_current_field(form: FORM, field: FIELD) -> FormResult
{ FormCode::result( unsafe { ll::set_current_field(form, field) } ) }

pub fn current_field(form: FORM) -> Option<FIELD>
{
    let ptr = unsafe { ll::current_field(form) };

    if ptr == ptr::null_mut() {
        Some(ptr)
    }
    else { None
    }
}

// pub fn unfocus_current_field(_:FORM) -> c_int
// { unsafe {  } }

// pub fn set_form_page(_:FORM, _:c_int) -> c_int { unsafe { ll:: } }
// pub fn form_page(_:FORM) -> c_int { unsafe { ll:: } }
// pub fn field_index(_:FIELD) -> c_int { unsafe { ll:: } }


// pub fn data_ahead(_:FORM) -> c_int { unsafe { ll:: } }
// pub fn data_behind(_:FORM) -> c_int { unsafe { ll:: } }


// pub fn new_field(_:c_int, _:c_int, _:c_int, _:c_int, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn dup_field(_:FIELD, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn link_field(_:FIELD, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn free_field(_:FIELD) -> c_int { unsafe { ll:: } }


// pub fn field_info(_:FIELD, _:*mut c_int, _:*mut c_int, _:*mut c_int, _:*mut c_int, _:*mut c_int, _:*mut c_int) -> c_int { unsafe { ll:: } }
// pub fn dynamic_field_info(_:FIELD, _:*mut c_int, _:*mut c_int, _:*mut c_int) -> c_int { unsafe { ll:: } }


// // pub fn int set_field_type(FIELD *field, FIELDTYPE *type, ...) { unsafe { ll:: } } TODO
// pub fn field_type(_:FIELD) -> FIELDTYPE { unsafe { ll:: } }
// // pub fn void *field_arg(const FIELD *field) { unsafe { ll:: } } TODO


// pub fn set_field_buffer(_:FIELD, _:c_int, _:*const c_char) -> c_int { unsafe { ll:: } }
// pub fn field_buffer(_:FIELD, _:c_int) -> *const c_char { unsafe { ll:: } }
// pub fn set_field_status(_:FIELD, _:c_bool) -> c_int { unsafe { ll:: } }
// pub fn field_status(_:FIELD) -> c_bool { unsafe { ll:: } }
// pub fn set_max_field(_:FIELD, _:c_int) -> c_int { unsafe { ll:: } }


// pub fn set_form_fields(_:FORM, _:*mut FIELD) -> c_int { unsafe { ll:: } }
// pub fn form_fields(_:FORM) -> *mut FIELD { unsafe { ll:: } }
// pub fn field_count(_:FORM) -> c_int { unsafe { ll:: } }
// pub fn move_field(_:FIELD, _:c_int, _:c_int) -> c_int { unsafe { ll:: } }


// pub fn set_field_fore(_:FIELD, _:chtype) -> c_int { unsafe { ll:: } }
// pub fn field_fore(_:FIELD) -> chtype { unsafe { ll:: } }
// pub fn set_field_back(_:FIELD, _:chtype) -> c_int { unsafe { ll:: } }
// pub fn field_back(_:FIELD) -> chtype { unsafe { ll:: } }
// pub fn set_field_pad(_:FIELD, _:c_int) -> c_int { unsafe { ll:: } }
// pub fn field_pad(_:FIELD) -> c_int { unsafe { ll:: } }


// // TODO
// // pub fn int set_field_init(FORM *form, Form_Hook func) { unsafe { ll:: } }
// // pub fn Form_Hook field_init(const FORM *form) { unsafe { ll:: } }
// // pub fn int set_field_term(FORM *form, Form_Hook func) { unsafe { ll:: } }
// // pub fn Form_Hook field_term(const FORM *form) { unsafe { ll:: } }
// // pub fn int set_form_init(FORM *form, Form_Hook func) { unsafe { ll:: } }
// // pub fn Form_Hook form_init(const FORM *form) { unsafe { ll:: } }
// // pub fn int set_form_term(FORM *form, Form_Hook func) { unsafe { ll:: } }
// // pub fn Form_Hook form_term(const FORM *form) { unsafe { ll:: } }


// pub fn set_field_just(_:FIELD, _:c_int) -> c_int { unsafe { ll:: } }
// pub fn field_just(_:FIELD) -> c_int { unsafe { ll:: } }


// pub fn set_field_opts(_:FIELD, _:FieldOptions) -> c_int { unsafe { ll:: } }
// pub fn field_opts_on(_:FIELD, _:FieldOptions) -> c_int { unsafe { ll:: } }
// pub fn field_opts_off(_:FIELD, _:FieldOptions) -> c_int { unsafe { ll:: } }
// pub fn field_opts(_:FIELD) -> FieldOptions { unsafe { ll:: } }


// pub fn form_driver(_:FORM, _:c_int) -> c_int { unsafe { ll:: } }
// // pub fn form_driver_w(_:FORM, _:c_int, wchar_t wch) -> c_int { unsafe { ll:: } } TODO wchar ?


// pub fn set_form_opts(_:FORM, _:FieldOptions) -> c_int { unsafe { ll:: } }
// pub fn form_opts_on(_:FORM, _:FieldOptions) -> c_int { unsafe { ll:: } }
// pub fn form_opts_off(_:FORM, _:FieldOptions) -> c_int { unsafe { ll:: } }
// pub fn form_opts(_:FORM) -> FieldOptions { unsafe { ll:: } }


// pub fn form_request_name(_:c_int) -> *const c_char { unsafe { ll:: } }
// pub fn form_request_by_name(_:*const c_char) -> c_int { unsafe { ll:: } }


// pub fn set_form_win(_:FORM, _:WINDOW) -> c_int { unsafe { ll:: } }
// pub fn form_win(_:FORM) -> WINDOW { unsafe { ll:: } }
// pub fn set_form_sub(_:FORM, _:WINDOW) -> c_int { unsafe { ll:: } }
// pub fn form_sub(_:FORM) -> WINDOW { unsafe { ll:: } }
// pub fn scale_form(_:FORM, _:*mut c_int, _:*mut c_int) -> c_int { unsafe { ll:: } }


// pub fn new_field(_:c_int, _:c_int, _:c_int, _:c_int, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn dup_field(_:FIELD, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn link_field(_:FIELD, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn free_field(_:FIELD) -> c_int { unsafe { ll:: } }


// // TODO
// // FIELDTYPE *new_fieldtype(bool (* const field_check)(FIELD *, const void *), bool (* const char_check)(int, const void *)) { unsafe { ll:: } }
// // int free_fieldtype(FIELDTYPE *fieldtype) { unsafe { ll:: } }
// // int set_fieldtype_arg(FIELDTYPE *fieldtype, void *(* const make_arg)(va_list *),
// //     void *(* const copy_arg)(const void *),
// //     void  (* const free_arg)(void *)) { unsafe { ll:: } }
// // int set_fieldtype_choice(
// //     FIELDTYPE *fieldtype,
// //     bool (* const next_choice)(FIELD *, const void *),
// //     bool (* const prev_choice)(FIELD *, const void *)) { unsafe { ll:: } }
// // FIELDTYPE *link_fieldtype(FIELDTYPE *type1,
// //                           FIELDTYPE *type2) { unsafe { ll:: } }


// pub fn new_form(_:*mut FIELD) -> FORM { unsafe { ll:: } }
// pub fn free_form(_:FORM) -> c_int { unsafe { ll:: } }


// pub fn set_new_page(_:FIELD, _:c_bool) -> c_int { unsafe { ll:: } }
// pub fn new_page(_:FIELD) -> c_bool { unsafe { ll:: } }


// pub fn pos_form_cursor(_:FORM) -> c_int { unsafe { ll:: } }


// pub fn post_form(_:FORM) -> c_int { unsafe { ll:: } }
// pub fn unpost_form(_:FORM) -> c_int { unsafe { ll:: } }



// #[cfg(feature="panel")]
// pub fn panel_window(panel: PANEL) -> WINDOW
// { unsafe { ll::panel_window(panel) } }

// #[cfg(feature="panel")]
// pub fn update_panels()
// { unsafe { ll::update_panels() { unsafe { ll:: } } } }

// #[cfg(feature="panel")]
// pub fn hide_panel(panel: PANEL) -> i32
// { unsafe { ll::hide_panel(panel) } }

// #[cfg(feature="panel")]
// pub fn show_panel(panel: PANEL) -> i32
// { unsafe { ll::show_panel(panel) } }

// #[cfg(feature="panel")]
// pub fn del_panel(panel: PANEL) -> i32
// { unsafe { ll::del_panel(panel) } }

// #[cfg(feature="panel")]
// pub fn top_panel(panel: PANEL) -> i32
// { unsafe { ll::top_panel(panel) } }

// #[cfg(feature="panel")]
// pub fn bottom_panel(panel: PANEL) -> i32
// { unsafe { ll::bottom_panel(panel) } }

// #[cfg(feature="panel")]
// pub fn new_panel(window: WINDOW) -> PANEL
// { unsafe { ll::new_panel(window) } }

// #[cfg(feature="panel")]
// pub fn panel_above(panel: PANEL) -> PANEL
// { unsafe { ll::panel_above(panel) } }

// #[cfg(feature="panel")]
// pub fn panel_below(panel: PANEL) -> PANEL
// { unsafe { ll::panel_below(panel) } }

// #[cfg(feature="panel")]
// pub fn move_panel(panel: PANEL, y: i32, x: i32) -> i32
// { unsafe { ll::move_panel(panel, y, x) } }

// #[cfg(feature="panel")]
// pub fn replace_panel(panel: PANEL, window: WINDOW) -> i32
// { unsafe { ll::replace_panel(panel, window) } }

// #[cfg(feature="panel")]
// pub fn panel_hidden(panel: PANEL) -> bool
// { unsafe { ll::panel_hidden(panel) != 0 } }
