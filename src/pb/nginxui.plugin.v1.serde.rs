impl serde::Serialize for BlocklistEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cidr.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.BlocklistEntry", len)?;
        if !self.cidr.is_empty() {
            struct_ser.serialize_field("cidr", &self.cidr)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlocklistEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["cidr", "reason"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cidr,
            Reason,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cidr" => Ok(GeneratedField::Cidr),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlocklistEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.BlocklistEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlocklistEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cidr__ = None;
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cidr => {
                            if cidr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cidr"));
                            }
                            cidr__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(BlocklistEntry {
                    cidr: cidr__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.BlocklistEntry",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for BlocklistFetchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.BlocklistFetchRequest", len)?;
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlocklistFetchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["source", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Config,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "source" => Ok(GeneratedField::Source),
                            "config" => Ok(GeneratedField::Config),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlocklistFetchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.BlocklistFetchRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BlocklistFetchRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(BlocklistFetchRequest {
                    source: source__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.BlocklistFetchRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for BlocklistFetchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        if self.ttl_seconds != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.BlocklistFetchResponse", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if self.ttl_seconds != 0 {
            struct_ser.serialize_field("ttl_seconds", &self.ttl_seconds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlocklistFetchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["entries", "ttl_seconds", "ttlSeconds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
            TtlSeconds,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "entries" => Ok(GeneratedField::Entries),
                            "ttlSeconds" | "ttl_seconds" => Ok(GeneratedField::TtlSeconds),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlocklistFetchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.BlocklistFetchResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<BlocklistFetchResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                let mut ttl_seconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TtlSeconds => {
                            if ttl_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttlSeconds"));
                            }
                            ttl_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(BlocklistFetchResponse {
                    entries: entries__.unwrap_or_default(),
                    ttl_seconds: ttl_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.BlocklistFetchResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for BlocklistSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.refresh_seconds != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.BlocklistSource", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if self.refresh_seconds != 0 {
            struct_ser.serialize_field("refresh_seconds", &self.refresh_seconds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlocklistSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "name",
            "configuration",
            "refresh_seconds",
            "refreshSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Name,
            Configuration,
            RefreshSeconds,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "name" => Ok(GeneratedField::Name),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "refreshSeconds" | "refresh_seconds" => {
                                Ok(GeneratedField::RefreshSeconds)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlocklistSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.BlocklistSource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlocklistSource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut name__ = None;
                let mut configuration__ = None;
                let mut refresh_seconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::RefreshSeconds => {
                            if refresh_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshSeconds"));
                            }
                            refresh_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(BlocklistSource {
                    code: code__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    configuration: configuration__,
                    refresh_seconds: refresh_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.BlocklistSource",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ConfigurationField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.help_text.is_empty() {
            len += 1;
        }
        if self.required {
            len += 1;
        }
        if self.secret {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ConfigurationField", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("display_name", &self.display_name)?;
        }
        if !self.help_text.is_empty() {
            struct_ser.serialize_field("help_text", &self.help_text)?;
        }
        if self.required {
            struct_ser.serialize_field("required", &self.required)?;
        }
        if self.secret {
            struct_ser.serialize_field("secret", &self.secret)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigurationField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "type",
            "display_name",
            "displayName",
            "help_text",
            "helpText",
            "required",
            "secret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Type,
            DisplayName,
            HelpText,
            Required,
            Secret,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "type" => Ok(GeneratedField::Type),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "helpText" | "help_text" => Ok(GeneratedField::HelpText),
                            "required" => Ok(GeneratedField::Required),
                            "secret" => Ok(GeneratedField::Secret),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigurationField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ConfigurationField")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConfigurationField, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut r#type__ = None;
                let mut display_name__ = None;
                let mut help_text__ = None;
                let mut required__ = None;
                let mut secret__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HelpText => {
                            if help_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("helpText"));
                            }
                            help_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ConfigurationField {
                    key: key__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    help_text: help_text__.unwrap_or_default(),
                    required: required__.unwrap_or_default(),
                    secret: secret__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ConfigurationField",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ConfigurationSchema {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fields.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ConfigurationSchema", len)?;
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigurationSchema {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["fields"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fields,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fields" => Ok(GeneratedField::Fields),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigurationSchema;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ConfigurationSchema")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConfigurationSchema, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ConfigurationSchema {
                    fields: fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ConfigurationSchema",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01CheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.fqdn.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.key_auth.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01CheckRequest", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.fqdn.is_empty() {
            struct_ser.serialize_field("fqdn", &self.fqdn)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.key_auth.is_empty() {
            struct_ser.serialize_field("key_auth", &self.key_auth)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01CheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider", "config", "options", "domain", "fqdn", "value", "key_auth", "keyAuth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Config,
            Options,
            Domain,
            Fqdn,
            Value,
            KeyAuth,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "config" => Ok(GeneratedField::Config),
                            "options" => Ok(GeneratedField::Options),
                            "domain" => Ok(GeneratedField::Domain),
                            "fqdn" => Ok(GeneratedField::Fqdn),
                            "value" => Ok(GeneratedField::Value),
                            "keyAuth" | "key_auth" => Ok(GeneratedField::KeyAuth),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01CheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01CheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01CheckRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut config__ = None;
                let mut options__ = None;
                let mut domain__ = None;
                let mut fqdn__ = None;
                let mut value__ = None;
                let mut key_auth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqdn => {
                            if fqdn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqdn"));
                            }
                            fqdn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyAuth => {
                            if key_auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAuth"));
                            }
                            key_auth__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01CheckRequest {
                    provider: provider__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    options: options__,
                    domain: domain__.unwrap_or_default(),
                    fqdn: fqdn__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    key_auth: key_auth__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01CheckRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01CheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ready {
            len += 1;
        }
        if !self.effective_fqdn.is_empty() {
            len += 1;
        }
        if !self.detail.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01CheckResponse", len)?;
        if self.ready {
            struct_ser.serialize_field("ready", &self.ready)?;
        }
        if !self.effective_fqdn.is_empty() {
            struct_ser.serialize_field("effective_fqdn", &self.effective_fqdn)?;
        }
        if !self.detail.is_empty() {
            struct_ser.serialize_field("detail", &self.detail)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01CheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ready", "effective_fqdn", "effectiveFqdn", "detail"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ready,
            EffectiveFqdn,
            Detail,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ready" => Ok(GeneratedField::Ready),
                            "effectiveFqdn" | "effective_fqdn" => Ok(GeneratedField::EffectiveFqdn),
                            "detail" => Ok(GeneratedField::Detail),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01CheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01CheckResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01CheckResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ready__ = None;
                let mut effective_fqdn__ = None;
                let mut detail__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ready => {
                            if ready__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ready"));
                            }
                            ready__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EffectiveFqdn => {
                            if effective_fqdn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveFqdn"));
                            }
                            effective_fqdn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Detail => {
                            if detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            detail__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01CheckResponse {
                    ready: ready__.unwrap_or_default(),
                    effective_fqdn: effective_fqdn__.unwrap_or_default(),
                    detail: detail__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01CheckResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01CleanupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.fqdn.is_empty() {
            len += 1;
        }
        if !self.effective_fqdn.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.token.is_empty() {
            len += 1;
        }
        if !self.key_auth.is_empty() {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01CleanupRequest", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.fqdn.is_empty() {
            struct_ser.serialize_field("fqdn", &self.fqdn)?;
        }
        if !self.effective_fqdn.is_empty() {
            struct_ser.serialize_field("effective_fqdn", &self.effective_fqdn)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        if !self.key_auth.is_empty() {
            struct_ser.serialize_field("key_auth", &self.key_auth)?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dry_run", &self.dry_run)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01CleanupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
            "config",
            "options",
            "domain",
            "fqdn",
            "effective_fqdn",
            "effectiveFqdn",
            "value",
            "token",
            "key_auth",
            "keyAuth",
            "dry_run",
            "dryRun",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Config,
            Options,
            Domain,
            Fqdn,
            EffectiveFqdn,
            Value,
            Token,
            KeyAuth,
            DryRun,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "config" => Ok(GeneratedField::Config),
                            "options" => Ok(GeneratedField::Options),
                            "domain" => Ok(GeneratedField::Domain),
                            "fqdn" => Ok(GeneratedField::Fqdn),
                            "effectiveFqdn" | "effective_fqdn" => Ok(GeneratedField::EffectiveFqdn),
                            "value" => Ok(GeneratedField::Value),
                            "token" => Ok(GeneratedField::Token),
                            "keyAuth" | "key_auth" => Ok(GeneratedField::KeyAuth),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01CleanupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01CleanupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01CleanupRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut config__ = None;
                let mut options__ = None;
                let mut domain__ = None;
                let mut fqdn__ = None;
                let mut effective_fqdn__ = None;
                let mut value__ = None;
                let mut token__ = None;
                let mut key_auth__ = None;
                let mut dry_run__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqdn => {
                            if fqdn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqdn"));
                            }
                            fqdn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EffectiveFqdn => {
                            if effective_fqdn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveFqdn"));
                            }
                            effective_fqdn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyAuth => {
                            if key_auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAuth"));
                            }
                            key_auth__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01CleanupRequest {
                    provider: provider__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    options: options__,
                    domain: domain__.unwrap_or_default(),
                    fqdn: fqdn__.unwrap_or_default(),
                    effective_fqdn: effective_fqdn__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    token: token__.unwrap_or_default(),
                    key_auth: key_auth__.unwrap_or_default(),
                    dry_run: dry_run__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01CleanupRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01CleanupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01CleanupResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01CleanupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01CleanupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01CleanupResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<Dns01CleanupResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Dns01CleanupResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01CleanupResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01OptionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01OptionsRequest", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01OptionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider", "config", "options"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Config,
            Options,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "config" => Ok(GeneratedField::Config),
                            "options" => Ok(GeneratedField::Options),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01OptionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01OptionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01OptionsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut config__ = None;
                let mut options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01OptionsRequest {
                    provider: provider__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01OptionsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01OptionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.propagation_timeout_seconds != 0 {
            len += 1;
        }
        if self.polling_interval_seconds != 0 {
            len += 1;
        }
        if self.sequential_interval_seconds != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01OptionsResponse", len)?;
        if self.propagation_timeout_seconds != 0 {
            struct_ser.serialize_field(
                "propagation_timeout_seconds",
                &self.propagation_timeout_seconds,
            )?;
        }
        if self.polling_interval_seconds != 0 {
            struct_ser
                .serialize_field("polling_interval_seconds", &self.polling_interval_seconds)?;
        }
        if self.sequential_interval_seconds != 0 {
            struct_ser.serialize_field(
                "sequential_interval_seconds",
                &self.sequential_interval_seconds,
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01OptionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "propagation_timeout_seconds",
            "propagationTimeoutSeconds",
            "polling_interval_seconds",
            "pollingIntervalSeconds",
            "sequential_interval_seconds",
            "sequentialIntervalSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PropagationTimeoutSeconds,
            PollingIntervalSeconds,
            SequentialIntervalSeconds,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "propagationTimeoutSeconds" | "propagation_timeout_seconds" => {
                                Ok(GeneratedField::PropagationTimeoutSeconds)
                            }
                            "pollingIntervalSeconds" | "polling_interval_seconds" => {
                                Ok(GeneratedField::PollingIntervalSeconds)
                            }
                            "sequentialIntervalSeconds" | "sequential_interval_seconds" => {
                                Ok(GeneratedField::SequentialIntervalSeconds)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01OptionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01OptionsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<Dns01OptionsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut propagation_timeout_seconds__ = None;
                let mut polling_interval_seconds__ = None;
                let mut sequential_interval_seconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PropagationTimeoutSeconds => {
                            if propagation_timeout_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "propagationTimeoutSeconds",
                                ));
                            }
                            propagation_timeout_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PollingIntervalSeconds => {
                            if polling_interval_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "pollingIntervalSeconds",
                                ));
                            }
                            polling_interval_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SequentialIntervalSeconds => {
                            if sequential_interval_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sequentialIntervalSeconds",
                                ));
                            }
                            sequential_interval_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01OptionsResponse {
                    propagation_timeout_seconds: propagation_timeout_seconds__.unwrap_or_default(),
                    polling_interval_seconds: polling_interval_seconds__.unwrap_or_default(),
                    sequential_interval_seconds: sequential_interval_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01OptionsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01PresentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.fqdn.is_empty() {
            len += 1;
        }
        if !self.effective_fqdn.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.token.is_empty() {
            len += 1;
        }
        if !self.key_auth.is_empty() {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01PresentRequest", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.fqdn.is_empty() {
            struct_ser.serialize_field("fqdn", &self.fqdn)?;
        }
        if !self.effective_fqdn.is_empty() {
            struct_ser.serialize_field("effective_fqdn", &self.effective_fqdn)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        if !self.key_auth.is_empty() {
            struct_ser.serialize_field("key_auth", &self.key_auth)?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dry_run", &self.dry_run)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01PresentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider",
            "config",
            "options",
            "domain",
            "fqdn",
            "effective_fqdn",
            "effectiveFqdn",
            "value",
            "token",
            "key_auth",
            "keyAuth",
            "dry_run",
            "dryRun",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Config,
            Options,
            Domain,
            Fqdn,
            EffectiveFqdn,
            Value,
            Token,
            KeyAuth,
            DryRun,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "config" => Ok(GeneratedField::Config),
                            "options" => Ok(GeneratedField::Options),
                            "domain" => Ok(GeneratedField::Domain),
                            "fqdn" => Ok(GeneratedField::Fqdn),
                            "effectiveFqdn" | "effective_fqdn" => Ok(GeneratedField::EffectiveFqdn),
                            "value" => Ok(GeneratedField::Value),
                            "token" => Ok(GeneratedField::Token),
                            "keyAuth" | "key_auth" => Ok(GeneratedField::KeyAuth),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01PresentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01PresentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01PresentRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut config__ = None;
                let mut options__ = None;
                let mut domain__ = None;
                let mut fqdn__ = None;
                let mut effective_fqdn__ = None;
                let mut value__ = None;
                let mut token__ = None;
                let mut key_auth__ = None;
                let mut dry_run__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqdn => {
                            if fqdn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqdn"));
                            }
                            fqdn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EffectiveFqdn => {
                            if effective_fqdn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveFqdn"));
                            }
                            effective_fqdn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyAuth => {
                            if key_auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAuth"));
                            }
                            key_auth__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01PresentRequest {
                    provider: provider__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    options: options__,
                    domain: domain__.unwrap_or_default(),
                    fqdn: fqdn__.unwrap_or_default(),
                    effective_fqdn: effective_fqdn__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    token: token__.unwrap_or_default(),
                    key_auth: key_auth__.unwrap_or_default(),
                    dry_run: dry_run__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01PresentRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01PresentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01PresentResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01PresentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01PresentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01PresentResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<Dns01PresentResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Dns01PresentResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01PresentResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01Provider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if self.links.is_some() {
            len += 1;
        }
        if self.propagation_timeout_seconds != 0 {
            len += 1;
        }
        if self.polling_interval_seconds != 0 {
            len += 1;
        }
        if self.form.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.DNS01Provider", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if let Some(v) = self.links.as_ref() {
            struct_ser.serialize_field("links", v)?;
        }
        if self.propagation_timeout_seconds != 0 {
            struct_ser.serialize_field(
                "propagation_timeout_seconds",
                &self.propagation_timeout_seconds,
            )?;
        }
        if self.polling_interval_seconds != 0 {
            struct_ser
                .serialize_field("polling_interval_seconds", &self.polling_interval_seconds)?;
        }
        if let Some(v) = self.form.as_ref() {
            struct_ser.serialize_field("form", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01Provider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "code",
            "links",
            "propagation_timeout_seconds",
            "propagationTimeoutSeconds",
            "polling_interval_seconds",
            "pollingIntervalSeconds",
            "form",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Code,
            Links,
            PropagationTimeoutSeconds,
            PollingIntervalSeconds,
            Form,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "code" => Ok(GeneratedField::Code),
                            "links" => Ok(GeneratedField::Links),
                            "propagationTimeoutSeconds" | "propagation_timeout_seconds" => {
                                Ok(GeneratedField::PropagationTimeoutSeconds)
                            }
                            "pollingIntervalSeconds" | "polling_interval_seconds" => {
                                Ok(GeneratedField::PollingIntervalSeconds)
                            }
                            "form" => Ok(GeneratedField::Form),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01Provider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01Provider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01Provider, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut code__ = None;
                let mut links__ = None;
                let mut propagation_timeout_seconds__ = None;
                let mut polling_interval_seconds__ = None;
                let mut form__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Links => {
                            if links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("links"));
                            }
                            links__ = map_.next_value()?;
                        }
                        GeneratedField::PropagationTimeoutSeconds => {
                            if propagation_timeout_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "propagationTimeoutSeconds",
                                ));
                            }
                            propagation_timeout_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PollingIntervalSeconds => {
                            if polling_interval_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "pollingIntervalSeconds",
                                ));
                            }
                            polling_interval_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Form => {
                            if form__.is_some() {
                                return Err(serde::de::Error::duplicate_field("form"));
                            }
                            form__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01Provider {
                    name: name__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    links: links__,
                    propagation_timeout_seconds: propagation_timeout_seconds__.unwrap_or_default(),
                    polling_interval_seconds: polling_interval_seconds__.unwrap_or_default(),
                    form: form__,
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.DNS01Provider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Dns01ProviderField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if !self.help.is_empty() {
            len += 1;
        }
        if !self.group.is_empty() {
            len += 1;
        }
        if self.optional {
            len += 1;
        }
        if self.secret {
            len += 1;
        }
        if !self.default.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if !self.link.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01ProviderField", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if !self.help.is_empty() {
            struct_ser.serialize_field("help", &self.help)?;
        }
        if !self.group.is_empty() {
            struct_ser.serialize_field("group", &self.group)?;
        }
        if self.optional {
            struct_ser.serialize_field("optional", &self.optional)?;
        }
        if self.secret {
            struct_ser.serialize_field("secret", &self.secret)?;
        }
        if !self.default.is_empty() {
            struct_ser.serialize_field("default", &self.default)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.link.is_empty() {
            struct_ser.serialize_field("link", &self.link)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01ProviderField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key", "label", "help", "group", "optional", "secret", "default", "unit", "link",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Label,
            Help,
            Group,
            Optional,
            Secret,
            Default,
            Unit,
            Link,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "label" => Ok(GeneratedField::Label),
                            "help" => Ok(GeneratedField::Help),
                            "group" => Ok(GeneratedField::Group),
                            "optional" => Ok(GeneratedField::Optional),
                            "secret" => Ok(GeneratedField::Secret),
                            "default" => Ok(GeneratedField::Default),
                            "unit" => Ok(GeneratedField::Unit),
                            "link" => Ok(GeneratedField::Link),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01ProviderField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01ProviderField")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01ProviderField, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut label__ = None;
                let mut help__ = None;
                let mut group__ = None;
                let mut optional__ = None;
                let mut secret__ = None;
                let mut default__ = None;
                let mut unit__ = None;
                let mut link__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Help => {
                            if help__.is_some() {
                                return Err(serde::de::Error::duplicate_field("help"));
                            }
                            help__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Optional => {
                            if optional__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optional"));
                            }
                            optional__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Default => {
                            if default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("default"));
                            }
                            default__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Link => {
                            if link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01ProviderField {
                    key: key__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    help: help__.unwrap_or_default(),
                    group: group__.unwrap_or_default(),
                    optional: optional__.unwrap_or_default(),
                    secret: secret__.unwrap_or_default(),
                    default: default__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    link: link__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01ProviderField",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01ProviderForm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fields.is_empty() {
            len += 1;
        }
        if !self.methods.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01ProviderForm", len)?;
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        if !self.methods.is_empty() {
            struct_ser.serialize_field("methods", &self.methods)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01ProviderForm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["fields", "methods"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fields,
            Methods,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fields" => Ok(GeneratedField::Fields),
                            "methods" => Ok(GeneratedField::Methods),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01ProviderForm;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01ProviderForm")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01ProviderForm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut fields__ = None;
                let mut methods__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Methods => {
                            if methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methods"));
                            }
                            methods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01ProviderForm {
                    fields: fields__.unwrap_or_default(),
                    methods: methods__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01ProviderForm",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01ProviderLinks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.api.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01ProviderLinks", len)?;
        if !self.api.is_empty() {
            struct_ser.serialize_field("api", &self.api)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01ProviderLinks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["api"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Api,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "api" => Ok(GeneratedField::Api),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01ProviderLinks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01ProviderLinks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01ProviderLinks, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut api__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Api => {
                            if api__.is_some() {
                                return Err(serde::de::Error::duplicate_field("api"));
                            }
                            api__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01ProviderLinks {
                    api: api__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01ProviderLinks",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01ProviderMethod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.recommended {
            len += 1;
        }
        if !self.fields.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01ProviderMethod", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.recommended {
            struct_ser.serialize_field("recommended", &self.recommended)?;
        }
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01ProviderMethod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name", "recommended", "fields", "values"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Recommended,
            Fields,
            Values,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "recommended" => Ok(GeneratedField::Recommended),
                            "fields" => Ok(GeneratedField::Fields),
                            "values" => Ok(GeneratedField::Values),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01ProviderMethod;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01ProviderMethod")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Dns01ProviderMethod, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut recommended__ = None;
                let mut fields__ = None;
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recommended => {
                            if recommended__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recommended"));
                            }
                            recommended__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01ProviderMethod {
                    name: name__.unwrap_or_default(),
                    recommended: recommended__.unwrap_or_default(),
                    fields: fields__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01ProviderMethod",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01ValidateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01ValidateRequest", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01ValidateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Config,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "config" => Ok(GeneratedField::Config),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01ValidateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01ValidateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<Dns01ValidateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Dns01ValidateRequest {
                    provider: provider__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01ValidateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for Dns01ValidateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DNS01ValidateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dns01ValidateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dns01ValidateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DNS01ValidateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<Dns01ValidateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Dns01ValidateResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DNS01ValidateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DeployCertificate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.domains.is_empty() {
            len += 1;
        }
        if !self.certificate_pem.is_empty() {
            len += 1;
        }
        if !self.private_key_pem.is_empty() {
            len += 1;
        }
        if !self.chain_pem.is_empty() {
            len += 1;
        }
        if !self.not_after.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DeployCertificate", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.domains.is_empty() {
            struct_ser.serialize_field("domains", &self.domains)?;
        }
        if !self.certificate_pem.is_empty() {
            struct_ser.serialize_field("certificate_pem", &self.certificate_pem)?;
        }
        if !self.private_key_pem.is_empty() {
            struct_ser.serialize_field("private_key_pem", &self.private_key_pem)?;
        }
        if !self.chain_pem.is_empty() {
            struct_ser.serialize_field("chain_pem", &self.chain_pem)?;
        }
        if !self.not_after.is_empty() {
            struct_ser.serialize_field("not_after", &self.not_after)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeployCertificate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "domains",
            "certificate_pem",
            "certificatePem",
            "private_key_pem",
            "privateKeyPem",
            "chain_pem",
            "chainPem",
            "not_after",
            "notAfter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Domains,
            CertificatePem,
            PrivateKeyPem,
            ChainPem,
            NotAfter,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "domains" => Ok(GeneratedField::Domains),
                            "certificatePem" | "certificate_pem" => {
                                Ok(GeneratedField::CertificatePem)
                            }
                            "privateKeyPem" | "private_key_pem" => {
                                Ok(GeneratedField::PrivateKeyPem)
                            }
                            "chainPem" | "chain_pem" => Ok(GeneratedField::ChainPem),
                            "notAfter" | "not_after" => Ok(GeneratedField::NotAfter),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeployCertificate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DeployCertificate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeployCertificate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut domains__ = None;
                let mut certificate_pem__ = None;
                let mut private_key_pem__ = None;
                let mut chain_pem__ = None;
                let mut not_after__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domains => {
                            if domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domains"));
                            }
                            domains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CertificatePem => {
                            if certificate_pem__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificatePem"));
                            }
                            certificate_pem__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivateKeyPem => {
                            if private_key_pem__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKeyPem"));
                            }
                            private_key_pem__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChainPem => {
                            if chain_pem__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainPem"));
                            }
                            chain_pem__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotAfter => {
                            if not_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notAfter"));
                            }
                            not_after__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeployCertificate {
                    name: name__.unwrap_or_default(),
                    domains: domains__.unwrap_or_default(),
                    certificate_pem: certificate_pem__.unwrap_or_default(),
                    private_key_pem: private_key_pem__.unwrap_or_default(),
                    chain_pem: chain_pem__.unwrap_or_default(),
                    not_after: not_after__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DeployCertificate",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DeployPushRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if self.certificate.is_some() {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DeployPushRequest", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if let Some(v) = self.certificate.as_ref() {
            struct_ser.serialize_field("certificate", v)?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dry_run", &self.dry_run)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeployPushRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["kind", "config", "certificate", "dry_run", "dryRun"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Config,
            Certificate,
            DryRun,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "config" => Ok(GeneratedField::Config),
                            "certificate" => Ok(GeneratedField::Certificate),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeployPushRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DeployPushRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeployPushRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut config__ = None;
                let mut certificate__ = None;
                let mut dry_run__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Certificate => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificate"));
                            }
                            certificate__ = map_.next_value()?;
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeployPushRequest {
                    kind: kind__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    certificate: certificate__,
                    dry_run: dry_run__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DeployPushRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DeployPushResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DeployPushResponse", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeployPushResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["message"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeployPushResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DeployPushResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeployPushResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeployPushResponse {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DeployPushResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DeployTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.DeployTarget", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeployTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code", "name", "configuration"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Name,
            Configuration,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "name" => Ok(GeneratedField::Name),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeployTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DeployTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeployTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut name__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeployTarget {
                    code: code__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.DeployTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeployValidateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DeployValidateRequest", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeployValidateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["kind", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Config,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "config" => Ok(GeneratedField::Config),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeployValidateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DeployValidateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<DeployValidateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeployValidateRequest {
                    kind: kind__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DeployValidateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DeployValidateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DeployValidateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeployValidateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeployValidateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DeployValidateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<DeployValidateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeployValidateResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DeployValidateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DiscoveryProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DiscoveryProvider", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoveryProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code", "name", "configuration"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Name,
            Configuration,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "name" => Ok(GeneratedField::Name),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoveryProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DiscoveryProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiscoveryProvider, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut name__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DiscoveryProvider {
                    code: code__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DiscoveryProvider",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DiscoveryResolveRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if !self.service.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DiscoveryResolveRequest", len)?;
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoveryResolveRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["provider", "config", "service"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Provider,
            Config,
            Service,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "provider" => Ok(GeneratedField::Provider),
                            "config" => Ok(GeneratedField::Config),
                            "service" => Ok(GeneratedField::Service),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoveryResolveRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DiscoveryResolveRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<DiscoveryResolveRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut provider__ = None;
                let mut config__ = None;
                let mut service__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DiscoveryResolveRequest {
                    provider: provider__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    service: service__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DiscoveryResolveRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DiscoveryResolveResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.targets.is_empty() {
            len += 1;
        }
        if self.ttl_seconds != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DiscoveryResolveResponse", len)?;
        if !self.targets.is_empty() {
            struct_ser.serialize_field("targets", &self.targets)?;
        }
        if self.ttl_seconds != 0 {
            struct_ser.serialize_field("ttl_seconds", &self.ttl_seconds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoveryResolveResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["targets", "ttl_seconds", "ttlSeconds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Targets,
            TtlSeconds,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "targets" => Ok(GeneratedField::Targets),
                            "ttlSeconds" | "ttl_seconds" => Ok(GeneratedField::TtlSeconds),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoveryResolveResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DiscoveryResolveResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<DiscoveryResolveResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut targets__ = None;
                let mut ttl_seconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Targets => {
                            if targets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targets"));
                            }
                            targets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TtlSeconds => {
                            if ttl_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttlSeconds"));
                            }
                            ttl_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DiscoveryResolveResponse {
                    targets: targets__.unwrap_or_default(),
                    ttl_seconds: ttl_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DiscoveryResolveResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for DiscoveryTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.port != 0 {
            len += 1;
        }
        if self.weight != 0 {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.DiscoveryTarget", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.port != 0 {
            struct_ser.serialize_field("port", &self.port)?;
        }
        if self.weight != 0 {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoveryTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "port", "weight", "tags"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Port,
            Weight,
            Tags,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "port" => Ok(GeneratedField::Port),
                            "weight" => Ok(GeneratedField::Weight),
                            "tags" => Ok(GeneratedField::Tags),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoveryTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.DiscoveryTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiscoveryTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut port__ = None;
                let mut weight__ = None;
                let mut tags__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DiscoveryTarget {
                    address: address__.unwrap_or_default(),
                    port: port__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.DiscoveryTarget",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ERROR_CODE_UNSPECIFIED",
            Self::ParseError => "ERROR_CODE_PARSE_ERROR",
            Self::InvalidRequest => "ERROR_CODE_INVALID_REQUEST",
            Self::MethodNotFound => "ERROR_CODE_METHOD_NOT_FOUND",
            Self::InvalidParams => "ERROR_CODE_INVALID_PARAMS",
            Self::InternalError => "ERROR_CODE_INTERNAL_ERROR",
            Self::PermissionDenied => "ERROR_CODE_PERMISSION_DENIED",
            Self::Unsupported => "ERROR_CODE_UNSUPPORTED",
            Self::InvalidConfig => "ERROR_CODE_INVALID_CONFIG",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_CODE_UNSPECIFIED",
            "ERROR_CODE_PARSE_ERROR",
            "ERROR_CODE_INVALID_REQUEST",
            "ERROR_CODE_METHOD_NOT_FOUND",
            "ERROR_CODE_INVALID_PARAMS",
            "ERROR_CODE_INTERNAL_ERROR",
            "ERROR_CODE_PERMISSION_DENIED",
            "ERROR_CODE_UNSUPPORTED",
            "ERROR_CODE_INVALID_CONFIG",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = ErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ERROR_CODE_UNSPECIFIED" => Ok(ErrorCode::Unspecified),
                    "ERROR_CODE_PARSE_ERROR" => Ok(ErrorCode::ParseError),
                    "ERROR_CODE_INVALID_REQUEST" => Ok(ErrorCode::InvalidRequest),
                    "ERROR_CODE_METHOD_NOT_FOUND" => Ok(ErrorCode::MethodNotFound),
                    "ERROR_CODE_INVALID_PARAMS" => Ok(ErrorCode::InvalidParams),
                    "ERROR_CODE_INTERNAL_ERROR" => Ok(ErrorCode::InternalError),
                    "ERROR_CODE_PERMISSION_DENIED" => Ok(ErrorCode::PermissionDenied),
                    "ERROR_CODE_UNSUPPORTED" => Ok(ErrorCode::Unsupported),
                    "ERROR_CODE_INVALID_CONFIG" => Ok(ErrorCode::InvalidConfig),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EventsOnRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        if self.ts != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.EventsOnRequest", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if self.ts != 0 {
            struct_ser.serialize_field("ts", &self.ts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventsOnRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["type", "data", "ts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Data,
            Ts,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "data" => Ok(GeneratedField::Data),
                            "ts" => Ok(GeneratedField::Ts),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventsOnRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.EventsOnRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventsOnRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut data__ = None;
                let mut ts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                        GeneratedField::Ts => {
                            if ts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ts"));
                            }
                            ts__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EventsOnRequest {
                    r#type: r#type__.unwrap_or_default(),
                    data: data__,
                    ts: ts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.EventsOnRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for EventsOnResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nginxui.plugin.v1.EventsOnResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventsOnResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventsOnResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.EventsOnResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventsOnResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(EventsOnResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.EventsOnResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HttpHandleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.method.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.body_base64.is_empty() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HTTPHandleRequest", len)?;
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.body_base64.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "body_base64",
                pbjson::private::base64::encode(&self.body_base64).as_str(),
            )?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpHandleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "method",
            "path",
            "query",
            "headers",
            "body_base64",
            "bodyBase64",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
            Path,
            Query,
            Headers,
            BodyBase64,
            User,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "method" => Ok(GeneratedField::Method),
                            "path" => Ok(GeneratedField::Path),
                            "query" => Ok(GeneratedField::Query),
                            "headers" => Ok(GeneratedField::Headers),
                            "bodyBase64" | "body_base64" => Ok(GeneratedField::BodyBase64),
                            "user" => Ok(GeneratedField::User),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpHandleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HTTPHandleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpHandleRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut method__ = None;
                let mut path__ = None;
                let mut query__ = None;
                let mut headers__ = None;
                let mut body_base64__ = None;
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ =
                                Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::BodyBase64 => {
                            if body_base64__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodyBase64"));
                            }
                            body_base64__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HttpHandleRequest {
                    method: method__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    body_base64: body_base64__.unwrap_or_default(),
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HTTPHandleRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HttpHandleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.body_base64.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HTTPHandleResponse", len)?;
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.body_base64.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field(
                "body_base64",
                pbjson::private::base64::encode(&self.body_base64).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpHandleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["status", "headers", "body_base64", "bodyBase64"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Headers,
            BodyBase64,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "headers" => Ok(GeneratedField::Headers),
                            "bodyBase64" | "body_base64" => Ok(GeneratedField::BodyBase64),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpHandleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HTTPHandleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpHandleResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut headers__ = None;
                let mut body_base64__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ =
                                Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::BodyBase64 => {
                            if body_base64__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodyBase64"));
                            }
                            body_base64__ = Some(
                                map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HttpHandleResponse {
                    status: status__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    body_base64: body_base64__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HTTPHandleResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HttpUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.HTTPUser", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "name"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HTTPUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpUser, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HttpUser {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.HTTPUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostActivitySetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        if self.active {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostActivitySetRequest", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        if self.active {
            struct_ser.serialize_field("active", &self.active)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostActivitySetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "label", "active"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Label,
            Active,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "label" => Ok(GeneratedField::Label),
                            "active" => Ok(GeneratedField::Active),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostActivitySetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostActivitySetRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostActivitySetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut label__ = None;
                let mut active__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Active => {
                            if active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostActivitySetRequest {
                    key: key__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                    active: active__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostActivitySetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostActivitySetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostActivitySetResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostActivitySetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostActivitySetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostActivitySetResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostActivitySetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostActivitySetResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostActivitySetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostCert {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.domains.is_empty() {
            len += 1;
        }
        if self.auto_renew {
            len += 1;
        }
        if !self.challenge_method.is_empty() {
            len += 1;
        }
        if !self.key_type.is_empty() {
            len += 1;
        }
        if !self.not_before.is_empty() {
            len += 1;
        }
        if !self.not_after.is_empty() {
            len += 1;
        }
        if !self.issuer.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.HostCert", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.domains.is_empty() {
            struct_ser.serialize_field("domains", &self.domains)?;
        }
        if self.auto_renew {
            struct_ser.serialize_field("auto_renew", &self.auto_renew)?;
        }
        if !self.challenge_method.is_empty() {
            struct_ser.serialize_field("challenge_method", &self.challenge_method)?;
        }
        if !self.key_type.is_empty() {
            struct_ser.serialize_field("key_type", &self.key_type)?;
        }
        if !self.not_before.is_empty() {
            struct_ser.serialize_field("not_before", &self.not_before)?;
        }
        if !self.not_after.is_empty() {
            struct_ser.serialize_field("not_after", &self.not_after)?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCert {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "domains",
            "auto_renew",
            "autoRenew",
            "challenge_method",
            "challengeMethod",
            "key_type",
            "keyType",
            "not_before",
            "notBefore",
            "not_after",
            "notAfter",
            "issuer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Domains,
            AutoRenew,
            ChallengeMethod,
            KeyType,
            NotBefore,
            NotAfter,
            Issuer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "domains" => Ok(GeneratedField::Domains),
                            "autoRenew" | "auto_renew" => Ok(GeneratedField::AutoRenew),
                            "challengeMethod" | "challenge_method" => {
                                Ok(GeneratedField::ChallengeMethod)
                            }
                            "keyType" | "key_type" => Ok(GeneratedField::KeyType),
                            "notBefore" | "not_before" => Ok(GeneratedField::NotBefore),
                            "notAfter" | "not_after" => Ok(GeneratedField::NotAfter),
                            "issuer" => Ok(GeneratedField::Issuer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCert;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCert")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostCert, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut domains__ = None;
                let mut auto_renew__ = None;
                let mut challenge_method__ = None;
                let mut key_type__ = None;
                let mut not_before__ = None;
                let mut not_after__ = None;
                let mut issuer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domains => {
                            if domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domains"));
                            }
                            domains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoRenew => {
                            if auto_renew__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoRenew"));
                            }
                            auto_renew__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChallengeMethod => {
                            if challenge_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("challengeMethod"));
                            }
                            challenge_method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyType => {
                            if key_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyType"));
                            }
                            key_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotBefore => {
                            if not_before__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notBefore"));
                            }
                            not_before__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotAfter => {
                            if not_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notAfter"));
                            }
                            not_after__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostCert {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    domains: domains__.unwrap_or_default(),
                    auto_renew: auto_renew__.unwrap_or_default(),
                    challenge_method: challenge_method__.unwrap_or_default(),
                    key_type: key_type__.unwrap_or_default(),
                    not_before: not_before__.unwrap_or_default(),
                    not_after: not_after__.unwrap_or_default(),
                    issuer: issuer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.HostCert", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostCertsListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostCertsListRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCertsListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCertsListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCertsListRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostCertsListRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostCertsListRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostCertsListRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostCertsListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.certs.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostCertsListResponse", len)?;
        if !self.certs.is_empty() {
            struct_ser.serialize_field("certs", &self.certs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCertsListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["certs"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Certs,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "certs" => Ok(GeneratedField::Certs),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCertsListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCertsListResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostCertsListResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut certs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Certs => {
                            if certs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certs"));
                            }
                            certs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostCertsListResponse {
                    certs: certs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostCertsListResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostCredentialsGetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostCredentialsGetRequest", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCredentialsGetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["kind", "id"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Id,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "id" => Ok(GeneratedField::Id),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCredentialsGetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCredentialsGetRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostCredentialsGetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostCredentialsGetRequest {
                    kind: kind__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostCredentialsGetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostCredentialsGetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.provider_code.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostCredentialsGetResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.provider_code.is_empty() {
            struct_ser.serialize_field("provider_code", &self.provider_code)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCredentialsGetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "name", "provider_code", "providerCode", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            ProviderCode,
            Config,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "providerCode" | "provider_code" => Ok(GeneratedField::ProviderCode),
                            "config" => Ok(GeneratedField::Config),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCredentialsGetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCredentialsGetResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostCredentialsGetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut provider_code__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProviderCode => {
                            if provider_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerCode"));
                            }
                            provider_code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostCredentialsGetResponse {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    provider_code: provider_code__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostCredentialsGetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostCronRegisterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.schedule.is_empty() {
            len += 1;
        }
        if !self.method.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostCronRegisterRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.schedule.is_empty() {
            struct_ser.serialize_field("schedule", &self.schedule)?;
        }
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCronRegisterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "schedule", "method"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Schedule,
            Method,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "schedule" => Ok(GeneratedField::Schedule),
                            "method" => Ok(GeneratedField::Method),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCronRegisterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCronRegisterRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostCronRegisterRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut schedule__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schedule => {
                            if schedule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            schedule__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostCronRegisterRequest {
                    id: id__.unwrap_or_default(),
                    schedule: schedule__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostCronRegisterRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostCronRegisterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostCronRegisterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCronRegisterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCronRegisterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCronRegisterResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostCronRegisterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostCronRegisterResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostCronRegisterResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostCronUnregisterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostCronUnregisterRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCronUnregisterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCronUnregisterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCronUnregisterRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostCronUnregisterRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostCronUnregisterRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostCronUnregisterRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostCronUnregisterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostCronUnregisterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCronUnregisterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCronUnregisterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostCronUnregisterResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostCronUnregisterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostCronUnregisterResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostCronUnregisterResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostI18nLocaleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostI18nLocaleRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostI18nLocaleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostI18nLocaleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostI18nLocaleRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostI18nLocaleRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostI18nLocaleRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostI18nLocaleRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostI18nLocaleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.locale.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostI18nLocaleResponse", len)?;
        if !self.locale.is_empty() {
            struct_ser.serialize_field("locale", &self.locale)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostI18nLocaleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["locale"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locale,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "locale" => Ok(GeneratedField::Locale),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostI18nLocaleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostI18nLocaleResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostI18nLocaleResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut locale__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locale => {
                            if locale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            locale__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostI18nLocaleResponse {
                    locale: locale__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostI18nLocaleResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.os.is_empty() {
            len += 1;
        }
        if !self.arch.is_empty() {
            len += 1;
        }
        if !self.locale.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.HostInfo", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.os.is_empty() {
            struct_ser.serialize_field("os", &self.os)?;
        }
        if !self.arch.is_empty() {
            struct_ser.serialize_field("arch", &self.arch)?;
        }
        if !self.locale.is_empty() {
            struct_ser.serialize_field("locale", &self.locale)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["version", "os", "arch", "locale"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Os,
            Arch,
            Locale,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "os" => Ok(GeneratedField::Os),
                            "arch" => Ok(GeneratedField::Arch),
                            "locale" => Ok(GeneratedField::Locale),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut os__ = None;
                let mut arch__ = None;
                let mut locale__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Os => {
                            if os__.is_some() {
                                return Err(serde::de::Error::duplicate_field("os"));
                            }
                            os__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Arch => {
                            if arch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arch"));
                            }
                            arch__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Locale => {
                            if locale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            locale__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostInfo {
                    version: version__.unwrap_or_default(),
                    os: os__.unwrap_or_default(),
                    arch: arch__.unwrap_or_default(),
                    locale: locale__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.HostInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostKvDeleteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostKVDeleteRequest", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostKvDeleteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostKvDeleteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostKVDeleteRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostKvDeleteRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostKvDeleteRequest {
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostKVDeleteRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostKvDeleteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostKVDeleteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostKvDeleteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostKvDeleteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostKVDeleteResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostKvDeleteResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostKvDeleteResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostKVDeleteResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostKvGetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostKVGetRequest", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostKvGetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostKvGetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostKVGetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostKvGetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostKvGetRequest {
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostKVGetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostKvGetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        if self.found {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostKVGetResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if self.found {
            struct_ser.serialize_field("found", &self.found)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostKvGetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value", "found"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Found,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "found" => Ok(GeneratedField::Found),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostKvGetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostKVGetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostKvGetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut found__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::Found => {
                            if found__.is_some() {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            found__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostKvGetResponse {
                    value: value__,
                    found: found__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostKVGetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostKvListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostKVListRequest", len)?;
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostKvListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["prefix"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prefix,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "prefix" => Ok(GeneratedField::Prefix),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostKvListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostKVListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostKvListRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut prefix__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostKvListRequest {
                    prefix: prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostKVListRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostKvListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostKVListResponse", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostKvListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["keys"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keys" => Ok(GeneratedField::Keys),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostKvListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostKVListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostKvListResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostKvListResponse {
                    keys: keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostKVListResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostKvSetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostKVSetRequest", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostKvSetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "value"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostKvSetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostKVSetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostKvSetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostKvSetRequest {
                    key: key__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostKVSetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostKvSetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nginxui.plugin.v1.HostKVSetResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostKvSetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostKvSetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostKVSetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostKvSetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostKvSetResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostKVSetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostLogFile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.config_file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.HostLogFile", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.config_file.is_empty() {
            struct_ser.serialize_field("config_file", &self.config_file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostLogFile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["path", "type", "source", "config_file", "configFile"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Type,
            Source,
            ConfigFile,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "type" => Ok(GeneratedField::Type),
                            "source" => Ok(GeneratedField::Source),
                            "configFile" | "config_file" => Ok(GeneratedField::ConfigFile),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostLogFile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostLogFile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostLogFile, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut r#type__ = None;
                let mut source__ = None;
                let mut config_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigFile => {
                            if config_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configFile"));
                            }
                            config_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostLogFile {
                    path: path__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    config_file: config_file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.HostLogFile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostLogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.level.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if self.fields.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostLogRequest", len)?;
        if !self.level.is_empty() {
            struct_ser.serialize_field("level", &self.level)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if let Some(v) = self.fields.as_ref() {
            struct_ser.serialize_field("fields", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostLogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["level", "message", "fields"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Level,
            Message,
            Fields,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "level" => Ok(GeneratedField::Level),
                            "message" => Ok(GeneratedField::Message),
                            "fields" => Ok(GeneratedField::Fields),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostLogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostLogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostLogRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut level__ = None;
                let mut message__ = None;
                let mut fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostLogRequest {
                    level: level__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    fields: fields__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostLogRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostLogResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nginxui.plugin.v1.HostLogResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostLogResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostLogResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostLogResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostLogResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostLogResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostLogResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostLogsListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostLogsListRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostLogsListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostLogsListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostLogsListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostLogsListRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostLogsListRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostLogsListRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostLogsListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.logs.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostLogsListResponse", len)?;
        if !self.logs.is_empty() {
            struct_ser.serialize_field("logs", &self.logs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostLogsListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["logs"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Logs,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "logs" => Ok(GeneratedField::Logs),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostLogsListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostLogsListResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostLogsListResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut logs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Logs => {
                            if logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logs"));
                            }
                            logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostLogsListResponse {
                    logs: logs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostLogsListResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostMetricsSnapshotRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostMetricsSnapshotRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostMetricsSnapshotRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostMetricsSnapshotRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostMetricsSnapshotRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostMetricsSnapshotRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostMetricsSnapshotRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostMetricsSnapshotRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostMetricsSnapshotResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.snapshot.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostMetricsSnapshotResponse", len)?;
        if let Some(v) = self.snapshot.as_ref() {
            struct_ser.serialize_field("snapshot", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostMetricsSnapshotResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["snapshot"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Snapshot,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "snapshot" => Ok(GeneratedField::Snapshot),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostMetricsSnapshotResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostMetricsSnapshotResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostMetricsSnapshotResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut snapshot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Snapshot => {
                            if snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            snapshot__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostMetricsSnapshotResponse {
                    snapshot: snapshot__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostMetricsSnapshotResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxConfigGetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxConfigGetRequest", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxConfigGetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["path"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxConfigGetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxConfigGetRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxConfigGetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxConfigGetRequest {
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxConfigGetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxConfigGetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxConfigGetResponse", len)?;
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxConfigGetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["content"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Content,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "content" => Ok(GeneratedField::Content),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxConfigGetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxConfigGetResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxConfigGetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxConfigGetResponse {
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxConfigGetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxConfigListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxConfigListRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxConfigListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxConfigListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxConfigListRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxConfigListRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostNginxConfigListRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxConfigListRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxConfigListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.files.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxConfigListResponse", len)?;
        if !self.files.is_empty() {
            struct_ser.serialize_field("files", &self.files)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxConfigListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["files"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Files,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "files" => Ok(GeneratedField::Files),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxConfigListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxConfigListResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxConfigListResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut files__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Files => {
                            if files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("files"));
                            }
                            files__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxConfigListResponse {
                    files: files__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxConfigListResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxSnippet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.include.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxSnippet", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.include.is_empty() {
            struct_ser.serialize_field("include", &self.include)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxSnippet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name", "include"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Include,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "include" => Ok(GeneratedField::Include),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxSnippet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxSnippet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostNginxSnippet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut include__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Include => {
                            if include__.is_some() {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            include__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxSnippet {
                    name: name__.unwrap_or_default(),
                    include: include__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxSnippet",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxSnippetDeleteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxSnippetDeleteRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxSnippetDeleteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxSnippetDeleteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxSnippetDeleteRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxSnippetDeleteRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxSnippetDeleteRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxSnippetDeleteRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxSnippetDeleteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.removed {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxSnippetDeleteResponse", len)?;
        if self.removed {
            struct_ser.serialize_field("removed", &self.removed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxSnippetDeleteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["removed"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Removed,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "removed" => Ok(GeneratedField::Removed),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxSnippetDeleteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxSnippetDeleteResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxSnippetDeleteResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut removed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Removed => {
                            if removed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removed"));
                            }
                            removed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxSnippetDeleteResponse {
                    removed: removed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxSnippetDeleteResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxSnippetListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxSnippetListRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxSnippetListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxSnippetListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxSnippetListRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxSnippetListRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostNginxSnippetListRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxSnippetListRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxSnippetListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.snippets.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxSnippetListResponse", len)?;
        if !self.snippets.is_empty() {
            struct_ser.serialize_field("snippets", &self.snippets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxSnippetListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["snippets"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Snippets,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "snippets" => Ok(GeneratedField::Snippets),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxSnippetListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxSnippetListResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxSnippetListResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut snippets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Snippets => {
                            if snippets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snippets"));
                            }
                            snippets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxSnippetListResponse {
                    snippets: snippets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxSnippetListResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxSnippetPutRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxSnippetPutRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxSnippetPutRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name", "content"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Content,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "content" => Ok(GeneratedField::Content),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxSnippetPutRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxSnippetPutRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxSnippetPutRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxSnippetPutRequest {
                    name: name__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxSnippetPutRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNginxSnippetPutResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.changed {
            len += 1;
        }
        if !self.include.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNginxSnippetPutResponse", len)?;
        if self.changed {
            struct_ser.serialize_field("changed", &self.changed)?;
        }
        if !self.include.is_empty() {
            struct_ser.serialize_field("include", &self.include)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNginxSnippetPutResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["changed", "include"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Changed,
            Include,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "changed" => Ok(GeneratedField::Changed),
                            "include" => Ok(GeneratedField::Include),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNginxSnippetPutResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNginxSnippetPutResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostNginxSnippetPutResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut changed__ = None;
                let mut include__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Changed => {
                            if changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changed"));
                            }
                            changed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Include => {
                            if include__.is_some() {
                                return Err(serde::de::Error::duplicate_field("include"));
                            }
                            include__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNginxSnippetPutResponse {
                    changed: changed__.unwrap_or_default(),
                    include: include__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNginxSnippetPutResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNotifyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.level.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNotifyRequest", len)?;
        if !self.level.is_empty() {
            struct_ser.serialize_field("level", &self.level)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNotifyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["level", "title", "content", "details"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Level,
            Title,
            Content,
            Details,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "level" => Ok(GeneratedField::Level),
                            "title" => Ok(GeneratedField::Title),
                            "content" => Ok(GeneratedField::Content),
                            "details" => Ok(GeneratedField::Details),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNotifyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNotifyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostNotifyRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut level__ = None;
                let mut title__ = None;
                let mut content__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostNotifyRequest {
                    level: level__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNotifyRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostNotifyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostNotifyResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostNotifyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostNotifyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostNotifyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostNotifyResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostNotifyResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostNotifyResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostSettingsGetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostSettingsGetRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostSettingsGetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostSettingsGetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostSettingsGetRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostSettingsGetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostSettingsGetRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostSettingsGetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostSettingsGetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostSettingsGetResponse", len)?;
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostSettingsGetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["settings"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Settings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostSettingsGetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostSettingsGetResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostSettingsGetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostSettingsGetResponse {
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostSettingsGetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostSite {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if !self.urls.is_empty() {
            len += 1;
        }
        if !self.config_file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.HostSite", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if !self.urls.is_empty() {
            struct_ser.serialize_field("urls", &self.urls)?;
        }
        if !self.config_file.is_empty() {
            struct_ser.serialize_field("config_file", &self.config_file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostSite {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name", "status", "urls", "config_file", "configFile"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Status,
            Urls,
            ConfigFile,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "status" => Ok(GeneratedField::Status),
                            "urls" => Ok(GeneratedField::Urls),
                            "configFile" | "config_file" => Ok(GeneratedField::ConfigFile),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostSite;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostSite")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostSite, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut status__ = None;
                let mut urls__ = None;
                let mut config_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Urls => {
                            if urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urls"));
                            }
                            urls__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigFile => {
                            if config_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configFile"));
                            }
                            config_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostSite {
                    name: name__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    urls: urls__.unwrap_or_default(),
                    config_file: config_file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.HostSite", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostSitesListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostSitesListRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostSitesListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostSitesListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostSitesListRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostSitesListRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HostSitesListRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostSitesListRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for HostSitesListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sites.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.HostSitesListResponse", len)?;
        if !self.sites.is_empty() {
            struct_ser.serialize_field("sites", &self.sites)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostSitesListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sites"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sites,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sites" => Ok(GeneratedField::Sites),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostSitesListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.HostSitesListResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<HostSitesListResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sites__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sites => {
                            if sites__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sites"));
                            }
                            sites__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostSitesListResponse {
                    sites: sites__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.HostSitesListResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for InvalidConfigData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.InvalidConfigData", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InvalidConfigData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["field"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "field" => Ok(GeneratedField::Field),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InvalidConfigData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.InvalidConfigData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InvalidConfigData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(InvalidConfigData {
                    field: field__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.InvalidConfigData",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for LogEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.timestamp.is_empty() {
            len += 1;
        }
        if !self.remote_addr.is_empty() {
            len += 1;
        }
        if !self.request_method.is_empty() {
            len += 1;
        }
        if !self.request_uri.is_empty() {
            len += 1;
        }
        if !self.protocol.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.body_bytes_sent != 0. {
            len += 1;
        }
        if !self.referer.is_empty() {
            len += 1;
        }
        if !self.user_agent.is_empty() {
            len += 1;
        }
        if !self.upstream_addr.is_empty() {
            len += 1;
        }
        if self.request_time != 0. {
            len += 1;
        }
        if self.upstream_response_time != 0. {
            len += 1;
        }
        if !self.host.is_empty() {
            len += 1;
        }
        if !self.raw.is_empty() {
            len += 1;
        }
        if !self.format.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.LogEntry", len)?;
        if !self.timestamp.is_empty() {
            struct_ser.serialize_field("timestamp", &self.timestamp)?;
        }
        if !self.remote_addr.is_empty() {
            struct_ser.serialize_field("remote_addr", &self.remote_addr)?;
        }
        if !self.request_method.is_empty() {
            struct_ser.serialize_field("request_method", &self.request_method)?;
        }
        if !self.request_uri.is_empty() {
            struct_ser.serialize_field("request_uri", &self.request_uri)?;
        }
        if !self.protocol.is_empty() {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if self.body_bytes_sent != 0. {
            struct_ser.serialize_field("body_bytes_sent", &self.body_bytes_sent)?;
        }
        if !self.referer.is_empty() {
            struct_ser.serialize_field("referer", &self.referer)?;
        }
        if !self.user_agent.is_empty() {
            struct_ser.serialize_field("user_agent", &self.user_agent)?;
        }
        if !self.upstream_addr.is_empty() {
            struct_ser.serialize_field("upstream_addr", &self.upstream_addr)?;
        }
        if self.request_time != 0. {
            struct_ser.serialize_field("request_time", &self.request_time)?;
        }
        if self.upstream_response_time != 0. {
            struct_ser.serialize_field("upstream_response_time", &self.upstream_response_time)?;
        }
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if !self.raw.is_empty() {
            struct_ser.serialize_field("raw", &self.raw)?;
        }
        if !self.format.is_empty() {
            struct_ser.serialize_field("format", &self.format)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "remote_addr",
            "remoteAddr",
            "request_method",
            "requestMethod",
            "request_uri",
            "requestUri",
            "protocol",
            "status",
            "body_bytes_sent",
            "bodyBytesSent",
            "referer",
            "user_agent",
            "userAgent",
            "upstream_addr",
            "upstreamAddr",
            "request_time",
            "requestTime",
            "upstream_response_time",
            "upstreamResponseTime",
            "host",
            "raw",
            "format",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            RemoteAddr,
            RequestMethod,
            RequestUri,
            Protocol,
            Status,
            BodyBytesSent,
            Referer,
            UserAgent,
            UpstreamAddr,
            RequestTime,
            UpstreamResponseTime,
            Host,
            Raw,
            Format,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "remoteAddr" | "remote_addr" => Ok(GeneratedField::RemoteAddr),
                            "requestMethod" | "request_method" => Ok(GeneratedField::RequestMethod),
                            "requestUri" | "request_uri" => Ok(GeneratedField::RequestUri),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "status" => Ok(GeneratedField::Status),
                            "bodyBytesSent" | "body_bytes_sent" => {
                                Ok(GeneratedField::BodyBytesSent)
                            }
                            "referer" => Ok(GeneratedField::Referer),
                            "userAgent" | "user_agent" => Ok(GeneratedField::UserAgent),
                            "upstreamAddr" | "upstream_addr" => Ok(GeneratedField::UpstreamAddr),
                            "requestTime" | "request_time" => Ok(GeneratedField::RequestTime),
                            "upstreamResponseTime" | "upstream_response_time" => {
                                Ok(GeneratedField::UpstreamResponseTime)
                            }
                            "host" => Ok(GeneratedField::Host),
                            "raw" => Ok(GeneratedField::Raw),
                            "format" => Ok(GeneratedField::Format),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.LogEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut remote_addr__ = None;
                let mut request_method__ = None;
                let mut request_uri__ = None;
                let mut protocol__ = None;
                let mut status__ = None;
                let mut body_bytes_sent__ = None;
                let mut referer__ = None;
                let mut user_agent__ = None;
                let mut upstream_addr__ = None;
                let mut request_time__ = None;
                let mut upstream_response_time__ = None;
                let mut host__ = None;
                let mut raw__ = None;
                let mut format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteAddr => {
                            if remote_addr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteAddr"));
                            }
                            remote_addr__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestMethod => {
                            if request_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMethod"));
                            }
                            request_method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestUri => {
                            if request_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestUri"));
                            }
                            request_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BodyBytesSent => {
                            if body_bytes_sent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodyBytesSent"));
                            }
                            body_bytes_sent__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Referer => {
                            if referer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referer"));
                            }
                            referer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpstreamAddr => {
                            if upstream_addr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamAddr"));
                            }
                            upstream_addr__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestTime => {
                            if request_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTime"));
                            }
                            request_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::UpstreamResponseTime => {
                            if upstream_response_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "upstreamResponseTime",
                                ));
                            }
                            upstream_response_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Raw => {
                            if raw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("raw"));
                            }
                            raw__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(LogEntry {
                    timestamp: timestamp__.unwrap_or_default(),
                    remote_addr: remote_addr__.unwrap_or_default(),
                    request_method: request_method__.unwrap_or_default(),
                    request_uri: request_uri__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    body_bytes_sent: body_bytes_sent__.unwrap_or_default(),
                    referer: referer__.unwrap_or_default(),
                    user_agent: user_agent__.unwrap_or_default(),
                    upstream_addr: upstream_addr__.unwrap_or_default(),
                    request_time: request_time__.unwrap_or_default(),
                    upstream_response_time: upstream_response_time__.unwrap_or_default(),
                    host: host__.unwrap_or_default(),
                    raw: raw__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.LogEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogSinkPushRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_path.is_empty() {
            len += 1;
        }
        if self.entry.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.LogSinkPushRequest", len)?;
        if !self.log_path.is_empty() {
            struct_ser.serialize_field("log_path", &self.log_path)?;
        }
        if let Some(v) = self.entry.as_ref() {
            struct_ser.serialize_field("entry", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogSinkPushRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["log_path", "logPath", "entry"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogPath,
            Entry,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "logPath" | "log_path" => Ok(GeneratedField::LogPath),
                            "entry" => Ok(GeneratedField::Entry),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogSinkPushRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.LogSinkPushRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogSinkPushRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut log_path__ = None;
                let mut entry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LogPath => {
                            if log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logPath"));
                            }
                            log_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Entry => {
                            if entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entry"));
                            }
                            entry__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(LogSinkPushRequest {
                    log_path: log_path__.unwrap_or_default(),
                    entry: entry__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.LogSinkPushRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for LogSinkPushResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.accepted != 0 {
            len += 1;
        }
        if self.rejected != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.LogSinkPushResponse", len)?;
        if self.accepted != 0 {
            struct_ser.serialize_field("accepted", &self.accepted)?;
        }
        if self.rejected != 0 {
            struct_ser.serialize_field("rejected", &self.rejected)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogSinkPushResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["accepted", "rejected"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Accepted,
            Rejected,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accepted" => Ok(GeneratedField::Accepted),
                            "rejected" => Ok(GeneratedField::Rejected),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogSinkPushResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.LogSinkPushResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogSinkPushResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut accepted__ = None;
                let mut rejected__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Accepted => {
                            if accepted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accepted"));
                            }
                            accepted__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Rejected => {
                            if rejected__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejected"));
                            }
                            rejected__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(LogSinkPushResponse {
                    accepted: accepted__.unwrap_or_default(),
                    rejected: rejected__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.LogSinkPushResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for McpCallRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tool.is_empty() {
            len += 1;
        }
        if self.arguments.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.MCPCallRequest", len)?;
        if !self.tool.is_empty() {
            struct_ser.serialize_field("tool", &self.tool)?;
        }
        if let Some(v) = self.arguments.as_ref() {
            struct_ser.serialize_field("arguments", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for McpCallRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["tool", "arguments"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tool,
            Arguments,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tool" => Ok(GeneratedField::Tool),
                            "arguments" => Ok(GeneratedField::Arguments),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = McpCallRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.MCPCallRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<McpCallRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tool__ = None;
                let mut arguments__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tool => {
                            if tool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tool"));
                            }
                            tool__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Arguments => {
                            if arguments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arguments"));
                            }
                            arguments__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(McpCallRequest {
                    tool: tool__.unwrap_or_default(),
                    arguments: arguments__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.MCPCallRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for McpCallResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.content.is_empty() {
            len += 1;
        }
        if self.is_error {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.MCPCallResponse", len)?;
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if self.is_error {
            struct_ser.serialize_field("is_error", &self.is_error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for McpCallResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["content", "is_error", "isError"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Content,
            IsError,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "content" => Ok(GeneratedField::Content),
                            "isError" | "is_error" => Ok(GeneratedField::IsError),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = McpCallResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.MCPCallResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<McpCallResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut content__ = None;
                let mut is_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsError => {
                            if is_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isError"));
                            }
                            is_error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(McpCallResponse {
                    content: content__.unwrap_or_default(),
                    is_error: is_error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.MCPCallResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for McpContent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.MCPContent", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for McpContent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["type", "text"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Text,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "text" => Ok(GeneratedField::Text),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = McpContent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.MCPContent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<McpContent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(McpContent {
                    r#type: r#type__.unwrap_or_default(),
                    text: text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.MCPContent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for McpTool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.input_schema.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.MCPTool", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.input_schema.as_ref() {
            struct_ser.serialize_field("input_schema", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for McpTool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["name", "description", "input_schema", "inputSchema"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            InputSchema,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "inputSchema" | "input_schema" => Ok(GeneratedField::InputSchema),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = McpTool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.MCPTool")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<McpTool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut input_schema__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InputSchema => {
                            if input_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputSchema"));
                            }
                            input_schema__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(McpTool {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    input_schema: input_schema__,
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.MCPTool", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Manifest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.homepage_url.is_empty() {
            len += 1;
        }
        if !self.icon_path.is_empty() {
            len += 1;
        }
        if self.api_version != 0 {
            len += 1;
        }
        if !self.min_nginx_ui_version.is_empty() {
            len += 1;
        }
        if self.server.is_some() {
            len += 1;
        }
        if self.webapp.is_some() {
            len += 1;
        }
        if self.content.is_some() {
            len += 1;
        }
        if !self.capabilities.is_empty() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        if !self.requires.is_empty() {
            len += 1;
        }
        if !self.requires_capabilities.is_empty() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if !self.cron.is_empty() {
            len += 1;
        }
        if !self.network_hosts.is_empty() {
            len += 1;
        }
        if self.dns01.is_some() {
            len += 1;
        }
        if self.http.is_some() {
            len += 1;
        }
        if self.settings_schema.is_some() {
            len += 1;
        }
        if self.notify.is_some() {
            len += 1;
        }
        if self.probe.is_some() {
            len += 1;
        }
        if self.mcp.is_some() {
            len += 1;
        }
        if self.storage.is_some() {
            len += 1;
        }
        if self.deploy.is_some() {
            len += 1;
        }
        if self.blocklist.is_some() {
            len += 1;
        }
        if self.discovery.is_some() {
            len += 1;
        }
        if self.log_sink.is_some() {
            len += 1;
        }
        if !self.i18n.is_empty() {
            len += 1;
        }
        if !self.conflicts.is_empty() {
            len += 1;
        }
        if !self.permission_reasons.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.Manifest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.homepage_url.is_empty() {
            struct_ser.serialize_field("homepage_url", &self.homepage_url)?;
        }
        if !self.icon_path.is_empty() {
            struct_ser.serialize_field("icon_path", &self.icon_path)?;
        }
        if self.api_version != 0 {
            struct_ser.serialize_field("api_version", &self.api_version)?;
        }
        if !self.min_nginx_ui_version.is_empty() {
            struct_ser.serialize_field("min_nginx_ui_version", &self.min_nginx_ui_version)?;
        }
        if let Some(v) = self.server.as_ref() {
            struct_ser.serialize_field("server", v)?;
        }
        if let Some(v) = self.webapp.as_ref() {
            struct_ser.serialize_field("webapp", v)?;
        }
        if let Some(v) = self.content.as_ref() {
            struct_ser.serialize_field("content", v)?;
        }
        if !self.capabilities.is_empty() {
            struct_ser.serialize_field("capabilities", &self.capabilities)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if !self.requires.is_empty() {
            struct_ser.serialize_field("requires", &self.requires)?;
        }
        if !self.requires_capabilities.is_empty() {
            struct_ser.serialize_field("requires_capabilities", &self.requires_capabilities)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if !self.cron.is_empty() {
            struct_ser.serialize_field("cron", &self.cron)?;
        }
        if !self.network_hosts.is_empty() {
            struct_ser.serialize_field("network_hosts", &self.network_hosts)?;
        }
        if let Some(v) = self.dns01.as_ref() {
            struct_ser.serialize_field("dns01", v)?;
        }
        if let Some(v) = self.http.as_ref() {
            struct_ser.serialize_field("http", v)?;
        }
        if let Some(v) = self.settings_schema.as_ref() {
            struct_ser.serialize_field("settings_schema", v)?;
        }
        if let Some(v) = self.notify.as_ref() {
            struct_ser.serialize_field("notify", v)?;
        }
        if let Some(v) = self.probe.as_ref() {
            struct_ser.serialize_field("probe", v)?;
        }
        if let Some(v) = self.mcp.as_ref() {
            struct_ser.serialize_field("mcp", v)?;
        }
        if let Some(v) = self.storage.as_ref() {
            struct_ser.serialize_field("storage", v)?;
        }
        if let Some(v) = self.deploy.as_ref() {
            struct_ser.serialize_field("deploy", v)?;
        }
        if let Some(v) = self.blocklist.as_ref() {
            struct_ser.serialize_field("blocklist", v)?;
        }
        if let Some(v) = self.discovery.as_ref() {
            struct_ser.serialize_field("discovery", v)?;
        }
        if let Some(v) = self.log_sink.as_ref() {
            struct_ser.serialize_field("log_sink", v)?;
        }
        if !self.i18n.is_empty() {
            struct_ser.serialize_field("i18n", &self.i18n)?;
        }
        if !self.conflicts.is_empty() {
            struct_ser.serialize_field("conflicts", &self.conflicts)?;
        }
        if !self.permission_reasons.is_empty() {
            struct_ser.serialize_field("permission_reasons", &self.permission_reasons)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Manifest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "version",
            "description",
            "homepage_url",
            "homepageUrl",
            "icon_path",
            "iconPath",
            "api_version",
            "apiVersion",
            "min_nginx_ui_version",
            "minNginxUiVersion",
            "server",
            "webapp",
            "content",
            "capabilities",
            "permissions",
            "requires",
            "requires_capabilities",
            "requiresCapabilities",
            "events",
            "cron",
            "network_hosts",
            "networkHosts",
            "dns01",
            "http",
            "settings_schema",
            "settingsSchema",
            "notify",
            "probe",
            "mcp",
            "storage",
            "deploy",
            "blocklist",
            "discovery",
            "log_sink",
            "logSink",
            "i18n",
            "conflicts",
            "permission_reasons",
            "permissionReasons",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Version,
            Description,
            HomepageUrl,
            IconPath,
            ApiVersion,
            MinNginxUiVersion,
            Server,
            Webapp,
            Content,
            Capabilities,
            Permissions,
            Requires,
            RequiresCapabilities,
            Events,
            Cron,
            NetworkHosts,
            Dns01,
            Http,
            SettingsSchema,
            Notify,
            Probe,
            Mcp,
            Storage,
            Deploy,
            Blocklist,
            Discovery,
            LogSink,
            I18n,
            Conflicts,
            PermissionReasons,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "version" => Ok(GeneratedField::Version),
                            "description" => Ok(GeneratedField::Description),
                            "homepageUrl" | "homepage_url" => Ok(GeneratedField::HomepageUrl),
                            "iconPath" | "icon_path" => Ok(GeneratedField::IconPath),
                            "apiVersion" | "api_version" => Ok(GeneratedField::ApiVersion),
                            "minNginxUiVersion" | "min_nginx_ui_version" => {
                                Ok(GeneratedField::MinNginxUiVersion)
                            }
                            "server" => Ok(GeneratedField::Server),
                            "webapp" => Ok(GeneratedField::Webapp),
                            "content" => Ok(GeneratedField::Content),
                            "capabilities" => Ok(GeneratedField::Capabilities),
                            "permissions" => Ok(GeneratedField::Permissions),
                            "requires" => Ok(GeneratedField::Requires),
                            "requiresCapabilities" | "requires_capabilities" => {
                                Ok(GeneratedField::RequiresCapabilities)
                            }
                            "events" => Ok(GeneratedField::Events),
                            "cron" => Ok(GeneratedField::Cron),
                            "networkHosts" | "network_hosts" => Ok(GeneratedField::NetworkHosts),
                            "dns01" => Ok(GeneratedField::Dns01),
                            "http" => Ok(GeneratedField::Http),
                            "settingsSchema" | "settings_schema" => {
                                Ok(GeneratedField::SettingsSchema)
                            }
                            "notify" => Ok(GeneratedField::Notify),
                            "probe" => Ok(GeneratedField::Probe),
                            "mcp" => Ok(GeneratedField::Mcp),
                            "storage" => Ok(GeneratedField::Storage),
                            "deploy" => Ok(GeneratedField::Deploy),
                            "blocklist" => Ok(GeneratedField::Blocklist),
                            "discovery" => Ok(GeneratedField::Discovery),
                            "logSink" | "log_sink" => Ok(GeneratedField::LogSink),
                            "i18n" => Ok(GeneratedField::I18n),
                            "conflicts" => Ok(GeneratedField::Conflicts),
                            "permissionReasons" | "permission_reasons" => {
                                Ok(GeneratedField::PermissionReasons)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Manifest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.Manifest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Manifest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut version__ = None;
                let mut description__ = None;
                let mut homepage_url__ = None;
                let mut icon_path__ = None;
                let mut api_version__ = None;
                let mut min_nginx_ui_version__ = None;
                let mut server__ = None;
                let mut webapp__ = None;
                let mut content__ = None;
                let mut capabilities__ = None;
                let mut permissions__ = None;
                let mut requires__ = None;
                let mut requires_capabilities__ = None;
                let mut events__ = None;
                let mut cron__ = None;
                let mut network_hosts__ = None;
                let mut dns01__ = None;
                let mut http__ = None;
                let mut settings_schema__ = None;
                let mut notify__ = None;
                let mut probe__ = None;
                let mut mcp__ = None;
                let mut storage__ = None;
                let mut deploy__ = None;
                let mut blocklist__ = None;
                let mut discovery__ = None;
                let mut log_sink__ = None;
                let mut i18n__ = None;
                let mut conflicts__ = None;
                let mut permission_reasons__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HomepageUrl => {
                            if homepage_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("homepageUrl"));
                            }
                            homepage_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IconPath => {
                            if icon_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconPath"));
                            }
                            icon_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApiVersion => {
                            if api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiVersion"));
                            }
                            api_version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinNginxUiVersion => {
                            if min_nginx_ui_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minNginxUiVersion"));
                            }
                            min_nginx_ui_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Server => {
                            if server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("server"));
                            }
                            server__ = map_.next_value()?;
                        }
                        GeneratedField::Webapp => {
                            if webapp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webapp"));
                            }
                            webapp__ = map_.next_value()?;
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = map_.next_value()?;
                        }
                        GeneratedField::Capabilities => {
                            if capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capabilities"));
                            }
                            capabilities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Requires => {
                            if requires__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requires"));
                            }
                            requires__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequiresCapabilities => {
                            if requires_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "requiresCapabilities",
                                ));
                            }
                            requires_capabilities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cron => {
                            if cron__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cron"));
                            }
                            cron__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NetworkHosts => {
                            if network_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkHosts"));
                            }
                            network_hosts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Dns01 => {
                            if dns01__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dns01"));
                            }
                            dns01__ = map_.next_value()?;
                        }
                        GeneratedField::Http => {
                            if http__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            http__ = map_.next_value()?;
                        }
                        GeneratedField::SettingsSchema => {
                            if settings_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settingsSchema"));
                            }
                            settings_schema__ = map_.next_value()?;
                        }
                        GeneratedField::Notify => {
                            if notify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notify"));
                            }
                            notify__ = map_.next_value()?;
                        }
                        GeneratedField::Probe => {
                            if probe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("probe"));
                            }
                            probe__ = map_.next_value()?;
                        }
                        GeneratedField::Mcp => {
                            if mcp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mcp"));
                            }
                            mcp__ = map_.next_value()?;
                        }
                        GeneratedField::Storage => {
                            if storage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storage"));
                            }
                            storage__ = map_.next_value()?;
                        }
                        GeneratedField::Deploy => {
                            if deploy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deploy"));
                            }
                            deploy__ = map_.next_value()?;
                        }
                        GeneratedField::Blocklist => {
                            if blocklist__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocklist"));
                            }
                            blocklist__ = map_.next_value()?;
                        }
                        GeneratedField::Discovery => {
                            if discovery__.is_some() {
                                return Err(serde::de::Error::duplicate_field("discovery"));
                            }
                            discovery__ = map_.next_value()?;
                        }
                        GeneratedField::LogSink => {
                            if log_sink__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logSink"));
                            }
                            log_sink__ = map_.next_value()?;
                        }
                        GeneratedField::I18n => {
                            if i18n__.is_some() {
                                return Err(serde::de::Error::duplicate_field("i18n"));
                            }
                            i18n__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Conflicts => {
                            if conflicts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conflicts"));
                            }
                            conflicts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PermissionReasons => {
                            if permission_reasons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionReasons"));
                            }
                            permission_reasons__ =
                                Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Manifest {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    homepage_url: homepage_url__.unwrap_or_default(),
                    icon_path: icon_path__.unwrap_or_default(),
                    api_version: api_version__.unwrap_or_default(),
                    min_nginx_ui_version: min_nginx_ui_version__.unwrap_or_default(),
                    server: server__,
                    webapp: webapp__,
                    content: content__,
                    capabilities: capabilities__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                    requires: requires__.unwrap_or_default(),
                    requires_capabilities: requires_capabilities__.unwrap_or_default(),
                    events: events__.unwrap_or_default(),
                    cron: cron__.unwrap_or_default(),
                    network_hosts: network_hosts__.unwrap_or_default(),
                    dns01: dns01__,
                    http: http__,
                    settings_schema: settings_schema__,
                    notify: notify__,
                    probe: probe__,
                    mcp: mcp__,
                    storage: storage__,
                    deploy: deploy__,
                    blocklist: blocklist__,
                    discovery: discovery__,
                    log_sink: log_sink__,
                    i18n: i18n__.unwrap_or_default(),
                    conflicts: conflicts__.unwrap_or_default(),
                    permission_reasons: permission_reasons__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.Manifest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManifestBlocklist {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sources.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestBlocklist", len)?;
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestBlocklist {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sources"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sources,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sources" => Ok(GeneratedField::Sources),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestBlocklist;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestBlocklist")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestBlocklist, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestBlocklist {
                    sources: sources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestBlocklist",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestContent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.templates.is_empty() {
            len += 1;
        }
        if !self.locales.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestContent", len)?;
        if !self.templates.is_empty() {
            struct_ser.serialize_field("templates", &self.templates)?;
        }
        if !self.locales.is_empty() {
            struct_ser.serialize_field("locales", &self.locales)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestContent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["templates", "locales"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Templates,
            Locales,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "templates" => Ok(GeneratedField::Templates),
                            "locales" => Ok(GeneratedField::Locales),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestContent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestContent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestContent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut templates__ = None;
                let mut locales__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Templates => {
                            if templates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("templates"));
                            }
                            templates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Locales => {
                            if locales__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locales"));
                            }
                            locales__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestContent {
                    templates: templates__.unwrap_or_default(),
                    locales: locales__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestContent",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestCron {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.schedule.is_empty() {
            len += 1;
        }
        if !self.method.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.ManifestCron", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.schedule.is_empty() {
            struct_ser.serialize_field("schedule", &self.schedule)?;
        }
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestCron {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "schedule", "method"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Schedule,
            Method,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "schedule" => Ok(GeneratedField::Schedule),
                            "method" => Ok(GeneratedField::Method),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestCron;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestCron")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestCron, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut schedule__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schedule => {
                            if schedule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            schedule__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestCron {
                    id: id__.unwrap_or_default(),
                    schedule: schedule__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.ManifestCron", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManifestDns01 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.providers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.ManifestDNS01", len)?;
        if !self.providers.is_empty() {
            struct_ser.serialize_field("providers", &self.providers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestDns01 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["providers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Providers,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "providers" => Ok(GeneratedField::Providers),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestDns01;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestDNS01")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestDns01, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut providers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Providers => {
                            if providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providers"));
                            }
                            providers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestDns01 {
                    providers: providers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.ManifestDNS01", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManifestDeploy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.targets.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestDeploy", len)?;
        if !self.targets.is_empty() {
            struct_ser.serialize_field("targets", &self.targets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestDeploy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["targets"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Targets,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "targets" => Ok(GeneratedField::Targets),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestDeploy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestDeploy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestDeploy, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut targets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Targets => {
                            if targets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targets"));
                            }
                            targets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestDeploy {
                    targets: targets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestDeploy",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestDiscovery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.providers.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestDiscovery", len)?;
        if !self.providers.is_empty() {
            struct_ser.serialize_field("providers", &self.providers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestDiscovery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["providers"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Providers,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "providers" => Ok(GeneratedField::Providers),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestDiscovery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestDiscovery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestDiscovery, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut providers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Providers => {
                            if providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providers"));
                            }
                            providers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestDiscovery {
                    providers: providers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestDiscovery",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestHttp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.listen.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.ManifestHTTP", len)?;
        if !self.listen.is_empty() {
            struct_ser.serialize_field("listen", &self.listen)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestHttp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["listen"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Listen,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "listen" => Ok(GeneratedField::Listen),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestHttp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestHTTP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestHttp, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut listen__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Listen => {
                            if listen__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listen"));
                            }
                            listen__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestHttp {
                    listen: listen__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.ManifestHTTP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManifestI18n {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.permission_reasons.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.ManifestI18n", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.permission_reasons.is_empty() {
            struct_ser.serialize_field("permission_reasons", &self.permission_reasons)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestI18n {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "permission_reasons",
            "permissionReasons",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            PermissionReasons,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "permissionReasons" | "permission_reasons" => {
                                Ok(GeneratedField::PermissionReasons)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestI18n;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestI18n")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestI18n, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut permission_reasons__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PermissionReasons => {
                            if permission_reasons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionReasons"));
                            }
                            permission_reasons__ =
                                Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestI18n {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    permission_reasons: permission_reasons__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.ManifestI18n", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManifestLogSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch_size != 0 {
            len += 1;
        }
        if self.flush_interval_ms != 0 {
            len += 1;
        }
        if !self.formats.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestLogSink", len)?;
        if self.batch_size != 0 {
            struct_ser.serialize_field("batch_size", &self.batch_size)?;
        }
        if self.flush_interval_ms != 0 {
            struct_ser.serialize_field("flush_interval_ms", &self.flush_interval_ms)?;
        }
        if !self.formats.is_empty() {
            struct_ser.serialize_field("formats", &self.formats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestLogSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_size",
            "batchSize",
            "flush_interval_ms",
            "flushIntervalMs",
            "formats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchSize,
            FlushIntervalMs,
            Formats,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "batchSize" | "batch_size" => Ok(GeneratedField::BatchSize),
                            "flushIntervalMs" | "flush_interval_ms" => {
                                Ok(GeneratedField::FlushIntervalMs)
                            }
                            "formats" => Ok(GeneratedField::Formats),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestLogSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestLogSink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestLogSink, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut batch_size__ = None;
                let mut flush_interval_ms__ = None;
                let mut formats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchSize => {
                            if batch_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchSize"));
                            }
                            batch_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FlushIntervalMs => {
                            if flush_interval_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flushIntervalMs"));
                            }
                            flush_interval_ms__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Formats => {
                            if formats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("formats"));
                            }
                            formats__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestLogSink {
                    batch_size: batch_size__.unwrap_or_default(),
                    flush_interval_ms: flush_interval_ms__.unwrap_or_default(),
                    formats: formats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestLogSink",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestMcp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tools.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.ManifestMCP", len)?;
        if !self.tools.is_empty() {
            struct_ser.serialize_field("tools", &self.tools)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestMcp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["tools"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tools,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tools" => Ok(GeneratedField::Tools),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestMcp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestMCP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestMcp, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tools__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tools => {
                            if tools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tools"));
                            }
                            tools__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestMcp {
                    tools: tools__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.ManifestMCP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManifestNotify {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channels.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestNotify", len)?;
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestNotify {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["channels"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channels,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "channels" => Ok(GeneratedField::Channels),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestNotify;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestNotify")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestNotify, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestNotify {
                    channels: channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestNotify",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestPage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.icon.is_empty() {
            len += 1;
        }
        if !self.file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.ManifestPage", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.icon.is_empty() {
            struct_ser.serialize_field("icon", &self.icon)?;
        }
        if !self.file.is_empty() {
            struct_ser.serialize_field("file", &self.file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestPage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["path", "title", "icon", "file"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Title,
            Icon,
            File,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "title" => Ok(GeneratedField::Title),
                            "icon" => Ok(GeneratedField::Icon),
                            "file" => Ok(GeneratedField::File),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestPage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestPage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestPage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut title__ = None;
                let mut icon__ = None;
                let mut file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Icon => {
                            if icon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("icon"));
                            }
                            icon__ = Some(map_.next_value()?);
                        }
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestPage {
                    path: path__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    icon: icon__.unwrap_or_default(),
                    file: file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.ManifestPage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManifestProbe {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kinds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.ManifestProbe", len)?;
        if !self.kinds.is_empty() {
            struct_ser.serialize_field("kinds", &self.kinds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestProbe {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["kinds"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kinds,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kinds" => Ok(GeneratedField::Kinds),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestProbe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestProbe")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestProbe, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut kinds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kinds => {
                            if kinds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kinds"));
                            }
                            kinds__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestProbe {
                    kinds: kinds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.ManifestProbe", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ManifestRequirement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestRequirement", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestRequirement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "version"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Version,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "version" => Ok(GeneratedField::Version),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestRequirement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestRequirement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestRequirement, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestRequirement {
                    id: id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestRequirement",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestResources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.memory_mb != 0 {
            len += 1;
        }
        if self.cpu_percent != 0 {
            len += 1;
        }
        if self.recommended_memory_mb != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestResources", len)?;
        if self.memory_mb != 0 {
            struct_ser.serialize_field("memory_mb", &self.memory_mb)?;
        }
        if self.cpu_percent != 0 {
            struct_ser.serialize_field("cpu_percent", &self.cpu_percent)?;
        }
        if self.recommended_memory_mb != 0 {
            struct_ser.serialize_field("recommended_memory_mb", &self.recommended_memory_mb)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestResources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "memory_mb",
            "memoryMb",
            "cpu_percent",
            "cpuPercent",
            "recommended_memory_mb",
            "recommendedMemoryMb",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MemoryMb,
            CpuPercent,
            RecommendedMemoryMb,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "memoryMb" | "memory_mb" => Ok(GeneratedField::MemoryMb),
                            "cpuPercent" | "cpu_percent" => Ok(GeneratedField::CpuPercent),
                            "recommendedMemoryMb" | "recommended_memory_mb" => {
                                Ok(GeneratedField::RecommendedMemoryMb)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestResources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestResources")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestResources, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut memory_mb__ = None;
                let mut cpu_percent__ = None;
                let mut recommended_memory_mb__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MemoryMb => {
                            if memory_mb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryMb"));
                            }
                            memory_mb__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CpuPercent => {
                            if cpu_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuPercent"));
                            }
                            cpu_percent__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RecommendedMemoryMb => {
                            if recommended_memory_mb__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "recommendedMemoryMb",
                                ));
                            }
                            recommended_memory_mb__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestResources {
                    memory_mb: memory_mb__.unwrap_or_default(),
                    cpu_percent: cpu_percent__.unwrap_or_default(),
                    recommended_memory_mb: recommended_memory_mb__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestResources",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestServer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.executables.is_empty() {
            len += 1;
        }
        if !self.command.is_empty() {
            len += 1;
        }
        if !self.lifecycle.is_empty() {
            len += 1;
        }
        if self.idle_timeout_seconds != 0 {
            len += 1;
        }
        if self.resources.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestServer", len)?;
        if !self.executables.is_empty() {
            struct_ser.serialize_field("executables", &self.executables)?;
        }
        if !self.command.is_empty() {
            struct_ser.serialize_field("command", &self.command)?;
        }
        if !self.lifecycle.is_empty() {
            struct_ser.serialize_field("lifecycle", &self.lifecycle)?;
        }
        if self.idle_timeout_seconds != 0 {
            struct_ser.serialize_field("idle_timeout_seconds", &self.idle_timeout_seconds)?;
        }
        if let Some(v) = self.resources.as_ref() {
            struct_ser.serialize_field("resources", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestServer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "executables",
            "command",
            "lifecycle",
            "idle_timeout_seconds",
            "idleTimeoutSeconds",
            "resources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Executables,
            Command,
            Lifecycle,
            IdleTimeoutSeconds,
            Resources,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "executables" => Ok(GeneratedField::Executables),
                            "command" => Ok(GeneratedField::Command),
                            "lifecycle" => Ok(GeneratedField::Lifecycle),
                            "idleTimeoutSeconds" | "idle_timeout_seconds" => {
                                Ok(GeneratedField::IdleTimeoutSeconds)
                            }
                            "resources" => Ok(GeneratedField::Resources),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestServer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestServer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestServer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut executables__ = None;
                let mut command__ = None;
                let mut lifecycle__ = None;
                let mut idle_timeout_seconds__ = None;
                let mut resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Executables => {
                            if executables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executables"));
                            }
                            executables__ =
                                Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Command => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("command"));
                            }
                            command__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Lifecycle => {
                            if lifecycle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycle"));
                            }
                            lifecycle__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdleTimeoutSeconds => {
                            if idle_timeout_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "idleTimeoutSeconds",
                                ));
                            }
                            idle_timeout_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestServer {
                    executables: executables__.unwrap_or_default(),
                    command: command__.unwrap_or_default(),
                    lifecycle: lifecycle__.unwrap_or_default(),
                    idle_timeout_seconds: idle_timeout_seconds__.unwrap_or_default(),
                    resources: resources__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestServer",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestStorage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backends.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestStorage", len)?;
        if !self.backends.is_empty() {
            struct_ser.serialize_field("backends", &self.backends)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestStorage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["backends"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Backends,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "backends" => Ok(GeneratedField::Backends),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestStorage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestStorage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestStorage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut backends__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Backends => {
                            if backends__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backends"));
                            }
                            backends__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestStorage {
                    backends: backends__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestStorage",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ManifestWebapp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bundle_path.is_empty() {
            len += 1;
        }
        if !self.style_path.is_empty() {
            len += 1;
        }
        if !self.shared.is_empty() {
            len += 1;
        }
        if !self.pages.is_empty() {
            len += 1;
        }
        if !self.chunks.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ManifestWebapp", len)?;
        if !self.bundle_path.is_empty() {
            struct_ser.serialize_field("bundle_path", &self.bundle_path)?;
        }
        if !self.style_path.is_empty() {
            struct_ser.serialize_field("style_path", &self.style_path)?;
        }
        if !self.shared.is_empty() {
            struct_ser.serialize_field("shared", &self.shared)?;
        }
        if !self.pages.is_empty() {
            struct_ser.serialize_field("pages", &self.pages)?;
        }
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ManifestWebapp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bundle_path",
            "bundlePath",
            "style_path",
            "stylePath",
            "shared",
            "pages",
            "chunks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BundlePath,
            StylePath,
            Shared,
            Pages,
            Chunks,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bundlePath" | "bundle_path" => Ok(GeneratedField::BundlePath),
                            "stylePath" | "style_path" => Ok(GeneratedField::StylePath),
                            "shared" => Ok(GeneratedField::Shared),
                            "pages" => Ok(GeneratedField::Pages),
                            "chunks" => Ok(GeneratedField::Chunks),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ManifestWebapp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ManifestWebapp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ManifestWebapp, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut bundle_path__ = None;
                let mut style_path__ = None;
                let mut shared__ = None;
                let mut pages__ = None;
                let mut chunks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BundlePath => {
                            if bundle_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bundlePath"));
                            }
                            bundle_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StylePath => {
                            if style_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stylePath"));
                            }
                            style_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Shared => {
                            if shared__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shared"));
                            }
                            shared__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Pages => {
                            if pages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pages"));
                            }
                            pages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Chunks => {
                            if chunks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ManifestWebapp {
                    bundle_path: bundle_path__.unwrap_or_default(),
                    style_path: style_path__.unwrap_or_default(),
                    shared: shared__.unwrap_or_default(),
                    pages: pages__.unwrap_or_default(),
                    chunks: chunks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ManifestWebapp",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for NotifyChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.NotifyChannel", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotifyChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code", "name", "configuration"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Name,
            Configuration,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "name" => Ok(GeneratedField::Name),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotifyChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.NotifyChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotifyChannel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut name__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NotifyChannel {
                    code: code__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.NotifyChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotifySendRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if !self.severity.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.NotifySendRequest", len)?;
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if !self.severity.is_empty() {
            struct_ser.serialize_field("severity", &self.severity)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotifySendRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["channel", "config", "title", "content", "severity"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
            Config,
            Title,
            Content,
            Severity,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "channel" => Ok(GeneratedField::Channel),
                            "config" => Ok(GeneratedField::Config),
                            "title" => Ok(GeneratedField::Title),
                            "content" => Ok(GeneratedField::Content),
                            "severity" => Ok(GeneratedField::Severity),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotifySendRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.NotifySendRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotifySendRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                let mut config__ = None;
                let mut title__ = None;
                let mut content__ = None;
                let mut severity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Severity => {
                            if severity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            severity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NotifySendRequest {
                    channel: channel__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    severity: severity__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.NotifySendRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for NotifySendResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.NotifySendResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotifySendResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotifySendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.NotifySendResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotifySendResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(NotifySendResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.NotifySendResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for NotifyValidateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.NotifyValidateRequest", len)?;
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotifyValidateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["channel", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
            Config,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "channel" => Ok(GeneratedField::Channel),
                            "config" => Ok(GeneratedField::Config),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotifyValidateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.NotifyValidateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<NotifyValidateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(NotifyValidateRequest {
                    channel: channel__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.NotifyValidateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for NotifyValidateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.NotifyValidateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotifyValidateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotifyValidateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.NotifyValidateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<NotifyValidateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(NotifyValidateResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.NotifyValidateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginConfigureRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginConfigureRequest", len)?;
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginConfigureRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["settings"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Settings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginConfigureRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginConfigureRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PluginConfigureRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PluginConfigureRequest {
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginConfigureRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginConfigureResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginConfigureResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginConfigureResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginConfigureResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginConfigureResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PluginConfigureResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginConfigureResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginConfigureResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.PluginError", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code", "message", "data"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
            Data,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            "data" => Ok(GeneratedField::Data),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PluginError, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PluginError {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.PluginError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PluginExitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nginxui.plugin.v1.PluginExitRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginExitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginExitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginExitRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PluginExitRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginExitRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginExitRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginExitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginExitResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginExitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginExitResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginExitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PluginExitResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginExitResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginExitResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginInitializeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host.is_some() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginInitializeRequest", len)?;
        if let Some(v) = self.host.as_ref() {
            struct_ser.serialize_field("host", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginInitializeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["host", "settings", "permissions"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Host,
            Settings,
            Permissions,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "host" => Ok(GeneratedField::Host),
                            "settings" => Ok(GeneratedField::Settings),
                            "permissions" => Ok(GeneratedField::Permissions),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginInitializeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginInitializeRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PluginInitializeRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut host__ = None;
                let mut settings__ = None;
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PluginInitializeRequest {
                    host: host__,
                    settings: settings__,
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginInitializeRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginInitializeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.api_version != 0 {
            len += 1;
        }
        if !self.capabilities.is_empty() {
            len += 1;
        }
        if !self.transports.is_empty() {
            len += 1;
        }
        if self.http_port != 0 {
            len += 1;
        }
        if self.rpc_port != 0 {
            len += 1;
        }
        if !self.rpc_token.is_empty() {
            len += 1;
        }
        if !self.rpc_socket.is_empty() {
            len += 1;
        }
        if !self.http_pipe.is_empty() {
            len += 1;
        }
        if !self.rpc_pipe.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginInitializeResponse", len)?;
        if self.api_version != 0 {
            struct_ser.serialize_field("api_version", &self.api_version)?;
        }
        if !self.capabilities.is_empty() {
            struct_ser.serialize_field("capabilities", &self.capabilities)?;
        }
        if !self.transports.is_empty() {
            struct_ser.serialize_field("transports", &self.transports)?;
        }
        if self.http_port != 0 {
            struct_ser.serialize_field("http_port", &self.http_port)?;
        }
        if self.rpc_port != 0 {
            struct_ser.serialize_field("rpc_port", &self.rpc_port)?;
        }
        if !self.rpc_token.is_empty() {
            struct_ser.serialize_field("rpc_token", &self.rpc_token)?;
        }
        if !self.rpc_socket.is_empty() {
            struct_ser.serialize_field("rpc_socket", &self.rpc_socket)?;
        }
        if !self.http_pipe.is_empty() {
            struct_ser.serialize_field("http_pipe", &self.http_pipe)?;
        }
        if !self.rpc_pipe.is_empty() {
            struct_ser.serialize_field("rpc_pipe", &self.rpc_pipe)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginInitializeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "api_version",
            "apiVersion",
            "capabilities",
            "transports",
            "http_port",
            "httpPort",
            "rpc_port",
            "rpcPort",
            "rpc_token",
            "rpcToken",
            "rpc_socket",
            "rpcSocket",
            "http_pipe",
            "httpPipe",
            "rpc_pipe",
            "rpcPipe",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ApiVersion,
            Capabilities,
            Transports,
            HttpPort,
            RpcPort,
            RpcToken,
            RpcSocket,
            HttpPipe,
            RpcPipe,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "apiVersion" | "api_version" => Ok(GeneratedField::ApiVersion),
                            "capabilities" => Ok(GeneratedField::Capabilities),
                            "transports" => Ok(GeneratedField::Transports),
                            "httpPort" | "http_port" => Ok(GeneratedField::HttpPort),
                            "rpcPort" | "rpc_port" => Ok(GeneratedField::RpcPort),
                            "rpcToken" | "rpc_token" => Ok(GeneratedField::RpcToken),
                            "rpcSocket" | "rpc_socket" => Ok(GeneratedField::RpcSocket),
                            "httpPipe" | "http_pipe" => Ok(GeneratedField::HttpPipe),
                            "rpcPipe" | "rpc_pipe" => Ok(GeneratedField::RpcPipe),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginInitializeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginInitializeResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PluginInitializeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut api_version__ = None;
                let mut capabilities__ = None;
                let mut transports__ = None;
                let mut http_port__ = None;
                let mut rpc_port__ = None;
                let mut rpc_token__ = None;
                let mut rpc_socket__ = None;
                let mut http_pipe__ = None;
                let mut rpc_pipe__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ApiVersion => {
                            if api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiVersion"));
                            }
                            api_version__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Capabilities => {
                            if capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capabilities"));
                            }
                            capabilities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transports => {
                            if transports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transports"));
                            }
                            transports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HttpPort => {
                            if http_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpPort"));
                            }
                            http_port__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RpcPort => {
                            if rpc_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rpcPort"));
                            }
                            rpc_port__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RpcToken => {
                            if rpc_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rpcToken"));
                            }
                            rpc_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RpcSocket => {
                            if rpc_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rpcSocket"));
                            }
                            rpc_socket__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HttpPipe => {
                            if http_pipe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpPipe"));
                            }
                            http_pipe__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RpcPipe => {
                            if rpc_pipe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rpcPipe"));
                            }
                            rpc_pipe__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PluginInitializeResponse {
                    api_version: api_version__.unwrap_or_default(),
                    capabilities: capabilities__.unwrap_or_default(),
                    transports: transports__.unwrap_or_default(),
                    http_port: http_port__.unwrap_or_default(),
                    rpc_port: rpc_port__.unwrap_or_default(),
                    rpc_token: rpc_token__.unwrap_or_default(),
                    rpc_socket: rpc_socket__.unwrap_or_default(),
                    http_pipe: http_pipe__.unwrap_or_default(),
                    rpc_pipe: rpc_pipe__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginInitializeResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginInitializedRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginInitializedRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginInitializedRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginInitializedRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginInitializedRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PluginInitializedRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginInitializedRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginInitializedRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginInitializedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginInitializedResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginInitializedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginInitializedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginInitializedResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PluginInitializedResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginInitializedResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginInitializedResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginPingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nginxui.plugin.v1.PluginPingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginPingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginPingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginPingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PluginPingRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginPingRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginPingRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginPingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginPingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginPingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginPingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginPingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PluginPingResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginPingResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginPingResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginShutdownRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginShutdownRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginShutdownRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginShutdownRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginShutdownRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PluginShutdownRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginShutdownRequest {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginShutdownRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for PluginShutdownResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.PluginShutdownResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginShutdownResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginShutdownResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.PluginShutdownResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<PluginShutdownResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PluginShutdownResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.PluginShutdownResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ProbeCheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.target.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if self.timeout_seconds != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ProbeCheckRequest", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if self.timeout_seconds != 0 {
            struct_ser.serialize_field("timeout_seconds", &self.timeout_seconds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProbeCheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "target",
            "config",
            "timeout_seconds",
            "timeoutSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Target,
            Config,
            TimeoutSeconds,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "target" => Ok(GeneratedField::Target),
                            "config" => Ok(GeneratedField::Config),
                            "timeoutSeconds" | "timeout_seconds" => {
                                Ok(GeneratedField::TimeoutSeconds)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProbeCheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ProbeCheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProbeCheckRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut target__ = None;
                let mut config__ = None;
                let mut timeout_seconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::TimeoutSeconds => {
                            if timeout_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutSeconds"));
                            }
                            timeout_seconds__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ProbeCheckRequest {
                    kind: kind__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    timeout_seconds: timeout_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ProbeCheckRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ProbeCheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.status.is_empty() {
            len += 1;
        }
        if self.latency_ms != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.ProbeCheckResponse", len)?;
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if self.latency_ms != 0 {
            struct_ser.serialize_field("latency_ms", &self.latency_ms)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProbeCheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["status", "latency_ms", "latencyMs", "message"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            LatencyMs,
            Message,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "latencyMs" | "latency_ms" => Ok(GeneratedField::LatencyMs),
                            "message" => Ok(GeneratedField::Message),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProbeCheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ProbeCheckResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProbeCheckResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut latency_ms__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LatencyMs => {
                            if latency_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latencyMs"));
                            }
                            latency_ms__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ProbeCheckResponse {
                    status: status__.unwrap_or_default(),
                    latency_ms: latency_ms__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.ProbeCheckResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for ProbeKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.ProbeKind", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProbeKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code", "name", "configuration"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Name,
            Configuration,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "name" => Ok(GeneratedField::Name),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProbeKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.ProbeKind")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProbeKind, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut name__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ProbeKind {
                    code: code__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.ProbeKind", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SettingsField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.help_text.is_empty() {
            len += 1;
        }
        if self.default.is_some() {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        if self.required {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.SettingsField", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("display_name", &self.display_name)?;
        }
        if !self.help_text.is_empty() {
            struct_ser.serialize_field("help_text", &self.help_text)?;
        }
        if let Some(v) = self.default.as_ref() {
            struct_ser.serialize_field("default", v)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        if self.required {
            struct_ser.serialize_field("required", &self.required)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SettingsField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "type",
            "display_name",
            "displayName",
            "help_text",
            "helpText",
            "default",
            "options",
            "required",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Type,
            DisplayName,
            HelpText,
            Default,
            Options,
            Required,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "type" => Ok(GeneratedField::Type),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "helpText" | "help_text" => Ok(GeneratedField::HelpText),
                            "default" => Ok(GeneratedField::Default),
                            "options" => Ok(GeneratedField::Options),
                            "required" => Ok(GeneratedField::Required),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SettingsField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.SettingsField")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SettingsField, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut r#type__ = None;
                let mut display_name__ = None;
                let mut help_text__ = None;
                let mut default__ = None;
                let mut options__ = None;
                let mut required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HelpText => {
                            if help_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("helpText"));
                            }
                            help_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Default => {
                            if default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("default"));
                            }
                            default__ = map_.next_value()?;
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SettingsField {
                    key: key__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    help_text: help_text__.unwrap_or_default(),
                    default: default__,
                    options: options__.unwrap_or_default(),
                    required: required__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.SettingsField", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SettingsOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.label.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.SettingsOption", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.label.is_empty() {
            struct_ser.serialize_field("label", &self.label)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SettingsOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["value", "label"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Label,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "label" => Ok(GeneratedField::Label),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SettingsOption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.SettingsOption")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SettingsOption, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut label__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SettingsOption {
                    value: value__.unwrap_or_default(),
                    label: label__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.SettingsOption",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for SettingsSchema {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header.is_empty() {
            len += 1;
        }
        if !self.footer.is_empty() {
            len += 1;
        }
        if !self.settings.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.SettingsSchema", len)?;
        if !self.header.is_empty() {
            struct_ser.serialize_field("header", &self.header)?;
        }
        if !self.footer.is_empty() {
            struct_ser.serialize_field("footer", &self.footer)?;
        }
        if !self.settings.is_empty() {
            struct_ser.serialize_field("settings", &self.settings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SettingsSchema {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["header", "footer", "settings"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Footer,
            Settings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "header" => Ok(GeneratedField::Header),
                            "footer" => Ok(GeneratedField::Footer),
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SettingsSchema;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.SettingsSchema")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SettingsSchema, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut footer__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Footer => {
                            if footer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("footer"));
                            }
                            footer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SettingsSchema {
                    header: header__.unwrap_or_default(),
                    footer: footer__.unwrap_or_default(),
                    settings: settings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.SettingsSchema",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageBackend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageBackend", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageBackend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["code", "name", "configuration"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Name,
            Configuration,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "name" => Ok(GeneratedField::Name),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageBackend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageBackend")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorageBackend, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut name__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StorageBackend {
                    code: code__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageBackend",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageDeleteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backend.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageDeleteRequest", len)?;
        if !self.backend.is_empty() {
            struct_ser.serialize_field("backend", &self.backend)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageDeleteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["backend", "config", "key"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Backend,
            Config,
            Key,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "backend" => Ok(GeneratedField::Backend),
                            "config" => Ok(GeneratedField::Config),
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageDeleteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageDeleteRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<StorageDeleteRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut backend__ = None;
                let mut config__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StorageDeleteRequest {
                    backend: backend__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageDeleteRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageDeleteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageDeleteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageDeleteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageDeleteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageDeleteResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<StorageDeleteResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StorageDeleteResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageDeleteResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageGetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backend.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.target_path.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageGetRequest", len)?;
        if !self.backend.is_empty() {
            struct_ser.serialize_field("backend", &self.backend)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.target_path.is_empty() {
            struct_ser.serialize_field("target_path", &self.target_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageGetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["backend", "config", "key", "target_path", "targetPath"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Backend,
            Config,
            Key,
            TargetPath,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "backend" => Ok(GeneratedField::Backend),
                            "config" => Ok(GeneratedField::Config),
                            "key" => Ok(GeneratedField::Key),
                            "targetPath" | "target_path" => Ok(GeneratedField::TargetPath),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageGetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageGetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorageGetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut backend__ = None;
                let mut config__ = None;
                let mut key__ = None;
                let mut target_path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetPath => {
                            if target_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPath"));
                            }
                            target_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StorageGetRequest {
                    backend: backend__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    target_path: target_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageGetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageGetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.size != 0. {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageGetResponse", len)?;
        if self.size != 0. {
            struct_ser.serialize_field("size", &self.size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageGetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["size"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Size,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "size" => Ok(GeneratedField::Size),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageGetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageGetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorageGetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StorageGetResponse {
                    size: size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageGetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backend.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if !self.prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageListRequest", len)?;
        if !self.backend.is_empty() {
            struct_ser.serialize_field("backend", &self.backend)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["backend", "config", "prefix"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Backend,
            Config,
            Prefix,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "backend" => Ok(GeneratedField::Backend),
                            "config" => Ok(GeneratedField::Config),
                            "prefix" => Ok(GeneratedField::Prefix),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorageListRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut backend__ = None;
                let mut config__ = None;
                let mut prefix__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StorageListRequest {
                    backend: backend__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    prefix: prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageListRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.objects.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageListResponse", len)?;
        if !self.objects.is_empty() {
            struct_ser.serialize_field("objects", &self.objects)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["objects"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objects,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objects" => Ok(GeneratedField::Objects),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorageListResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut objects__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objects => {
                            if objects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objects"));
                            }
                            objects__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StorageListResponse {
                    objects: objects__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageListResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageObject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.size != 0. {
            len += 1;
        }
        if !self.modified_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nginxui.plugin.v1.StorageObject", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.size != 0. {
            struct_ser.serialize_field("size", &self.size)?;
        }
        if !self.modified_at.is_empty() {
            struct_ser.serialize_field("modified_at", &self.modified_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageObject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "size", "modified_at", "modifiedAt"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Size,
            ModifiedAt,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "size" => Ok(GeneratedField::Size),
                            "modifiedAt" | "modified_at" => Ok(GeneratedField::ModifiedAt),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageObject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageObject")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorageObject, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut size__ = None;
                let mut modified_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ModifiedAt => {
                            if modified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifiedAt"));
                            }
                            modified_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StorageObject {
                    key: key__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                    modified_at: modified_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nginxui.plugin.v1.StorageObject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StoragePutRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backend.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.source_path.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StoragePutRequest", len)?;
        if !self.backend.is_empty() {
            struct_ser.serialize_field("backend", &self.backend)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.source_path.is_empty() {
            struct_ser.serialize_field("source_path", &self.source_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StoragePutRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["backend", "config", "key", "source_path", "sourcePath"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Backend,
            Config,
            Key,
            SourcePath,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "backend" => Ok(GeneratedField::Backend),
                            "config" => Ok(GeneratedField::Config),
                            "key" => Ok(GeneratedField::Key),
                            "sourcePath" | "source_path" => Ok(GeneratedField::SourcePath),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoragePutRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StoragePutRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StoragePutRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut backend__ = None;
                let mut config__ = None;
                let mut key__ = None;
                let mut source_path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourcePath => {
                            if source_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePath"));
                            }
                            source_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StoragePutRequest {
                    backend: backend__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    source_path: source_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StoragePutRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StoragePutResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.size != 0. {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StoragePutResponse", len)?;
        if self.size != 0. {
            struct_ser.serialize_field("size", &self.size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StoragePutResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["size"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Size,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "size" => Ok(GeneratedField::Size),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StoragePutResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StoragePutResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StoragePutResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StoragePutResponse {
                    size: size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StoragePutResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageValidateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backend.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageValidateRequest", len)?;
        if !self.backend.is_empty() {
            struct_ser.serialize_field("backend", &self.backend)?;
        }
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageValidateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["backend", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Backend,
            Config,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "backend" => Ok(GeneratedField::Backend),
                            "config" => Ok(GeneratedField::Config),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageValidateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageValidateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<StorageValidateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut backend__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map_.next_value::<std::collections::BTreeMap<_, _>>()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(StorageValidateRequest {
                    backend: backend__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageValidateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for StorageValidateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("nginxui.plugin.v1.StorageValidateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageValidateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageValidateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nginxui.plugin.v1.StorageValidateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> std::result::Result<StorageValidateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StorageValidateResponse {})
            }
        }
        deserializer.deserialize_struct(
            "nginxui.plugin.v1.StorageValidateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
