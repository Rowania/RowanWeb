//! API 元数据和类型反射系统
//!
//! 提供 API 端点的自动化元数据生成功能，包括：
//! - 类型反射机制，用于自动推导请求/响应类型
//! - API 端点注解，支持路径、方法、描述等信息
//! - 注解路由器，结合标准路由器和元数据管理
//!
//! 这个模块是实现 API 文档自动生成的基础。

use axum::{Router, routing::MethodRouter};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

/// 结构体字段的元数据描述
#[derive(Debug, Serialize, Clone)]
pub struct FieldDescriptor {
    pub name: String,
    pub field_type: TypeDescriptor,
    pub optional: bool,
}

/// 枚举变体的元数据描述
#[derive(Debug, Serialize, Clone)]
pub struct VariantDescriptor {
    pub name: String,
    pub fields: Option<Vec<FieldDescriptor>>,
}

/// 类型描述符
///
/// 递归的类型系统表示，用于描述任意 Rust 类型的结构。
/// 支持基础类型、容器类型、结构体、枚举等。
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind", content = "details")]
#[allow(dead_code)]
pub enum TypeDescriptor {
    String,
    Bool,
    I64,
    U64,
    F64,
    Vec(Box<TypeDescriptor>),
    Option(Box<TypeDescriptor>),
    Map(Box<TypeDescriptor>, Box<TypeDescriptor>),
    Struct {
        name: String,
        fields: Vec<FieldDescriptor>,
    },
    Enum {
        name: String,
        variants: Vec<VariantDescriptor>,
    },
}

/// 类型反射 trait
///
/// 提供编译时类型信息的运行时访问能力。
/// 通常通过过程宏 `#[derive(Schema)]` 自动实现。
pub trait Schema {
    /// 生成类型的描述符
    fn schema() -> TypeDescriptor;
}

impl Schema for String {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::String
    }
}

impl Schema for u64 {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::U64
    }
}

impl Schema for bool {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::Bool
    }
}

impl Schema for i64 {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::I64
    }
}

impl Schema for f64 {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::F64
    }
}

impl Schema for &str {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::String
    }
}

impl Schema for i32 {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::I64
    }
}

impl Schema for u32 {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::U64
    }
}

impl Schema for f32 {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::F64
    }
}

impl Schema for usize {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::U64
    }
}

impl<T: Schema> Schema for Vec<T> {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::Vec(Box::new(T::schema()))
    }
}

impl<T: Schema> Schema for Option<T> {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::Option(Box::new(T::schema()))
    }
}

impl<K: Schema, V: Schema> Schema for BTreeMap<K, V> {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::Map(Box::new(K::schema()), Box::new(V::schema()))
    }
}

impl<K: Schema, V: Schema> Schema for HashMap<K, V> {
    fn schema() -> TypeDescriptor {
        TypeDescriptor::Map(Box::new(K::schema()), Box::new(V::schema()))
    }
}

// --- API 端点相关结构 ---

/// HTTP 方法枚举
#[derive(Debug, Serialize, Clone)]
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

/// API 请求参数类型
#[derive(Debug, Serialize, Clone)]
pub enum RequestParams {
    Query(TypeDescriptor),
    Body(TypeDescriptor),
    None,
}
/// API 端点描述符
///
/// 存储单个 API 端点的完整信息，包括路径、方法、描述和类型信息。
/// 用于自动生成 API 文档和类型检查。
#[derive(Debug, Serialize, Clone)]
pub struct ApiEndpoint {
    pub path: String,
    pub method: Method,
    pub description: String,
    pub params: RequestParams,
    pub response_type: Option<TypeDescriptor>,
}

impl ApiEndpoint {
    /// 创建新的 API 端点描述符
    pub fn new(path: String, method: Method, description: String) -> Self {
        Self {
            path,
            method,
            description,
            params: RequestParams::None,
            response_type: None,
        }
    }

    /// 设置响应体类型
    pub fn with_response_type<T: Schema>(mut self) -> Self {
        self.response_type = Some(T::schema());
        self
    }

    /// 设置请求体类型
    #[allow(dead_code)]
    pub fn with_body_type<T: Schema>(mut self) -> Self {
        self.params = RequestParams::Body(T::schema());
        self
    }

    /// 设置查询参数类型
    #[allow(dead_code)]
    pub fn with_query_type<T: Schema>(mut self) -> Self {
        self.params = RequestParams::Query(T::schema());
        self
    }
}

// --- 注解路由器 ---

/// 注解路由器
///
/// 结合标准 Axum 路由器和 API 元数据管理的复合结构。
/// 在注册路由的同时收集 API 端点信息，用于自动生成文档。
pub struct AnnotatedRouter {
    /// 内部的 Axum 路由器
    inner: Router,
    /// 收集的 API 端点注解信息
    annotations: Vec<ApiEndpoint>,
}

