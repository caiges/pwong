extern crate sdl2;

pub fn new_game(event_subsystem: &sdl2::EventSubsystem) -> sdl2::event::Event {
    let custom_event_type_id = unsafe { event_subsystem.register_event().unwrap() };
    sdl2::event::Event::User {
        timestamp: 0,
        window_id: 0,
        type_: custom_event_type_id,
        code: 400,
        data1: 0x1234 as *mut ::sdl2::libc::c_void,
        data2: 0x5678 as *mut ::sdl2::libc::c_void,
    }
}

pub fn quit_game(event_subsystem: &sdl2::EventSubsystem) -> sdl2::event::Event {
    let custom_event_type_id = unsafe { event_subsystem.register_event().unwrap() };
    sdl2::event::Event::User {
        timestamp: 0,
        window_id: 0,
        type_: custom_event_type_id,
        code: 500,
        data1: 0x1234 as *mut ::sdl2::libc::c_void,
        data2: 0x5678 as *mut ::sdl2::libc::c_void,
    }
}
