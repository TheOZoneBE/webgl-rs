//! Bindings for all objects and method associated with WebGL2
//!
//! Documentation taken straight from https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext
//! and https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext
use glenum::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type HTMLDocument;
    static document: HTMLDocument;

    #[wasm_bindgen(method, js_name = getElementById)]
    fn get_element_by_id(this: &HTMLDocument, id: &str) -> HTMLCanvasElement;

    pub type HTMLCanvasElement;
    #[wasm_bindgen(method)]
    fn get_context(this: &HTMLCanvasElement, context: &str) -> WebGL2RenderingContext;
}

impl WebGL2RenderingContext {
    pub fn new(id: &str) -> WebGL2RenderingContext {
        document.get_element_by_id(id).get_context("webgl2")
    }

    /// Maps to `get_buffer_parameter` when return type is `i32`
    pub fn get_buffer_size(&self, target: BufferKind) -> i32 {
        self._get_buffer_size(target, BufferParameter::Size)
    }

    /// Maps to `get_buffer_parameter` when return type is `DataHint`
    pub fn get_buffer_usage(&self, target: BufferKind) -> DataHint {
        self._get_buffer_usage(target, BufferParameter::Usage)
    }

    /// Maps to `get_renderbuffer_parameter` when the parameter is format. This way the constant attribute pname does not need to be given.
    pub fn get_renderbuffer_format(&self, target: RenderbufferKind) -> RenderbufferFormat {
        self._get_renderbuffer_format(target, 0x8D44)
    }

    /// Maps to `WebGL2RenderingContext.getQueryParameter()` when pname equals QUERY_RESULT_AVAILABLE
    pub fn get_query_status(self, query: &WebGLQuery) -> bool {
        self._get_query_status(query, QueryParameter::ResultAvailable)
    }

    /// Maps to `WebGL2RenderingContext.getQueryParameter()` when pname equals QUERY_RESULT
    pub fn get_query_result(&self, query: &WebGLQuery) -> u32 {
        self._get_query_result(query, QueryParameter::Result)
    }
}

