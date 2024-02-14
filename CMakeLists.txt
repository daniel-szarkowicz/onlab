cmake_minimum_required(VERSION 3.24)

project(onlab)
add_executable(
  main
  src/main.cpp
  src/engine/camera.cpp
  src/engine/context.cpp
  src/engine/imgui/backends/imgui_impl_glfw.cpp
  src/engine/imgui/backends/imgui_impl_opengl3.cpp
  src/engine/imgui/imgui.cpp
  src/engine/imgui/imgui_draw.cpp
  src/engine/imgui/imgui_tables.cpp
  src/engine/imgui/imgui_widgets.cpp
)
include_directories(src src/engine src/engine/imgui)
target_link_libraries(main glfw GLEW GL)
