/*
    Credits to https://github.com/oussama/glenum-rs/ for the major part of these enums

    Extended with wasm_bindgen and webgl2 constants and restructured to allow safety in webgl methods    
    
    Documentation taken from https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants
*/
use wasm_bindgen::prelude::*;

/// Constants passed to WebGLRenderingContext.vertexAttribPointer()
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum AttributeSize {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

/// Constants passed to WebGLRenderingContext.createShader()
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum ShaderKind {
    /// Passed to createShader to define a fragment shader.
    Fragment = 0x8B30,
    /// Passed to createShader to define a vertex shader
    Vertex = 0x8B31,
}

///FIXME categorize values elsewhere
pub enum NotProgramParameter {
    /// The maximum number of entries possible in the vertex attribute list.
    MaxVertexAttribs = 0x8869,
    ///
    MaxVertexUniformVectors = 0x8DFB,
    ///
    MaxVaryingVectors = 0x8DFC,
    ///
    MaxCombinedTextureImageUnits = 0x8B4D,
    ///
    MaxVertexTextureImageUnits = 0x8B4C,
    /// Implementation dependent number of maximum texture units. At least 8.
    MaxTextureImageUnits = 0x8872,
    ///
    MaxFragmentUniformVectors = 0x8DFD,
    ///
    ShadingLanguageVersion = 0x8B8C,
    ///
    CurrentProgram = 0x8B8D,
}

/// Constants passed to WebGLRenderingContext.getProgramParameter()
/// TODO decide if im keeping it public or move to shader_program as it is only used internally i think
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum ProgramParameter {
    /// Passed to getProgramParameter to determine if a shader was deleted via deleteProgram. Returns true if it was, false otherwise.
    DeleteStatus = 0x8B80,
    /// Passed to getProgramParameter after calling linkProgram to determine if a program was linked correctly. Returns false if there were errors. Use getProgramInfoLog to find the exact error.
    LinkStatus = 0x8B82,
    /// Passed to getProgramParameter after calling validateProgram to determine if it is valid. Returns false if errors were found.
    ValidateStatus = 0x8B83,
    /// Passed to getProgramParameter after calling attachShader to determine if the shader was attached correctly. Returns false if errors occurred.
    AttachedShaders = 0x8B85,
    /// Passed to getProgramParameter to get the number of attributes active in a program.
    ActiveAttributes = 0x8B89,
    /// Passed to getProgramParameter to get the number of uniforms active in a program.
    ActiveUniforms = 0x8B86,
    /// Passed to getProgramParameter to get the buffer mode when transform feedback is active.
    TransformFeedbackBufferMode = 0x8C7F,
    /// Passed to getProgramParameter to get the number of varying variables to capture in transform feedback mode
    TransformFeedbackVaryings = 0x8C83,
    /// Passed to getProgramParameter to get the number of uniform blocks containing active uniforms
    ActiveUniformBlocks = 0x8A36,
}

/// Constants passed to WebGLRenderingContext.getShaderParameter()
/// TODO decide if im keeping it public or move to shader_program as it is only used internally i think
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum ShaderParameter {
    /// Passed to getShaderParameter to get the status of the compilation. Returns false if the shader was not compiled. You can then query getShaderInfoLog to find the exact error
    CompileStatus = 0x8B81,
    /// Passed to getShaderParameter to determine if a shader was deleted via deleteShader. Returns true if it was, false otherwise.
    DeleteStatus = 0x8B80,
    /// Passed to getShaderParameter to get the shader type.
    ShaderType = 0x8B4F,
}

/// Passed to bindBuffer or bufferData to specify the type of buffer being used.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum BufferKind {
    Array = 0x8892,
    ElementArray = 0x8893,
    /// Buffer for copying from one buffer object to another.
    CopyReadBuffer = 0x8F36,
    /// Buffer for copying from one buffer object to another.
    CopyWriteBuffer = 0x8F37,
    /// Buffer for transform feedback operations.
    TransformFeedbackBuffer = 0x8C8E,
    /// Buffer used for storing uniform blocks.
    UniformBuffer = 0x8A11,
    /// Buffer used for pixel transfer operations.
    PixelPackBuffer = 0x88EB,
    /// Buffer used for pixel transfer operations.
    PixelUnpackBuffer = 0x88EC,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum DataHint {
    /// Passed to bufferData as a hint about whether the contents of the buffer are likely to be used often and not change often.
    StaticDraw = 0x88E4,
    /// Passed to bufferData as a hint about whether the contents of the buffer are likely to be used often and change often.
    DynamicDraw = 0x88E8,
    /// Passed to bufferData as a hint about whether the contents of the buffer are likely to not be used often.
    StreamDraw = 0x88E0,
    /// Contents of the buffer are likely to be used often and not change often. Contents are read from the buffer, but not written.
    StaticRead = 0x88E5,
    /// Contents of the buffer are likely to be used often and change often. Contents are read from the buffer, but not written.
    DynamicRead = 0x88E9,
    /// Contents of the buffer are likely to not be used often. Contents are read from the buffer, but not written.
    StreamRead = 0x88E1,
    /// Contents of the buffer are likely to be used often and not change often. Contents are neither written or read by the user.
    StaticCopy = 0x88E6,
    /// Contents of the buffer are likely to be used often and change often. Contents are neither written or read by the user.
    DynamicCopy = 0x88EA,
    /// Contents of the buffer are likely to be used often and not change often. Contents are neither written or read by the user.
    StreamCopy = 0x88E2,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum BufferParameter {
    /// Passed to getBufferParameter to get a buffer's size.
    Size = 0x8764,
    /// Passed to getBufferParameter to get the hint for the buffer passed in when it was created.
    Usage = 0x8765,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum DataType {
    I8 = 0x1400,
    U8 = 0x1401,
    I16 = 0x1402,
    U16 = 0x1403,
    I32 = 0x1404,
    U32 = 0x1405,
    Float = 0x1406,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    /// Passed to enable/disable to turn on/off blending. Can also be used with getParameter to find the current blending method.
    Blend = 0x0BE2,
    /// Passed to enable/disable to turn on/off culling. Can also be used with getParameter to find the current culling method.
    CullFace = 0x0B44,
    /// Passed to enable/disable to turn on/off the depth test. Can also be used with getParameter to query the depth test.
    DepthTest = 0x0B71,
    /// Passed to enable/disable to turn on/off dithering. Can also be used with getParameter to find the current dithering method.
    Dither = 0x0BD0,
    /// Passed to enable/disable to turn on/off the polygon offset. Useful for rendering hidden-line images, decals, and or solids with highlighted edges. Can also be used with getParameter to query the scissor test.
    PolygonOffsetFill = 0x8037,
    /// Passed to enable/disable to turn on/off the alpha to coverage. Used in multi-sampling alpha channels.
    SampleAlphaToCoverage = 0x809E,
    /// Passed to enable/disable to turn on/off the sample coverage. Used in multi-sampling.
    SampleCoverage = 0x80A0,
    /// Passed to enable/disable to turn on/off the scissor test. Can also be used with getParameter to query the scissor test.
    ScissorTest = 0x0C11,
    /// Passed to enable/disable to turn on/off the stencil test. Can also be used with getParameter to query the stencil test.
    StencilTest = 0x0B90,
    /// Passed to enable/disable to turn on/off that primitives are discarded immediately before the rasterization stage,
    /// but after the optional transform feedback stage. gl.clear() commands are ignored.
    RasterizerDiscard = 0x8C89,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum BufferBit {
    /// Passed to clear to clear the current depth buffer.
    Depth = 0x00000100,
    /// Passed to clear to clear the current stencil buffer.
    Stencil = 0x00000400,
    /// Passed to clear to clear the current color buffer.
    Color = 0x00004000,
}

/// Passed to drawElements or drawArrays to draw primitives.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Primitives {
    /// Passed to drawElements or drawArrays to draw single points.
    Points = 0x0000,
    /// Passed to drawElements or drawArrays to draw lines. Each vertex connects to the one after it.
    Lines = 0x0001,
    /// Passed to drawElements or drawArrays to draw lines. Each set of two vertices is treated as a separate line segment.
    LineLoop = 0x0002,
    /// Passed to drawElements or drawArrays to draw a connected group of line segments from the first vertex to the last.
    LineStrip = 0x0003,
    /// Passed to drawElements or drawArrays to draw triangles. Each set of three vertices creates a separate triangle.
    Triangles = 0x0004,
    /// Passed to drawElements or drawArrays to draw a connected group of triangles.
    TriangleStrip = 0x0005,
    /// Passed to drawElements or drawArrays to draw a connected group of triangles. Each vertex connects to the previous and the first vertex in the fan.
    TriangleFan = 0x0006,
}

/// Constants passed to WebGLRenderingContext.blendFunc() or WebGLRenderingContext.blendFuncSeparate() to specify the blending mode (for both, RBG and alpha, or separately).
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum BlendMode {
    /// Passed to blendFunc or blendFuncSeparate to turn off a component.
    Zero = 0,
    /// Passed to blendFunc or blendFuncSeparate to turn on a component.
    One = 1,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by the source elements color.
    SrcColor = 0x0300,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by one minus the source elements color.
    OneMinusSrcColor = 0x0301,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by the source's alpha.
    SrcAlpha = 0x0302,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by one minus the source's alpha.
    OneMinusSrcAlpha = 0x0303,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by the destination's alpha.
    DstAlpha = 0x0304,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by one minus the destination's alpha.
    OneMinusDstAlpha = 0x0305,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by the destination's color.
    DstColor = 0x0306,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by one minus the destination's color.
    OneMinusDstColor = 0x0307,
    /// Passed to blendFunc or blendFuncSeparate to multiply a component by the minimum of source's alpha or one minus the destination's alpha.
    SrcAlphaSaturate = 0x0308,
    /// Passed to blendFunc or blendFuncSeparate to specify a constant color blend function.
    ConstantColor = 0x8001,
    /// Passed to blendFunc or blendFuncSeparate to specify one minus a constant color blend function.
    OneMinusConstantColor = 0x8002,
    /// Passed to blendFunc or blendFuncSeparate to specify a constant alpha blend function.
    ConstantAlpha = 0x8003,
    /// Passed to blendFunc or blendFuncSeparate to specify one minus a constant alpha blend function.
    OneMinusConstantAlpha = 0x8004,
}

/// Constants passed to WebGLRenderingContext.blendEquation()
/// or WebGLRenderingContext.blendEquationSeparate() to control
/// how the blending is calculated (for both, RBG and alpha, or separately).
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum BlendEquation {
    /// Passed to blendEquation or blendEquationSeparate to set an addition blend function.
    FuncAdd = 0x8006,
    /// Passed to blendEquation or blendEquationSeparate to specify a subtraction blend function (source - destination).
    FuncSubstract = 0x800A,
    /// Passed to blendEquation or blendEquationSeparate to specify a reverse subtraction blend function (destination - source).
    FuncReverseSubtract = 0x800B,
    /// Minimum of source and destination,
    Min = 0x8007,
    /// Maximum of source and destination,
    Max = 0x8008,
}

/// Constants passed to WebGLRenderingContext.getParameter() to specify what information to return.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Parameter {
    /// Passed to getParameter to get the current RGB blend function. same as BlendEquationRgb
    BlendEquation = 0x8009,
    /// Passed to getParameter to get the current alpha blend function. Same as BLEND_EQUATION
    BlendEquationAlpha = 0x883D,
    /// Passed to getParameter to get the current destination RGB blend function.
    BlendDstRgb = 0x80C8,
    /// Passed to getParameter to get the current destination RGB blend function.
    BlendSrcRgb = 0x80C9,
    /// Passed to getParameter to get the current destination alpha blend function.
    BlendDstAlpha = 0x80CA,
    /// Passed to getParameter to get the current source alpha blend function.
    BlendSrcAlpha = 0x80CB,
    /// Passed to getParameter to return a the current blend color.
    BlendColor = 0x8005,
    /// Passed to getParameter to get the array buffer binding.
    ArrayBufferBinding = 0x8894,
    /// Passed to getParameter to get the current element array buffer.
    ElementArrayBufferBinding = 0x8895,
    /// Passed to getParameter to get the current lineWidth (set by the lineWidth method).
    LineWidth = 0x0B21,
    /// Passed to getParameter to get the current size of a point drawn with gl.POINTS
    AliasedPointSizeRange = 0x846D,
    /// Passed to getParameter to get the range of available widths for a line. Returns a length-2 array with the lo value at 0, and hight at 1.
    AliasedLineWidthRange = 0x846E,
    /// Passed to getParameter to get the current value of cullFace. Should return FRONT, BACK, or FRONT_AND_BACK
    CullFaceMode = 0x0B45,
    /// Passed to getParameter to determine the current value of frontFace. Should return CW or CCW.
    FrontFace = 0x0B46,
    /// Passed to getParameter to return a length-2 array of floats giving the current depth range.
    DepthRange = 0x0B70,
    /// Passed to getParameter to determine if the depth write mask is enabled.
    DepthWritemask = 0x0B72,
    /// Passed to getParameter to determine the current depth clear value.
    DepthClearValue = 0x0B73,
    /// Passed to getParameter to get the current depth function. Returns NEVER, ALWAYS, LESS, EQUAL, LEQUAL, GREATER, GEQUAL, or NOTEQUAL.
    DepthFunc = 0x0B74,
    /// Passed to getParameter to get the value the stencil will be cleared to.
    StencilClearValue = 0x0B91,
    /// Passed to getParameter to get the current stencil function. Returns NEVER, ALWAYS, LESS, EQUAL, LEQUAL, GREATER, GEQUAL, or NOTEQUAL.
    StencilFunc = 0x0B92,
    /// Passed to getParameter to get the current stencil fail function. Should return KEEP, REPLACE, INCR, DECR, INVERT, INCR_WRAP, or DECR_WRAP.
    StencilFail = 0x0B94,
    /// Passed to getParameter to get the current stencil fail function should the depth buffer test fail. Should return KEEP, REPLACE, INCR, DECR, INVERT, INCR_WRAP, or DECR_WRAP.
    StencilPassDepthFail = 0x0B95,
    /// Passed to getParameter to get the current stencil fail function should the depth buffer test pass. Should return KEEP, REPLACE, INCR, DECR, INVERT, INCR_WRAP, or DECR_WRAP.
    StencilPassDepthPass = 0x0B96,
    /// Passed to getParameter to get the reference value used for stencil tests.
    StencilRef = 0x0B97,
    ///
    StencilValueMask = 0x0B93,
    ///
    StencilWritemask = 0x0B98,
    ///
    StencilBackFunc = 0x8800,
    ///
    StencilBackFail = 0x8801,
    ///
    StencilBackPassDepthFail = 0x8802,
    ///
    StencilBackPassDepthPass = 0x8803,
    ///
    StencilBackRef = 0x8CA3,
    ///
    StencilBackValueMask = 0x8CA4,
    ///
    StencilBackWritemask = 0x8CA5,
    /// Returns an Int32Array with four elements for the current viewport dimensions.
    Viewport = 0x0BA2,
    /// Returns an Int32Array with four elements for the current scissor box dimensions.
    ScissorBox = 0x0C10,
    ///
    ColorClearValue = 0x0C22,
    ///
    ColorWritemask = 0x0C23,
    ///
    UnpackAlignment = 0x0CF5,
    ///
    PackAlignment = 0x0D05,
    ///
    MaxTextureSize = 0x0D33,
    ///
    MaxViewportDims = 0x0D3A,
    ///
    SubpixelBits = 0x0D50,
    ///
    RedBits = 0x0D52,
    ///
    GreenBits = 0x0D53,
    ///
    BlueBits = 0x0D54,
    ///
    AlphaBits = 0x0D55,
    ///
    DepthBits = 0x0D56,
    ///
    StencilBits = 0x0D57,
    ///
    PolygonOffsetUnits = 0x2A00,
    ///
    PolygonOffsetFactor = 0x8038,
    ///
    TextureBinding2d = 0x8069,
    ///
    SampleBuffers = 0x80A8,
    ///
    Samples = 0x80A9,
    ///
    SampleCoverageValue = 0x80AA,
    ///
    SampleCoverageInvert = 0x80AB,
    ///
    CompressedTextureFormats = 0x86A3,
    ///
    Vendor = 0x1F00,
    ///
    Renderer = 0x1F01,
    ///
    Version = 0x1F02,
    ///
    ImplementationColorReadType = 0x8B9A,
    ///
    ImplementationColorReadFormat = 0x8B9B,
    ///
    BrowserDefaultWebgl = 0x9244,

    ///
    TextureBindingCubeMap = 0x8514,

    ///
    MaxCubeMapTextureSize = 0x851C,
}

/// Constants passed to WebGLRenderingContext.getVertexAttrib().
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum VertexAttrib {
    /// Passed to getVertexAttrib to read back the current vertex attribute.
    Current = 0x8626,
    ///
    ArrayEnabled = 0x8622,
    ///
    ArraySize = 0x8623,
    ///
    ArrayStride = 0x8624,
    ///
    ArrayType = 0x8625,
    ///
    ArrayNormalized = 0x886A,
    ///
    ArrayPointer = 0x8645,
    ///
    ArrayBufferBinding = 0x889F,
}

/// Constants passed to WebGLRenderingContext.cullFace().
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Culling {
    /// Passed to cullFace to specify that only front faces should be drawn.
    Front = 0x0404,
    /// Passed to cullFace to specify that only back faces should be drawn.
    Back = 0x0405,
    /// Passed to cullFace to specify that front and back faces should be drawn.
    FrontAndBack = 0x0408,
}

/// Constants returned from WebGLRenderingContext.getError().
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Returned from getError.
    NoError = 0,
    /// Returned from getError.
    InvalidEnum = 0x0500,
    /// Returned from getError.
    InvalidValue = 0x0501,
    /// Returned from getError.
    InvalidOperation = 0x0502,
    /// Returned from getError.
    InvalidFramebufferOperation = 0x0506,
    /// Returned from getError.
    OutOfMemory = 0x0505,
    /// Returned from getError.
    ContextLostWebgl = 0x9242,
}

/// Constants passed to WebGLRenderingContext.frontFace().
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum FrontFaceDirection {
    /// Passed to frontFace to specify the front face of a polygon is drawn in the clockwise direction
    CW = 0x0900,
    /// Passed to frontFace to specify the front face of a polygon is drawn in the counter clockwise direction
    CCW = 0x0901,
}

/// Constants passed to WebGLRenderingContext.depthFunc().
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum DepthTest {
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will never pass. i.e. Nothing will be drawn.
    Never = 0x0200,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will always pass. i.e. Pixels will be drawn in the order they are drawn.
    Always = 0x0207,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is less than the stored value.
    Less = 0x0201,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is equals to the stored value.
    Equal = 0x0202,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is less than or equal to the stored value.
    Lequal = 0x0203,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is greater than the stored value.
    Greater = 0x0204,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is greater than or equal to the stored value.
    Gequal = 0x0206,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is not equal to the stored value.
    Notequal = 0x0205,
}

/// Constants passed to WebGLRenderingContext.stencilFunc().
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum StencilTest {
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will never pass. i.e. Nothing will be drawn.
    Never = 0x0200,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will always pass. i.e. Pixels will be drawn in the order they are drawn.
    Always = 0x0207,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is less than the stored value.
    Less = 0x0201,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is equals to the stored value.
    Equal = 0x0202,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is less than or equal to the stored value.
    Lequal = 0x0203,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is greater than the stored value.
    Greater = 0x0204,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is greater than or equal to the stored value.
    Gequal = 0x0206,
    /// Passed to depthFunction or stencilFunction to specify depth or stencil tests will pass if the new depth value is not equal to the stored value.
    Notequal = 0x0205,
}

/// Constants passed to WebGLRenderingContext.stencilOp().
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum StencilAction {
    Zero = 0,
    ///
    Keep = 0x1E00,
    ///
    Replace = 0x1E01,
    ///
    Incr = 0x1E02,
    ///
    Decr = 0x1E03,
    ///
    Invert = 0x150A,
    ///
    IncrWrap = 0x8507,
    ///
    DecrWrap = 0x8508,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PixelType {
    ///
    UnsignedByte = 0x1401,
    ///
    UnsignedShort4444 = 0x8033,
    ///
    UnsignedShort5551 = 0x8034,
    ///
    UnsignedShort565 = 0x8363,
    ///
    Float = 0x1406,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    ///
    DepthComponent = 0x1902,
    ///
    Alpha = 0x1906,
    ///
    Rgb = 0x1907,
    ///
    Rgba = 0x1908,
    ///
    Luminance = 0x1909,
    ///
    LuminanceAlpha = 0x190A,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PixelReadFormat {
    ///
    Alpha = 0x1906,
    ///
    Rgb = 0x1907,
    ///
    Rgba = 0x1908,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PixelCopyFormat {
    ///
    Alpha = 0x1906,
    ///
    Rgb = 0x1907,
    ///
    Rgba = 0x1908,
    ///
    Luminance = 0x1909,
    ///
    LuminanceAlpha = 0x190A,
}

/// Constants passed to WebGLRenderingContext.hint() mode argument
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum HintMode {
    /// There is no preference for this behavior.
    DontCare = 0x1100,
    /// The most efficient behavior should be used.
    Fastest = 0x1101,
    /// The most correct or the highest quality option should be used.
    Nicest = 0x1102,
}

/// Constants passed to WebGLRenderingContext.hint() target argument
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum HintTarget {
    /// Hint for the quality of filtering when generating mipmap images with WebGLRenderingContext.generateMipmap().
    GenerateMipmapHint = 0x8192,
    /// Accuracy of the derivative calculation for the GLSL built-in functions: dFdx, dFdy, and fwidth.
    FragmentShaderDerivativeHint = 0x8B8B,
}

/// WebGLRenderingContext.texParameter[fi]() or WebGLRenderingContext.bindTexture() "target" parameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextureKind {
    ///
    Texture2d = 0x0DE1,
    ///
    TextureCubeMap = 0x8513,
    /// A three-dimensional texture.
    Texture3d = 0x806F,
    /// A two-dimensional array texture.
    Texture2dArray = 0x8C1A,
}

/// WebGLRenderingContext.texStorage2D() `target` parameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Texture2DKind {
    ///
    Texture2d = 0x0DE1,
    ///
    TextureCubeMap = 0x8513,
}

/// WebGLRenderingContext.texStorage3D() `target` parameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Texture3DKind {
    /// A three-dimensional texture.
    Texture3d = 0x806F,
    /// A two-dimensional array texture.
    Texture2dArray = 0x8C1A,
}

/// WebGLRenderingContext.texParameter[fi]() "pname" parameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextureParameter {
    ///
    MagFilter = 0x2800,
    ///
    MinFilter = 0x2801,
    ///
    WrapS = 0x2802,
    ///
    WrapT = 0x2803,
    /// Texture mipmap level
    BaseLevel = 0x813C,
    /// Comparison function
    CompareFunc = 0x884D,
    /// Texture comparison mode
    CompareMode = 0x884C,
    /// Immutability of the texture format and size
    ImmutableFormat = 0x912F,
    ///
    ImmutableLevels = 0x82DF,
    /// Maximum texture mipmap array level
    MaxLevel = 0x813D,
    /// Texture maximum level-of-detail value
    MaxLod = 0x813B,
    /// Texture minimum level-of-detail value
    MinLod = 0x813A,
    /// gl.TEXTURE_WRAP_R Wrapping function for texture coordinate r
    WrapR = 0x8072,
}

