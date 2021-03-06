#[macro_use]
extern crate glium;

use glium::Surface;

mod support;

#[test]
fn get_opengl_version() {
    let display = support::build_display();
    let version = display.get_opengl_version();
    display.assert_no_error(None);

    assert!(version.1 >= 1);
}

#[test]
fn clear_color() {
    let display = support::build_display();

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(1.0, 0.0, 0.0, 1.0);

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();

    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(255, 0, 0, 255));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn clear_color_rect() {
    let display = support::build_display();

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(1.0, 0.0, 0.0, 1.0);

    let rect = glium::Rect { left: 512, bottom: 0, width: 512, height: 1024 };
    texture.as_surface().clear(Some(&rect), Some((0.0, 1.0, 0.0, 1.0)), None, None);

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();

    for row in data.iter() {
        for (col, pixel) in row.iter().enumerate() {
            if col >= 512 {
                assert_eq!(pixel, &(0, 255, 0, 255));
            } else {
                assert_eq!(pixel, &(255, 0, 0, 255));
            }
        }
    }

    display.assert_no_error(None);
}

#[test]
fn release_shader_compiler() {
    let display = support::build_display();
    display.release_shader_compiler();
    display.assert_no_error(None);
}

#[test]
fn viewport_too_large() {
    let display = support::build_display();

    let params = glium::DrawParameters::new(&display)
                    .with_viewport(glium::Rect {
                        left: 0,
                        bottom: 0,
                        width: 4294967295,
                        height: 4294967295,
                    });

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let mut frame = display.draw();
    match frame.draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms, &params) {
        Err(glium::DrawError::ViewportTooLarge) => (),
        a => panic!("{:?}", a)
    };
    frame.finish().unwrap();

    display.assert_no_error(None);
}

#[test]
fn timestamp_query() {
    let display = support::build_display();

    let query1 = glium::debug::TimestampQuery::new(&display);
    let query1 = query1.map(|q| q.get());

    let query2 = glium::debug::TimestampQuery::new(&display);
    let query2 = query2.map(|q| q.get());

    match (query1, query2) {
        (Some(q1), Some(q2)) => assert!(q2 >= q1 && q1 != 0),
        _ => ()
    };

    display.assert_no_error(None);
}

#[test]
fn wrong_depth_range() {
    let display = support::build_display();

    let params = glium::DrawParameters {
        depth_range: (-0.1, 1.0),
        .. Default::default()
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let mut frame = display.draw();
    match frame.draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms, &params) {
        Err(glium::DrawError::InvalidDepthRange) => (),
        a => panic!("{:?}", a)
    };
    frame.finish().unwrap();

    display.assert_no_error(None);
}

