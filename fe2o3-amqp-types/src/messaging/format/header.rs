//! Implementation of message header

use serde_amqp::{
    primitives::{Boolean, Uint},
    DeserializeComposite,
};

use crate::definitions::Milliseconds;

use super::Priority;

/// 3.2.1 Header
/// Transport headers for a message.
/// <type name="header" class="composite" source="list" provides="section">
///     <descriptor name="amqp:header:list" code="0x00000000:0x00000070"/>
/// </type>
#[derive(Debug, Clone, Default, DeserializeComposite, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[amqp_contract(
    name = "amqp:header:list",
    code = "0x0000_0000:0x0000_0070",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct Header {
    /// <field name="durable" type="boolean" default="false"/>
    #[amqp_contract(default)]
    pub durable: Boolean,

    /// <field name="priority" type="ubyte" default="4"/>
    #[amqp_contract(default)]
    pub priority: Priority,

    /// <field name="ttl" type="milliseconds"/>
    pub ttl: Option<Milliseconds>,

    /// <field name="first-acquirer" type="boolean" default="false"/>
    #[amqp_contract(default)]
    pub first_acquirer: Boolean,

    /// <field name="delivery-count" type="uint" default="0"/>
    #[amqp_contract(default)]
    pub delivery_count: Uint,
}

/// Manual Serialize implementation for Header.
///
/// The derive macro (`SerializeComposite`) skips fields that equal their
/// AMQP default values when they are trailing in the list encoding.
/// `delivery_count` (default 0) is the last field, so it gets omitted when 0.
/// The Azure .NET SDK requires `delivery_count` to always be present on the
/// wire (it reads it as a nullable and crashes with "Nullable object must
/// have a value" when the field is missing). The real Azure Service Bus
/// always sends this field explicitly.
///
/// This manual implementation always serializes `delivery_count`, while
/// still skipping other trailing default fields per the AMQP spec.
impl serde_amqp::serde::Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_amqp::serde::Serializer,
    {
        use serde_amqp::serde::ser::SerializeStruct;

        // 5 fields + 1 for the descriptor
        let mut state =
            serializer.serialize_struct(serde_amqp::__constants::DESCRIBED_LIST, 5 + 1)?;

        // Serialize descriptor
        state.serialize_field(
            serde_amqp::__constants::DESCRIPTOR,
            &serde_amqp::descriptor::Descriptor::Code(0x0000_0000_0000_0070u64),
        )?;

        // For the first 4 fields, we use the same trailing-null-skip logic
        // as the derive macro: buffer default/None fields and only flush
        // them when a non-default field follows.
        let mut nulls: Vec<&str> = Vec::new();

        // durable (default = false)
        if self.durable != <Boolean as Default>::default() {
            for field_name in nulls.drain(..) {
                state.serialize_field(field_name, &())?;
            }
            state.serialize_field("durable", &self.durable)?;
        } else {
            nulls.push("durable");
        }

        // priority (default = 4)
        if self.priority != <Priority as Default>::default() {
            for field_name in nulls.drain(..) {
                state.serialize_field(field_name, &())?;
            }
            state.serialize_field("priority", &self.priority)?;
        } else {
            nulls.push("priority");
        }

        // ttl (Option — buffer if None)
        if self.ttl.is_some() {
            for field_name in nulls.drain(..) {
                state.serialize_field(field_name, &())?;
            }
            state.serialize_field("ttl", &self.ttl)?;
        } else {
            nulls.push("ttl");
        }

        // first_acquirer (default = false)
        if self.first_acquirer != <Boolean as Default>::default() {
            for field_name in nulls.drain(..) {
                state.serialize_field(field_name, &())?;
            }
            state.serialize_field("first-acquirer", &self.first_acquirer)?;
        } else {
            nulls.push("first-acquirer");
        }

        // delivery_count — ALWAYS serialize (flush any buffered nulls first)
        for field_name in nulls.drain(..) {
            state.serialize_field(field_name, &())?;
        }
        state.serialize_field("delivery-count", &self.delivery_count)?;

        state.end()
    }
}

impl Header {
    /// Creates a builder for header
    pub fn builder() -> Builder {
        Default::default()
    }
}

/// Builder for [`Header`]
#[derive(Debug, Default, Clone)]
pub struct Builder {
    inner: Header,
}

impl Builder {
    /// Set the `durable` field of [`Header`]
    pub fn durable(mut self, value: Boolean) -> Self {
        self.inner.durable = value;
        self
    }

    /// Set the `priority` field of [`Header`]
    pub fn priority(mut self, value: impl Into<Priority>) -> Self {
        self.inner.priority = value.into();
        self
    }

    /// Set the `ttl` field of [`Header`]
    pub fn ttl(mut self, value: impl Into<Option<Milliseconds>>) -> Self {
        self.inner.ttl = value.into();
        self
    }

    /// Set the `first_acquirer` field of [`Header`]
    pub fn first_acquirer(mut self, value: Boolean) -> Self {
        self.inner.first_acquirer = value;
        self
    }

    /// Set teh `delivery_count` field of [`Header`]
    pub fn delivery_count(mut self, value: Uint) -> Self {
        self.inner.delivery_count = value;
        self
    }

    /// Builds the [`Header`]
    pub fn build(self) -> Header {
        self.inner
    }
}

impl From<Builder> for Header {
    fn from(builder: Builder) -> Self {
        builder.build()
    }
}

impl From<Builder> for Option<Header> {
    fn from(builder: Builder) -> Self {
        Some(builder.build())
    }
}
