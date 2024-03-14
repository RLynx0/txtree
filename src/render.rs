mod alignment;
mod symbols;

pub use self::{
    alignment::{Alignment, AlignmentBuilder},
    symbols::{Symbols, SymbolsBuilder},
};

#[derive(Debug, Clone)]
pub struct RenderMode {
    symbol_set: Symbols,
    lines: u8,
    gaps: u8,
    chain: bool,
    vertical: bool,
    invert: bool,
    align_root: Alignment,
}
impl RenderMode {
    pub fn new(
        symbol_set: Symbols,
        lines: u8,
        gaps: u8,
        chain: bool,
        vertical: bool,
        invert: bool,
        align_root: Alignment,
    ) -> Self {
        RenderMode {
            symbol_set,
            lines,
            gaps,
            chain,
            vertical,
            invert,
            align_root,
        }
    }

    pub fn demo_grid(&self) -> String {
        let sym = &self.symbol_set;
        format!(
            "{j}{b}{h}{b}{k}\n\
             {f}{b}{e}{b}{g}\n\
             {f}{c}{a}{d}{g}\n\
             {l}{b}{i}{b}{m}",
            a = sym.vertical,
            b = sym.horizontal,
            c = sym.cap_right,
            d = sym.cap_left,
            e = sym.crossing,
            f = sym.branch_right,
            g = sym.branch_left,
            h = sym.branch_down,
            i = sym.branch_up,
            j = sym.corner_down_right,
            k = sym.corner_down_left,
            l = sym.corner_up_right,
            m = sym.corner_up_left,
        )
    }
}