/// WebGLRenderingContext.texImage2D() "target" parameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextureBindPoint {
    ///
    Texture2d = 0x0DE1,
    ///
    TextureCubeMapPositiveX = 0x8515,
    ///
    TextureCubeMapNegativeX = 0x8516,
    ///
    TextureCubeMapPositiveY = 0x8517,
    ///
    TextureCubeMapNegativeY = 0x8518,
    ///
    TextureCubeMapPositiveZ = 0x8519,
    ///
    TextureCubeMapNegativeZ = 0x851A,
}

/// WebGLRenderingContext.texParameter[fi]() "param" parameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextureMagFilter {
    ///
    Nearest = 0x2600,
    ///
    Linear = 0x2601,
}

/// WebGLRenderingContext.texParameter[fi]() "param" parameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextureMinFilter {
    ///
    Nearest = 0x2600,
    ///
    Linear = 0x2601,
    ///
    NearestMipmapNearest = 0x2700,
    ///
    LinearMipmapNearest = 0x2701,
    ///
    NearestMipmapLinear = 0x2702,
    ///
    LinearMipmapLinear = 0x2703,
}

/// WebGLRenderingContext.texParameter[fi]() "param" parameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureWrap {
    ///
    Repeat = 0x2901,
    ///
    ClampToEdge = 0x812F,
    ///
    MirroredRepeat = 0x8370,
}

