//! TODO: Move this somewhere else. Maybe out of core and maybe properly expose?
//!
//! TODO: Option to build types with generics???

use std::{borrow::Cow, fmt::Debug};

use crate::{
    datatype::{
        DeprecatedType, EnumRepr, EnumType, EnumVariant, Field, Fields, List, NamedDataType,
        NamedFields, StructType, UnnamedFields,
    },
    DataType,
};

// TDO: `Debug` and `Clone` on everything

impl List {
    #[doc(hidden)] // TODO: Expose
    pub fn new(ty: DataType) -> Self {
        Self {
            ty: Box::new(ty),
            length: None,
            unique: false,
        }
    }

    #[doc(hidden)] // TODO: Expose
                   // TODO: Should `len` be a `Range` with an upper and lower bound?
    pub fn new_with_len(ty: DataType, len: usize) -> Self {
        Self {
            ty: Box::new(ty),
            length: Some(len),
            unique: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructBuilder<F = ()> {
    name: Cow<'static, str>,
    fields: F,
}

impl StructBuilder<()> {
    pub fn unit(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            fields: (),
        }
    }

    pub fn build(self) -> DataType {
        DataType::Struct(StructType {
            name: self.name,
            sid: None,
            generics: vec![],
            fields: Fields::Unit,
        })
    }
}

impl StructBuilder<NamedFields> {
    pub fn named(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            fields: NamedFields {
                fields: Default::default(),
                tag: Default::default(),
            },
        }
    }

    pub fn field(mut self, name: impl Into<Cow<'static, str>>, field: FieldBuilder) -> Self {
        self.fields.fields.push((name.into(), field.0));
        self
    }

    // TODO: Should this take `FieldBuilder` or `Field`? cause it's inconstent with the rest of this module.
    pub fn field_mut(&mut self, name: impl Into<Cow<'static, str>>, field: FieldBuilder) {
        self.fields.fields.push((name.into(), field.0));
    }

    pub fn build(self) -> DataType {
        DataType::Struct(StructType {
            name: self.name,
            sid: None,
            generics: vec![],
            fields: Fields::Named(self.fields),
        })
    }
}

impl StructBuilder<UnnamedFields> {
    pub fn unnamed(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            fields: UnnamedFields {
                fields: Default::default(),
            },
        }
    }

    pub fn field(mut self, field: FieldBuilder) -> Self {
        self.fields.fields.push(field.0);
        self
    }

    pub fn field_mut(&mut self, field: FieldBuilder) {
        self.fields.fields.push(field.0);
    }

