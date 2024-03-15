mod alignment;
mod symbols;

use std::fmt::Debug;

pub use self::{
    alignment::{Alignment, AlignmentBuilder},
    symbols::{Symbols, SymbolsBuilder},
};

#[derive(Debug, Clone)]
pub struct RenderMode {
    #[allow(unused)]
    demo_grid: DemoGrid,
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
            demo_grid: DemoGrid::new(&symbol_set),
            symbol_set,
            lines,
            gaps,
            chain,
            vertical,
            invert,
            align_root,
        }
    }
}

#[derive(Clone)]
struct DemoGrid(String);
impl DemoGrid {
    fn new(sym: &Symbols) -> Self {
        let cap_right = if sym.cap_right.is_empty() {
            " "
        } else {
            sym.cap_right.as_str()
        };
        let cap_left = if sym.cap_left.is_empty() {
            " "
        } else {
            sym.cap_left.as_str()
        };

        let horizontal_a = sym.horizontal.to_string().repeat(cap_right.chars().count());
        let horizontal_b = sym.horizontal.to_string().repeat(cap_left.chars().count());

        DemoGrid(format!(
            "{i}{b}{g}{c}{j}\n\
             {e}{b}{d}{c}{f}\n\
             {e}{m}{a}{n}{f}\n\
             {k}{b}{h}{c}{l}",
            a = sym.vertical,
            b = horizontal_a,
            c = horizontal_b,
            d = sym.crossing,
            e = sym.branch_right,
            f = sym.branch_left,
            g = sym.branch_down,
            h = sym.branch_up,
            i = sym.corner_down_right,
            j = sym.corner_down_left,
            k = sym.corner_up_right,
            l = sym.corner_up_left,
            m = cap_right,
            n = cap_left,
        ))
    }
}
impl Debug for DemoGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self.0.lines();
        let grid = grid
            .map(|line| format!("    {}", line))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "DemoGrid {{\n{}\n}}", grid)
    }
}