impl Default for AnnotatedRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl AnnotatedRouter {
    /// 创建新的注解路由器
    pub fn new() -> Self {
        Self {
            inner: Router::new(),
            annotations: Vec::new(),
        }
    }

    /// 添加带有类型注解的路由
    pub fn route<T>(
        mut self,
        path: &str,
        method_router: MethodRouter,
        method: Method,
        description: &str,
    ) -> Self
    where
        T: Schema + 'static,
    {
        let endpoint = ApiEndpoint::new(path.to_string(), method, description.to_string())
            .with_response_type::<T>();
        self.annotations.push(endpoint);
        self.inner = self.inner.route(path, method_router);
        self
    }

    /// 构建最终的路由器
    pub fn build(self) -> Router {
        self.inner
    }

    /// 获取收集的注解信息
    pub fn annotations(&self) -> &Vec<ApiEndpoint> {
        &self.annotations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_schema_implementations() {
        // 测试基础类型
        assert!(matches!(String::schema(), TypeDescriptor::String));
        assert!(matches!(bool::schema(), TypeDescriptor::Bool));
        assert!(matches!(u64::schema(), TypeDescriptor::U64));
        assert!(matches!(i64::schema(), TypeDescriptor::I64));
        assert!(matches!(f64::schema(), TypeDescriptor::F64));
    }

    #[test]
    fn test_generic_schema_implementations() {
        // 测试泛型类型
        let vec_string = Vec::<String>::schema();
        if let TypeDescriptor::Vec(inner) = vec_string {
            assert!(matches!(*inner, TypeDescriptor::String));
        } else {
            panic!("Vec<String> should schema as Vec(String)");
        }

        let option_u64 = Option::<u64>::schema();
        if let TypeDescriptor::Option(inner) = option_u64 {
            assert!(matches!(*inner, TypeDescriptor::U64));
        } else {
            panic!("Option<u64> should schema as Option(U64)");
        }
    }

//     #[test]
//     fn test_derive_macro_struct() {
//         // 测试派生宏生成的结构体反射
//         use crate::api::materials::{Material, RecentMaterialsResponse};

//         let material_descriptor = Material::schema();
//         if let TypeDescriptor::Struct { name, fields } = material_descriptor {
//             assert_eq!(name, "Material");
//             assert_eq!(fields.len(), 8); // id, title, file_type, created_at, course, stats, comment_count, uploader

//             // 检查字段类型
//             assert_eq!(fields[0].name, "id");
//             assert!(matches!(fields[0].field_type, TypeDescriptor::U64));

//             assert_eq!(fields[1].name, "title");
//             assert!(matches!(fields[1].field_type, TypeDescriptor::String));
//         } else {
//             panic!("Material should schema as Struct");
//         }

//         let response_descriptor = RecentMaterialsResponse::schema();
//         if let TypeDescriptor::Struct { name, fields } = response_descriptor {
//             assert_eq!(name, "RecentMaterialsResponse");
//             assert_eq!(fields.len(), 1); // materials

//             assert_eq!(fields[0].name, "materials");
//             // 应该是 Vec<Material>
//             if let TypeDescriptor::Vec(inner) = &fields[0].field_type {
//                 // 检查内部类型是否为 Material 结构体
//                 if let TypeDescriptor::Struct { name, .. } = &**inner {
//                     assert_eq!(name, "Material");
//                 } else {
//                     panic!("Vec should contain Material struct");
//                 }
//             } else {
//                 panic!("materials field should be Vec type");
//             }
//         } else {
//             panic!("RecentMaterialsResponse should schema as Struct");
//         }
//     }

//     #[test]
//     fn test_derive_macro_enum() {
//         // 创建一个测试枚举来验证枚举反射
//         use meta_macros::Schema;

//         #[derive(Schema)]
//         #[allow(dead_code)]
//         enum TestEnum {
//             Unit,
//             Named { value: String, count: u32 },
//         }

//         let enum_descriptor = TestEnum::schema();
//         if let TypeDescriptor::Enum { name, variants } = enum_descriptor {
//             assert_eq!(name, "TestEnum");
//             assert_eq!(variants.len(), 2);

//             // 检查 Unit 变体
//             assert_eq!(variants[0].name, "Unit");
//             assert!(variants[0].fields.is_none());

//             // 检查 Named 变体
//             assert_eq!(variants[1].name, "Named");
//             if let Some(ref fields) = variants[1].fields {
//                 assert_eq!(fields.len(), 2);
//                 assert_eq!(fields[0].name, "value");
//                 assert!(matches!(fields[0].field_type, TypeDescriptor::String));
//                 assert_eq!(fields[1].name, "count");
//                 assert!(matches!(fields[1].field_type, TypeDescriptor::U64)); // u32 被简化为 U64
//             } else {
//                 panic!("Named variant should have fields");
//             }
//         } else {
//             panic!("TestEnum should schema as Enum");
//         }
//     }
}
