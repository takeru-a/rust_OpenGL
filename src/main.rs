use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::mem;
use std::os::raw::c_void;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};

mod shader;
mod vertex;

use shader::Shader;
use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    // sdl2の初期化 
    // Sdl構造体の取得 - 各種設定やメソッドが定義された
    let sdl_context = sdl2::init().unwrap();
    // VideoSubsystem構造体- ウィンドウ、ディスプレイの機能
    let video_subsystem = sdl_context.video().unwrap();
    
    // OpenGLに対応したウィンドウを設定
    // {}で囲むことで設定が終わるとgl_attrが自動的に破棄される
    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);  // version 3.1
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version={}.{}", major, minor);
    }
    // ウィンドウを作成
    let window = video_subsystem
    .window("SDL", 640, 480)
    .opengl()// OpenGLを有効にする
    .position_centered() //ウィンドウをディスプレイの中央に配置
    .build()// ウィンドウを作成 Result<Window, Error>WindowとErrorがパックされて返ってくる。
    .unwrap(); // 本来ならエラー処理を書くべき

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _); //APIの関数ポインタを手に入れて、読み込む

    // shaderの生成 それぞれのファイルから中身を読み込む- CString型のデータ
    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");
    
    // rustfmtの効果を無効化
    #[rustfmt::skip]
    // 三角形の座標　(x,y,z) × 3つの頂点
    let buffer_array: [f32; BUF_LEN] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
    ];

    let vertex = Vertex::new(
        (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
        buffer_array.as_ptr() as *const c_void,
        gl::STATIC_DRAW, // 更新頻度:低、使用頻度:高
        vec![gl::FLOAT],
        vec![FLOAT_NUM as i32],
        FLOAT_NUM as i32 * mem::size_of::<GLfloat>() as GLsizei,
        VERTEX_NUM as i32,
    );


    //  イベントループ - 初期化後-ユーザからの入力などのイベントによる条件分岐
    let mut event_pump = sdl_context.event_pump().unwrap();
        // runningというラベルがついたループを行う
        'running: loop {
            //　キューに溜まったイベントを取り出す
            for event in event_pump.poll_iter()  {
                // 条件分岐
                match event {
                    // 終了イベント - (x)button
                    // .. 残りの部分を無視
                    Event::Quit { .. } 
                    | 
                    //　エスケープキーの押されたとき
                    Event::KeyDown { 
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running, // escKey or (x)button -> loopを抜ける
                    _ => {} // その他
                }
            }

            unsafe {
                // 描画する内容、描画する位置を指定
                gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
                // 色指定
                gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                let model_matrix = Matrix4::identity();
                let view_matrix = Matrix4::look_at(
                    Point3{
                        x: 0.0,
                        y: 0.0,
                        z: 5.0,
                    },
                    Point3{
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    Vector3{
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                );
                
                let projection_matrix: Matrix4 = perspective(
                    cgmath::Deg(45.0f32),
                    WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                    0.1,
                    100.0,
                );

                shader.use_program();
                shader.set_mat4(c_str!("uModel"), &model_matrix);
                shader.set_mat4(c_str!("uView"), &view_matrix);
                shader.set_mat4(c_str!("uProjection"), &projection_matrix);

                vertex.draw();

                window.gl_swap_window();
            }

            // 更新頻度を調整 60FPS
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
}
