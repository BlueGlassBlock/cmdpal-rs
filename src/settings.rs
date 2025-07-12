//! Types for building extension settings page.

use crate::bindings::*;
use crate::page::content::ContentPage;

use crate::utils::ComBuilder;
use serde_json::{Map, Value as JsonValue, json};
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{E_FAIL, ERROR_FILE_INVALID};
use windows_core::{ComObject, Error, implement};

/// A raw implementation of the [`ICommandSettings`] interface.
#[implement(ICommandSettings)]
pub struct CommandSettings(pub ComObject<ContentPage>);

impl ICommandSettings_Impl for CommandSettings_Impl {
    fn SettingsPage(&self) -> windows_core::Result<IContentPage> {
        Ok(self.0.to_interface())
    }
}

#[derive(Debug, Clone)]
struct BasePropSetting {
    id: String,
    is_required: bool,
    error_message: String,
    title: String,
    label: String,
}

impl BasePropSetting {
    fn new(id: impl ToString) -> Self {
        Self {
            id: id.to_string(),
            is_required: false,
            error_message: String::new(),
            title: String::new(),
            label: String::new(),
        }
    }

    fn serialize(&self) -> Map<String, JsonValue> {
        let mut map = Map::new();
        map.insert("id".into(), json!(self.id));
        map.insert("isRequired".into(), json!(self.is_required));
        map.insert("errorMessage".into(), json!(self.error_message));
        map.insert("title".into(), json!(self.title));
        map.insert("label".into(), json!(self.label));
        map
    }
}

trait SettingItem {
    fn id(&self) -> &str;
    fn base_prop_mut(&mut self) -> &mut BasePropSetting;
    fn serialize_adaptive_card(&self) -> serde_json::Value;
    fn serialize_value(&self) -> Option<serde_json::Value>;
    fn update(&self, data: &Map<String, JsonValue>);
}

/// Helper trait to modify base properties of a setting item.
pub trait SettingBasePropModifier {
    /// Specify whether this setting is required.
    fn is_required(self, is_required: bool) -> Self;
    /// Specify an error message to show when the setting is invalid.
    fn error_message(self, error_message: impl ToString) -> Self;
    /// Caption which will be shown next to the input.
    fn caption(self, caption: impl ToString) -> Self;
    /// Descriptive text shown for the setting item.
    fn description(self, description: impl ToString) -> Self;
}

impl<T> SettingBasePropModifier for T
where
    T: SettingItem,
{
    fn caption(mut self, caption: impl ToString) -> Self {
        self.base_prop_mut().label = caption.to_string();
        self
    }

    fn error_message(mut self, error_message: impl ToString) -> Self {
        self.base_prop_mut().error_message = error_message.to_string();
        self
    }

    fn description(mut self, description: impl ToString) -> Self {
        self.base_prop_mut().title = description.to_string();
        self
    }

    fn is_required(mut self, is_required: bool) -> Self {
        self.base_prop_mut().is_required = is_required;
        self
    }
}

/// A setting item that allows users to input text.
#[derive(Debug, Clone)]
pub struct TextSetting {
    placeholder: String,
    pattern: String,
    is_multiline: bool,
    base_prop: BasePropSetting,
    value: Arc<Mutex<Option<String>>>,
}

impl TextSetting {
    /// Creates a new `TextSetting` with the given ID.
    /// 
    /// The ID should be unique across all settings in this `CommandSettings`.
    pub fn new(id: impl ToString) -> Self {
        Self {
            placeholder: String::new(),
            pattern: String::new(),
            is_multiline: false,
            base_prop: BasePropSetting::new(id),
            value: Arc::new(Mutex::new(None)),
        }
    }

    /// Sets the placeholder text for the input field.
    pub fn placeholder(mut self, placeholder: impl ToString) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    /// Sets the regex pattern that the input must match.
    pub fn pattern(mut self, pattern: impl ToString) -> Self {
        self.pattern = pattern.to_string();
        self
    }

    /// Specifies whether the input field should allow multiple lines.
    pub fn is_multiline(mut self, is_multiline: bool) -> Self {
        self.is_multiline = is_multiline;
        self
    }

