#pragma once

#include "object.hpp"
#include <vector>
class Simulation {
public:
  bool paused;

  Simulation();
  void simulate(float dt, std::vector<Object>& objects);
};