/// WebGL2RenderingContext
#[wasm_bindgen]
#[derive(Clone, Copy)]
extern "C" {
    /// The WebGL2RenderingContext interface provides the OpenGL ES 3.0 rendering context
    /// for the drawing surface of an HTML <canvas> element.
    #[derive(Clone)]
    pub type WebGL2RenderingContext;

    /// The `WebGLRenderingContext.canvas` property is a read-only reference to the `HTMLCanvasElement`
    /// or `OffscreenCanvas` object that is associated with the context. It might be null if it is not
    /// associated with a <canvas> element or an `OffscreenCanvas` object.
    #[wasm_bindgen(method, getter)]
    pub fn canvas(this: &WebGL2RenderingContext) -> HTMLCanvasElement;

    /// The read-only `WebGLRenderingContext.drawingBufferWidth` property represents the actual width
    /// of the current drawing buffer. It should match the width attribute of the `<canvas>` element
    /// associated with this context, but might differ if the implementation is not able to provide
    /// the requested width.
    #[wasm_bindgen(method, getter = drawingBufferWidth)]
    pub fn drawing_buffer_width(this: &WebGL2RenderingContext) -> u32;

    /// The read-only `WebGLRenderingContext.drawingBufferHeight` property represents the actual height
    /// of the current drawing buffer. It should match the height attribute of the `<canvas>` element
    /// associated with this context, but might differ if the implementation is not able to provide
    /// the requested height.
    #[wasm_bindgen(method, getter = drawingBufferHeight)]
    pub fn drawing_buffer_height(this: &WebGL2RenderingContext) -> u32;

    // WebGL1 methods

    /// The `WebGLRenderingContext.getContextAttributes()` method returns a `WebGLContextAttributes`
    /// object that contains the actual context parameters. Might return `null`, if the context is lost.
    /* FIXME: the object is not defined when imported
    #[wasm_bindgen(method, js_name = getContextAttributes)]
    pub fn get_context_attributes(this: &WebGL2RenderingContext) -> WebGLContextAttributes;
    */

    /// The WebGLRenderingContext.isContextLost() method returns a Boolean indicating whether or not
    /// the WebGL context has been lost.
    #[wasm_bindgen(method, js_name = isContextLost)]
    pub fn is_context_lost(this: &WebGL2RenderingContext) -> bool;

    /// The `WebGLRenderingContext.scissor()` method of the WebGL API sets a scissor box, which limits
    /// the drawing to a specified rectangle.
    #[wasm_bindgen(method)]
    pub fn scissor(this: &WebGL2RenderingContext, x: i32, y: i32, width: u32, height: u32);

    /// The `WebGLRenderingContext.viewport()` method of the WebGL API sets the viewport, which
    /// specifies the affine transformation of x and y from normalized device coordinates to window
    /// coordinates.
    #[wasm_bindgen(method)]
    pub fn viewport(this: &WebGL2RenderingContext, x: i32, y: i32, width: u32, height: u32);

    /// The `WebGLRenderingContext.activeTexture()` method of the WebGL API specifies which texture
    /// unit to make active.
    #[wasm_bindgen(method, js_name = activeTexture)]
    pub fn active_texture(this: &WebGL2RenderingContext, texture: TextureUnit);

    /// The `WebGLRenderingContext.blendColor()` method of the WebGL API is used to set the source and
    /// destination blending factors.
    #[wasm_bindgen(method, js_name = blendColor)]
    pub fn blend_color(this: &WebGL2RenderingContext, red: f32, green: f32, blue: f32, alpha: f32);

    /// The `WebGLRenderingContext.blendEquation()` method of the WebGL API is used to set both the RGB
    /// blend equation and alpha blend equation to a single equation.
    ///
    /// The blend equation determines how a new pixel is combined with a pixel already in the
    /// WebGLFramebuffer.
    #[wasm_bindgen(method, js_name = blendEquation)]
    pub fn blend_equation(this: &WebGL2RenderingContext, mode: BlendEquation);

    /// The `WebGLRenderingContext.blendEquationSeparate()` method of the WebGL API is used to set
    /// the RGB blend equation and alpha blend equation separately.
    ///
    /// The blend equation determines how a new pixel is combined with a pixel already in the
    /// WebGLFramebuffer.
    #[wasm_bindgen(method, js_name = blendEquationSeparate)]
    pub fn blend_equation_separate(
        this: &WebGL2RenderingContext,
        mode_rgb: BlendEquation,
        mode_alpha: BlendEquation,
    );

    /// The `WebGLRenderingContext.blendFunc()` method of the WebGL API defines which function is used
    /// for blending pixel arithmetic.
    #[wasm_bindgen(method, js_name = blendFunc)]
    pub fn blend_func(this: &WebGL2RenderingContext, sfactor: BlendMode, dfactor: BlendMode);

    /// The `WebGLRenderingContext.blendFuncSeparate()` method of the WebGL API defines which function
    /// is used for blending pixel arithmetic for RGB and alpha components separately.
    #[wasm_bindgen(method, js_name = blendFuncSeparate)]
    pub fn blend_func_separate(
        this: &WebGL2RenderingContext,
        src_rgb: BlendMode,
        dst_rgb: BlendMode,
        src_alpha: BlendMode,
        dst_alpha: BlendMode,
    );

    /// The `WebGLRenderingContext.clearColor()` method of the WebGL API specifies the color values
    /// used when clearing color buffers.
    ///
    /// This specifies what color values to use when calling the clear() method. The values are clamped
    /// between 0 and 1.
    #[wasm_bindgen(method, js_name = clearColor)]
    pub fn clear_color(this: &WebGL2RenderingContext, red: f32, green: f32, blue: f32, alpha: f32);

    /// The `WebGLRenderingContext.clearDepth()` method of the WebGL API specifies the clear value for
    /// the depth buffer.
    ///
    /// This specifies what depth value to use when calling the clear() method. The value is clamped
    /// between 0 and 1.
    #[wasm_bindgen(method, js_name = clearDepth)]
    pub fn clear_depth(this: &WebGL2RenderingContext, depth: f32);

    /// The `WebGLRenderingContext.clearStencil()` method of the WebGL API specifies the clear value
    /// for the stencil buffer.
    ///
    /// This specifies what stencil value to use when calling the clear() method.
    #[wasm_bindgen(method, js_name = clearStencil)]
    pub fn clear_stencil(this: &WebGL2RenderingContext, s: i32);

    /// The `WebGLRenderingContext.colorMask()`  method of the WebGL API sets which color components
    /// to enable or to disable when drawing or rendering to a WebGLFramebuffer.
    #[wasm_bindgen(method, js_name = colorMask)]
    pub fn color_mask(
        this: &WebGL2RenderingContext,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    );

    /// The `WebGLRenderingContext.cullFace()` method of the WebGL API specifies whether or not
    /// front- and/or back-facing polygons can be culled.
    #[wasm_bindgen(method, js_name = cullFace)]
    pub fn cull_face(this: &WebGL2RenderingContext, mode: Culling);

    /// The `WebGLRenderingContext.depthFunc()` method of the WebGL API specifies a function that
    /// compares incoming pixel depth to the current depth buffer value.
    #[wasm_bindgen(method, js_name = depthFunc)]
    pub fn depth_func(this: &WebGL2RenderingContext, func: DepthTest);

    /// The `WebGLRenderingContext.depthMask()` method of the WebGL API sets whether writing
    /// into the depth buffer is enabled or disabled.
    #[wasm_bindgen(method, js_name = depthMask)]
    pub fn depth_mask(this: &WebGL2RenderingContext, flag: bool);

    /// The `WebGLRenderingContext.depthRange()` method of the WebGL API specifies the depth
    /// range mapping from normalized device coordinates to window or viewport coordinates.
    #[wasm_bindgen(method, js_name = depthRange)]
    pub fn depth_range(this: &WebGL2RenderingContext, z_near: f32, z_far: f32);

    /// The `WebGLRenderingContext.disable()` method of the WebGL API disables specific WebGL
    /// capabilities for this context.
    #[wasm_bindgen(method)]
    pub fn disable(this: &WebGL2RenderingContext, cap: Flag);

    /// The `WebGLRenderingContext.enable()` method of the WebGL API enables specific WebGL
    /// capabilities for this context.
    #[wasm_bindgen(method)]
    pub fn enable(this: &WebGL2RenderingContext, cap: Flag);

    /// The `WebGLRenderingContext.frontFace()` method of the WebGL API specifies whether polygons
    /// are front- or back-facing by setting a winding orientation.
    #[wasm_bindgen(method, js_name = frontFace)]
    pub fn front_face(this: &WebGL2RenderingContext, mode: FrontFaceDirection);

    /// The `WebGLRenderingContext.getParameter()` method of the WebGL API returns a value for the
    /// passed parameter name.
    //#[wasm_bindgen(method, js_name = getParameter)]
    // TODO save for later, this is a very convoluted method
    //pub fn get_parameter(this: &WebGL2RenderingContext, pname: )

    /// The `WebGLRenderingContext.getError()` method of the WebGL API returns error information.
    #[wasm_bindgen(method, js_name = getError)]
    pub fn get_error(this: &WebGL2RenderingContext) -> Error;

    /// The `WebGLRenderingContext.hint()` method of the WebGL API specifies hints for certain behaviors.
    /// The interpretation of these hints depend on the implementation.
    #[wasm_bindgen(method)]
    pub fn hint(this: &WebGL2RenderingContext, target: HintTarget, mode: HintMode);

    /// The `WebGLRenderingContext.isEnabled()` method of the WebGL API tests whether a specific WebGL
    /// capability is enabled or not for this context.
    ///
    /// By default, all capabilities except `gl.DITHER` are disabled.
    #[wasm_bindgen(method, js_name = isEnabled)]
    pub fn is_enabled(this: &WebGL2RenderingContext, cap: Flag);

    /// The `WebGLRenderingContext.lineWidth()` method of the WebGL API sets the line width of rasterized lines.
    #[wasm_bindgen(method, js_name = lineWidth)]
    pub fn line_width(this: &WebGL2RenderingContext, width: f32);

    /// The `WebGLRenderingContext.pixelStorei()` method of the WebGL API specifies the pixel storage modes.
    #[wasm_bindgen(method, js_name = pixelStorei)]
    pub fn pixel_storei(this: &WebGL2RenderingContext, pname: PixelStorageMode, param: i32);

    /// The `WebGLRenderingContext.polygonOffset()` method of the WebGL API specifies the scale factors and
    /// units to calculate depth values.
    ///
    /// The offset is added before the depth test is performed and before the value is written into the depth buffer.
    #[wasm_bindgen(method, js_name = polygonOffset)]
    pub fn polygon_offset(this: &WebGL2RenderingContext, factor: f32, units: f32);

    /// The `WebGLRenderingContext.sampleCoverage()` method of the WebGL API specifies multi-sample coverage parameters
    /// for anti-aliasing effects.
    #[wasm_bindgen(method, js_name = sampleCoverage)]
    pub fn sample_coverage(this: &WebGL2RenderingContext, value: f32, invert: bool);

    /// The `WebGLRenderingContext.stencilFunc()` method of the WebGL API sets the front and back function and
    /// reference value for stencil testing.
    ///
    /// Stencilling enables and disables drawing on a per-pixel basis. It is typically used in multipass rendering
    /// to achieve special effects.
    #[wasm_bindgen(method, js_name = stencilFunc)]
    pub fn stencil_func(
        this: &WebGL2RenderingContext,
        func: StencilTest,
        reference: i32,
        mask: u32,
    );

    /// The `WebGLRenderingContext.stencilFuncSeparate()` method of the WebGL API sets the front and/or back
    /// function and reference value for stencil testing.
    ///
    /// Stencilling enables and disables drawing on a per-pixel basis. It is typically used in multipass rendering to achieve special effects.
    #[wasm_bindgen(method, js_name = stencilFuncSeparate)]
    pub fn stencil_func_separate(
        this: &WebGL2RenderingContext,
        face: Culling,
        func: StencilTest,
        reference: i32,
        mask: u32,
    );

    /// The `WebGLRenderingContext.stencilMask()` method of the WebGL API controls enabling and disabling
    /// of both the front and back writing of individual bits in the stencil planes.
    ///
    /// The `WebGLRenderingContext.stencilMaskSeparate()` method can set front and back stencil writemasks
    /// to different values.
    #[wasm_bindgen(method, js_name = stencilMask)]
    pub fn stencil_mask(this: &WebGL2RenderingContext, mask: u32);

    /// The `WebGLRenderingContext.stencilMaskSeparate()` method of the WebGL API controls enabling and
    /// disabling of front and/or back writing of individual bits in the stencil planes.
    ///
    /// The `WebGLRenderingContext.stencilMask()` method can set both, the front and back stencil writemasks
    /// to one value at the same time.
    #[wasm_bindgen(method, js_name = stencilMaskSeparate)]
    pub fn stencil_mask_separate(this: &WebGL2RenderingContext, face: Culling, mask: u32);

    /// The `WebGLRenderingContext.stencilOp()` method of the WebGL API sets both the front and back-facing
    /// stencil test actions.
    #[wasm_bindgen(method, js_name = stencilOp)]
    pub fn stencil_op(
        this: &WebGL2RenderingContext,
        fail: StencilAction,
        zfail: StencilAction,
        zpass: StencilAction,
    );

    /// The `WebGLRenderingContext.stencilOpSeparate()` method of the WebGL API sets the front and/or
    /// back-facing stencil test actions.
    #[wasm_bindgen(method, js_name = stencilOpSeparate)]
    pub fn stencil_op_separate(
        this: &WebGL2RenderingContext,
        face: Culling,
        fail: StencilAction,
        zfail: StencilAction,
        zpass: StencilAction,
    );

    /// TODO maybe add a method for every buffer type

    /// The `WebGLRenderingContext.bufferData()` method of the WebGL API initializes and creates the
    /// buffer object's data store.
    #[wasm_bindgen(method, js_name = bufferData)]
    pub fn buffer_data(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        srcData: Vec<u8>,
        usage: DataHint,
    );

    //TODO buffer_data with offset

    /// The `WebGLRenderingContext.bufferSubData()` method of the WebGL API updates a subset of a
    /// buffer object's data store.
    #[wasm_bindgen(method, js_name = bufferSubData)]
    pub fn buffer_sub_data(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        dst_byte_offset: u32,
        srcData: Vec<u8>,
        srcOffset: u32,
        length: u32,
    );

    /// The `WebGLRenderingContext.getBufferParameter()` method of the WebGL API returns information about the buffer.
    #[wasm_bindgen(method, js_name = getBufferParameter)]
    fn _get_buffer_size(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        pname: BufferParameter,
    ) -> i32;
    #[wasm_bindgen(method, js_name = getBufferParameter)]
    fn _get_buffer_usage(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        pname: BufferParameter,
    ) -> DataHint;

    /// The `WebGLRenderingContext.checkFramebufferStatus()` method of the WebGL API returns the completeness
    /// status of the WebGLFramebuffer object.
    #[wasm_bindgen(method, js_name = checkFramebufferStatus)]
    pub fn check_framebuffer_status(this: &WebGL2RenderingContext, target: FramebufferKind);

    // TODO getFramebufferAttachmentParameter()
    // later because of awful return structure

    /// The `WebGLRenderingContext.readPixels()` method of the WebGL API reads a block of pixels from a
    /// specified rectangle of the current color framebuffer into an ArrayBufferView object.
    // TODO rework because of variability of pixels datatype
    #[wasm_bindgen(method, js_name = readPixels)]
    pub fn read_pixels(
        this: &WebGL2RenderingContext,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
        pixels: Vec<u8>,
        dstOffset: i32,
    );

    /// The `WebGLRenderingContext.getRenderbufferParameter()` method of the WebGL API returns information
    /// about the renderbuffer.
    #[wasm_bindgen(method, js_name = getRenderbufferParameter)]
    pub fn get_renderbuffer_parameter(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        pname: RenderbufferParameter,
    ) -> i32;
    #[wasm_bindgen(method, js_name = getRenderbufferParameter)]
    fn _get_renderbuffer_format(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        pname: i32,
    ) -> RenderbufferFormat;

    /// The `WebGLRenderingContext.renderbufferStorage()` method of the WebGL API creates and initializes
    /// a renderbuffer object's data store.
    #[wasm_bindgen(method, js_name = renderbufferStorage)]
    pub fn renderbuffer_storage(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        internalFormat: RenderbufferFormat,
        width: u32,
        height: u32,
    );

    /// The `WebGLRenderingContext.copyTexImage2D()` method of the WebGL API copies pixels from the current
    /// WebGLFramebuffer into a 2D texture image.
    #[wasm_bindgen(method, js_name = copyTexImage2D)]
    pub fn copy_tex_image_2d(
        this: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border: u32,
    );

    /// The `WebGLRenderingContext.copyTexSubImage2D()` method of the WebGL API copies pixels from the current
    /// WebGLFramebuffer into an existing 2D texture sub-image.
    #[wasm_bindgen(method, js_name = copyTexSubImage2D)]
    pub fn copy_tex_sub_image_2d(
        this: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: i32,
        yoffset: i32,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    );

    /// The `WebGLRenderingContext.generateMipmap()` method of the WebGL API generates a set of mipmaps for a
    /// WebGLTexture object.
    ///
    /// Mipmaps are used to create distance with objects. A higher-resolution mipmap is used for objects that
    /// are closer, and a lower-resolution mipmap is used for objects that are farther away. It starts with the
    /// resolution of the texture image and halves the resolution until a 1x1 dimension texture image is created.
    #[wasm_bindgen(method, js_name = generateMipmap)]
    pub fn generate_mipmap(this: &WebGL2RenderingContext, target: TextureKind);

    //TODO getTexParameter

    /// The `WebGLRenderingContext.texImage2D()` method of the WebGL API specifies a two-dimensional texture image.
    /// FIXME type safety for format, polymorphism of original, srcdata type, webgl2 extensions
    #[wasm_bindgen(method, js_name = texImage2D)]
    pub fn tex_image_2d(
        this: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        border: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
        srcData: Vec<u8>,
        srcOffset: u32,
    );

    // TODO texSubImage2d

    /// The `WebGLRenderingContext.texParameter[fi]()` methods of the WebGL API set texture parameters.
    #[wasm_bindgen(method, js_name = texParameterf)]
    pub fn tex_parameter_f(
        this: &WebGL2RenderingContext,
        target: TextureKind,
        pname: TextureParameter,
        param: f32,
    );

    /// The `WebGLRenderingContext.texParameter[fi]()` methods of the WebGL API set texture parameters.
    #[wasm_bindgen(method, js_name = texParameterf)]
    pub fn tex_parameter_i(
        this: &WebGL2RenderingContext,
        target: TextureKind,
        pname: TextureParameter,
        param: i32,
    );

    /// The `WebGLRenderingContext.getAttachedShaders()` method of the WebGL API returns a list of WebGLShader objects
    /// attached to a WebGLProgram.
    /* FIXME: this is not possible in wasm_bindgen atm
    #[wasm_bindgen(method, js_name = getAttachedShaders)]
    pub fn get_attached_shaders(
        this: &WebGL2RenderingContext,
        program: WebGLProgram,
    ) -> Vec<WebGLShader>;
    */

    //TODO getProgramParameter



    //TODO getShaderParameter

    /// The `WebGLRenderingContext.getShaderPrecisionFormat()` method of the WebGL API returns a new WebGLShaderPrecisionFormat
    /// object describing the range and precision for the specified shader numeric format.
    #[wasm_bindgen(method, js_name = getShaderPrecisionFormat)]
    pub fn get_shader_precision_format(
        this: &WebGL2RenderingContext,
        shader_type: ShaderKind,
        precision_type: ShaderPrecision,
    ) -> WebGLShaderPrecisionFormat;

    /// The `WebGLRenderingContext.disableVertexAttribArray()` method of the WebGL API turns the generic
    /// vertex attribute array off at a given index position.
    #[wasm_bindgen(method, js_name = disableVertexAttribArray)]
    pub fn disable_vertex_attrib_array(this: &WebGL2RenderingContext, index: u32);

    /// The `WebGLRenderingContext method enableVertexAttribArray()`, part of the WebGL API, turns on the generic
    /// vertex attribute array at the specified index into the list of attribute arrays.
    #[wasm_bindgen(method, js_name = enableVertexAttribArray)]
    pub fn enable_vertex_attrib_array(this: &WebGL2RenderingContext, index: u32);

    /// The `WebGLRenderingContext.getUniform()` method of the WebGL API returns the value of a uniform variable
    /// at a given location.
    /* FIXME: this method can have a lot of different return types -> figure out what to do
    #[wasm_bindgen(method, js_name = getUniform)]
    pub fn get_uniform(this: &WebGL2RenderingContext, program: WebGLProgram, location: WebGLUniformLocation)
    */

    /// The `WebGLRenderingContext.getVertexAttrib()` method of the WebGL API returns information about a vertex
    /// attribute at a given position.
    /* FIXME: a lot of different return value possibilities
    #[wasm_bindgen(method, js_name = getVertexAttrib)]
    pub fn get_vertex_attrib(this: &WebGL2RenderingContext, index: u32, pname: );
    */

    /// The `WebGLRenderingContext.getVertexAttribOffset()` method of the WebGL API returns the address of a
    /// specified vertex attribute.
    // FIXME: pname can only be gl.VERTEX_ATTRIB_ARRAY_POINTER
    #[wasm_bindgen(method, js_name = getVertexAttribOffset)]
    pub fn get_vertex_attrib_offset(
        this: &WebGL2RenderingContext,
        index: u32,
        pname: VertexAttrib,
    ) -> i64;

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1f)]
    pub fn uniform_1f(this: &WebGL2RenderingContext, location: &WebGLUniformLocation, v0: f32);

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2f)]
    pub fn uniform_2f(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: f32,
        v1: f32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3f)]
    pub fn uniform_3f(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: f32,
        v1: f32,
        v2: f32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4f)]
    pub fn uniform_4f(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: f32,
        v1: f32,
        v2: f32,
        v3: f32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1i)]
    pub fn uniform_1i(this: &WebGL2RenderingContext, location: &WebGLUniformLocation, v0: i32);

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2i)]
    pub fn uniform_2i(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: i32,
        v1: i32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3i)]
    pub fn uniform_3i(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: i32,
        v1: i32,
        v2: i32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4i)]
    pub fn uniform_4i(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: i32,
        v1: i32,
        v2: i32,
        v3: i32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1fv)]
    pub fn uniform_1fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2fv)]
    pub fn uniform_2fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3fv)]
    pub fn uniform_3fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4fv)]
    pub fn uniform_4fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1iv)]
    pub fn uniform_1iv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<i32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2iv)]
    pub fn uniform_2iv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<i32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3iv)]
    pub fn uniform_3iv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<i32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4iv)]
    pub fn uniform_4iv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<i32>,
    );

    /// The `WebGLRenderingContext.uniformMatrix[234]fv()` methods of the WebGL API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix2fv)]
    pub fn uniform_matrix_2fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniformMatrix[234]fv()` methods of the WebGL API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix3fv)]
    pub fn uniform_matrix_3fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniformMatrix[234]fv()` methods of the WebGL API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix4fv)]
    pub fn uniform_matrix_4fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        value: Vec<f32>,
    );

    /// The WebGLRenderingContext.vertexAttrib[1234]f[v]() methods of the WebGL API specify constant values for generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttrib1f)]
    pub fn vertex_attrib_1f(this: &WebGL2RenderingContext, index: u32, v0: f32);

    /// The WebGLRenderingContext.vertexAttrib[1234]f[v]() methods of the WebGL API specify constant values for generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttrib2f)]
    pub fn vertex_attrib_2f(this: &WebGL2RenderingContext, index: u32, v0: f32, v1: f32);

    /// The WebGLRenderingContext.vertexAttrib[1234]f[v]() methods of the WebGL API specify constant values for generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttrib3f)]
    pub fn vertex_attrib_3f(this: &WebGL2RenderingContext, index: u32, v0: f32, v1: f32, v2: f32);

    /// The WebGLRenderingContext.vertexAttrib[1234]f[v]() methods of the WebGL API specify constant values for generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttrib4f)]
    pub fn vertex_attrib_4f(
        this: &WebGL2RenderingContext,
        index: u32,
        v0: f32,
        v1: f32,
        v2: f32,
        v3: f32,
    );

    /// The WebGLRenderingContext.vertexAttrib[1234]f[v]() methods of the WebGL API specify constant values for generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttrib1fv)]
    pub fn vertex_attrib_1fv(this: &WebGL2RenderingContext, index: u32, value: Vec<f32>);

    /// The WebGLRenderingContext.vertexAttrib[1234]f[v]() methods of the WebGL API specify constant values for generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttrib2fv)]
    pub fn vertex_attrib_2fv(this: &WebGL2RenderingContext, index: u32, value: Vec<f32>);

    /// The WebGLRenderingContext.vertexAttrib[1234]f[v]() methods of the WebGL API specify constant values for generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttrib3fv)]
    pub fn vertex_attrib_3fv(this: &WebGL2RenderingContext, index: u32, value: Vec<f32>);

    /// The WebGLRenderingContext.vertexAttrib[1234]f[v]() methods of the WebGL API specify constant values for generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttrib4fv)]
    pub fn vertex_attrib_4fv(this: &WebGL2RenderingContext, index: u32, value: Vec<f32>);

    /// The `WebGLRenderingContext.vertexAttribPointer()` method of the WebGL API binds the buffer currently
    /// bound to gl.ARRAY_BUFFER to a generic vertex attribute of the current vertex buffer object and specifies
    /// its layout.
    #[wasm_bindgen(method, js_name = vertexAttribPointer)]
    pub fn vertex_attrib_pointer(
        this: &WebGL2RenderingContext,
        index: u32,
        size: AttributeSize,
        attribute_type: AttributeType,
        normalized: bool,
        stride: u8,
        offset: i32,
    );

    /// The `WebGLRenderingContext.clear()` method of the WebGL API clears buffers to preset values.
    ///
    /// The preset values can be set by clearColor(), clearDepth() or clearStencil().
    ///
    /// The scissor box, dithering, and buffer writemasks can affect the clear() method.
    #[wasm_bindgen(method)]
    pub fn clear(this: &WebGL2RenderingContext, mask: BufferBit);

    /// The `WebGLRenderingContext.drawArrays()` method of the WebGL API renders primitives from array data.
    #[wasm_bindgen(method, js_name = drawArrays)]
    pub fn draw_arrays(this: &WebGL2RenderingContext, mode: Primitives, first: u32, count: u32);

    /// The `WebGLRenderingContext.drawElements()` method of the WebGL API renders primitives from array data.
    // FIXME: datatype enum has elements that can provoke errors
    #[wasm_bindgen(method, js_name = drawElements)]
    pub fn draw_elements(
        this: &WebGL2RenderingContext,
        mode: Primitives,
        count: u32,
        data_type: DataType,
        offset: i64,
    );

    /// The `WebGLRenderingContext.finish()` method of the WebGL API blocks execution until all previously
    /// called commands are finished.
    #[wasm_bindgen(method)]
    pub fn finish(this: &WebGL2RenderingContext);

    /// The `WebGLRenderingContext.flush()` method of the WebGL API empties different buffer commands, causing
    /// all commands to be executed as quickly as possible.
    #[wasm_bindgen(method)]
    pub fn flush(this: &WebGL2RenderingContext);

    // WebGL2 methods

    //TODO: getIndexedParameter

    /// The `WebGL2RenderingContext.copyBufferSubData()` method of the WebGL 2 API copies part of the data of a
    /// buffer to another buffer.
    #[wasm_bindgen(method, js_name = copyBufferSubData)]
    pub fn copy_buffer_sub_data(
        this: &WebGL2RenderingContext,
        readTarget: BufferKind,
        writeTarget: BufferKind,
        readOffset: i64,
        writeOffset: i64,
        size: u32,
    );

    /// The `WebGL2RenderingContext.getBufferSubData()` method of the WebGL 2 API reads data from a buffer binding
    /// point and writes them to an ArrayBuffer or SharedArrayBuffer.
    /// FIXME: dstData prob not correct type
    #[wasm_bindgen(method, js_name = getBufferSubData)]
    pub fn get_buffer_sub_data(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        srcByteOffset: i64,
        dstData: Vec<u8>,
        dstOffset: i64,
        length: u32,
    );

    /// The `WebGL2RenderingContext.blitFramebuffer()` method of the WebGL 2 API transfers a block of pixels from the
    /// read framebuffer to the draw framebuffer.
    #[wasm_bindgen(method, js_name = blitFramebuffer)]
    pub fn blit_framebuffer(
        this: &WebGL2RenderingContext,
        srcX0: i32,
        srcY0: i32,
        srcX1: i32,
        srcY1: i32,
        dstX0: i32,
        dstY0: i32,
        dstX1: i32,
        dstY1: i32,
        mask: BufferBit,
        filter: TextureMagFilter,
    );

    /// The `WebGL2RenderingContext.invalidateFramebuffer()` method of the WebGL 2 API invalidates the contents
    /// of attachments in a framebuffer.
    /* FIXME: currently not supported by wasm_bindgen
    #[wasm_bindgen(method, js_name = invalidateFramebuffer)]
    pub fn invalidate_framebuffer(
        this: &WebGL2RenderingContext,
        target: FramebufferKind,
        attachments: &[Attachment],
    );*/

    //FIXME: invalidateSubFramebuffer same issue as invalidateFramebuffer

    /// The `WebGL2RenderingContext.readBuffer()` method of the WebGL 2 API selects a color buffer as the source
    /// for pixels for subsequent calls to copyTexImage2D, copyTexSubImage2D, copyTexSubImage3D or readPixels.
    #[wasm_bindgen(method, js_name = readBuffer)]
    pub fn read_buffer(this: &WebGL2RenderingContext, src: ColorBuffer);

    /// The `WebGL2RenderingContext.getInternalformatParameter()` method of the WebGL 2 API returns information
    /// about implementation-dependent support for internal formats.
    /// FIXME: not sure about internal_format enum
    #[wasm_bindgen(method, js_name = getInternalformatParameter)]
    pub fn get_internal_format_parameter(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        internal_format: RenderbufferFormat,
        pname: InformationType,
    ) -> Vec<i32>;

    /// The `WebGL2RenderingContext.renderbufferStorageMultisample()` method of the WebGL 2 API returns creates
    /// and initializes a renderbuffer object's data store and allows specifying a number of samples to be used.
    #[wasm_bindgen(method, js_name = renderbufferStorageMultisample)]
    pub fn renderbuffer_storage_multisample(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        samples: u32,
        internal_format: RenderbufferFormat,
        width: u32,
        height: u32,
    );

    /// The `WebGL2RenderingContext.texStorage2D()` method of the WebGL API specifies all levels of two-dimensional
    /// texture storage.
    /// FIXME: revisit internal format
    #[wasm_bindgen(method, js_name = texStorage2D)]
    pub fn tex_storage_2d(
        this: &WebGL2RenderingContext,
        target: Texture2DKind,
        levels: u32,
        internal_format: RenderbufferFormat,
        width: u32,
        height: u32,
    );

    /// The `WebGL2RenderingContext.texStorage3D()` method of the WebGL API specifies all levels of a three-dimensional
    /// texture or two-dimensional array texture.
    /// FIXME: revisit internal format
    #[wasm_bindgen(method, js_name = texStorage3D)]
    pub fn tex_storage_3d(
        this: &WebGL2RenderingContext,
        target: Texture3DKind,
        levels: u32,
        internalformat: RenderbufferFormat,
        width: u32,
        height: u32,
        depth: u32,
    );

    /// The `WebGLRenderingContext.texImage3D()` method of the WebGL API specifies a three-dimensional texture image.
    /// FIXME: revisit internalformat, format, data_type
    /// FIXME: border is always 0
    /// FIXME: different src types
    #[wasm_bindgen(method, js_name = texImage3D)]
    pub fn tex_image_3d(
        this: &WebGL2RenderingContext,
        target: Texture3DKind,
        level: u32,
        internalformat: RenderbufferFormat,
        width: u32,
        height: u32,
        depth: u32,
        border: u32,
        format: RenderbufferFormat,
        data_type: RenderbufferFormat,
        srcData: Vec<u8>,
        srcOffset: u32,
    );

    /// The `WebGL2RenderingContext.texSubImage3D()` method of the WebGL API specifies a sub-rectangle of the current texture.
    /// FIXME: revisit format, data_type
    /// FIXME: srcdata more types
    #[wasm_bindgen(method, js_name = texSubImage3D)]
    pub fn tex_sub_image_3d(
        this: &WebGL2RenderingContext,
        target: Texture3DKind,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        zoffset: u32,
        width: u32,
        height: u32,
        depth: u32,
        format: RenderbufferFormat,
        data_type: RenderbufferFormat,
        srcData: Vec<u8>,
        srcOffset: u32,
    );

    /// The `WebGL2RenderingContext.copyTexSubImage3D()` method of the WebGL API copies pixels from the current WebGLFramebuffer
    /// into an existing 3D texture sub-image.
    #[wasm_bindgen(method, js_name = copyTexSubImage3D)]
    pub fn copy_tex_sub_image_3d(
        this: &WebGL2RenderingContext,
        target: Texture3DKind,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        zoffset: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1ui)]
    pub fn uniform_1ui(this: &WebGL2RenderingContext, location: &WebGLUniformLocation, v0: u32);

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2ui)]
    pub fn uniform_2ui(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: u32,
        v1: u32,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3ui)]
    pub fn uniform_3ui(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: u32,
        v1: u32,
        v2: u32,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4ui)]
    pub fn uniform_4ui(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1uiv)]
    pub fn uniform_1uiv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<u32>,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2uiv)]
    pub fn uniform_2uiv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<u32>,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3uiv)]
    pub fn uniform_3uiv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<u32>,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4uiv)]
    pub fn uniform_4uiv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<u32>,
    );

    //TODO all uniform vector methods with optional srcoffset and srclength

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix2x3fv)]
    pub fn uniform_matrix_2x3fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix2x4fv)]
    pub fn uniform_matrix_2x4fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix3x2fv)]
    pub fn uniform_matrix_3x2fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix3x4fv)]
    pub fn uniform_matrix_3x4fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix4x2fv)]
    pub fn uniform_matrix_4x2fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix4x3fv)]
    pub fn uniform_matrix_4x3fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.vertexAttribI4[u]i[v]()` methods of the WebGL 2 API specify integer values for
    /// generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttribI4i)]
    pub fn vertex_attrib_i_4i(
        this: &WebGL2RenderingContext,
        index: u32,
        v0: i32,
        v1: i32,
        v2: i32,
        v3: i32,
    );

    /// The `WebGL2RenderingContext.vertexAttribI4[u]i[v]()` methods of the WebGL 2 API specify integer values for
    /// generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttribI4ui)]
    pub fn vertex_attrib_i_4ui(
        this: &WebGL2RenderingContext,
        index: u32,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
    );

    /// The `WebGL2RenderingContext.vertexAttribI4[u]i[v]()` methods of the WebGL 2 API specify integer values for
    /// generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttribI4iv)]
    pub fn vertex_attrib_i_4iv(this: &WebGL2RenderingContext, index: u32, value: Vec<i32>);

    /// The `WebGL2RenderingContext.vertexAttribI4[u]i[v]()` methods of the WebGL 2 API specify integer values for
    /// generic vertex attributes.
    #[wasm_bindgen(method, js_name = vertexAttribI4uiv)]
    pub fn vertex_attrib_i_4uiv(this: &WebGL2RenderingContext, index: u32, value: Vec<u32>);

    /// The `WebGL2RenderingContext.vertexAttribIPointer()` method of the WebGL 2 API specifies integer data formats
    /// and locations of vertex attributes in a vertex attributes array.
    /// FIXME: revisit data_type
    #[wasm_bindgen(method, js_name = vertexAttribIPointer)]
    pub fn vertex_attrib_i_pointer(
        this: &WebGL2RenderingContext,
        index: u32,
        size: AttributeSize,
        data_type: AttributeType,
        stride: u32,
        offset: i64,
    );

    /// The `WebGL2RenderingContext.vertexAttribDivisor()` method of the WebGL 2 API modifies the rate at which generic
    /// vertex attributes advance when rendering multiple instances of primitives with gl.drawArraysInstanced() and gl.drawElementsInstanced().
    #[wasm_bindgen(method, js_name = vertexAttribDivisor)]
    pub fn vertex_attrib_divisor(this: &WebGL2RenderingContext, index: u32, divisor: u32);

    /// The `WebGL2RenderingContext.drawArraysInstanced()` method of the WebGL 2 API renders primitives from array data like the gl.drawArrays()
    /// method. In addition, it can execute multiple instances of the range of elements.
    #[wasm_bindgen(method, js_name = drawArraysInstanced)]
    pub fn draw_arrays_instanced(
        this: &WebGL2RenderingContext,
        mode: Primitives,
        first: i32,
        count: u32,
        instanceCount: u32,
    );

    /// The `WebGL2RenderingContext.drawElementsInstanced()` method of the WebGL 2 API renders primitives from array data like the gl.drawElements()
    /// method. In addition, it can execute multiple instances of a set of elements.
    /// FIXME: revisit data_type
    #[wasm_bindgen(method, js_name = drawElementsInstanced)]
    pub fn draw_elements_instanced(
        this: &WebGL2RenderingContext,
        mode: Primitives,
        count: u32,
        data_type: AttributeType,
        offset: i64,
        instanceCount: u32,
    );

    /// The `WebGL2RenderingContext.drawRangeElements()` method of the WebGL API renders primitives from array data in a given range.
    /// FIXME: revisit data_type
    #[wasm_bindgen(method, js_name = drawRangeElements)]
    pub fn draw_range_elements(
        this: &WebGL2RenderingContext,
        mode: Primitives,
        start: u32,
        end: u32,
        count: u32,
        data_type: AttributeType,
        offset: i64,
    );

    /// The `WebGL2RenderingContext.drawBuffers()` method of the WebGL 2 API defines draw buffers to which fragment
    /// colors are written into. The draw buffer settings are part of the state of the currently bound framebuffer
    /// or the drawingbuffer if no framebuffer is bound.
    /* FIXME: currently wasm_bindgen doesn not support Vec<T>
    #[wasm_bindgen(method, js_name = drawBuffers)]
    pub fn draw_buffers(this: &WebGL2RenderingContext, buffers: Vec<ColorBuffer>);
    */

    /// The `WebGL2RenderingContext.clearBuffer[fiuv]()` methods of the WebGL 2 API clear buffers from the currently
    /// bound framebuffer.
    /// FIXME values is array of rgba
    #[wasm_bindgen(method, js_name = clearBufferfv)]
    pub fn clear_buffer_fv(
        this: &WebGL2RenderingContext,
        buffer: BufferBit,
        drawbuffer: i32,
        values: Vec<u8>,
        srcOffset: u32,
    );

    /// The `WebGL2RenderingContext.clearBuffer[fiuv]()` methods of the WebGL 2 API clear buffers from the currently
    /// bound framebuffer.
    /// FIXME values is array of rgba
    #[wasm_bindgen(method, js_name = clearBufferiv)]
    pub fn clear_buffer_iv(
        this: &WebGL2RenderingContext,
        buffer: BufferBit,
        drawbuffer: i32,
        values: Vec<u8>,
        srcOffset: u32,
    );

    /// The `WebGL2RenderingContext.clearBuffer[fiuv]()` methods of the WebGL 2 API clear buffers from the currently
    /// bound framebuffer.
    /// FIXME values is array of rgba
    #[wasm_bindgen(method, js_name = clearBufferuiv)]
    pub fn clear_buffer_uiv(
        this: &WebGL2RenderingContext,
        buffer: BufferBit,
        drawbuffer: i32,
        values: Vec<u8>,
        srcOffset: u32,
    );

    /// The `WebGL2RenderingContext.clearBuffer[fiuv]()` methods of the WebGL 2 API clear buffers from the currently
    /// bound framebuffer.
    /// FIXME buffer can only be DEPTH_STENCIL
    #[wasm_bindgen(method, js_name = clearBufferfv)]
    pub fn clear_buffer_fi(
        this: &WebGL2RenderingContext,
        buffer: BufferBit,
        drawbuffer: i32,
        depth: f32,
        stencil: i32,
    );

    /// The `WebGL2RenderingContext.createQuery()` method of the WebGL 2 API creates and initializes WebGLQuery objects,
    /// which provide ways to asynchronously query for information.
    #[wasm_bindgen(method, js_name = createQuery)]
    pub fn create_query(this: &WebGL2RenderingContext) -> WebGLQuery;

    /// The `WebGL2RenderingContext.deleteQuery()` method of the WebGL 2 API deletes a given WebGLQuery object.
    #[wasm_bindgen(method, js_name = deleteQuery)]
    pub fn delete_query(this: &WebGL2RenderingContext, query: &WebGLQuery);

    /// The `WebGL2RenderingContext.isQuery()` method of the WebGL 2 API returns true if the passed object is a valid
    /// WebGLQuery object.
    #[wasm_bindgen(method, js_name = isQuery)]
    pub fn is_query(this: &WebGL2RenderingContext, query: &WebGLQuery);

    /// The `WebGL2RenderingContext.beginQuery()` method of the WebGL 2 API starts an asynchronous query. The target
    /// parameter indicates which kind of query to begin.
    #[wasm_bindgen(method, js_name = beginQuery)]
    pub fn begin_query(this: &WebGL2RenderingContext, target: QueryTarget, query: &WebGLQuery);

    /// The `WebGL2RenderingContext.endQuery()` method of the WebGL 2 API marks the end of a given query target.
    #[wasm_bindgen(method, js_name = endQuery)]
    pub fn end_query(this: &WebGL2RenderingContext, target: QueryTarget);

    /// The `WebGL2RenderingContext.getQuery()` method of the WebGL 2 API returns the currently active WebGLQuery for the
    /// target, or null.
    #[wasm_bindgen(method, js_name = getQuery)]
    pub fn get_query(
        this: &WebGL2RenderingContext,
        target: QueryTarget,
        pname: Query,
    ) -> WebGLQuery;

    /// The `WebGL2RenderingContext.getQueryParameter()` method of the WebGL 2 API returns parameter information
    /// of a WebGLQuery object
    #[wasm_bindgen(method, js_name = getQueryParameter)]
    fn _get_query_status(
        this: &WebGL2RenderingContext,
        query: &WebGLQuery,
        pname: QueryParameter,
    ) -> bool;
    #[wasm_bindgen(method, js_name = getQueryParameter)]
    fn _get_query_result(
        this: &WebGL2RenderingContext,
        query: &WebGLQuery,
        pname: QueryParameter,
    ) -> u32;

    /// The `WebGL2RenderingContext.createSampler()` method of the WebGL 2 API creates and initializes
    /// WebGLSampler objects.
    #[wasm_bindgen(method, js_name = createSampler)]
    pub fn create_sampler(this: &WebGL2RenderingContext) -> WebGLSampler;

    /// The `WebGL2RenderingContext.deleteSampler()` method of the WebGL 2 API deletes a given WebGLSampler
    /// object.
    #[wasm_bindgen(method, js_name = deleteSampler)]
    pub fn delete_sampler(this: &WebGL2RenderingContext, sampler: &WebGLSampler);

    /// The `WebGL2RenderingContext.bindSampler()` method of the WebGL 2 API binds a passed WebGLSampler object to
    /// the texture unit at the passed index.
    #[wasm_bindgen(method, js_name = bindSampler)]
    pub fn bind_sampler(this: &WebGL2RenderingContext, unit: u32, sampler: &WebGLSampler);

    /// The WebGL2RenderingContext.isSampler() method of the WebGL 2 API returns true if the passed object is a valid
    /// WebGLSampler object.
    #[wasm_bindgen(method, js_name = isSampler)]
    pub fn is_sampler(this: &WebGL2RenderingContext, sampler: &WebGLSampler);

    //TODO samplerParameter[if] and getSamplerParameter

    /// The `WebGL2RenderingContext.fenceSync()` method of the WebGL 2 API creates a new WebGLSync object and inserts
    /// it into the GL command stream.
    #[wasm_bindgen(method, js_name = fenceSync)]
    pub fn fence_sync(this: &WebGL2RenderingContext, conditions: GPUState, flags: u32)
        -> WebGLSync;

    /// The `WebGL2RenderingContext.isSync()` method of the WebGL 2 API returns true if the passed object is a valid
    /// WebGLSync object.
    #[wasm_bindgen(method, js_name = isSync)]
    pub fn is_sync(this: &WebGL2RenderingContext, sync: &WebGLSync) -> bool;

    /// The WebGL2RenderingContext.deleteSync() method of the WebGL 2 API deletes a given WebGLSync object.
    #[wasm_bindgen(method, js_name = deleteSync)]
    pub fn delete_sync(this: &WebGL2RenderingContext, sync: &WebGLSync);

    /// The `WebGL2RenderingContext.clientWaitSync()` method of the WebGL 2 API blocks and waits for a WebGLSync object to
    /// become signaled or a given timeout to be passed.
    #[wasm_bindgen(method, js_name = clientWaitSync)]
    pub fn client_wait_sync(
        this: &WebGL2RenderingContext,
        sync: &WebGLSync,
        flags: u32,
        timeout: i64,
    ) -> SyncStatus;

    /// The WebGL2RenderingContext.waitSync() method of the WebGL 2 API returns immediately, but waits on the GL server
    /// until the given WebGLSync object is signaled.
    ///
    /// The method is a no-op in the absence of the possibility of synchronizing between multiple GL contexts.
    /// FIXME: timeout must be gl.TIMEOUT_IGNORED.
    #[wasm_bindgen(method, js_name = waitSync)]
    pub fn wait_sync(this: &WebGL2RenderingContext, sync: &WebGLSync, flags: u32, timeout: i64);

    //TODO: getSyncParameter

    /// The `WebGL2RenderingContext.createTransformFeedback()` method of the WebGL 2 API creates and initializes
    /// WebGLTransformFeedback objects.
    #[wasm_bindgen(method, js_name = createTransformFeedback)]
    pub fn create_transform_feedback(this: &WebGL2RenderingContext) -> WebGLTransformFeedback;

    /// The `WebGL2RenderingContext.deleteTransformFeedback()` method of the WebGL 2 API deletes a given WebGLTransformFeedback
    /// object.
    #[wasm_bindgen(method, js_name = deleteTransformFeedback)]
    pub fn delete_transform_feedback(
        this: &WebGL2RenderingContext,
        transform_feedback: &WebGLTransformFeedback,
    );

    /// The `WebGL2RenderingContext.isTransformFeedback()` method of the WebGL 2 API returns true if the passed object is a
    /// valid WebGLTransformFeedback object.
    #[wasm_bindgen(method, js_name = isTransformFeedback)]
    pub fn is_transform_feedback(
        this: &WebGL2RenderingContext,
        transform_feedback: &WebGLTransformFeedback,
    ) -> bool;

    /// The `WebGL2RenderingContext.bindTransformFeedback()` method of the WebGL 2 API binds a passed WebGLTransformFeedback
    /// object to the current GL state.
    #[wasm_bindgen(method, js_name = bindTransformFeedback)]
    pub fn bind_transform_feedback(
        this: &WebGL2RenderingContext,
        target: TransformFeedback,
        transform_feedback: &WebGLTransformFeedback,
    );

    /// The WebGL2RenderingContext.beginTransformFeedback() method of the WebGL 2 API starts a transform feedback operation.
    #[wasm_bindgen(method, js_name = beginTransformFeedback)]
    pub fn begin_transform_feedback(
        this: &WebGL2RenderingContext,
        primitive_mode: TransformFeedbackMode,
    );

    /// The WebGL2RenderingContext.endTransformFeedback() method of the WebGL 2 API ends a transform feedback operation.
    #[wasm_bindgen(method, js_name = endTransformFeedback)]
    pub fn end_transform_feedback(this: &WebGL2RenderingContext);

    /// The `WebGL2RenderingContext.transformFeedbackVaryings()` method of the WebGL 2 API specifies values to record in
    /// WebGLTransformFeedback buffers.
    /* FIXME: Vec of string currently not supported by wasm_bindgen
    #[wasm_bindgen(method, js_name = transformFeedbackVaryings)]
    pub fn transform_feedback_varyings(
        this: &WebGL2RenderingContext,
        program: WebGLProgram,
        varyings: Vec<String>,
        buffer_mode: TransformFeedbackBufferMode,
    );*/

    /// The WebGL2RenderingContext.pauseTransformFeedback() method of the WebGL 2 API pauses a transform feedback operation.
    #[wasm_bindgen(method, js_name = pauseTransformFeedback)]
    pub fn pause_transform_feedback(this: &WebGL2RenderingContext);

    /// The WebGL2RenderingContext.resumeTransformFeedback() method of the WebGL 2 API resumes a transform feedback operation.
    #[wasm_bindgen(method, js_name = resumeTransformFeedback)]
    pub fn resume_transform_feedback(this: &WebGL2RenderingContext);

// The `WebGL2RenderingContext.getUniformIndices()` method of the WebGL 2 API retrieves the indices of a number of uniforms
    // within a WebGLProgram.
    /* FIXME: vec<string> not yet supported
    #[wasm_bindgen(method, js_name = getUniformIndices)]
    pub fn get_uniform_indices(this: &WebGL2RenderingContext, program: WebGLProgram, uniformNames: Vec<String>);
    */

// TODO getActiveUniforms

// TODO getAcitveUniformBlockParameter

}

// WebGLContextAttributes
/* FIXME: not found when exported
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLContextAttributes;
    #[wasm_bindgen(method, getter)]
    pub fn alpha(this: &WebGLContextAttributes) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn antialias(this: &WebGLContextAttributes) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn depth(this: &WebGLContextAttributes) -> bool;
    #[wasm_bindgen(method, getter = premultipliedAlpha)]
    pub fn premultiplied_alpha(this: &WebGLContextAttributes) -> bool;
    #[wasm_bindgen(method, getter = preserveDrawingBuffer)]
    pub fn preserve_drawing_buffer(this: &WebGLContextAttributes) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn stencil(this: &WebGLContextAttributes) -> bool;
    #[wasm_bindgen(method, getter = failIfMajorPerformanceCaveat )]
    pub fn fail_if_major_performance_caveat(this: &WebGLContextAttributes) -> bool;
    #[wasm_bindgen(method, getter = powerPreference)]
    pub fn power_preference(this: &WebGLContextAttributes) -> String;
}
*/

/// WebGLShaderPrecisionFormat;
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLShaderPrecisionFormat;

    /// The base 2 log of the absolute value of the minimum value that can be represented.
    #[wasm_bindgen(method, getter = rangeMin)]
    pub fn range_min(this: &WebGLShaderPrecisionFormat) -> u32;

    /// The base 2 log of the absolute value of the maximum value that can be represented.
    #[wasm_bindgen(method, getter = rangeMax)]
    pub fn range_max(this: &WebGLShaderPrecisionFormat) -> u32;

    /// The number of bits of precision that can be represented. For integer formats this value is always 0.
    #[wasm_bindgen(method, getter)]
    pub fn precision(this: &WebGLShaderPrecisionFormat) -> u32;
}

/// WebGLUniformLocation
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLUniformLocation;
}

/// WebGLQuery
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLQuery;
}

/// WebGLSampler
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLSampler;
}

/// WebGLSync
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLSync;
}

/// WebGLTransformFeedback
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLTransformFeedback;
}
