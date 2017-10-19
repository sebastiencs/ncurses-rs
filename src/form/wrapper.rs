#![allow(dead_code)]
#![allow(unused_imports)]

use form::ll;
use ll::{WINDOW, chtype};
use constants::TRUE;
use std::ptr;
use std::slice;
use ToCStr;
use FromCStr;

use libc::c_int;

use constants::KEY_MAX;

pub const REQ_NEXT_PAGE: c_int = KEY_MAX + 1;
pub const REQ_PREV_PAGE: c_int = KEY_MAX + 2;
pub const REQ_FIRST_PAGE: c_int = KEY_MAX + 3;
pub const REQ_LAST_PAGE: c_int = KEY_MAX + 4;
pub const REQ_NEXT_FIELD: c_int = KEY_MAX + 5;
pub const REQ_PREV_FIELD: c_int = KEY_MAX + 6;
pub const REQ_FIRST_FIELD: c_int = KEY_MAX + 7;
pub const REQ_LAST_FIELD: c_int = KEY_MAX + 8;
pub const REQ_SNEXT_FIELD: c_int = KEY_MAX + 9;
pub const REQ_SPREV_FIELD: c_int = KEY_MAX + 10;
pub const REQ_SFIRST_FIELD: c_int = KEY_MAX + 11;
pub const REQ_SLAST_FIELD: c_int = KEY_MAX + 12;
pub const REQ_LEFT_FIELD: c_int = KEY_MAX + 13;
pub const REQ_RIGHT_FIELD: c_int = KEY_MAX + 14;
pub const REQ_UP_FIELD: c_int = KEY_MAX + 15;
pub const REQ_DOWN_FIELD: c_int = KEY_MAX + 16;
pub const REQ_NEXT_CHAR: c_int = KEY_MAX + 17;
pub const REQ_PREV_CHAR: c_int = KEY_MAX + 18;
pub const REQ_NEXT_LINE: c_int = KEY_MAX + 19;
pub const REQ_PREV_LINE: c_int = KEY_MAX + 20;
pub const REQ_NEXT_WORD: c_int = KEY_MAX + 21;
pub const REQ_PREV_WORD: c_int = KEY_MAX + 22;
pub const REQ_BEG_FIELD: c_int = KEY_MAX + 23;
pub const REQ_END_FIELD: c_int = KEY_MAX + 24;
pub const REQ_BEG_LINE: c_int = KEY_MAX + 25;
pub const REQ_END_LINE: c_int = KEY_MAX + 26;
pub const REQ_LEFT_CHAR: c_int = KEY_MAX + 27;
pub const REQ_RIGHT_CHAR: c_int = KEY_MAX + 28;
pub const REQ_UP_CHAR: c_int = KEY_MAX + 29;
pub const REQ_DOWN_CHAR: c_int = KEY_MAX + 30;
pub const REQ_NEW_LINE: c_int = KEY_MAX + 31;
pub const REQ_INS_CHAR: c_int = KEY_MAX + 32;
pub const REQ_INS_LINE: c_int = KEY_MAX + 33;
pub const REQ_DEL_CHAR: c_int = KEY_MAX + 34;
pub const REQ_DEL_PREV: c_int = KEY_MAX + 35;
pub const REQ_DEL_LINE: c_int = KEY_MAX + 36;
pub const REQ_DEL_WORD: c_int = KEY_MAX + 37;
pub const REQ_CLR_EOL: c_int = KEY_MAX + 38;
pub const REQ_CLR_EOF: c_int = KEY_MAX + 39;
pub const REQ_CLR_FIELD: c_int = KEY_MAX + 40;
pub const REQ_OVL_MODE: c_int = KEY_MAX + 41;
pub const REQ_INS_MODE: c_int = KEY_MAX + 42;
pub const REQ_SCR_FLINE: c_int = KEY_MAX + 43;
pub const REQ_SCR_BLINE: c_int = KEY_MAX + 44;
pub const REQ_SCR_FPAGE: c_int = KEY_MAX + 45;
pub const REQ_SCR_BPAGE: c_int = KEY_MAX + 46;
pub const REQ_SCR_FHPAGE: c_int = KEY_MAX + 47;
pub const REQ_SCR_BHPAGE: c_int = KEY_MAX + 48;
pub const REQ_SCR_FCHAR: c_int = KEY_MAX + 49;
pub const REQ_SCR_BCHAR: c_int = KEY_MAX + 50;
pub const REQ_SCR_HFLINE: c_int = KEY_MAX + 51;
pub const REQ_SCR_HBLINE: c_int = KEY_MAX + 52;
pub const REQ_SCR_HFHALF: c_int = KEY_MAX + 53;
pub const REQ_SCR_HBHALF: c_int = KEY_MAX + 54;
pub const REQ_VALIDATION: c_int = KEY_MAX + 55;
pub const REQ_NEXT_CHOICE: c_int = KEY_MAX + 56;
pub const REQ_PREV_CHOICE: c_int = KEY_MAX + 57;


pub type FORM = ll::FORM;
pub type FIELD = ll::FIELD;
pub type FIELDTYPE = ll::FIELDTYPE;
pub type FieldOptions = ll::FieldOptions;