    /// Specifies a default value for the input field.
    /// This will be used when there are no prior settings saved.
    pub fn default(self, value: impl ToString) -> Self {
        self.value
            .lock()
            .ok()
            .map(|mut v| v.replace(value.to_string()));
        self
    }
}

/// A setting item that allows users to input a number.
/// 
/// The number input will always be a floating point value.
#[derive(Debug, Clone)]
pub struct NumberSetting {
    placeholder: String,
    min: Option<f64>,
    max: Option<f64>,
    base_prop: BasePropSetting,
    value: Arc<Mutex<Option<f64>>>,
}

impl NumberSetting {
    /// Creates a new `NumberSetting` with the given ID.
    /// 
    /// The ID should be unique across all settings in this `CommandSettings`.
    pub fn new(id: impl ToString) -> Self {
        Self {
            placeholder: String::new(),
            min: None,
            max: None,
            base_prop: BasePropSetting::new(id),
            value: Arc::new(Mutex::new(None)),
        }
    }

    /// Sets the placeholder text for the input field.
    /// Doesn't have to be a valid number.
    pub fn placeholder(mut self, placeholder: impl ToString) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    /// Sets the minimum value for the input field.
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Sets the maximum value for the input field.
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Specifies a default value for the input field.
    /// 
    /// This will be used when there are no prior settings saved.
    pub fn default(self, value: f64) -> Self {
        self.value.lock().ok().map(|mut v| v.replace(value));
        self
    }
}

/// A setting item that allows users to toggle a boolean value.
/// 
/// This is typically used for settings that can be enabled or disabled, like a switch.
#[derive(Debug, Clone)]
pub struct ToggleSetting {
    base_prop: BasePropSetting,
    value: Arc<Mutex<Option<bool>>>,
}

impl ToggleSetting {
    /// Creates a new `ToggleSetting` with the given ID.
    /// 
    /// The ID should be unique across all settings in this `CommandSettings`.
    pub fn new(id: impl ToString) -> Self {
        Self {
            base_prop: BasePropSetting::new(id),
            value: Arc::new(Mutex::new(None)),
        }
    }

    /// Specifies a default value for the toggle setting.
    /// 
    /// This will be used when there are no prior settings saved.
    pub fn default(self, value: bool) -> Self {
        self.value.lock().ok().map(|mut v| v.replace(value));
        self
    }
}

/// A trait for defining choices in a choice set setting.
/// 
/// The trait has already been implemented for `String`, `&str`, and tuples of `(&str, &str)`.
/// 
/// You can implement this trait for your own types to use them in a [`ChoiceSetSetting`]
/// (as long as their `value` don't collide with each other):
/// 
/// ```rust
/// # use cmdpal::settings::Choice;
/// 
/// #[derive(Debug, Clone)]
/// enum TextSize {
///     Small,
///     Medium,
///     Large
/// }
/// 
/// impl Choice for TextSize {
///     fn value(&self) -> &str {
///         match self {
///             TextSize::Small => "small",
///             TextSize::Medium => "medium",
///             TextSize::Large => "large",
///         }
///     }
///     fn title(&self) -> &str {
///         match self {
///            TextSize::Small => "Small",
///            TextSize::Medium => "Medium",
///           TextSize::Large => "Large",
///        }
///     }   
/// }
/// ```
pub trait Choice: Sized + Clone + 'static {
    /// Returns the value of the choice.
    /// 
    /// This value should be unique across all choices in the set.
    fn value(&self) -> &str;

    /// Returns the title of the choice.
    fn title(&self) -> &str;
}

impl Choice for String {
    fn value(&self) -> &str {
        self
    }

    fn title(&self) -> &str {
        self
    }
}

impl Choice for (&'static str, &'static str) {
    fn value(&self) -> &str {
        self.0
    }

    fn title(&self) -> &str {
        self.1
    }
}

impl Choice for &'static str {
    fn value(&self) -> &str {
        self
    }

    fn title(&self) -> &str {
        self
    }
}