/// Constants passed to WebGLRenderingContext.hint()
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Buffers {
    ///
    Framebuffer = 0x8D40,
    ///
    Renderbuffer = 0x8D41,
    ///
    Rgba4 = 0x8056,
    ///
    Rgb5A1 = 0x8057,
    ///
    Rgb565 = 0x8D62,
    ///
    DepthComponent16 = 0x81A5,
    ///
    StencilIndex = 0x1901,
    ///
    StencilIndex8 = 0x8D48,
    ///
    DepthStencil = 0x84F9,
    ///
    RenderbufferWidth = 0x8D42,
    ///
    RenderbufferHeight = 0x8D43,
    ///
    RenderbufferInternalFormat = 0x8D44,
    ///
    RenderbufferRedSize = 0x8D50,
    ///
    RenderbufferGreenSize = 0x8D51,
    ///
    RenderbufferBlueSize = 0x8D52,
    ///
    RenderbufferAlphaSize = 0x8D53,
    ///
    RenderbufferDepthSize = 0x8D54,
    ///
    RenderbufferStencilSize = 0x8D55,
    ///
    FramebufferAttachmentObjectType = 0x8CD0,
    ///
    FramebufferAttachmentObjectName = 0x8CD1,
    ///
    FramebufferAttachmentTextureLevel = 0x8CD2,
    ///
    FramebufferAttachmentTextureCubeMapFace = 0x8CD3,
    ///
    ColorAttachment0 = 0x8CE0,
    ///
    DepthAttachment = 0x8D00,
    ///
    StencilAttachment = 0x8D20,
    ///
    DepthStencilAttachment = 0x821A,
    ///
    None = 0,
    ///
    FramebufferComplete = 0x8CD5,
    ///
    FramebufferIncompleteAttachment = 0x8CD6,
    ///
    FramebufferIncompleteMissingAttachment = 0x8CD7,
    ///
    FramebufferIncompleteDimensions = 0x8CD9,
    ///
    FramebufferUnsupported = 0x8CDD,
    ///
    FramebufferBinding = 0x8CA6,
    ///
    RenderbufferBinding = 0x8CA7,
    ///
    MaxRenderbufferSize = 0x84E8,
    ///
    InvalidFramebufferOperation = 0x0506,
}

