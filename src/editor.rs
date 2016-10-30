//!  A module for editing images.

// crates
extern crate image;

// from rust
use std::path::Path;

// from external crate


// from local crate
use blend;
use image::Image;
use position::Position;
use color::Color;

/// Blend 2 images into one. The image1 is the base and image2 is the top. 
/// 
/// Supported blend modes: "normal", "difference", multiply", "overlay", "screen"
/// Position: "top-left", "top-center", "top-right", "center-left", "center", "center-right", "bottom-left", "bottom-center", "bottom-right"
/// Opacity is any value from 0.0 - 1.0
/// offset_x and offset_y are added to the final position. Can also be negative offsets.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create images from file
/// let image1 = Image::from_file("tests/image/sample.jpg").unwrap();
/// let image2 = Image::from_file("tests/image/watermark.png").unwrap();
/// 
/// // Blend image2 on top of image1 using normal mode, opacity of 1.0 (100%), with image2 at the center, with 0 x and 0 y offsets. whew
/// let normal = editor::blend(&image1, &image2, "normal", 1.0, "center", 0, 0).unwrap();
///
/// // All the other blend modes
/// let difference = editor::blend(&image1, &image2, "difference", 1.0, "center", 0, 0).unwrap();
/// let multiply = editor::blend(&image1, &image2, "multiply", 1.0, "center", 0, 0).unwrap();
/// let overlay = editor::blend(&image1, &image2, "overlay", 1.0, "center", 0, 0).unwrap();
/// let screen = editor::blend(&image1, &image2, "screen", 1.0, "center", 0, 0).unwrap();
///
/// // Save it
/// let _ = editor::save(&normal, "tests/out/test_blend_normal.png");
/// let _ = editor::save(&difference, "tests/out/test_blend_difference.png");
/// let _ = editor::save(&multiply, "tests/out/test_blend_multiply.png");
/// let _ = editor::save(&overlay, "tests/out/test_blend_overlay.png");
/// let _ = editor::save(&screen, "tests/out/test_blend_screen.png");
/// ```
pub fn blend<'a>(image1: &Image, image2: &Image, blend_mode: &str, opacity: f32, position: &str, offset_x: i32, offset_y: i32) -> Result<Image, String> {
    
    let mut opacity = opacity;
    if opacity > 1.0 {
        opacity = 1.0
    } else if opacity < 0.0 {
        opacity = 0.0
    }

    // Turn into positioner struct
    let positioner = Position::new(position, offset_x, offset_y);

    // Position is for image2, image1 is canvas.
    let (offset_x, offset_y) = try!(positioner.get_x_y( image1.width, image1.height, image2.width, image2.height));

    let (w1, h1) = (image1.width, image1.height);
    let (w2, h2) = (image2.width, image2.height);

    // Check if it overlaps
    if (offset_x >= w1 ) ||
        (offset_x + w2 <= 0) ||
        (offset_y >= h1) ||
        (offset_y + h2 <= 0) {

        return Err("Invalid blending. Image 2 is outside the canvas.".to_string());
    }

    // Loop start X
    let mut loop_start_x = 0;
    let canvas_start_x = offset_x;
    if canvas_start_x < 0 {
        let diff = 0 - canvas_start_x;
        loop_start_x += diff;
    }

    // Loop end X
    let mut loop_end_x = w2;
    let canvas_end_x = offset_x + w2;
    if canvas_end_x > w1{
        let diff = canvas_end_x - w1;
        loop_end_x -= diff;
    }

    // Loop start Y
    let mut loop_start_y = 0;
    let canvas_start_y = offset_y;
    if canvas_start_y < 0 {
        let diff = 0 - canvas_start_y;
        loop_start_y += diff;
    }

    // Loop end Y
    let mut loop_end_y = h2;
    let canvas_end_y = offset_y + h2;
    if canvas_end_y > h1 {
        let diff = canvas_end_y - h1;
        loop_end_y -= diff;
    }

    let blend_mode = blend_mode.to_lowercase();
    match &*blend_mode {
        "normal" => {
            let image3 = try!(blend::normal( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        "difference" => {
            let image3 = try!(blend::difference( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        "multiply" => {
            let image3 = try!(blend::multiply( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        "overlay" => {
            let image3 = try!(blend::overlay( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        "screen" => {
            let image3 = try!(blend::screen( &image1, &image2, loop_start_y, loop_end_y, loop_start_x, loop_end_x, offset_x, offset_y, opacity ));
            Ok(image3)
        },
        _ => {
            Err(format!("Invalid blend type {}.", &*blend_mode))
        }
    }
}

/// Create a copy of an image as another image.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create image from file
/// let original = Image::from_file("tests/image/sample.jpg").unwrap();
///
/// // Copy it
/// let copy = editor::copy(&original);
/// ```
pub fn copy(src: &Image) -> Image {
    Image{
        width: src.width,
        height: src.height,
        bytes: src.bytes.clone(),
    }
}

/// Crop the image to the given dimension and position.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create image from file
/// let src = Image::from_file("tests/image/sample.gif").unwrap();
/// 
/// // Crop it
/// let top_left = editor::crop(&src, 250, 128, "top-left", 0, 0).unwrap();
/// let top_right = editor::crop(&src, 250, 128, "top-right", 0, 0).unwrap();
/// let center = editor::crop(&src, 250, 128, "center", 0, 0).unwrap();
///
/// // Save it
/// let _ = editor::save(&top_left, "tests/out/test_crop_top_left.png");
/// let _ = editor::save(&top_right, "tests/out/test_crop_top_right.png");
/// let _ = editor::save(&center, "tests/out/test_crop_center.png");
/// ```
pub fn crop(src: &Image, crop_width: i32, crop_height: i32, position: &str, offset_x: i32, offset_y: i32) -> Result<Image, String> {

    // Turn into positioner struct
    let positioner = Position::new(position, offset_x, offset_y);

    let (offset_x, offset_y) = try!(positioner.get_x_y( src.width, src.height, crop_width, crop_height));
    let offset_x = if offset_x < 0 { 0 } else { offset_x };
    let offset_y = if offset_y < 0 { 0 } else { offset_y };


    let mut height2 = offset_y + crop_height;
    if height2 > src.height { 
        height2 = src.height 
    }

    let mut width2 = offset_x + crop_width;
    if width2 > src.width { 
        width2 = src.width 
    }

    let mut dest = Image::blank(width2-offset_x, height2-offset_y);

    for y in 0..dest.height {
        for x in 0..dest.width {
            let pixel = try!(src.get_pixel(offset_x + x, offset_y + y));
            try!(dest.set_pixel(x, y, Color::rgba(pixel.r, pixel.g, pixel.b, pixel.a)));
        }
    }
    Ok(dest)
}

/// Fill an image with color.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
/// use raster::color::Color;
///
/// // Create a 100x100 image
/// let image = Image::blank(100, 100);
///
/// // Fill it with red
/// let image = editor::fill(&image, Color::rgb(255, 0, 0)).unwrap();
///
/// // Save it
/// let _ = editor::save(&image, "tests/out/test_fill.png");
/// ```
pub fn fill(src: &Image, color: Color) -> Result<Image, String> {

    let mut dest = Image::blank(src.width, src.height);

    for y in 0..dest.height {
        for x in 0..dest.width {
            try!(dest.set_pixel(x, y, color.clone()));
        }
    }

    Ok(dest)
}

/// Wrapper function for the resizeXXX family of functions. 
/// Resize an image to a given width, height and mode.
pub fn resize(src: &Image, w: i32, h: i32, mode: &str) -> Result<Image, String> {
    
    match mode {
        "exact" => {
            let dest = try!(resize_exact(&src, w, h));
            Ok(dest)
        }
        "exact_width" => {
            let dest = try!(resize_exact_width(&src, w));
            Ok(dest)
        }
        "exact_height" => {
            let dest = try!(resize_exact_height(&src, h));
            Ok(dest)
        }
        "fit" => {
            let dest = try!(resize_fit(&src, w, h));
            Ok(dest)
        },
        "fill" => {
            let dest = try!(resize_fill(&src, w, h));
            Ok(dest)
        },
        _ => {
            Err(format!("Invalid resize mode '{}'.", mode))
        },
    }
}

/// Resize image to exact dimensions ignoring aspect ratio. 
/// Useful if you want to force exact width and height.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create an image from file
/// let image = Image::from_file("tests/image/sample.jpg").unwrap();
/// 
/// let image = editor::resize_exact(&image, 100, 100).unwrap();
/// editor::save(&image, "tests/out/resize_exact.jpg");
/// ```
pub fn resize_exact(src: &Image, w: i32, h: i32) -> Result<Image, String> {

    let result = try!(resample(&src, w, h, "bicubic"));
    Ok(result)
}

/// Resize image to exact height. Width is auto calculated.
/// Useful for creating row of images with the same height.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create an image from file
/// let image = Image::from_file("tests/image/sample.jpg").unwrap();
/// 
/// let image = editor::resize_exact_height(&image, 200).unwrap();
/// editor::save(&image, "tests/out/resize_exact_height.jpg");
/// ```
pub fn resize_exact_height(src: &Image, h: i32) -> Result<Image, String> {

    let width = src.width;
    let height = src.height;
    let ratio = width as f32 / height as f32;

    let resize_height = h;
    let resize_width = (h as f32 * ratio) as i32;

    let result = try!(resample(&src, resize_width, resize_height, "bicubic"));
    Ok(result)
}

/// Resize image to exact width. Height is auto calculated. 
/// Useful for creating column of images with the same width.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create an image from file
/// let image = Image::from_file("tests/image/sample.jpg").unwrap();
/// 
/// let image = editor::resize_exact_width(&image, 200).unwrap();
/// editor::save(&image, "tests/out/resize_exact_width.jpg");
/// ```
pub fn resize_exact_width(src: &Image, w: i32) -> Result<Image, String> {
    let width  = src.width;
    let height = src.height;
    let ratio  = width as f32 / height as f32;

    let resize_width  = w;
    let resize_height = (w as f32 / ratio).round() as i32;

    let result = try!(resample(&src, resize_width, resize_height, "bicubic"));
    Ok(result)
}

/// Resize image to fill all the space in the given dimension. Excess parts are removed.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create an image from file
/// let image = Image::from_file("tests/image/sample.jpg").unwrap();
/// 
/// let image = editor::resize_fill(&image, 200, 200).unwrap();
/// editor::save(&image, "tests/out/resize_fill.jpg");
/// ```
pub fn resize_fill(src: &Image, w: i32, h: i32) -> Result<Image, String> {
    let width  = src.width;
    let height = src.height;
    let ratio  = width as f32 / height as f32;

    // Base optimum size on new width
    let mut optimum_width  = w;
    let mut optimum_height = (w as f32 / ratio).round() as i32;

    if (optimum_width < w) || (optimum_height < h) { // Oops, where trying to fill and there are blank areas
        // So base optimum size on height instead
        optimum_width  = (h as f32 * ratio) as i32;
        optimum_height = h;
    }

    let resized = try!(resample(&src, optimum_width, optimum_height, "bicubic"));
    let result = try!(crop(&resized, w, h, "top-left", 0, 0)); // Trim excess parts
    
    Ok(result)
}

/// Resize an image to fit within the given width and height. 
/// The re-sized image will not exceed the given dimension. 
/// Preserves the aspect ratio.
///
/// # Examples
/// ```
/// use raster::image::Image;
/// use raster::editor;
///
/// // Create an image from file
/// let image = Image::from_file("tests/image/sample.jpg").unwrap();
/// 
/// let image = editor::resize_fit(&image, 200, 200).unwrap();
/// editor::save(&image, "tests/out/resize_fit.jpg");
/// ```
pub fn resize_fit(src: &Image, w: i32, h: i32) -> Result<Image, String> {
    
    let ratio: f64 = src.width as f64 / src.height as f64;

    // Try basing it on width first
    let mut resize_width  = w;
    let mut resize_height = (w as f64 / ratio).round() as i32;

    if (resize_width > w) || (resize_height > h) { // Oops, either width or height does not fit
        // So base on height instead
        resize_height = h;
        resize_width  = (h as f64 * ratio).round() as i32;
    }

    let result = try!(resample(&src, resize_width, resize_height, "bicubic"));
    Ok(result)
}

/// Save an image into a file.
pub fn save(image: &Image, out: &str){
    image::save_buffer(&Path::new(out), &image.bytes, image.width as u32, image.height as u32, image::RGBA(8)).unwrap();
}


// Private functions

// Interpolate using nearest neighbor.
fn interpolate_nearest(src: &Image, w: i32, h: i32) -> Result<Image, String> {
    
    let x_ratio: f64 = src.width as f64 / w as f64;
    let y_ratio: f64 = src.height as f64 / h as f64;
    
    let mut dest = Image::blank(w, h);
    for y in 0..h {
        for x in 0..w {

            let px: i32 = ( x as f64 * x_ratio ).floor() as i32;
            let py: i32 = ( y as f64 * y_ratio ).floor() as i32;
            let pixel = try!(src.get_pixel(px, py));
            
            try!(dest.set_pixel(x, y, pixel));
        }
    }
    
    Ok(dest)
}

// Resample an image into a new size.
fn resample(src: &Image, w: i32, h: i32, interpolation: &str) -> Result<Image, String> {
    
    match interpolation {
        "bilinear" => {
            let result = try!(interpolate_nearest(&src, w, h)); // TODO
            Ok(result)
        },
        "bicubic" => {
            let result = try!(interpolate_nearest(&src, w, h)); // TODO
            Ok(result)
        },
        "nearest" => {
            let result = try!(interpolate_nearest(&src, w, h));
            Ok(result)
        },
        _ => {
            Err(format!("Invalid interpolation '{}'", interpolation))
        }
    }
}

fn _bilinear(a: u8, b: u8, c: u8, d: u8, x_diff: f64, y_diff: f64) -> u8 {
    // Y = A(1-w)(1-h) + B(w)(1-h) + C(h)(1-w) + Dwh
    (
        a as f64 * (1.0 - x_diff) * (1.0 - y_diff) + b as f64 * (x_diff) * (1.0 - y_diff) +
        c as f64  * (y_diff) * (1.0 - x_diff) + d as f64  * (x_diff * y_diff)
    ) as u8
}