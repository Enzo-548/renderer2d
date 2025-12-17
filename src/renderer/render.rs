    pub use winit::application::ApplicationHandler;
    pub use winit::event::WindowEvent;
    pub use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
    use winit::window::{self, WindowAttributes, WindowButtons};
    pub use winit::window::{Window, WindowId};
    
    #[derive(Default)]
    pub struct RenderApp{
        //Framebuffer proprio de desenho, iniciar em 0 0 0 0 para desenha-lo
        //pub framebuffer : Framebuffer,
        window : Option<Window>,

    }

    impl ApplicationHandler for RenderApp{
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let mut atributosjanela: WindowAttributes = Window::default_attributes();
            atributosjanela.resizable = false;

            let botoesjanela = WindowButtons::CLOSE;
            atributosjanela.enabled_buttons = botoesjanela;
            
            self.window = Some(event_loop.create_window(atributosjanela).unwrap());
            
            if let Some(window) = self.window.as_ref(){
                let (w,h) = (window.inner_size().height,window.inner_size().width);
                println!("largura: {w}");
                println!("altura: {h}");
            }
        }

        fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
            match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                },
                WindowEvent::RedrawRequested => {
                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in AboutToWait, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.

                    // Draw.

                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw in
                    // applications which do not always need to. Applications that redraw continuously
                    // can render here instead.
                    self.window.as_ref().unwrap().request_redraw();
                }
                _ => (),
            }
        }
    }

    impl RenderApp{
        pub fn build_window(){
            let event_loop = EventLoop::new().unwrap();     
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            //event_loop.set_control_flow(ControlFlow::Poll);

            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            event_loop.set_control_flow(ControlFlow::Wait);
            
            let mut app = RenderApp::default();
            let _ = event_loop.run_app(&mut app);
        }
    }