/// Constants passed to WebGLRenderingContext.pixelStorei()
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PixelStorageMode {
    ///
    UnpackFlipYWebgl = 0x9240,
    ///
    UnpackPremultiplyAlphaWebgl = 0x9241,
    ///
    UnpackColorspaceConversionWebgl = 0x9243,
    /// Packing of pixel data into memory.
    /// Can be 1, 2, 4, 8 defaults to 4
    PackAlignment = 0x0D05,
    /// Unpacking of pixel data from memory
    /// Can be 1, 2, 4, 8 defaults to 4
    UnpackAlignment = 0x0CF5,
    /// Number of pixels in a row.
    PackRowLength = 0x0D02,
    /// Number of pixel locations skipped before the first pixel is written into memory.
    PackSkipPixels = 0x0D04,
    /// Number of rows of pixel locations skipped before the first pixel is written into memory
    PackSkipRows = 0x0D03,
    /// Number of pixels in a row.
    UnpackRowLength = 0x0CF2,
    /// Image height used for reading pixel data from memory
    UnpackImageHeight = 0x806E,
    /// Number of pixel images skipped before the first pixel is read from memory
    UnpackSkipPixels = 0x0CF4,
    /// Number of rows of pixel locations skipped before the first pixel is read from memory
    UnpackSkipRows = 0x0CF3,
    /// Number of pixel images skipped before the first pixel is read from memory
    UnpackSkipImages = 0x806D,
}

