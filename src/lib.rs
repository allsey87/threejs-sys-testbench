use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use threejs_sys::{Scene, PerspectiveCamera, BoxGeometry, MeshStandardMaterial, Mesh, Color, PointLight, WebGLRenderer};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_renderer(renderer: &WebGLRenderer);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_scene(scene: &Scene);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let window_inner_width = window.inner_width()?.as_f64().expect("could not get inner width");
    let window_inner_height = window.inner_height()?.as_f64().expect("could not get inner height");
    
    let renderer = WebGLRenderer::new();
    renderer.set_size(window_inner_width, window_inner_height);
    body.append_child(&renderer.dom_element())?;

    let scene = Scene::new();
    let camera = PerspectiveCamera::new(75.0, window_inner_width / window_inner_height, 0.1, 1000.0);
    camera.translate_z(5.0);
    let geometry = BoxGeometry::new(1.0, 1.0, 1.0);
    let color = Color::new(0.0, 1.0, 0.0);
    let material = MeshStandardMaterial::new();
    material.set_color(&color);
    let cube = Mesh::new(&geometry, &material);
    
    let light = PointLight::new(0xffffff, 1.0, 0.0, 1.0);

    //light.set_position(&Vector3::new(0.0,2.0,0.0));
    light.translate_y(1.0);
    light.translate_z(5.0);

    scene.add( &cube );
    scene.add( &light);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        cube.rotation().set_x(cube.rotation().x() + 0.01);
        cube.rotation().set_y(cube.rotation().y() + 0.01);
        renderer.render(&scene, &camera);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
   
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