/// A setting item that allows users to select one from a set of choices.
#[derive(Debug, Clone)]
pub struct ChoiceSetSetting<T> {
    choices: Vec<T>,
    base_prop: BasePropSetting,
    value: Arc<Mutex<Option<T>>>,
}

impl<T: Choice> ChoiceSetSetting<T> {
    /// Creates a new `ChoiceSetSetting` with the given ID.
    /// 
    /// The ID should be unique across all settings in this `CommandSettings`.
    pub fn new(id: impl ToString) -> Self {
        Self {
            choices: Vec::new(),
            base_prop: BasePropSetting::new(id),
            value: Arc::new(Mutex::new(None)),
        }
    }

    /// Adds a choice to the set.
    /// 
    /// The choice must implement the [`Choice`] trait.
    pub fn add_choice(mut self, choice: T) -> Self {
        self.choices.push(choice);
        self
    }

    /// Sets the choices for the set.
    /// 
    /// This will replace any existing choices.
    pub fn choices(mut self, choices: Vec<T>) -> Self {
        self.choices = choices;
        self
    }

    /// Specifies a default choice for the set.
    /// 
    /// This will be used when there are no prior settings saved.
    pub fn default(self, value: T) -> Self {
        self.value.lock().ok().map(|mut v| v.replace(value));
        self
    }
}

impl SettingItem for TextSetting {
    fn base_prop_mut(&mut self) -> &mut BasePropSetting {
        &mut self.base_prop
    }

    fn id(&self) -> &str {
        &self.base_prop.id
    }

    fn serialize_adaptive_card(&self) -> serde_json::Value {
        let mut map = self.base_prop.serialize();
        map.insert("type".into(), json!("Input.Text"));
        map.insert("placeholder".into(), json!(self.placeholder));
        map.insert("isMultiline".into(), json!(self.is_multiline));
        map.insert("pattern".into(), json!(self.pattern));
        if let Some(value) = self.value.lock().ok().and_then(|v| (*v).clone()) {
            map.insert("value".into(), json!(value));
        }
        json!(map)
    }

    fn update(&self, data: &Map<String, JsonValue>) {
        if let Some(value) = data
            .get(self.id())
            .and_then(|v| v.as_str().map(|s| s.to_string()))
        {
            self.value.lock().ok().map(|mut v| v.replace(value));
        }
    }

    fn serialize_value(&self) -> Option<serde_json::Value> {
        self.value
            .lock()
            .ok()
            .and_then(|v| v.clone())
            .map(|v| json!(v))
    }
}

// NOTE: Special case for NumberSetting since it uses number as default value field and string for value returns.

impl SettingItem for NumberSetting {
    fn base_prop_mut(&mut self) -> &mut BasePropSetting {
        &mut self.base_prop
    }

    fn id(&self) -> &str {
        &self.base_prop.id
    }

    fn serialize_adaptive_card(&self) -> serde_json::Value {
        let mut map = self.base_prop.serialize();
        map.insert("type".into(), json!("Input.Number"));
        map.insert("placeholder".into(), json!(self.placeholder));
        if let Some(min) = self.min {
            map.insert("min".into(), json!(min));
        }
        if let Some(max) = self.max {
            map.insert("max".into(), json!(max));
        }
        if let Some(value) = self.value.lock().ok().and_then(|v| *v) {
            map.insert("value".into(), json!(value));
        }

        json!(map)
    }

    fn update(&self, data: &Map<String, JsonValue>) {
        if let Some(value) = data
            .get(self.id())
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
        {
            self.value.lock().ok().map(|mut v| v.replace(value));
        }
    }

    fn serialize_value(&self) -> Option<serde_json::Value> {
        self.value
            .lock()
            .ok()
            .and_then(|v| v.clone())
            .map(|v| json!(v.to_string()))
    }
}

// NOTE: Special case for ToggleSetting since it uses string as default value field and returns.

impl SettingItem for ToggleSetting {
    fn base_prop_mut(&mut self) -> &mut BasePropSetting {
        &mut self.base_prop
    }

    fn id(&self) -> &str {
        &self.base_prop.id
    }

