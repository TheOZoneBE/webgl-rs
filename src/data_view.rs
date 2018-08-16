use glenum::{BufferKind, DataHint, PixelCopyFormat, PixelType, TextureBindPoint};
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::JsValue;

pub trait Buffer {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint);
}

pub trait Image {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue>;
}

impl Buffer for Vec<u8> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_u8(target, self, usage);
    }
}

impl Image for Vec<u8> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_u8(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
}

impl Buffer for Vec<i8> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_i8(target, self, usage);
    }
}

impl Image for Vec<i8> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_i8(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
}

impl Buffer for Vec<u16> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_u16(target, self, usage);
    }
}

impl Image for Vec<u16> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_u16(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
}

impl Buffer for Vec<i16> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_i16(target, self, usage);
    }
}

impl Image for Vec<i16> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_i16(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
}

impl Buffer for Vec<u32> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_u32(target, self, usage);
    }
}

impl Image for Vec<u32> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_u32(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
}

impl Buffer for Vec<i32> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_i32(target, self, usage);
    }
}

impl Image for Vec<i32> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_i32(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
}

impl Buffer for Vec<f32> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_f32(target, self, usage);
    }
}

impl Image for Vec<f32> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_f32(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
}

impl Buffer for Vec<f64> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_f64(target, self, usage);
    }
}