pub const O_VISIBLE: FieldOptions = 0x0001;
pub const O_ACTIVE: FieldOptions = 0x0002;
pub const O_PUBLIC: FieldOptions = 0x0004;
pub const O_EDIT: FieldOptions = 0x0008;
pub const O_WRAP: FieldOptions = 0x0010;
pub const O_BLANK: FieldOptions = 0x0020;
pub const O_AUTOSKIP: FieldOptions = 0x0040;
pub const O_NULLOK: FieldOptions = 0x0080;
pub const O_PASSOK: FieldOptions = 0x0100;
pub const O_STATIC: FieldOptions = 0x0200;
pub const O_DYNAMIC_JUSTIFY: FieldOptions = 0x0400;
pub const O_NO_LEFT_STRIP: FieldOptions =  0x0800;

#[derive(Debug, Copy, Clone)]
pub enum Justification {
    Disable = 0,
    Left = 1,
    Center = 2,
    Right = 3,
}

impl From<c_int> for Justification {
    fn from(int: c_int) -> Self {
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

pub type FormResult = Result<i32, FormCode>;

trait FromForm {
    fn from_value(_:c_int) -> Self;
}


impl FromForm for FormResult {
    fn from_value(x: c_int) -> FormResult {
        if x >= 0 {
            Ok(x)
        } else {
            Err(match x {
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
            })
        }
    }
}


pub fn set_current_field(form: FORM, field: FIELD) -> FormResult
{ FormResult::from_value( unsafe { ll::set_current_field(form, field) } ) }

pub fn current_field(form: FORM) -> Option<FIELD>
{ unsafe { ll::current_field(form).as_mut().map(|x| {x as FIELD}) } }

pub fn unfocus_current_field(form: FORM) -> FormResult
{ FormResult::from_value( unsafe { ll::unfocus_current_field(form) } ) }

pub fn set_form_page(form: FORM, n: i32) -> FormResult
{ FormResult::from_value( unsafe { ll::set_form_page(form, n) } ) }

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
{ FormResult::from_value( unsafe { ll::free_field(field) } ) }


pub fn field_info(field: FIELD) -> Result<(i32, i32, i32, i32, i32, i32), FormCode> {
    let (rows, cols, frow, fcol, nrow, nbuf): (i32, i32, i32, i32, i32, i32) = (0, 0, 0, 0, 0, 0);
    let result = FormResult::from_value( unsafe { ll::field_info(field, rows as *mut c_int,
                                                              cols as *mut c_int, frow as *mut c_int,
                                                              fcol as *mut c_int, nrow as *mut c_int,
                                                              nbuf as *mut c_int) } );
    match result {
        Ok(_) => Ok((rows, cols, frow, fcol, nrow, nbuf)),
        Err(x) => Err(x)
    }
}

pub fn dynamic_field_info(field: FIELD) -> Result<(i32, i32, i32), FormCode>
{
    let (rows, cols, max): (i32, i32, i32) = (0, 0, 0);
    let result = FormResult::from_value( unsafe { ll::dynamic_field_info(field, rows as *mut c_int,
                                                                         cols as *mut c_int, max as *mut c_int) } );
    match result {
        Ok(_) => Ok((rows, cols, max)),
        Err(x) => Err(x)
    }
}


// // pub fn int set_field_type(FIELD *field, FIELDTYPE *type, ...) { unsafe { ll:: } } TODO
//pub fn field_type(field: FIELD) -> FIELDTYPE { unsafe { ll:: } }
// // pub fn void *field_arg(const FIELD *field) { unsafe { ll:: } } TODO


pub fn set_field_buffer(field: FIELD, buf: i32, value: &str) -> FormResult
{ FormResult::from_value( unsafe { ll::set_field_buffer(field, buf, value.to_c_str().as_ptr()) } ) }

pub fn field_buffer(field: FIELD, buffer: i32) -> String
{ unsafe { FromCStr::from_c_str(ll::field_buffer(field, buffer)) } }

pub fn set_field_status(field: FIELD, status: bool) -> FormResult
{ FormResult::from_value( unsafe { ll::set_field_status(field, if status { 1 } else { 0 }) } ) }

pub fn field_status(field: FIELD) -> bool
{ unsafe { if ll::field_status(field) == 1 { true } else { false } } }

pub fn set_max_field(field: FIELD, max: i32) -> FormResult
{ FormResult::from_value( unsafe { ll::set_max_field(field, max) } ) }


pub fn set_form_fields(form: FORM, fields: &mut Vec<FIELD>) -> FormResult {
    fields.push(ptr::null_mut());
    let res = unsafe { ll::set_form_fields(form, fields.as_mut_ptr()) };
    fields.pop();
    FormResult::from_value(res)
}

pub fn form_fields(form: FORM) -> Vec<FIELD>
{ unsafe { slice::from_raw_parts(ll::form_fields(form), ll::field_count(form) as usize).to_vec() } }

pub fn field_count(form: FORM) -> i32
{ unsafe { ll::field_count(form) } }

pub fn move_field(field: FIELD, frow: i32, fcol: i32) -> FormResult
{ FormResult::from_value( unsafe { ll::move_field(field, frow, fcol) } ) }


pub fn set_field_fore(field: FIELD, attr: chtype) -> FormResult
{ FormResult::from_value( unsafe { ll::set_field_fore(field, attr) } ) }

pub fn field_fore(field: FIELD) -> chtype
{ unsafe { ll::field_fore(field) } }

pub fn set_field_back(field: FIELD, attr: chtype) -> FormResult
{ FormResult::from_value( unsafe { ll::set_field_back(field, attr) } ) }

pub fn field_back(field: FIELD) -> chtype
{ unsafe { ll::field_back(field) } }

pub fn set_field_pad(field: FIELD, pad: i32) -> FormResult
{ FormResult::from_value( unsafe { ll::set_field_pad(field, pad) } ) }

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
{ FormResult::from_value( unsafe { ll::set_field_just(field, value as c_int) } ) }
//{ FormResult::from_value( unsafe { ll::set_field_just(field, Justification::to_int(value)) } ) }

pub fn field_just(field: FIELD) -> Justification
{ unsafe { Justification::from(ll::field_just(field)) } }


pub fn set_field_opts(field: FIELD, options: FieldOptions) -> FormResult
{ FormResult::from_value( unsafe { ll::set_field_opts(field, options) } ) }

pub fn field_opts_on(field: FIELD, options: FieldOptions) -> FormResult
{ FormResult::from_value( unsafe { ll::field_opts_on(field, options) } ) }

pub fn field_opts_off(field: FIELD, options: FieldOptions) -> FormResult
{ FormResult::from_value( unsafe { ll::field_opts_off(field, options) } ) }

pub fn field_opts(field: FIELD) -> FieldOptions
{ unsafe { ll::field_opts(field) } }


// TODO: improve the parameter type
pub fn form_driver(form: FORM, c: i32) -> FormResult
{ FormResult::from_value( unsafe { ll::form_driver(form, c) } ) }

// // pub fn form_driver_w(_:FORM, _:c_int, wchar_t wch) -> FormResult { FormResult::from_value( unsafe { ll:: } ) } TODO wchar ?


pub fn set_form_opts(form: FORM, options: FieldOptions) -> FormResult
{ FormResult::from_value( unsafe { ll::set_form_opts(form, options) } ) }

pub fn form_opts_on(form: FORM, options: FieldOptions) -> FormResult
{ FormResult::from_value( unsafe { ll::form_opts_on(form, options) } ) }

pub fn form_opts_off(form: FORM, options: FieldOptions) -> FormResult
{ FormResult::from_value( unsafe { ll::form_opts_off(form, options) } ) }

pub fn form_opts(form: FORM) -> FieldOptions
{ unsafe { ll::form_opts(form) } }

pub fn form_request_name(request: i32) -> String
{ unsafe { FromCStr::from_c_str(ll::form_request_name(request))} }

pub fn form_request_by_name(name: &str) -> FormResult
{ FormResult::from_value( unsafe { ll::form_request_by_name(name.to_c_str().as_ptr()) } ) }


pub fn set_form_win(form: FORM, window: WINDOW) -> FormResult
{ FormResult::from_value( unsafe { ll::set_form_win(form, window) } ) }

pub fn form_win(form: FORM) -> Option<WINDOW>
{ unsafe { ll::form_win(form).as_mut().map(|x| x as WINDOW) } }

pub fn set_form_sub(form: FORM, window: WINDOW) -> FormResult
{ FormResult::from_value( unsafe { ll::set_form_sub(form, window) } ) }

pub fn form_sub(form: FORM) -> Option<WINDOW>
{ unsafe { ll::form_sub(form).as_mut().map(|x| x as WINDOW) } }

pub fn scale_form(form: FORM, rows: &mut i32, cols: &mut i32) -> FormResult
{ FormResult::from_value( unsafe { ll::scale_form(form, rows as *mut c_int, cols as *mut c_int) } ) }


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


pub fn new_form(fields: &mut Vec<FIELD>) -> Option<FORM> {
    fields.push(ptr::null_mut());
    let form = unsafe { ll::new_form(fields.as_mut_ptr()).as_mut().map(|x| x as FORM) };
    fields.pop();
    form
}

pub fn free_form(form: FORM) -> FormResult
{ FormResult::from_value( unsafe { ll::free_form(form) } ) }


pub fn set_new_page(field: FIELD, new_page_flag: bool) -> FormResult
{ FormResult::from_value( unsafe { ll::set_new_page(field, if new_page_flag { 1 } else { 0 }) } ) }

pub fn new_page(field: FIELD) -> bool
{ unsafe { if ll::new_page(field) == 1 { true } else { false } } }


pub fn pos_form_cursor(form: FORM) -> FormResult
{ FormResult::from_value( unsafe { ll::pos_form_cursor(form) } ) }


pub fn post_form(form: FORM) -> FormResult
{ FormResult::from_value( unsafe { ll::post_form(form) } ) }

pub fn unpost_form(form: FORM) -> FormResult
{ FormResult::from_value( unsafe { ll::unpost_form(form) } ) }
