#![allow(dead_code)]
#![allow(unused_imports)]

use form::ll;
use ll::{WINDOW, chtype};
use constants::TRUE;
use std::ptr;
use std::slice;
use std::cmp::PartialEq;
use ToCStr;
use FromCStr;

use libc::c_int;

pub type FORM = ll::FORM;
pub type FIELD = ll::FIELD;
pub type FIELDTYPE = ll::FIELDTYPE;
pub type FieldOptions = ll::FieldOptions;

#[derive(Debug, Copy, Clone)]
pub enum Justification {
    Right,
    Center,
    Left,
    Disable
}

trait ConvertJustify {
    fn to_int(_:Justification) -> c_int;
    fn from_int(_:c_int) -> Justification;
}

impl ConvertJustify for Justification {
    fn to_int(j: Justification) -> c_int {
        match j {
            Justification::Disable => 0,
            Justification::Left => 1,
            Justification::Center => 2,
            Justification::Right => 3
        }
    }

    fn from_int(int: c_int) -> Justification {
        match int {
            1 => Justification::Left,
            2 => Justification::Center,
            3 => Justification::Right,
            _ => Justification::Disable,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    // fn result_ptr<T>(_: T) -> Option<T> where T: PartialEq;
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

    // fn result_ptr<T>(ptr: T) -> Option<T> where T: PartialEq {
    //     if ptr == ptr::null::<T>() { Some(ptr) }
    //     else { None }
    // }
}

pub type FormResult = Result<(), FormCode>;

pub fn set_current_field(form: FORM, field: FIELD) -> FormResult
{ FormCode::result( unsafe { ll::set_current_field(form, field) } ) }

pub fn current_field(form: FORM) -> Option<FIELD>
{ unsafe { ll::current_field(form).as_mut().map(|x| {x as FIELD}) } }

pub fn unfocus_current_field(form: FORM) -> FormResult
{ FormCode::result( unsafe { ll::unfocus_current_field(form) } ) }

pub fn set_form_page(form: FORM, n: i32) -> FormResult
{ FormCode::result( unsafe { ll::set_form_page(form, n) } ) }

pub fn form_page(form: FORM) -> i32
{ unsafe { ll::form_page(form) } }

pub fn field_index(field: FIELD) -> i32
{ unsafe { ll::field_index(field) } }


pub fn data_ahead(form: FORM) -> bool
{ if unsafe { ll::data_ahead(form) } == 1 { true } else { false } }

pub fn data_behind(form: FORM) -> bool
{ if unsafe { ll::data_behind(form) } == 1 { true } else { false } }


pub fn new_field(height: i32, width: i32, toprow: i32, leftcol: i32, offscreen: i32, nbuffers: i32) -> Option<FIELD>
{ unsafe { ll::new_field(height, width, toprow, leftcol, offscreen, nbuffers).as_mut().map(|x| {x as FIELD}) } }

pub fn dup_field(field: FIELD, toprow: i32, leftcol: i32) -> Option<FIELD>
{ unsafe { ll::dup_field(field, toprow, leftcol).as_mut().map(|x| {x as FIELD}) } }

pub fn link_field(field: FIELD, toprow: i32, leftcol: i32) -> Option<FIELD>
{ unsafe { ll::link_field(field, toprow, leftcol).as_mut().map(|x| {x as FIELD}) } }

pub fn free_field(field: FIELD) -> FormResult
{ FormCode::result( unsafe { ll::free_field(field) } ) }


pub fn field_info(field: FIELD, rows: &mut i32, cols: &mut i32,
                  frow: &mut i32, fcol: &mut i32, nrow: &mut i32, nbuf:&mut i32) -> FormResult
{ FormCode::result( unsafe { ll::field_info(field, rows as *mut c_int,
                                            cols as *mut c_int, frow as *mut c_int,
                                            fcol as *mut c_int, nrow as *mut c_int,
                                            nbuf as *mut c_int) } ) }

pub fn dynamic_field_info(field: FIELD, rows: &mut i32, cols: &mut i32, max: &mut i32) -> FormResult
{ FormCode::result( unsafe { ll::dynamic_field_info(field, rows as *mut c_int, cols as *mut c_int, max as *mut c_int) } ) }


// // pub fn int set_field_type(FIELD *field, FIELDTYPE *type, ...) { unsafe { ll:: } } TODO
//pub fn field_type(field: FIELD) -> FIELDTYPE { unsafe { ll:: } }
// // pub fn void *field_arg(const FIELD *field) { unsafe { ll:: } } TODO


pub fn set_field_buffer(field: FIELD, buf: i32, value: &str) -> FormResult
{ FormCode::result( unsafe { ll::set_field_buffer(field, buf, value.to_c_str().as_ptr()) } ) }

pub fn field_buffer(field: FIELD, buffer: i32) -> String
{ unsafe { FromCStr::from_c_str(ll::field_buffer(field, buffer)) } }

pub fn set_field_status(field: FIELD, status: bool) -> FormResult
{ FormCode::result( unsafe { ll::set_field_status(field, if status { 1 } else { 0 }) } ) }

pub fn field_status(field: FIELD) -> bool
{ unsafe { if ll::field_status(field) == 1 { true } else { false } } }

pub fn set_max_field(field: FIELD, max: i32) -> FormResult
{ FormCode::result( unsafe { ll::set_max_field(field, max) } ) }


pub fn set_form_fields(form: FORM, fields: &mut Vec<FIELD>) -> FormResult {
    fields.push(ptr::null_mut());
    let res = unsafe { ll::set_form_fields(form, fields.as_mut_ptr()) };
    fields.pop();
    FormCode::result(res)
}

pub fn form_fields(form: FORM) -> Vec<FIELD>
{ unsafe { slice::from_raw_parts(ll::form_fields(form), ll::field_count(form) as usize).to_vec() } }

pub fn field_count(form: FORM) -> i32
{ unsafe { ll::field_count(form) } }

pub fn move_field(field: FIELD, frow: i32, fcol: i32) -> FormResult
{ FormCode::result( unsafe { ll::move_field(field, frow, fcol) } ) }


pub fn set_field_fore(field: FIELD, attr: chtype) -> FormResult
{ FormCode::result( unsafe { ll::set_field_fore(field, attr) } ) }

pub fn field_fore(field: FIELD) -> chtype
{ unsafe { ll::field_fore(field) } }

pub fn set_field_back(field: FIELD, attr: chtype) -> FormResult
{ FormCode::result( unsafe { ll::set_field_back(field, attr) } ) }

pub fn field_back(field: FIELD) -> chtype
{ unsafe { ll::field_back(field) } }

pub fn set_field_pad(field: FIELD, pad: i32) -> FormResult
{ FormCode::result( unsafe { ll::set_field_pad(field, pad) } ) }

pub fn field_pad(field: FIELD) -> i32
{ unsafe { ll::field_pad(field) } }


// // TODO
// // pub fn int set_field_init(FORM *form, Form_Hook func) { unsafe { ll:: } }
// // pub fn Form_Hook field_init(const FORM *form) { unsafe { ll:: } }
// // pub fn int set_field_term(FORM *form, Form_Hook func) { unsafe { ll:: } }
// // pub fn Form_Hook field_term(const FORM *form) { unsafe { ll:: } }
// // pub fn int set_form_init(FORM *form, Form_Hook func) { unsafe { ll:: } }
// // pub fn Form_Hook form_init(const FORM *form) { unsafe { ll:: } }
// // pub fn int set_form_term(FORM *form, Form_Hook func) { unsafe { ll:: } }
// // pub fn Form_Hook form_term(const FORM *form) { unsafe { ll:: } }


pub fn set_field_just(field: FIELD, value: Justification) -> FormResult
{ FormCode::result( unsafe { ll::set_field_just(field, Justification::to_int(value)) } ) }

pub fn field_just(field: FIELD) -> Justification
{ Justification::from_int( unsafe { ll::field_just(field) } ) }


pub fn set_field_opts(field: FIELD, options: FieldOptions) -> FormResult
{ FormCode::result( unsafe { ll:: } ) }

// pub fn field_opts_on(_:FIELD, _:FieldOptions) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn field_opts_off(_:FIELD, _:FieldOptions) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn field_opts(_:FIELD) -> FieldOptions { unsafe { ll:: } }


// pub fn form_driver(_:FORM, _:c_int) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// // pub fn form_driver_w(_:FORM, _:c_int, wchar_t wch) -> FormResult { FormCode::result( unsafe { ll:: } ) } TODO wchar ?


// pub fn set_form_opts(_:FORM, _:FieldOptions) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn form_opts_on(_:FORM, _:FieldOptions) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn form_opts_off(_:FORM, _:FieldOptions) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn form_opts(_:FORM) -> FieldOptions { unsafe { ll:: } }


// pub fn form_request_name(_:c_int) -> *const c_char { unsafe { ll:: } }
// pub fn form_request_by_name(_:*const c_char) -> FormResult { FormCode::result( unsafe { ll:: } ) }


// pub fn set_form_win(_:FORM, _:WINDOW) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn form_win(_:FORM) -> WINDOW { unsafe { ll:: } }
// pub fn set_form_sub(_:FORM, _:WINDOW) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn form_sub(_:FORM) -> WINDOW { unsafe { ll:: } }
// pub fn scale_form(_:FORM, _:*mut c_int, _:*mut c_int) -> FormResult { FormCode::result( unsafe { ll:: } ) }


// pub fn new_field(_:c_int, _:c_int, _:c_int, _:c_int, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn dup_field(_:FIELD, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn link_field(_:FIELD, _:c_int, _:c_int) -> FIELD { unsafe { ll:: } }
// pub fn free_field(_:FIELD) -> FormResult { FormCode::result( unsafe { ll:: } ) }


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
// pub fn free_form(_:FORM) -> FormResult { FormCode::result( unsafe { ll:: } ) }


// pub fn set_new_page(_:FIELD, _:c_bool) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn new_page(_:FIELD) -> c_bool { unsafe { ll:: } }


// pub fn pos_form_cursor(_:FORM) -> FormResult { FormCode::result( unsafe { ll:: } ) }


// pub fn post_form(_:FORM) -> FormResult { FormCode::result( unsafe { ll:: } ) }
// pub fn unpost_form(_:FORM) -> FormResult { FormCode::result( unsafe { ll:: } ) }



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
