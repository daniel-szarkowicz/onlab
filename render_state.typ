For now this is put on the back burner.

RenderState should have compile-checked contexts. For example:
```rust
// self.shader is of type Shader<PNVertex>
// self.mesh1 is of type Mesh<PNVertex>
// self.mesh2 is of type Mesh<PVertex>
fn draw(
  shader: &Shader<PNVertex>,
  mesh1: &Mesh<PNVertex>,
  mesh2: &Mesh<PVertex>,
  render_state: &mut RenderState<NoShader>,
) {
  render_state.set_line_width(1.0);
  render_state.with(shader, |state: RenderState<Shader<PNVertex>>| {
    // some functions are legal in both places
    state.set_line_width(2.0);
    state.draw_mesh(mesh1); // ok
    state.draw_mesh(mesh2); // error: incompatible vertex type
    state.with(other_shader, |s| {}); // error: shader already bound
  });
  // render state can be used once again
  render_state.set_line_width(2.0);
}
```

The implementation of RenderState should look something like this:
```rust
pub struct RenderState<T> {
  inner: InnerRenderState,
  phantom: PhantomData<T>,
}

// general implementation for all states
impl<T> RenderState<T> {
  pub fn new(...) -> RenderState<NoShader> { ... }
  pub fn set_line_width(&mut self, ...) { ... }
  pub fn with<V>(&mut self, shader: Shader<V>, f: FnOnce(&mut RenderState<Shader<V>>)) {
    let shader_state = 
    f()
  }
}

// implementation when no shader is bound
impl RenderState<NoShader> {
  
}

// implementation when a shader is bound
impl<V> RenderState<Shader<V>> {
  
}
```