    fn serialize_adaptive_card(&self) -> serde_json::Value {
        let mut map = self.base_prop.serialize();
        map.insert("type".into(), json!("Input.Toggle"));
        if let Some(value) = self.value.lock().ok().and_then(|v| *v) {
            map.insert("value".into(), json!(value.to_string()));
        }
        json!(map)
    }

    fn update(&self, data: &Map<String, JsonValue>) {
        if let Some(value) = data.get(self.id()).and_then(|v| match v.as_str() {
            Some("true") => Some(true),
            Some("false") => Some(false),
            _ => None,
        }) {
            self.value.lock().ok().map(|mut v| v.replace(value));
        }
    }

    fn serialize_value(&self) -> Option<serde_json::Value> {
        self.value
            .lock()
            .ok()
            .and_then(|v| v.clone())
            .map(|v| json!(v))
    }
}

impl<T: Choice> SettingItem for ChoiceSetSetting<T> {
    fn base_prop_mut(&mut self) -> &mut BasePropSetting {
        &mut self.base_prop
    }

    fn id(&self) -> &str {
        &self.base_prop.id
    }

    fn serialize_adaptive_card(&self) -> serde_json::Value {
        let mut map = self.base_prop.serialize();
        map.insert("type".into(), json!("Input.ChoiceSet"));
        map.insert(
            "choices".into(),
            json!(
                self.choices
                    .iter()
                    .map(|c| {
                        json!({
                            "value": c.value(),
                            "title": c.title(),
                        })
                    })
                    .collect::<Vec<_>>()
            ),
        );
        if let Some(value) = self.value.lock().ok().and_then(|v| (*v).clone()) {
            map.insert("value".into(), json!(value.value()));
        }
        json!(map)
    }

    fn update(&self, data: &Map<String, JsonValue>) {
        if let Some(value) = data
            .get(self.id())
            .and_then(|v| v.as_str().map(|s| s.to_string()))
        {
            if let Some(choice) = self.choices.iter().find(|c| c.value() == value) {
                self.value
                    .lock()
                    .ok()
                    .map(|mut v| v.replace(choice.clone()));
            }
        }
    }

    fn serialize_value(&self) -> Option<serde_json::Value> {
        self.value
            .lock()
            .ok()
            .and_then(|v| v.clone())
            .map(|v| json!(v.value()))
    }
}

trait ValueLock {
    type Value: Clone + 'static;
    fn value_lock(&self) -> Arc<Mutex<Option<Self::Value>>>;
}

impl ValueLock for TextSetting {
    type Value = String;
    fn value_lock(&self) -> Arc<Mutex<Option<Self::Value>>> {
        self.value.clone()
    }
}

impl ValueLock for NumberSetting {
    type Value = f64;
    fn value_lock(&self) -> Arc<Mutex<Option<Self::Value>>> {
        self.value.clone()
    }
}

impl ValueLock for ToggleSetting {
    type Value = bool;
    fn value_lock(&self) -> Arc<Mutex<Option<Self::Value>>> {
        self.value.clone()
    }
}

impl<T: Choice> ValueLock for ChoiceSetSetting<T> {
    type Value = T;
    fn value_lock(&self) -> Arc<Mutex<Option<Self::Value>>> {
        self.value.clone()
    }
}

/// A detailed implementation of the [`ICommandSettings`] interface which preserves config in a JSON file.
/// 
/// This struct automatically handles read and write of JSON settings file,
/// and exposes `Arc<Mutex<Option<T>>>` for each setting item for developer to access the value.
#[implement(ICommandSettings)]
#[derive(Clone)]
pub struct JsonCommandSettings {
    path: std::path::PathBuf,
    settings: Vec<Arc<dyn SettingItem + Send + Sync>>,
    page: ComObject<ContentPage>,
}

impl JsonCommandSettings {
    /// Creates a new `JsonCommandSettings` with the given path.
    /// 
    /// The path should point to a JSON file where the settings will be stored.
    /// If the file does not exist, it will be created.
    /// 
    /// It will also attempt to create parent directories when writing the config file,
    /// if they do not exist.
    pub fn new(path: std::path::PathBuf) -> Self {
        use crate::cmd::BaseCommandBuilder;
        use crate::icon::{IconData, IconInfo};
        use crate::page::BasePageBuilder;
        use crate::page::content::ContentPageBuilder;
        let page = ContentPageBuilder::new(
            BasePageBuilder::new(
                BaseCommandBuilder::new()
                    .name("Settings")
                    .icon(IconInfo::new(IconData::from("\u{E713}")))
                    .build(),
            )
            .build(),
        )
        .build();
        Self {
            path,
            settings: Vec::new(),
            page,
        }
    }

