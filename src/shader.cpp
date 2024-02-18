#include "shader.hpp"
#include <glm/ext/quaternion_common.hpp>
#include <glm/ext/quaternion_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <iostream>
#define GLM_ENABLE_EXPERIMENTAL
#include <glm/gtx/quaternion.hpp>

GeometryShader::GeometryShader() {
  program = glCreateProgram();
  GLuint vertex = glCreateShader(GL_VERTEX_SHADER);
  auto vertex_source = R"(
    #version 430

    // struct ModelMatricies {
    //   mat4 model;
    //   mat4 model_inv;
    // };

    // layout(std430, binding = 0) buffer model_buffer {
      // ModelMatricies models[];
    // };

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    uniform mat4 model_inv;

    layout(location = 0) in vec4 vertexPosition;
    layout(location = 1) in vec3 vertexNormal;

    out vec3 normal;
    out vec3 position;

    void main() {
      // mat4 model = models[gl_InstanceID].model;
      // mat4 model_inv = models[gl_InstanceID].model_inv;
      gl_Position = projection * view * model * vertexPosition;
      position = vec3(model * vertexPosition);
      normal = normalize(vec3(vec4(vertexNormal, 0) * model_inv));
    }
  )";
  glShaderSource(vertex, 1, &vertex_source, NULL);
  glCompileShader(vertex);
  glAttachShader(program, vertex);

  GLuint fragment = glCreateShader(GL_FRAGMENT_SHADER);
  auto fragment_source = R"(
    #version 430

    in vec3 normal;
    in vec3 position;

    out vec4 frag_color;

    void main() {
        vec3 light_dir = vec3(-0.7, 1, 0.5);
        float ambient_lightness = 0.3;
        float bright_lightness = 0.7 * max(0, dot(light_dir, normalize(normal)));
        float lightness = ambient_lightness + bright_lightness;
        vec3 color = vec3(1, 1, 1) * lightness;
        frag_color = vec4(color, 1);
    }
  )";
  glShaderSource(fragment, 1, &fragment_source, NULL);
  glCompileShader(fragment);
  glAttachShader(program, fragment);

  glLinkProgram(program);

  // glCreateBuffers(1, &model_buffer);

  model = glGetUniformLocation(program, "model");
  view = glGetUniformLocation(program, "view");
  projection = glGetUniformLocation(program, "projection");
  model_inv = glGetUniformLocation(program, "model_inv");
}

void GeometryShader::drawObjects(
                                 const Camera& camera,
                                 const std::vector<Object>& objects) {

  // struct ModelMatricies {
  //   glm::mat4 model, model_inv;
  // };

  glUseProgram(program);
  glUniformMatrix4fv(view, 1, GL_FALSE, glm::value_ptr(camera.view()));
  glUniformMatrix4fv(projection, 1, GL_FALSE, glm::value_ptr(camera.projection()));

  // std::vector<ModelMatricies> models(objects.size());

  // #pragma omp parallel for
  for (size_t i = 0; i < objects.size(); ++i) {
    auto& object = objects[i];
    auto model = glm::translate(glm::mat4(1), object.position)
      * glm::mat4_cast(object.rotation)
      * object.geometry.transform;

    auto model_inv = glm::inverse(model);

    // models[i].model = model;
    // models[i].model_inv = model_inv;

    glUniformMatrix4fv(this->model, 1, GL_FALSE, glm::value_ptr(model));
    glUniformMatrix4fv(this->model_inv, 1, GL_FALSE, glm::value_ptr(model_inv));

    glBindVertexArray(object.geometry.mesh.vertex_array);
    glDrawElements(GL_TRIANGLES,
      object.geometry.mesh.vertex_count, GL_UNSIGNED_SHORT, NULL);
  }

  // glBindBuffer(GL_SHADER_STORAGE_BUFFER, model_buffer);
  // glBufferData(GL_SHADER_STORAGE_BUFFER, sizeof(ModelMatricies) * models.size(), models.data(), GL_DYNAMIC_DRAW);
  // glBindBufferBase(GL_SHADER_STORAGE_BUFFER, 0, model_buffer);
  // glBindVertexArray(objects[0].geometry.mesh.vertex_array);
  // glDrawElementsInstanced(GL_TRIANGLES,
  //   objects[0].geometry.mesh.vertex_count, GL_UNSIGNED_SHORT, NULL, models.size());

}

AABBShader::AABBShader() {
  program = glCreateProgram();
  GLuint vertex = glCreateShader(GL_VERTEX_SHADER);
  auto vertex_source = R"(
    #version 430

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    layout(location = 0) in vec4 vertexPosition;

    void main() {
      gl_Position = projection * view * model * vertexPosition;
    }
  )";
  glShaderSource(vertex, 1, &vertex_source, NULL);
  glCompileShader(vertex);
  glAttachShader(program, vertex);

  GLuint fragment = glCreateShader(GL_FRAGMENT_SHADER);
  auto fragment_source = R"(
    #version 430

    out vec4 frag_color;

    void main() {
        frag_color = vec4(1, 0, 1, 1);
    }
  )";
  glShaderSource(fragment, 1, &fragment_source, NULL);
  glCompileShader(fragment);
  glAttachShader(program, fragment);

  glLinkProgram(program);

  model = glGetUniformLocation(program, "model");
  view = glGetUniformLocation(program, "view");
  projection = glGetUniformLocation(program, "projection");

  mesh = Mesh::bounding_box();
}

void AABBShader::drawObjects(const Camera& camera,
                             const std::vector<Object>& objects) {
  glUseProgram(program);
  glUniformMatrix4fv(view, 1, GL_FALSE, glm::value_ptr(camera.view()));
  glUniformMatrix4fv(projection, 1, GL_FALSE, glm::value_ptr(camera.projection()));

  for (size_t i = 0; i < objects.size(); ++i) {
    auto& object = objects[i];
    auto aabb = object.aabb();
    auto pos = (aabb.low + aabb.high)/2.0f;
    auto size = aabb.high - aabb.low;
    auto model = glm::translate(glm::mat4(1), pos) * glm::scale(glm::mat4(1), size);

    glUniformMatrix4fv(this->model, 1, GL_FALSE, glm::value_ptr(model));

    glBindVertexArray(mesh.vertex_array);
    glDrawElements(GL_LINES,
      mesh.vertex_count, GL_UNSIGNED_SHORT, NULL);
  }
}
