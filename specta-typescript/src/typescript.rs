use std::{borrow::Cow, io, path::Path};

use specta::{datatype::DeprecatedType, TypeCollection};
use specta_serde::is_valid_ty;

use crate::{comments, ExportError};

#[derive(Debug)]
#[non_exhaustive]
pub struct CommentFormatterArgs<'a> {
    pub docs: &'a Cow<'static, str>,
    pub deprecated: Option<&'a DeprecatedType>,
}

/// The signature for a function responsible for exporting Typescript comments.
pub type CommentFormatterFn = fn(CommentFormatterArgs) -> String; // TODO: Returning `Cow`???

/// The signature for a function responsible for formatter a Typescript file.
pub type FormatterFn = fn(&Path) -> io::Result<()>;

/// Allows you to configure how Specta's Typescript exporter will deal with BigInt types ([i64], [i128] etc).
///
/// WARNING: None of these settings affect how your data is actually ser/deserialized.
/// It's up to you to adjust your ser/deserialize settings.
#[derive(Debug, Clone, Default)]
pub enum BigIntExportBehavior {
    /// Export BigInt as a Typescript `string`
    ///
    /// Doing this is serde is [pretty simple](https://github.com/serde-rs/json/issues/329#issuecomment-305608405).
    String,
    /// Export BigInt as a Typescript `number`.
    ///
    /// WARNING: `JSON.parse` in JS will truncate your number resulting in data loss so ensure your deserializer supports large numbers.
    Number,
    /// Export BigInt as a Typescript `BigInt`.
    BigInt,
    /// Abort the export with an error.
    ///
    /// This is the default behavior because without integration from your serializer and deserializer we can't guarantee data loss won't occur.
    #[default]
    Fail,
    /// Same as `Self::Fail` but it allows a library to configure the message shown to the end user.
    #[doc(hidden)]
    FailWithReason(&'static str),
}

/// Typescript language exporter.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Typescript {
    /// The users file header
    pub header: Cow<'static, str>,
    /// The framework's header
    pub framework_header: Cow<'static, str>,
    /// How BigInts should be exported.
    pub bigint: BigIntExportBehavior,
    /// How comments should be rendered.
    pub comment_exporter: Option<CommentFormatterFn>,
    /// How the resulting file should be formatted.
    pub formatter: Option<FormatterFn>,
}

impl Default for Typescript {
    fn default() -> Self {
        Self {
            header: Cow::Borrowed(""),
            framework_header: Cow::Borrowed(
                "// This file has been generated by Specta. DO NOT EDIT.",
            ),
            bigint: Default::default(),
            comment_exporter: Some(comments::js_doc),
            formatter: None,
        }
    }
}

impl Typescript {
    /// Construct a new Typescript exporter with the default options configured.
    pub fn new() -> Self {
        Default::default()
    }

    /// Override the header for the exported file.
    /// You should prefer `Self::header` instead unless your a framework.
    #[doc(hidden)] // Although this is hidden it's still public API.
    pub fn framework_header(mut self, header: impl Into<Cow<'static, str>>) -> Self {
        self.framework_header = header.into();
        self
    }

    /// Configure a header for the file.
    ///
    /// This is perfect for configuring lint ignore rules or other file-level comments.
    pub fn header(mut self, header: impl Into<Cow<'static, str>>) -> Self {
        self.header = header.into();
        self
    }

    /// Configure the BigInt handling behaviour
    pub fn bigint(mut self, bigint: BigIntExportBehavior) -> Self {
        self.bigint = bigint;
        self
    }

    /// Configure a function which is responsible for styling the comments to be exported
    ///
    /// Implementations:
    ///  - [`js_doc`](specta_typescript::js_doc)
    ///
    /// Not calling this method will default to the [`js_doc`](specta_typescript::js_doc) exporter.
    /// `None` will disable comment exporting.
    /// `Some(exporter)` will enable comment exporting using the provided exporter.
    pub fn comment_style(mut self, exporter: CommentFormatterFn) -> Self {
        self.comment_exporter = Some(exporter);
        self
    }

    /// Configure a function which is responsible for formatting the result file or files
    ///
    ///
    /// Built-in implementations:
    ///  - [`prettier`](crate:formatter:::prettier)
    ///  - [`ESLint`](crate::formatter::eslint)
    ///  - [`Biome`](crate::formatter::biome)e
    pub fn formatter(mut self, formatter: FormatterFn) -> Self {
        self.formatter = Some(formatter);
        self
    }

    /// TODO
    pub fn export(&self, types: &TypeCollection) -> Result<String, ExportError> {
        let mut out = self.header.to_string();
        if !out.is_empty() {
            out.push('\n');
        }
        out += &self.framework_header;
        out.push_str("\n\n");

        // if let Some((ty_name, l0, l1)) = detect_duplicate_type_names(&types).into_iter().next() {
        //     return Err(ExportError::DuplicateTypeName(ty_name, l0, l1));
        // }

        // for (_, ty) in types.into_iter() {
        //     is_valid_ty(&ty.inner, &types)?;

        //     out += &export_named_datatype(self, ty, &types)?;
        //     out += "\n\n";
        // }
        todo!();

        Ok(out)
    }

    /// TODO
    pub fn export_to(
        &self,
        path: impl AsRef<Path>,
        types: &TypeCollection,
    ) -> Result<(), ExportError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(
            &path,
            self.export(types).map(|s| format!("{}{s}", self.header))?,
        )?;
        if let Some(formatter) = self.formatter {
            formatter(path)?;
        }
        Ok(())
    }

    /// TODO
    pub fn format(&self, path: impl AsRef<Path>) -> Result<(), ExportError> {
        if let Some(formatter) = self.formatter {
            formatter(path.as_ref())?;
        }
        Ok(())
    }
}
