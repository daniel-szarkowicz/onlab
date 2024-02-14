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
    }

    if (!imgui) {
        IMGUI_CHECKVERSION();
        ImGui::CreateContext();
        ImGuiIO& io = ImGui::GetIO();
        io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;
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