///
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum ShaderPrecision {
    ///
    LowFloat = 0x8DF0,
    ///
    MediumFloat = 0x8DF1,
    ///
    HighFloat = 0x8DF2,
    ///
    LowInt = 0x8DF3,
    ///
    MediumInt = 0x8DF4,
    ///
    HighInt = 0x8DF5,
}

/// Constants passed to WebGLRenderingContext.hint()
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum UniformType {
    ///
    FloatVec2 = 0x8B50,
    ///
    FloatVec3 = 0x8B51,
    ///
    FloatVec4 = 0x8B52,
    ///
    IntVec2 = 0x8B53,
    ///
    IntVec3 = 0x8B54,
    ///
    IntVec4 = 0x8B55,
    ///
    Bool = 0x8B56,
    ///
    BoolVec2 = 0x8B57,
    ///
    BoolVec3 = 0x8B58,
    ///
    BoolVec4 = 0x8B59,
    ///
    FloatMat2 = 0x8B5A,
    ///
    FloatMat3 = 0x8B5B,
    ///
    FloatMat4 = 0x8B5C,
    ///
    Sampler2d = 0x8B5E,
    ///
    SamplerCube = 0x8B60,
}

///
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextureCompression {
    /// A DXT1-compressed image in an RGB image format.
    RgbDxt1 = 0x83F0,
    /// A DXT1-compressed image in an RGB image format with a simple on/off alpha value.
    RgbaDxt1 = 0x83F1,
    ///	A DXT3-compressed image in an RGBA image format.
    /// Compared to a 32-bit RGBA texture, it offers 4:1 compression.
    RgbaDxt3 = 0x83F2,
    /// A DXT5-compressed image in an RGBA image format.
    /// It also provides a 4:1 compression,
    /// but differs to the DXT3 compression in how the alpha compression is done.
    RgbaDxt5 = 0x83F3,
}

