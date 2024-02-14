#include "camera.hpp"
#include <glm/gtc/matrix_transform.hpp>
#include <GL/glew.h>

glm::mat4 Camera::projection() {
    return glm::perspective(fov(), aspect(), near(), far());
}

glm::mat4 Camera::view() { return glm::lookAt(position(), look_at(), up()); }

glm::mat4 Camera::view_projection() { return projection() * view(); }

float Camera::fov() { return glm::radians(60.0f); }

float Camera::aspect() {
    GLint vp[4];
    glGetIntegerv(GL_VIEWPORT, vp);
    return (float)vp[2]/(float)vp[3];
}

float Camera::near() { return 0.1f; }

float Camera::far() { return 1000.0f; }

glm::vec3 Camera::up() { return glm::vec3(0.0f, 1.0f, 0.0f); }

OrbitingCamera::OrbitingCamera(glm::vec3 center, float distance, float pitch,
                               float yaw)
    : center(center), distance(distance), pitch(pitch), yaw(yaw) {}

glm::vec3 OrbitingCamera::position() {
    glm::mat4 rot = glm::mat4(1.0f);
    rot = glm::rotate(rot, glm::radians(yaw), glm::vec3(0.0f, 1.0f, 0.0f));
    rot = glm::rotate(rot, glm::radians(-pitch), glm::vec3(1.0f, 0.0f, 0.0f));
    return rot * glm::vec4(0.0f, 0.0f, distance, 1.0f);
}

glm::vec3 OrbitingCamera::look_at() { return center; }

FirstPersonCamera::FirstPersonCamera(glm::vec3 position, float yaw,
                                     float pitch)
    : pos(position), yaw(yaw), pitch(pitch) {}

glm::vec3 FirstPersonCamera::position() {
  return pos;
}

glm::vec3 FirstPersonCamera::look_at() {
  glm::mat4 rot = glm::mat4(1.0f);
  rot = glm::rotate(rot, glm::radians(yaw), glm::vec3(0, 1, 0));
  rot = glm::rotate(rot, glm::radians(pitch), glm::vec3(0, 0, 1));
  return pos + glm::vec3(rot * glm::vec4(1, 0, 0, 1));
}

void FirstPersonCamera::move_facing(glm::vec3 movement) {
  glm::mat4 rot = 
    glm::rotate(glm::mat4(1.0f), glm::radians(yaw), glm::vec3(0, 1, 0));
  pos += glm::vec3(rot * glm::vec4(movement, 1));
}

