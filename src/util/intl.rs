use js_sys::*;
use wasm_bindgen::prelude::*;

// Borrowed from https://github.com/rustwasm/wasm-bindgen
// License: APACHE-2.0 or MIT
// TODO: Use js_sys::Intl directly once https://github.com/rustwasm/wasm-bindgen/pull/2506 is merged

// Intl
#[wasm_bindgen]
extern "C" {
    /// The `Intl.getCanonicalLocales()` method returns an array containing
    /// the canonical locale names. Duplicates will be omitted and elements
    /// will be validated as structurally valid language tags.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/getCanonicalLocales)
    #[wasm_bindgen(js_name = getCanonicalLocales, js_namespace = Intl)]
    pub fn get_canonical_locales(s: &JsValue) -> Array;
}

// Intl.Collator
#[wasm_bindgen]
extern "C" {
    /// The `Intl.Collator` object is a constructor for collators, objects
    /// that enable language sensitive string comparison.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Collator)
    #[wasm_bindgen(extends = Object, js_namespace = Intl, typescript_type = "Intl.Collator")]
    #[derive(Clone, Debug)]
    pub type Collator;

    /// The `Intl.Collator` object is a constructor for collators, objects
    /// that enable language sensitive string comparison.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Collator)
    #[wasm_bindgen(constructor, js_namespace = Intl)]
    pub fn new(locales: &Array, options: &Object) -> Collator;

    /// The Intl.Collator.prototype.compare property returns a function that
    /// compares two strings according to the sort order of this Collator
    /// object.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Collator/compare)
    #[wasm_bindgen(method, js_class = "Intl.Collator")]
    pub fn compare(this: &Collator, string1: &str, string2: &str) -> i32;

    /// The `Intl.Collator.prototype.resolvedOptions()` method returns a new
    /// object with properties reflecting the locale and collation options
    /// computed during initialization of this Collator object.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Collator/resolvedOptions)
    #[wasm_bindgen(method, js_namespace = Intl, js_name = resolvedOptions)]
    pub fn resolved_options(this: &Collator) -> Object;

    /// The `Intl.Collator.supportedLocalesOf()` method returns an array
    /// containing those of the provided locales that are supported in
    /// collation without having to fall back to the runtime's default
    /// locale.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Collator/supportedLocalesOf)
    #[wasm_bindgen(static_method_of = Collator, js_namespace = Intl, js_name = supportedLocalesOf)]
    pub fn supported_locales_of(locales: &Array, options: &Object) -> Array;
}

// Intl.DateTimeFormat
#[wasm_bindgen]
extern "C" {
    /// The `Intl.DateTimeFormat` object is a constructor for objects
    /// that enable language-sensitive date and time formatting.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat)
    #[wasm_bindgen(extends = Object, js_namespace = Intl, typescript_type = "Intl.DateTimeFormat")]
    #[derive(Clone, Debug)]
    pub type DateTimeFormat;

    /// The `Intl.DateTimeFormat` object is a constructor for objects
    /// that enable language-sensitive date and time formatting.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat)
    #[wasm_bindgen(constructor, js_namespace = Intl)]
    pub fn new(locales: &Array, options: &Object) -> DateTimeFormat;

    /// The Intl.DateTimeFormat.prototype.format property returns a getter function that
    /// formats a date according to the locale and formatting options of this
    /// Intl.DateTimeFormat object.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat/format)
    #[wasm_bindgen(method, js_class = "Intl.DateTimeFormat")]
    pub fn format(this: &DateTimeFormat, date: &Date) -> JsString;

    /// The `Intl.DateTimeFormat.prototype.formatToParts()` method allows locale-aware
    /// formatting of strings produced by DateTimeFormat formatters.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat/formatToParts)
    #[wasm_bindgen(method, js_class = "Intl.DateTimeFormat", js_name = formatToParts)]
    pub fn format_to_parts(this: &DateTimeFormat, date: &Date) -> Array;

    /// The `Intl.DateTimeFormat.prototype.resolvedOptions()` method returns a new
    /// object with properties reflecting the locale and date and time formatting
    /// options computed during initialization of this DateTimeFormat object.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat/resolvedOptions)
    #[wasm_bindgen(method, js_namespace = Intl, js_name = resolvedOptions)]
    pub fn resolved_options(this: &DateTimeFormat) -> Object;

    /// The `Intl.DateTimeFormat.supportedLocalesOf()` method returns an array
    /// containing those of the provided locales that are supported in date
    /// and time formatting without having to fall back to the runtime's default
    /// locale.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat/supportedLocalesOf)
    #[wasm_bindgen(static_method_of = DateTimeFormat, js_namespace = Intl, js_name = supportedLocalesOf)]
    pub fn supported_locales_of(locales: &Array, options: &Object) -> Array;
}