/// A texture unit
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TextureUnit {
    Texture0 = 0x84C0,
    Texture1 = 0x84C1,
    Texture2 = 0x84C2,
    Texture3 = 0x84C3,
    Texture4 = 0x84C4,
    Texture5 = 0x84C5,
    Texture6 = 0x84C6,
    Texture7 = 0x84C7,
    Texture8 = 0x84C8,
    Texture9 = 0x84C9,
    Texture10 = 0x84CA,
    Texture11 = 0x84CB,
    Texture12 = 0x84CC,
    Texture13 = 0x84CD,
    Texture14 = 0x84CE,
    Texture15 = 0x84CF,
    Texture16 = 0x84D0,
    Texture17 = 0x84D1,
    Texture18 = 0x84D2,
    Texture19 = 0x84D3,
    Texture20 = 0x84D4,
    Texture21 = 0x84D5,
    Texture22 = 0x84D6,
    Texture23 = 0x84D7,
    Texture24 = 0x84D8,
    Texture25 = 0x84D9,
    Texture26 = 0x84DA,
    Texture27 = 0x84DB,
    Texture28 = 0x84DC,
    Texture29 = 0x84DD,
    Texture30 = 0x84DE,
    Texture31 = 0x84DF,
}

/// Constants passed to WebGLRenderingContext.bindFramebuffer() and other framebuffer methods
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum FramebufferKind {
    /// Collection buffer data storage of color, alpha, depth and stencil buffers used to render an image.
    Framebuffer = 0x8D40,
    /// Equivalent to `Framebuffer`. Used as a destination for drawing, rendering, clearing, and writing operations.
    DrawFramebuffer = 0x8CA9,
    /// Used as a source for reading operations.
    ReadFramebuffer = 0x8CA8,
}

