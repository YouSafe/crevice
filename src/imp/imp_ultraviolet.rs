minty_impl! {
    mint::Vector2<f32> => ultraviolet::Vec2,
    mint::Vector3<f32> => ultraviolet::Vec3,
    mint::Vector4<f32> => ultraviolet::Vec4,

    mint::Vector2<f64> => ultraviolet::DVec2,
    mint::Vector3<f64> => ultraviolet::DVec3,
    mint::Vector4<f64> => ultraviolet::DVec4,

    mint::ColumnMatrix2<f32> => ultraviolet::Mat2,
    mint::ColumnMatrix3<f32> => ultraviolet::Mat3,
    mint::ColumnMatrix4<f32> => ultraviolet::Mat4,

    mint::ColumnMatrix2<f64> => ultraviolet::DMat2,
    mint::ColumnMatrix3<f64> => ultraviolet::DMat3,
    mint::ColumnMatrix4<f64> => ultraviolet::DMat4,
}

#[cfg(feature = "int")]
minty_impl! {
    mint::Vector2<u32> => ultraviolet::UVec2,
    mint::Vector3<u32> => ultraviolet::UVec3,
    mint::Vector4<u32> => ultraviolet::UVec4,

    mint::Vector2<i32> => ultraviolet::IVec2,
    mint::Vector3<i32> => ultraviolet::IVec3,
    mint::Vector4<i32> => ultraviolet::IVec4,
}