// Intl.NumberFormat
#[wasm_bindgen]
extern "C" {
    /// The `Intl.NumberFormat` object is a constructor for objects
    /// that enable language sensitive number formatting.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/NumberFormat)
    #[wasm_bindgen(extends = Object, js_namespace = Intl, typescript_type = "Intl.NumberFormat")]
    #[derive(Clone, Debug)]
    pub type NumberFormat;

    /// The `Intl.NumberFormat` object is a constructor for objects
    /// that enable language sensitive number formatting.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/NumberFormat)
    #[wasm_bindgen(constructor, js_namespace = Intl)]
    pub fn new(locales: &Array, options: &Object) -> NumberFormat;

    /// The Intl.NumberFormat.prototype.format property returns a getter function that
    /// formats a number according to the locale and formatting options of this
    /// NumberFormat object.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/NumberFormat/format)
    #[wasm_bindgen(method, js_class = "Intl.NumberFormat")]
    pub fn format(this: &NumberFormat, number: f64) -> JsString;

    /// The `Intl.Numberformat.prototype.formatToParts()` method allows locale-aware
    /// formatting of strings produced by NumberTimeFormat formatters.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/NumberFormat/formatToParts)
    #[wasm_bindgen(method, js_class = "Intl.NumberFormat", js_name = formatToParts)]
    pub fn format_to_parts(this: &NumberFormat, number: f64) -> Array;

    /// The `Intl.NumberFormat.prototype.resolvedOptions()` method returns a new
    /// object with properties reflecting the locale and number formatting
    /// options computed during initialization of this NumberFormat object.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/NumberFormat/resolvedOptions)
    #[wasm_bindgen(method, js_namespace = Intl, js_name = resolvedOptions)]
    pub fn resolved_options(this: &NumberFormat) -> Object;

    /// The `Intl.NumberFormat.supportedLocalesOf()` method returns an array
    /// containing those of the provided locales that are supported in number
    /// formatting without having to fall back to the runtime's default locale.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/NumberFormat/supportedLocalesOf)
    #[wasm_bindgen(static_method_of = NumberFormat, js_namespace = Intl, js_name = supportedLocalesOf)]
    pub fn supported_locales_of(locales: &Array, options: &Object) -> Array;
}

// Intl.PluralRules
#[wasm_bindgen]
extern "C" {
    /// The `Intl.PluralRules` object is a constructor for objects
    /// that enable plural sensitive formatting and plural language rules.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/PluralRules)
    #[wasm_bindgen(extends = Object, js_namespace = Intl, typescript_type = "Intl.PluralRules")]
    #[derive(Clone, Debug)]
    pub type PluralRules;

    /// The `Intl.PluralRules` object is a constructor for objects
    /// that enable plural sensitive formatting and plural language rules.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/PluralRules)
    #[wasm_bindgen(constructor, js_namespace = Intl)]
    pub fn new(locales: &Array, options: &Object) -> PluralRules;

    /// The `Intl.PluralRules.prototype.resolvedOptions()` method returns a new
    /// object with properties reflecting the locale and plural formatting
    /// options computed during initialization of this PluralRules object.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/PluralRules/resolvedOptions)
    #[wasm_bindgen(method, js_namespace = Intl, js_name = resolvedOptions)]
    pub fn resolved_options(this: &PluralRules) -> Object;

    /// The `Intl.PluralRules.prototype.select()` method returns a String indicating
    /// which plural rule to use for locale-aware formatting.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/PluralRules/select)
    #[wasm_bindgen(method, js_namespace = Intl)]
    pub fn select(this: &PluralRules, number: f64) -> JsString;

    /// The `Intl.PluralRules.supportedLocalesOf()` method returns an array
    /// containing those of the provided locales that are supported in plural
    /// formatting without having to fall back to the runtime's default locale.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/PluralRules/supportedLocalesOf)
    #[wasm_bindgen(static_method_of = PluralRules, js_namespace = Intl, js_name = supportedLocalesOf)]
    pub fn supported_locales_of(locales: &Array, options: &Object) -> Array;
}
