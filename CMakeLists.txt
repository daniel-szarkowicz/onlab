cmake_minimum_required(VERSION 3.24)

project(onlab)
add_executable(
  main
  src/main.cpp
  src/mesh.cpp
  src/geometry.cpp
  src/shader.cpp
  src/object.cpp
  src/collider.cpp
  src/simulation.cpp
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

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

set(CMAKE_CXX_FLAGS "-Wall -Wextra -fopenmp")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