#[test]
fn scissor() {
    let display = support::build_display();

    let params = glium::DrawParameters {
        scissor: Some(glium::Rect {
            left: 0,
            bottom: 0,
            width: 1,
            height: 1,
        }),
        .. Default::default()
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms, &params).unwrap();

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();

    assert_eq!(data[0][0], (255, 0, 0, 255));
    assert_eq!(data[1][0], (0, 0, 0, 0));
    assert_eq!(data[0][1], (0, 0, 0, 0));
    assert_eq!(data[1][1], (0, 0, 0, 0));

    for row in data.iter().skip(1) {
        for pixel in row.iter().skip(1) {
            assert_eq!(pixel, &(0, 0, 0, 0));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn sync() {
    let display = support::build_display();

    let fence = glium::SyncFence::new_if_supported(&display);
    fence.map(|f| f.wait());

    display.assert_no_error(None);
}

#[test]
fn scissor_followed_by_clear() {
    let display = support::build_display();

    let params = glium::DrawParameters {
        scissor: Some(glium::Rect {
            left: 2,
            bottom: 2,
            width: 2,
            height: 2,
        }),
        .. Default::default()
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms,
                              &params).unwrap();
    texture.as_surface().clear_color(1.0, 0.0, 1.0, 1.0);

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(255, 0, 255, 255));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn viewport_followed_by_clear() {
    let display = support::build_display();

    let params = glium::DrawParameters {
        viewport: Some(glium::Rect {
            left: 2,
            bottom: 2,
            width: 2,
            height: 2,
        }),
        .. Default::default()
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms,
                              &params).unwrap();
    texture.as_surface().clear_color(1.0, 0.0, 1.0, 1.0);

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(255, 0, 255, 255));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn viewport() {
    let display = support::build_display();

    let params = glium::DrawParameters {
        viewport: Some(glium::Rect {
            left: 0,
            bottom: 0,
            width: 1,
            height: 1,
        }),
        .. Default::default()
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms, &params).unwrap();

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();

    assert_eq!(data[0][0], (255, 0, 0, 255));
    assert_eq!(data[1][0], (0, 0, 0, 0));
    assert_eq!(data[0][1], (0, 0, 0, 0));
    assert_eq!(data[1][1], (0, 0, 0, 0));

    for row in data.iter().skip(1) {
        for pixel in row.iter().skip(1) {
            assert_eq!(pixel, &(0, 0, 0, 0));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn dont_draw_primitives() {
    let display = support::build_display();

    let params = glium::DrawParameters {
        draw_primitives: false,
        .. Default::default()
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 1.0, 0.0, 0.0);
    match texture.as_surface().draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms, &params) {
        Ok(_) => (),
        Err(glium::DrawError::TransformFeedbackNotSupported) => return,
        e => e.unwrap()
    }

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(0, 255, 0, 0));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn dont_draw_primitives_then_draw() {
    let display = support::build_display();

    let params = glium::DrawParameters {
        draw_primitives: false,
        .. Default::default()
    };

    let (vb, ib, program) = support::build_fullscreen_red_pipeline(&display);

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 1.0, 0.0, 0.0);
    match texture.as_surface().draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms, &params) {
        Ok(_) => (),
        Err(glium::DrawError::TransformFeedbackNotSupported) => return,
        e => e.unwrap()
    }
    texture.as_surface().draw(&vb, &ib, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

    let data: Vec<Vec<(u8, u8, u8, u8)>> = texture.read();
    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(255, 0, 0, 255));
        }
    }

    display.assert_no_error(None);
}

#[test]
fn multiple_displays() {
    let display1 = support::build_display();
    let display2 = support::build_display();

    let (vb1, ib1, program1) = support::build_fullscreen_red_pipeline(&display1);
    let (vb2, ib2, program2) = support::build_fullscreen_red_pipeline(&display2);

    let texture1 = support::build_renderable_texture(&display1);
    let texture2 = support::build_renderable_texture(&display2);

    texture1.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture2.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    texture1.as_surface().draw(&vb1, &ib1, &program1, &glium::uniforms::EmptyUniforms,
                               &Default::default()).unwrap();

    texture2.as_surface().draw(&vb2, &ib2, &program2, &glium::uniforms::EmptyUniforms,
                               &Default::default()).unwrap();
    texture2.as_surface().clear_color(0.0, 1.0, 0.0, 0.0);

    let read_back: Vec<Vec<(u8, u8, u8, u8)>> = texture1.read();
    for row in read_back.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(255, 0, 0, 255));
        }
    }

    let read_back: Vec<Vec<(u8, u8, u8, u8)>> = texture2.read();
    for row in read_back.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(0, 255, 0, 0));
        }
    }

    display1.assert_no_error(None);
    display2.assert_no_error(None);
}

#[test]
fn debug_string() {
    // tests that `insert_debug_marker` doesn't trigger an OpenGL error
    let display = support::build_display();
    display.insert_debug_marker("Hello world").ok();
    display.assert_no_error(None);
}


#[test]
fn is_context_lost() {
    // tests that `is_context_lost` doesn't trigger an OpenGL error
    let display = support::build_display();
    display.is_context_lost();
    display.assert_no_error(None);
}