/// Constants passed to `WebGLRenderingContext.checkFramebufferStatus()`
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum FramebufferStatus {
    /// The framebuffer is ready to display.
    FramebufferComplete = 0x8CD5,
    /// The attachment types are mismatched or not all framebuffer attachment points are framebuffer attachment complete.
    FramebufferIncompleteAttachment = 0x8CD6,
    /// There is no attachment.
    FramebufferIncompleteMissingAttachment = 0x8CD7,
    /// Height and width of the attachment are not the same.
    FramebufferIncompleteDimensions = 0x8CD9,
    /// The format of the attachment is not supported or if depth and stencil attachments are not the same renderbuffer.
    FramebufferUnsupported = 0x8CDD,
    /// The values of gl.RENDERBUFFER_SAMPLES are different among attached renderbuffers, or are non-zero if the
    /// attached images are a mix of renderbuffers and textures.
    FramebufferIncompleteMultisample = 0x8D56,
}

/// Constants passed to `WebGLRenderingContext.framebufferRenderbuffer()`
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum RenderbufferKind {
    /// Buffer data storage for single images in a renderable internal format.
    Renderbuffer = 0x8D41,
}

/// Constants passed to `WebGLRenderingContext.framebufferRenderbuffer()`
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Attachment {
    /// color buffer.
    ColorAttachment0 = 0x8CE0,
    /// depth buffer.
    DepthAttachment = 0x8D00,
    /// stencil buffer.
    StencilAttachment = 0x8D20,
    /// depth and stencil buffer.
    DepthStencilAttachment = 0x821A,
    ColorAttachment1 = 0x8CE1,
    ColorAttachment2 = 0x8CE2,
    ColorAttachment3 = 0x8CE3,
    ColorAttachment4 = 0x8CE4,
    ColorAttachment5 = 0x8CE5,
    ColorAttachment6 = 0x8CE6,
    ColorAttachment7 = 0x8CE7,
    ColorAttachment8 = 0x8CE8,
    ColorAttachment9 = 0x8CE9,
    ColorAttachment10 = 0x8CEA,
    ColorAttachment11 = 0x8CEB,
    ColorAttachment12 = 0x8CEC,
    ColorAttachment13 = 0x8CED,
    ColorAttachment14 = 0x8CEE,
    ColorAttachment15 = 0x8CEF,
}

/// Constants passed to `WebGLRenderingContext.getRenderbufferParameter()`
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum RenderbufferParameter {
    /// Returns a GLint indicating the width of the image of the currently bound renderbuffer.
    Width = 0x8D42,
    /// Returns a GLint indicating the height of the image of the currently bound renderbuffer.
    Height = 0x8D43,
    /// Returns a GLint that is the resolution size (in bits) for the green color.
    GreenSize = 0x8D51,
    /// Returns a GLint that is the resolution size (in bits) for the blue color.
    BlueSize = 0x8D52,
    /// Returns a GLint that is the resolution size (in bits) for the red color.
    RedSize = 0x8D50,
    /// Returns a GLint that is the resolution size (in bits) for the alpha component.
    AlphaSize = 0x8D53,
    /// Returns a GLint that is the resolution size (in bits) for the depth component.
    DepthSize = 0x8D54,
    /// Returns a GLint that is the resolution size (in bits) for the stencil component.
    StencilSize = 0x8D55,
    /// Returns a GLint indicating the number of samples of the image of the currently bound renderbuffer.
    Samples = 0x8CAB,
    Format = 0x8D44,
}

