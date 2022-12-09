use std::mem;
use std::os::raw::c_void;

use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};

pub struct Vertex{
    vao: u32,
    _vbo: u32,
    vertex_num: i32,
}

impl  Vertex {
    pub fn new(
        size: GLsizeiptr, // 頂点データのサイズ
        data: *const c_void, // 頂点データのポインタ
        usage: GLenum, // アクセス頻度
        attribute_type_vec: std::vec::Vec<GLenum>, // 各頂点属性のデータ型
        attribute_size_vec: std::vec::Vec<GLint>, // 各頂点属性のデータサイズ
        stride: GLsizei, // 何個を置きに並んでいるか
        vertex_num: i32, // 頂点の数
    ) -> Vertex{
        // VAO,VBO: cpu からgpuにデータを送る際の形式
        // idを格納
        let mut vao = 0;
        let mut vbo = 0;

        // OpenGLはC言語のコードでできているので、Rustコンパイラでは検証ができないため明示的にunsafeで示している
        // unsafeは開発者がメモリ管理を意識しなければいけない
        unsafe {
            // VAO,VBOのGPU上のメモリを確保
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            
            // VAO, VBOを指定
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // 一回目のデータ転送
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);

            let mut offset = 0;
            for i in 0..attribute_size_vec.len(){
                // i番目の頂点属性の配列を有効にする
                gl::EnableVertexAttribArray(i as u32);
                // GPUに送るデータの設定
                gl::VertexAttribPointer(
                    i as u32,
                    attribute_size_vec[i],
                    attribute_type_vec[i],
                    gl::FALSE, // 正規化の有無
                    stride,
                    (offset * mem::size_of::<GLfloat>()) as *const c_void,
                );
                offset += attribute_size_vec[i] as usize;
            }
            // VAO, VBOの紐付け解除
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Vertex { vao: vao, _vbo: vbo, vertex_num: vertex_num }
    }

    pub fn draw(&self){
        unsafe{
            // 再びVAOを紐づけ
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num);
            // 紐づけ解除
            gl::BindVertexArray(0);
        }
    }
}