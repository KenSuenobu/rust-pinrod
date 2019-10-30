// OpenGL Drawing Texture
// OpenGL Canvas Support
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gl::types::GLuint;
use opengl_graphics::{Texture, TextureSettings};

use crate::core::point::Size;

pub struct DrawingTexture {
    texture_buffer: Vec<u8>,
    pub texture: Texture,
    pub fbo: GLuint,
}

impl DrawingTexture {
    pub fn new() -> Self {
        Self {
            texture_buffer: vec![0u8; 1],
            texture: Texture::empty(&TextureSettings::new()).unwrap(),
            fbo: 0,
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.texture_buffer = vec![0u8; size.w as usize * size.h as usize];
        self.texture = Texture::from_memory_alpha(
            &self.texture_buffer,
            size.w as u32,
            size.h as u32,
            &TextureSettings::new(),
        )
        .unwrap();

        unsafe {
            let mut fbos: [GLuint; 1] = [0];

            gl::GenFramebuffers(1, fbos.as_mut_ptr());
            self.fbo = fbos[0];

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.texture.get_id(),
                0,
            );
        }
    }

    pub fn switch_to_texture(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
        }
    }

    pub fn switch_to_fb(&mut self, fbo: GLuint) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
        }
    }
}
