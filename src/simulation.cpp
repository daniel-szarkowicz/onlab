#include "simulation.hpp"

Simulation::Simulation() : paused(true) {}

void Simulation::simulate(float dt, std::vector<Object>& objects) {
    if (!paused) {
      for (auto& object : objects) {
        object.update(dt);
      }
    }

    for (auto& object : objects) {
      object.reset_force();
    }

}
