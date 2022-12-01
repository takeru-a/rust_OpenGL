use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    // sdl2の初期化 
    // Sdl構造体の取得 - 各種設定やメソッドが定義された
    let sdl_context = sdl2::init().unwrap();
    // VideoSubsystem構造体- ウィンドウ、ディスプレイの機能
    let video_subsystem = sdl_context.video().unwrap();
    // ウィンドウを作成
    let window = video_subsystem
    .window("SDL", 640, 480)
    .position_centered() //ウィンドウをディスプレイの中央に配置
    .build()// ウィンドウを作成 Result<Window, Error>WindowとErrorがパックされて返ってくる。
    .unwrap(); // 本来ならエラー処理を書くべき

    // キャンパスを取得
    let mut canvas = window
                                    .into_canvas()
                                    .build()
                                    .unwrap();
    // 塗りつぶしする色を指定
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear(); //　塗りつぶし処理
    canvas.present(); //レンダリングの結果を画面に反映

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
            canvas.present();
            // 更新頻度を調整 60FPS
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
}