// TODO extend with https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/renderbufferStorage
/// Constants returned from `WebGLRenderingContext.getRenderbufferParameter()`
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum RenderbufferFormat {
    /// 4 red bits, 4 green bits, 4 blue bits 4 alpha bits.
    Rgba4 = 0x8056,
    /// 5 red bits, 6 green bits, 5 blue bits.
    Rgb565 = 0x8D62,
    /// 5 red bits, 5 green bits, 5 blue bits, 1 alpha bit.
    Rgb5A1 = 0x8057,
    /// 16 depth bits.
    DepthComponent16 = 0x81A5,
    /// 8 stencil bits.
    StencilIndex8 = 0x8D48,
}

/// Constants passed to vertexAttribPointer
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum AttributeType {
    /// signed 8-bit integer, with values in [-128, 127]
    Byte = 0x1400,
    /// signed 16-bit integer, with values in [-32768, 32767]
    Short = 0x1402,
    /// unsigned 8-bit integer, with values in [0, 255]
    UnsignedByte = 0x1401,
    /// unsigned 16-bit integer, with values in [0, 65535]
    UnsignedShort = 0x1403,
    /// 32-bit IEEE floating point number
    Float = 0x1406,
    /// 16-bit IEEE floating point number
    HalfFloat = 0x140B,
}

/// Constants passed to readBuffer
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum ColorBuffer {
    None = 0,
    Back = 0x0405,
    /// color buffer.
    ColorAttachment0 = 0x8CE0,
    ColorAttachment1 = 0x8CE1,
    ColorAttachment2 = 0x8CE2,
    ColorAttachment3 = 0x8CE3,
    ColorAttachment4 = 0x8CE4,
    ColorAttachment5 = 0x8CE5,
    ColorAttachment6 = 0x8CE6,
    ColorAttachment7 = 0x8CE7,
    ColorAttachment8 = 0x8CE8,
    ColorAttachment9 = 0x8CE9,
    ColorAttachment10 = 0x8CEA,
    ColorAttachment11 = 0x8CEB,
    ColorAttachment12 = 0x8CEC,
    ColorAttachment13 = 0x8CED,
    ColorAttachment14 = 0x8CEE,
    ColorAttachment15 = 0x8CEF,
}

/// Constants passed to getInternalformatParameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum InformationType {
    /// Returns a Int32Array containing sample counts supported for internalformat in descending order.
    Samples = 0x80A9,
}

/// Constants passed to beginQuery
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum QueryTarget {
    /// Specifies an occlusion query: these queries detect whether an object is visible (whether the scoped drawing commands pass the depth test and if so, how many samples pass).
    AnySamplesPassed = 0x8C2F,
    /// Same as above above, but less accurate and faster version.
    AnySamplesPassedConservative = 0x8D6A,
    /// Number of primitives that are written to transform feedback buffers.
    TransformFeedbackPrimitivesWritten = 0x8C88,
}

/// Constants passed to getQuery
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Query {
    Current = 0x8865,
}

/// Constants passed to getQueryParameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum QueryParameter {
    Result = 0x8866,
    ResultAvailable = 0x8867,
}

/// Constants passed to fenceSync
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum GPUState {
    CommandsComplete = 0x9117,
}

/// Constants passed to clientWaitSync
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum WaitStatus {
    /// Indicates that the sync object was signaled when this method was called.
    AlreadySignaled = 0x911A,
    /// Indicates that the timeout time passed and that the sync object did not become signaled.
    TimeoutExpired = 0x911B,
    /// Indicates that the sync object was signaled before the timeout expired.
    ConditionSatisfied = 0x911C,
    /// Indicates that an error occurred during the execution.
    WaitFailed = 0x911D,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum SyncStatus {
    Signaled = 0x9119,
    Unsignaled = 0x9118,
}

/// Constants passed to getSyncParameter
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum SyncParameter {
    Type = 0x9112,
    Status = 0x9114,
    Condition = 0x9113,
    Flags = 0x9115,
}

/// Constants passed to bindTransformFeedback
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TransformFeedback {
    TransformFeedback = 0x8E22,
}

/// Passed to beginTransformFeedback.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TransformFeedbackMode {
    /// Passed to drawElements or drawArrays to draw single points.
    Points = 0x0000,
    /// Passed to drawElements or drawArrays to draw lines. Each vertex connects to the one after it.
    Lines = 0x0001,
    /// Passed to drawElements or drawArrays to draw triangles. Each set of three vertices creates a separate triangle.
    Triangles = 0x0004,
}

/// Passed to transformFeedbackVaryings.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum TransformFeedbackBufferMode {
    InterleavedAttribs = 0x8C8C,
    SeparateAttribs = 0x8C8D,
}

/// Passed to bindBufferBase
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum BufferBase {
    /// Buffer for transform feedback operations.
    TransformFeedbackBuffer = 0x8C8E,
    /// Buffer used for storing uniform blocks.
    UniformBuffer = 0x8A11,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum CompareMode {
    None = 0,
    CompareRefToTexture = 0x884E,
}
