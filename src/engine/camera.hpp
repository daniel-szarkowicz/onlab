#pragma once

#include <glm/mat4x4.hpp>
#include <glm/vec3.hpp>

class Camera {
public:
    glm::mat4 projection() const;
    glm::mat4 view() const;
    glm::mat4 view_projection() const;

protected:
    virtual float fov() const;
    virtual float aspect() const;
    virtual float near() const;
    virtual float far() const;
    virtual glm::vec3 position() const = 0;
    virtual glm::vec3 look_at() const = 0;
    virtual glm::vec3 up() const;
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
    virtual glm::vec3 position() const override;
    virtual glm::vec3 look_at() const override;
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
    virtual glm::vec3 position() const override;
    virtual glm::vec3 look_at() const override;
};
