#pragma once

#include <glm/mat4x4.hpp>
#include <glm/vec3.hpp>

class Camera {
public:
    glm::mat4 projection();
    glm::mat4 view();
    glm::mat4 view_projection();

protected:
    virtual float fov();
    virtual float aspect();
    virtual float near();
    virtual float far();
    virtual glm::vec3 position() = 0;
    virtual glm::vec3 look_at() = 0;
    virtual glm::vec3 up();
};

class OrbitingCamera : public Camera {
public:
    glm::vec3 center;
    float distance;
    float pitch;
    float yaw;

public:
    OrbitingCamera(glm::vec3 center, float distance, float pitch, float yaw);

protected:
    virtual glm::vec3 position() override;
    virtual glm::vec3 look_at() override;
};

class FirstPersonCamera : public Camera {
public:
    glm::vec3 pos;
    float yaw;
    float pitch;

public:
    FirstPersonCamera(glm::vec3 position, float yaw, float pitch);
    void move_facing(glm::vec3 movement);

protected:
    virtual glm::vec3 position() override;
    virtual glm::vec3 look_at() override;
};
