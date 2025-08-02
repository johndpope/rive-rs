use rive_rs::File;

// Mock types
#[derive(Debug)]
struct MockBuffer;
#[derive(Debug)]
struct MockGradient;
#[derive(Debug)]
struct MockImage;
#[derive(Debug)]
struct MockPaint;
#[derive(Debug)]
struct MockPath;
#[derive(Debug)]
struct MockRenderer;

impl rive_rs::renderer::Buffer for MockBuffer {
    fn new(_: rive_rs::renderer::BufferType, _: rive_rs::renderer::BufferFlags, _: usize) -> Self { 
        MockBuffer 
    }
    fn map(&mut self) -> &mut [u8] { 
        &mut [] 
    }
    fn unmap(&mut self) {}
}

impl Default for MockBuffer {
    fn default() -> Self { MockBuffer }
}

impl Default for MockPaint {
    fn default() -> Self { MockPaint }
}

impl Default for MockPath {
    fn default() -> Self { MockPath }
}

impl rive_rs::renderer::Gradient for MockGradient {
    fn new_linear(_: f32, _: f32, _: f32, _: f32, _: &[rive_rs::renderer::Color], _: &[f32]) -> Self { 
        MockGradient 
    }
    fn new_radial(_: f32, _: f32, _: f32, _: &[rive_rs::renderer::Color], _: &[f32]) -> Self { 
        MockGradient 
    }
}

impl rive_rs::renderer::Image for MockImage {
    fn decode(_: &[u8]) -> Option<Self> { 
        Some(MockImage) 
    }
}

impl rive_rs::renderer::Paint for MockPaint {
    type Gradient = MockGradient;

    fn set_style(&mut self, _: rive_rs::renderer::PaintStyle) {}
    fn set_color(&mut self, _: rive_rs::renderer::Color) {}
    fn set_thickness(&mut self, _: f32) {}
    fn set_join(&mut self, _: rive_rs::renderer::StrokeJoin) {}
    fn set_cap(&mut self, _: rive_rs::renderer::StrokeCap) {}
    fn set_blend_mode(&mut self, _: rive_rs::renderer::BlendMode) {}
    fn set_gradient(&mut self, _: &Self::Gradient) {}
    fn invalidate_stroke(&mut self) {}
}

impl rive_rs::renderer::Path for MockPath {
    fn new(_: &mut rive_rs::path::Commands, _: rive_rs::path::FillRule) -> Self { 
        MockPath 
    }
    fn reset(&mut self) {}
    fn extend(&mut self, _: &Self, _: &[f32; 6]) {}
    fn set_fill_rule(&mut self, _: rive_rs::path::FillRule) {}
    fn move_to(&mut self, _: f32, _: f32) {}
    fn line_to(&mut self, _: f32, _: f32) {}
    fn cubic_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {}
    fn close(&mut self) {}
}

impl rive_rs::renderer::Renderer for MockRenderer {
    type Buffer = MockBuffer;
    type Gradient = MockGradient;
    type Image = MockImage;
    type Paint = MockPaint;
    type Path = MockPath;

    fn state_push(&mut self) {}
    fn state_pop(&mut self) {}
    fn transform(&mut self, _: &[f32; 6]) {}
    fn set_clip(&mut self, _: &Self::Path) {}
    fn draw_path(&mut self, _: &Self::Path, _: &Self::Paint) {}
    fn draw_image(&mut self, _: &Self::Image, _: rive_rs::renderer::BlendMode, _: f32) {}
    fn draw_image_mesh(&mut self, _: &Self::Image, _: &Self::Buffer, _: &Self::Buffer, _: &Self::Buffer, _: rive_rs::renderer::BlendMode, _: f32) {}
}

fn main() {
    println!("Testing file creation and export...");
    
    // Create an empty file
    let file: File<MockRenderer> = File::create();
    println!("✓ File created successfully");
    
    // Test JSON export
    match file.export_json() {
        Ok(json) => {
            println!("✓ JSON export successful");
            println!("  Export preview: {}", 
                if json.len() > 100 { 
                    format!("{}...", &json[..100]) 
                } else { 
                    json 
                }
            );
        }
        Err(e) => {
            println!("✗ JSON export failed: {}", e);
        }
    }
    
    // Test binary export (expected to return None)
    match file.export() {
        Some(data) => println!("✓ Binary export successful: {} bytes", data.len()),
        None => println!("✓ Binary export not yet supported (expected)"),
    }
    
    println!("File creation and export test completed!");
}