    pub fn build(self) -> DataType {
        DataType::Struct(StructType {
            name: self.name,
            sid: None,
            generics: vec![],
            fields: Fields::Unnamed(self.fields),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FieldBuilder(Field);

impl FieldBuilder {
    pub fn new(ty: DataType) -> Self {
        Self(Field {
            optional: false,
            flatten: false,
            deprecated: None,
            docs: "".into(),
            inline: false,
            ty: Some(ty),
        })
    }

    pub fn skip(mut self) -> Self {
        self.0.ty = None;
        self
    }

    pub fn set_skip(&mut self) {
        self.0.ty = None;
    }

    pub fn optional(mut self) -> Self {
        self.0.optional = true;
        self
    }

    pub fn set_optional(&mut self, optional: bool) {
        self.0.optional = optional;
    }

    pub fn flatten(mut self) -> Self {
        self.0.flatten = true;
        self
    }

    pub fn set_flatten(&mut self, flatten: bool) {
        self.0.flatten = flatten;
    }

    pub fn inline(mut self) -> Self {
        self.0.inline = true;
        self
    }

    pub fn set_inline(&mut self, inline: bool) {
        self.0.inline = inline;
    }

    pub fn deprecated(mut self, reason: DeprecatedType) -> Self {
        self.0.deprecated = Some(reason);
        self
    }

    pub fn set_deprecated(&mut self, reason: DeprecatedType) {
        self.0.deprecated = Some(reason);
    }

    pub fn docs(mut self, docs: impl Into<Cow<'static, str>>) -> Self {
        self.0.docs = docs.into();
        self
    }

    pub fn set_docs(&mut self, docs: impl Into<Cow<'static, str>>) {
        self.0.docs = docs.into();
    }

    pub fn build(self) -> Field {
        self.0
    }
}

pub struct EnumBuilder {
    name: Cow<'static, str>,
    repr: EnumRepr,
    variants: Vec<(Cow<'static, str>, EnumVariant)>,
}

impl EnumBuilder {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            repr: EnumRepr::External,
            variants: vec![],
        }
    }

    pub fn repr(mut self, repr: EnumRepr) -> Self {
        self.repr = repr;
        self
    }

    // TODO: Configurable `repr`

    pub fn variant(mut self, name: impl Into<Cow<'static, str>>, v: EnumVariant) -> Self {
        self.variants.push((name.into(), v));
        self
    }

    pub fn variant_mut(&mut self, name: impl Into<Cow<'static, str>>, v: EnumVariant) {
        self.variants.push((name.into(), v));
    }

    pub fn build(self) -> DataType {
        DataType::Enum(EnumType {
            name: self.name,
            sid: None,
            skip_bigint_checks: false,
            repr: self.repr,
            generics: Default::default(),
            variants: self.variants,
        })
    }
}

#[derive(Debug, Clone)]
pub struct VariantBuilder<V = ()> {
    skip: bool,
    docs: Cow<'static, str>,
    deprecated: Option<DeprecatedType>,
    fields: V,
}

impl VariantBuilder<()> {
    pub fn unit() -> Self {
        Self {
            skip: false,
            docs: "".into(),
            deprecated: None,
            fields: (),
        }
    }

    pub fn skip(mut self) -> Self {
        self.skip = true;
        self
    }

    pub fn docs(mut self, docs: Cow<'static, str>) -> Self {
        self.docs = docs;
        self
    }

    pub fn deprecated(mut self, reason: DeprecatedType) -> Self {
        self.deprecated = Some(reason);
        self
    }

    pub fn build(self) -> EnumVariant {
        EnumVariant {
            skip: self.skip,
            docs: self.docs,
            deprecated: self.deprecated,
            fields: Fields::Unit,
        }
    }
}

impl Into<EnumVariant> for VariantBuilder<()> {
    fn into(self) -> EnumVariant {
        self.build()
    }
}

impl VariantBuilder<NamedFields> {
    pub fn named() -> Self {
        Self {
            skip: false,
            docs: "".into(),
            deprecated: None,
            fields: NamedFields {
                fields: Default::default(),
                // TODO: Configurable
                tag: None,
            },
        }
    }

    pub fn skip(mut self) -> Self {
        self.skip = true;
        self
    }

    pub fn docs(mut self, docs: Cow<'static, str>) -> Self {
        self.docs = docs;
        self
    }

    pub fn deprecated(mut self, reason: DeprecatedType) -> Self {
        self.deprecated = Some(reason);
        self
    }

    pub fn field(mut self, name: impl Into<Cow<'static, str>>, field: Field) -> Self {
        self.fields.fields.push((name.into(), field));
        self
    }

    pub fn field_mut(mut self, name: impl Into<Cow<'static, str>>, field: Field) -> Self {
        self.fields.fields.push((name.into(), field));
        self
    }

    pub fn build(self) -> EnumVariant {
        EnumVariant {
            skip: self.skip,
            docs: self.docs,
            deprecated: self.deprecated,
            fields: Fields::Named(self.fields),
        }
    }
}

impl Into<EnumVariant> for VariantBuilder<NamedFields> {
    fn into(self) -> EnumVariant {
        self.build()
    }
}

impl VariantBuilder<UnnamedFields> {
    pub fn unnamed() -> Self {
        Self {
            skip: false,
            docs: "".into(),
            deprecated: None,
            fields: UnnamedFields {
                fields: Default::default(),
            },
        }
    }

    pub fn skip(mut self) -> Self {
        self.skip = true;
        self
    }

    pub fn docs(mut self, docs: Cow<'static, str>) -> Self {
        self.docs = docs;
        self
    }

    pub fn deprecated(mut self, reason: DeprecatedType) -> Self {
        self.deprecated = Some(reason);
        self
    }

    pub fn field(mut self, field: Field) -> Self {
        self.fields.fields.push(field);
        self
    }

    pub fn field_mut(mut self, field: Field) -> Self {
        self.fields.fields.push(field);
        self
    }

    pub fn build(self) -> EnumVariant {
        EnumVariant {
            skip: self.skip,
            docs: self.docs,
            deprecated: self.deprecated,
            fields: Fields::Unnamed(self.fields),
        }
    }
}

impl Into<EnumVariant> for VariantBuilder<UnnamedFields> {
    fn into(self) -> EnumVariant {
        self.build()
    }
}

pub struct NamedDataTypeBuilder(NamedDataType);

impl NamedDataTypeBuilder {
    // TODO: Taking `name` is super wierd with enums/structs which *also* have a name on the `Builder::new` method
    pub fn new(name: impl Into<Cow<'static, str>>, dt: DataType) -> Self {
        Self(NamedDataType {
            name: name.into(),
            docs: "".into(),
            deprecated: None,
            ext: None,
            inner: dt,
        })
    }

    pub fn docs(mut self, docs: impl Into<Cow<'static, str>>) -> Self {
        self.0.docs = docs.into();
        self
    }

    pub fn deprecated(mut self, deprecated: DeprecatedType) -> Self {
        self.0.deprecated = Some(deprecated);
        self
    }

    // pub fn ext(mut self, ext: impl Into<Cow<'static, str>>) -> Self {
    //     self.0.ext = Some(ext.into());
    //     self
    // }

    pub fn build(self) -> NamedDataType {
        self.0
    }
}
