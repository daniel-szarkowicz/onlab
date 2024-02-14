#pragma once

class Context {
    Context() = delete;
protected:
    static bool should_loop();
    static void frame_start();
    static void frame_end();
public:
    static void init(int window_width, int window_height, const char* title);
    template<typename F, typename... T>
    static void loop(F frame, T... initial_state) {
        while (should_loop()) {
            frame_start();
            frame(initial_state...);
            frame_end();
        }
    }
    static void uninit();
};