    /// Adds a setting item to the settings page.
    /// 
    /// Valid setting items are:
    /// - [`TextSetting`]
    /// - [`NumberSetting`]
    /// - [`ToggleSetting`]
    /// - [`ChoiceSetSetting`]
    /// 
    /// Returns an `Arc<Mutex<Option<V>>>` that can be used to access the real-time value of the setting.
    #[allow(private_bounds, reason = "Trait bounds are only for internal use")]
    pub fn add_setting<
        T: SettingItem + ValueLock<Value = V> + Send + Sync + 'static,
        V: Clone + 'static,
    >(
        &mut self,
        setting: T,
    ) -> Arc<Mutex<Option<V>>> {
        let lock = setting.value_lock();
        self.settings.push(Arc::new(setting));
        lock
    }

    fn template_json(&self) -> JsonValue {
        let body: Vec<_> = self
            .settings
            .iter()
            .map(|s| s.serialize_adaptive_card())
            .collect();
        let mut keys: Map<String, JsonValue> = Map::new();
        for id in self.settings.iter().map(|s| s.id()) {
            keys.insert(id.to_string(), json!(id));
        }

        let v = json!({
          "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
          "type": "AdaptiveCard",
          "version": "1.5",
          "body": body,
          "actions": [
            {
              "type": "Action.Submit",
        "title": "Save",
              "data": keys
            }
          ]
        });
        dbg!(v.clone());
        v
    }

    fn read_settings(&self) -> Option<()> {
        let data = std::fs::read_to_string(&self.path).ok()?;
        let data: Map<String, JsonValue> = serde_json::from_str(&data).ok()?;
        for setting in self.settings.iter() {
            setting.update(&data);
        }
        Some(())
    }

    fn write_settings(&self) -> Option<()> {
        let mut data = Map::new();
        for setting in self.settings.iter() {
            if let Some(value) = setting.serialize_value() {
                data.insert(setting.id().to_string(), value);
            }
        }
        let json = serde_json::to_string(&data).ok()?;
        let path = self.path.canonicalize().ok()?;
        std::fs::create_dir_all(path.parent()?).ok()?;
        std::fs::write(&path, json).ok()?;
        Some(())
    }
}

impl ICommandSettings_Impl for JsonCommandSettings_Impl {
    fn SettingsPage(&self) -> windows_core::Result<IContentPage> {
        use crate::cmd_result::CommandResult;
        use crate::content::FormContentBuilder;
        let slf = self.this.clone();
        slf.read_settings();
        let form = FormContentBuilder::new()
            .template_json(
                serde_json::to_string(&slf.template_json())
                    .map_err(|e| Error::new(E_FAIL, e.to_string()))?,
            )
            .submit(move |form, input, _| {
                let data: Map<String, JsonValue> =
                    serde_json::from_str(&input.to_string_lossy())
                        .map_err(|e| Error::new(E_FAIL, e.to_string()))?;
                for setting in slf.settings.iter() {
                    setting.update(&data);
                }

                slf.write_settings().ok_or(Error::new(
                    ERROR_FILE_INVALID.to_hresult(),
                    format!("Failed to write settings to {}", slf.path.display()),
                ))?;

                let new_template = serde_json::to_string(&slf.template_json())
                    .map_err(|e| Error::new(E_FAIL, e.to_string()))?;
                let mut guard = form.template_json_mut()?;
                *guard = new_template.into();
                drop(guard);
                slf.page.emit_items_changed(slf.page.to_interface(), -1);
                Ok(CommandResult::GoHome)
            })
            .build();

        let mut guard = self.page.contents_mut()?;
        *guard = vec![form.into()];
        drop(guard);

        Ok(self.page.to_interface())
    }
}
