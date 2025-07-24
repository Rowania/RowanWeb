//! 元编程过程宏库
//!
//! 提供自动实现类型元数据功能的派生宏，用于在编译时生成类型描述符。
//! 支持结构体和枚举的自动元数据生成，是 API 文档自动生成系统的核心。
//!
//! # 编译时错误
//!
//! 本宏旨在提供明确的编译时反馈。当遇到不支持的类型时，
//! 宏会立即 `panic!`，从而导致编译失败并给出清晰的错误信息。
//! 开发者不应对尚不支持的类型派生 `Schema`。
//!
//! **不支持的类型包括**:
//! - **元组结构体**: 例如 `struct Point(i32, i32);`
//! - **联合体 (Unions)**

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DeriveInput, Fields, FieldsNamed};

/// 自动实现 `Schema` trait 的派生宏
///
/// 为结构体和枚举自动生成 `Schema` trait 的实现，
/// 用于在编译时获取类型的结构信息。
///
/// # 支持的类型
///
/// - **命名字段结构体**: 生成包含所有字段信息的 `TypeDescriptor::Struct`
/// - **单元结构体**: 生成空字段列表的结构体描述符
/// - **枚举**: 生成包含所有变体信息的 `TypeDescriptor::Enum`
// --- 修改点 2: 同样更新函数文档，说明不支持的类型会 panic ---
///
/// # Panic
///
/// 如果在一个不支持的类型上使用此宏（例如元组结构体或联合体），
/// 宏将 `panic!`，并导致编译失败。这是为了提供即时和明确的错误反馈。
///
/// # Examples
///
/// ```rust
/// // use meta_macros::Schema; // 在实际项目中需要
/// // use serde::Serialize;
///
/// // trait Schema { fn schema() -> &'static str; }
/// // #[derive(Schema)] pub struct User { id: u64, name: String }
/// // #[derive(Schema)] pub enum Status { Active, Inactive }
/// ```
///
/// # 生成的代码
///
/// 该宏会生成类似以下的代码：
///
/// ```rust
/// // impl crate::schema::Schema for User {
/// //     fn schema() -> crate::schema::TypeDescriptor {
/// //         /* ... */
/// //     }
/// // }
/// ```
#[proc_macro_derive(Schema)]
pub fn schema_derive(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream 为 DeriveInput
    let input = syn::parse_macro_input!(input as DeriveInput);

    // 获取类型的标识符（名称）
    let name = &input.ident;
    let name_str = name.to_string();

    // 根据数据类型生成相应的实现
    let schema_impl = match &input.data {
        Data::Struct(data_struct) => {
            // 处理结构体的不同字段类型
            match &data_struct.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    // 为每个命名字段生成 FieldDescriptor
                    let field_descriptors = named.iter().map(|field| {
                        let field_name = field
                            .ident
                            .as_ref()
                            .expect("Fields in a named struct must have an identifier")
                            .to_string();
                        let field_type = &field.ty;

                        quote! {
                            crate::schema::FieldDescriptor {
                                name: #field_name.to_string(),
                                field_type: <#field_type as crate::schema::Schema>::schema(),
                                optional: false, // 简化实现，暂时都设为 false
                            }
                        }
                    });

                    // 生成结构体类型描述符
                    quote! {
                        crate::schema::TypeDescriptor::Struct {
                            name: #name_str.to_string(),
                            fields: vec![#(#field_descriptors),*],
                        }
                    }
                }
                Fields::Unnamed(_) => {
                    panic!(
                        "Deriving `Schema` for tuple structs (e.g., struct MyType(i32);) is not supported."
                    );
                }
                Fields::Unit => {
                    // 单元结构体返回空字段的结构体描述符
                    quote! {
                        crate::schema::TypeDescriptor::Struct {
                            name: #name_str.to_string(),
                            fields: vec![],
                        }
                    }
                }
            }
        }
        Data::Enum(data_enum) => {
            // 处理枚举类型
            let variant_descriptors = data_enum.variants.iter().map(|variant| {
                let variant_name = variant.ident.to_string();

                match &variant.fields {
                    Fields::Named(FieldsNamed { named, .. }) => {
                        // 有命名字段的变体
                        let field_descriptors = named.iter().map(|field| {
                            let field_name = field
                                .ident
                                .as_ref()
                                .expect("Fields in a named enum variant must have an identifier")
                                .to_string();
                            let field_type = &field.ty;

                            quote! {
                                crate::schema::FieldDescriptor {
                                    name: #field_name.to_string(),
                                    field_type: <#field_type as crate::schema::Schema>::schema(),
                                    optional: false,
                                }
                            }
                        });

                        quote! {
                            crate::schema::VariantDescriptor {
                                name: #variant_name.to_string(),
                                fields: Some(vec![#(#field_descriptors),*]),
                            }
                        }
                    }
                    Fields::Unnamed(_) => {
                        panic!("Deriving `Schema` for tuple variants (e.g., enum MyEnum {{ MyVariant(i32) }}) is not supported yet.");
                    }
                    Fields::Unit => {
                        // 单元变体
                        quote! {
                            crate::schema::VariantDescriptor {
                                name: #variant_name.to_string(),
                                fields: None,
                            }
                        }
                    }
                }
            });

            quote! {
                crate::schema::TypeDescriptor::Enum {
                    name: #name_str.to_string(),
                    variants: vec![#(#variant_descriptors),*],
                }
            }
        }
        Data::Union(_) => {
            panic!("Deriving `Schema` for unions is not supported.");
        }
    };

    // 生成最终的 impl 块
    let expanded = quote! {
        impl crate::schema::Schema for #name {
            fn schema() -> crate::schema::TypeDescriptor {
                #schema_impl
            }
        }
    };

    TokenStream::from(expanded)
}
