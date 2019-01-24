#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{self, Parse, ParseStream, Result},
    parse_macro_input,
    spanned::Spanned,
    Expr, ExprArray, Ident, LitBool, LitInt, Token,
};

const TOBOOT_LOCK_ENTRY_MAGIC: u32 = 0x18349420;

#[derive(Default)]
struct ParsedTobootConfig {
    config: Option<ExprArray>,
    lock_entry: Option<LitBool>,
    erase_mask_lo: Option<LitInt>,
    erase_mask_hi: Option<LitInt>,
}

impl Parse for ParsedTobootConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut result_config = ParsedTobootConfig {
            ..Default::default()
        };

        loop {
            if input.is_empty() {
                break;
            }

            if !input.peek(Ident) {
                return Err(input.error("expecting identifier, either `config`, `lock_entry`, `erase_mask_lo`, and/or `erase_mask_hi`"));
            }

            let id: Ident = input.parse()?;

            input.parse::<Token![:]>()?;

            match &*id.to_string() {
                "config" => {
                    let expr: Expr = input.parse()?;
                    match expr {
                        Expr::Array(exp_array) => {
                            result_config.config = Some(exp_array);
                        },
                        _ => {
                            return Err(input.error("expecting array of configuration flags, e.g. [autorun_enable]"));
                        },
                    }
                },
                "lock_entry" => {
                    result_config.lock_entry = Some(input.parse()?);
                },
                "erase_mask_lo" => {
                    result_config.erase_mask_lo = Some(input.parse()?);
                },
                "erase_mask_hi" => {
                    result_config.erase_mask_hi = Some(input.parse()?);
                },
                _ => return Err(parse::Error::new(
                    id.span(),
                    &format!("unexpected identifier `{}`, expecting either `config`, `lock_entry`, `erase_mask_lo`, and/or `erase_mask_hi`", id.to_string())
                )),
            }

            input.parse::<Token![,]>()?;
        }

        Ok(result_config)
    }
}

impl ParsedTobootConfig {
    fn config_val(&self) -> Result<u8> {
        match &self.config {
            Some(cfg) => self.parse_config_array_flag(cfg),
            None => Ok(0),
        }
    }

    fn lock_entry_val(&self) -> u32 {
        match &self.lock_entry {
            Some(lock) if lock.value => TOBOOT_LOCK_ENTRY_MAGIC,
            _ => 0,
        }
    }

    fn erase_mask_lo_val(&self) -> u32 {
        match &self.erase_mask_lo {
            Some(mask) => mask.value() as u32,
            None => 0,
        }
    }

    fn erase_mask_hi_val(&self) -> u32 {
        match &self.erase_mask_hi {
            Some(mask) => mask.value() as u32,
            None => 0,
        }
    }

    fn parse_config_array_flag(&self, cfg: &ExprArray) -> Result<u8> {
        let mut result: u8 = 0;

        for flag in cfg.elems.iter() {
            let flag_id: Ident = self.expr_to_ident(flag)?;

            result = match &*flag_id.to_string() {
                "irq_enable" => result | 1,
                "autorun_enable" => result | 2,
                s => return Err(self.new_error(flag_id.span(), &format!(
                    "config `{}` is not supported, supported config: `irq_enable`, `autorun_enable`",
                    s
                ))),
            }
        }

        Ok(result)
    }

    fn expr_to_ident(&self, expr: &Expr) -> Result<Ident> {
        match expr {
            Expr::Path(expr_path) => {
                if expr_path.path.segments.len() != 1 {
                    return Err(self.new_error(expr.span(), "unexpected path, expecting config flags, supported config: `irq_enable`, `autorun_enable`"));
                }

                Ok(expr_path.path.segments[0].ident.clone())
            }
            _ => Err(self.new_error(expr.span(), "unexpected token, expecting config flags, supported config: `irq_enable`, `autorun_enable`")),
        }
    }

    fn new_error(&self, span: Span, msg: &str) -> parse::Error {
        parse::Error::new(span, msg)
    }
}

/// Function-like macro for toboot configuration
/// **IMPORTANT** this macro may only be invoked once.
///
/// This macro define `config`, `lock_entry`, `erase_mask_lo`, or `erase_mask_hi`
/// values to be used in tomu bootloader (toboot)
///
/// Valid values:
///
/// - `config` [array] (order doesn't matter)
///   either: [irq_enable] or [autorun_enable] or [irq_enable, autorun_enable]
///
/// - `lock_entry` [bool]
///   either: true or false
///
/// - erase_mask_lo (integer)
///
/// - erase_mask_hi (integer)
///
///
/// for example:
/// ``` no_run
/// # #![feature(proc_macro_hygiene)]
/// # use tomu_hal_macros::toboot_config;
///
/// toboot_config! {
///     config: [irq_enable, autorun_enable],
///     lock_entry: true,
/// }
/// ```
///
/// will resulted in this toboot config:
/// ``` no_run
/// #[used]
/// #[no_mangle]
/// static CONFIG: tomu_hal::toboot::TobootConfig = tomu_hal::toboot::TobootConfig{
///     magic: tomu_hal::toboot::TOBOOT_V2_MAGIC,
///     reserved_gen: 0,
///     start: 16,
///     config: 3,
///     lock_entry: tomu_hal::toboot::TOBOOT_LOCK_ENTRY_MAGIC,
///     erase_mask_lo: 0,
///     erase_mask_hi: 0,
///     reserved_hash: 0,
/// };
/// ```
#[proc_macro]
pub fn toboot_config(input: crate::proc_macro::TokenStream) -> crate::proc_macro::TokenStream {
    let parsed_config = parse_macro_input!(input as ParsedTobootConfig);
    let config_val = match parsed_config.config_val() {
        Ok(val) => val,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };
    let lock_val = parsed_config.lock_entry_val();
    let erase_mask_lo_val = parsed_config.erase_mask_lo_val();
    let erase_mask_hi_val = parsed_config.erase_mask_hi_val();

    if lock_val == TOBOOT_LOCK_ENTRY_MAGIC {
        parsed_config
            .lock_entry
            .span()
            .unstable()
            .warning("*CAUTION* this will lock you from entering bootloader")
            .emit();
    }

    let result = quote! {
        #[used]
        #[no_mangle]
        static CONFIG: tomu_hal::toboot::TobootConfig = tomu_hal::toboot::TobootConfig{
            magic: tomu_hal::toboot::TOBOOT_V2_MAGIC,
            reserved_gen: 0,
            start: 16,
            config: #config_val,
            lock_entry: #lock_val,
            erase_mask_lo: #erase_mask_lo_val,
            erase_mask_hi: #erase_mask_hi_val,
            reserved_hash: 0,
        };
    };

    result.into()
}
