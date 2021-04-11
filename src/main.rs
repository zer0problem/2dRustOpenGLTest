extern crate glfw;
extern crate gl;

#[macro_use]
extern crate c_string;

mod tpixel;
mod heaven;

use glfw::{Context};

fn main() {
    let mut engine = tpixel::engine::Engine::new();
    let mut game = heaven::game::Game::new();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    //glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(1280, 720, "Qaucking Wholesome", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    engine.init();
    game.init(&mut engine);

    while !window.should_close() {
        engine.start_frame();
        game.update(&mut engine);
        engine.render();

        window.swap_buffers();
        glfw.poll_events();
        engine.update_input(&window);
        for (_, event) in glfw::flush_messages(&events) {
            engine.process_event(&mut window, &event);
        }
    }
}
