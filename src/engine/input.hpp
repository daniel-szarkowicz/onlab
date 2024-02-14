#pragma once

#include "context.hpp"
#include <glm/vec2.hpp>

class Input {
public:
  bool key_pressed(KEY);
  bool key_released(KEY);
  bool key_just_pressed(KEY);
  bool key_just_released(KEY);

  bool mouse_pressed(BUTTON);
  bool mouse_released(BUTTON);
  bool mouse_just_pressed(BUTTON);
  bool mouse_just_released(BUTTON);

  glm::vec2 mouse_pos();
  glm::vec2 mouse_speed();
private:
  bool _key_pressed[];
  bool _prev_key_pressed[];

  bool _mouse_pressed[];
  bool _prev_mouse_pressed[];

  glm::vec2 _mouse_pos[];
  glm::vec2 _prev_mouse_pos[];

  friend class Context;
};
