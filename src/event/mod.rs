extern crate sdl2;

pub fn register_custom_events(event_subsystem: &sdl2::EventSubsystem) {
    let custom_event_type_id = unsafe { event_subsystem.register_event().unwrap() };
    let start_game = sdl2::event::Event::User {
        timestamp: 0,
        window_id: 0,
        type_: custom_event_type_id,
        code: 470,
        data1: 0x1234 as *mut ::sdl2::libc::c_void,
        data2: 0x5678 as *mut ::sdl2::libc::c_void,
    };

    event_subsystem.push_event(start_game);

    let quit = sdl2::event::Event::User {
        timestamp: 0,
        window_id: 0,
        type_: custom_event_type_id,
        code: 456,
        data1: 0x1234 as *mut ::sdl2::libc::c_void,
        data2: 0x5678 as *mut ::sdl2::libc::c_void,
    };

    event_subsystem.push_event(quit);
}

pub fn quit_game_event(event_subsystem: &sdl2::EventSubsystem) -> sdl2::event::Event {
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
