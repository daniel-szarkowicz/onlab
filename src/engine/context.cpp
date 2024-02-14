#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <cstdio>
#include <cstdlib>
#include "context.hpp"
#include "imgui/imgui.h"
#include "imgui/backends/imgui_impl_glfw.h"
#include "imgui/backends/imgui_impl_opengl3.h"

static bool glfw = false;
static GLFWwindow* window = NULL;
static bool imgui = false;
static bool imgui_glfw = false;
static bool imgui_opengl = false;
bool Context::key_pressed[GLFW_KEY_LAST];
double Context::mouse_x;
double Context::mouse_y;
double Context::mouse_dx;
double Context::mouse_dy;
// key pressed, just pressed, just released
// mouse buttons pressed, just pressed, just released
// mouse pos, delta

static void GLAPIENTRY message_callback(GLenum source, GLenum type, GLuint id,
                                        GLenum severity, GLsizei length,
                                        const GLchar* message,
                                        const void* userParam) {
    (void)source;
    (void)type;
    (void)id;
    (void)length;
    (void)userParam;
    switch (severity) {
    case GL_DEBUG_SEVERITY_HIGH: {
        fprintf(stderr, "[ERROR]: OpenGL: %s\n", message);
        exit(1);
    } break;
    case GL_DEBUG_SEVERITY_MEDIUM: {
        fprintf(stderr, "[WARNING]: OpenGL: %s\n", message);
    } break;
    }
}

static void framebuffer_size_callback(GLFWwindow* window, int width,
                                      int height) {
    (void)window;
    glViewport(0, 0, width, height);
}

static void key_callback(GLFWwindow* window, int key, int scancode, int action,
                          int mods) {
    if (action == GLFW_PRESS) {
        Context::key_pressed[key] = true;
    } else if (action == GLFW_RELEASE) {
        Context::key_pressed[key] = false;
    }
}

void Context::init(int window_width, int window_height, const char* title) {
    atexit(Context::uninit);

    if (!glfw) {
        if (!glfwInit()) {
            fprintf(stderr, "[ERROR]: GLFW init error\n");
            exit(1);
        }
        glfw = true;
    }

    if (!window) {
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        window = glfwCreateWindow(window_width, window_height, title, NULL, NULL);
        if (!window) {
            fprintf(stderr, "[ERROR]: GLFW window error\n");
            exit(1);
        }
        glfwMakeContextCurrent(window);
        glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);
        glfwSetKeyCallback(window, key_callback);
    }

    if (!imgui) {
        IMGUI_CHECKVERSION();
        ImGui::CreateContext();
        ImGuiIO& io = ImGui::GetIO();
        io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;
        io.ConfigFlags |= ImGuiConfigFlags_NoMouseCursorChange;
        ImGui::StyleColorsDark();
        imgui = true;
    }

    if (!imgui_glfw) {
        ImGui_ImplGlfw_InitForOpenGL(window, true);
        imgui_glfw = true;
    }

    if (!imgui_opengl) {
        ImGui_ImplOpenGL3_Init("#version 430");
        imgui_opengl = true;
    }

    glewInit();
    printf("Renderer: %s\n", glGetString(GL_RENDERER));
    printf("OpenGL version: %s\n", glGetString(GL_VERSION));

    glEnable(GL_DEBUG_OUTPUT);
    glDebugMessageCallback(message_callback, 0);
    glEnable(GL_DEPTH_TEST);
    glDepthFunc(GL_LESS);
}

bool Context::should_loop() {
    return !glfwWindowShouldClose(window);
}

void Context::frame_start() {
    glfwPollEvents();
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    ImGui_ImplOpenGL3_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();
    double x, y;
    glfwGetCursorPos(window, &x, &y);
    mouse_dx = x - mouse_x;
    mouse_dy = y - mouse_y;
    mouse_x = x;
    mouse_y = y;
}

void Context::frame_end() {
    ImGui::Render();
    ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
    glfwSwapBuffers(window);
}

void Context::uninit() {
    if (imgui_opengl) {
        ImGui_ImplOpenGL3_Shutdown();
        imgui_opengl = false;
    }

    if (imgui_glfw) {
        ImGui_ImplGlfw_Shutdown();
        imgui_glfw = false;
    }

    if (imgui) {
        ImGui::DestroyContext();
        imgui = false;
    }

    if (window) {
        glfwDestroyWindow(window);
        window = NULL;
    }

    if (glfw) {
        glfwTerminate();
        glfw = false;
    }
}

void Context::grab_mouse() {
    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_DISABLED);
    ImGui::GetIO().ConfigFlags |= ImGuiConfigFlags_NoMouse;
}

void Context::release_mouse() {
    glfwSetInputMode(window, GLFW_CURSOR, GLFW_CURSOR_NORMAL);
    ImGui::GetIO().ConfigFlags &= ~ImGuiConfigFlags_NoMouse;
}

