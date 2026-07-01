use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::thread_local;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SymbolType {
    #[default]
    Undefined,
    Identifier,
    Token,
    Function,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SymbolFlag {
    #[default]
    None,
    Alias,
    Local,
    Parm,
    Start,
    Target,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Storage {
    #[default]
    Extern,
    Static,
    Auto,
    Other,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TableEntry {
    pub sym: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CollectData {
    pub sym: Vec<usize>,
    pub count: usize,
    pub index: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModuleSrcLinkedListEntry02 {
    pub symbol: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModuleSrcCollectData10 {
    pub symbols: Vec<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReturnSym {
    pub symbols: Vec<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub flag: SymbolFlag,
    pub storage: Storage,
    pub arity: i32,
    pub decl: Option<String>,
    pub source: Option<String>,
    pub def_line: i32,
    pub level: i32,
    pub ord: isize,
    pub owner: Option<usize>,
    pub next: Option<usize>,
    pub alias: Option<usize>,
    pub entry: Option<ListKind>,
    pub ref_line: Vec<String>,
    pub caller: Vec<usize>,
    pub callee: Vec<usize>,
    pub active: bool,
    pub visible: bool,
    alive: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListKind {
    Static,
    Auto,
    Start,
    Target,
    StaticFunc,
    UnitLocal,
}

#[derive(Default)]
struct SymbolState {
    symbols: Vec<Symbol>,
    symbol_table: HashMap<String, usize>,
    static_symbol_list: Vec<usize>,
    auto_symbol_list: Vec<usize>,
    start_symbol_list: Vec<usize>,
    target_symbol_list: Vec<usize>,
    static_func_list: Vec<usize>,
    unit_local_list: Vec<usize>,
    output_visible: bool,
    reverse_tree: bool,
    print_xref: bool,
    canonical_filename: Option<String>,
    filename: Option<String>,
}

thread_local! {
    static SYMBOL_STATE: RefCell<SymbolState> = RefCell::new(SymbolState::default());
}

impl Symbol {
    fn with_state<R>(f: impl FnOnce(&mut SymbolState) -> R) -> R {
        SYMBOL_STATE.with(|state| f(&mut state.borrow_mut()))
    }

    fn list_mut(state: &mut SymbolState, kind: ListKind) -> &mut Vec<usize> {
        match kind {
            ListKind::Static => &mut state.static_symbol_list,
            ListKind::Auto => &mut state.auto_symbol_list,
            ListKind::Start => &mut state.start_symbol_list,
            ListKind::Target => &mut state.target_symbol_list,
            ListKind::StaticFunc => &mut state.static_func_list,
            ListKind::UnitLocal => &mut state.unit_local_list,
        }
    }

    fn in_list(state: &SymbolState, kind: ListKind, id: usize) -> bool {
        let list = match kind {
            ListKind::Static => &state.static_symbol_list,
            ListKind::Auto => &state.auto_symbol_list,
            ListKind::Start => &state.start_symbol_list,
            ListKind::Target => &state.target_symbol_list,
            ListKind::StaticFunc => &state.static_func_list,
            ListKind::UnitLocal => &state.unit_local_list,
        };
        list.contains(&id)
    }

    fn unlink_from_specific_list(state: &mut SymbolState, id: usize, kind: ListKind) {
        let list = Self::list_mut(state, kind);
        if let Some(pos) = list.iter().position(|&item| item == id) {
            list.remove(pos);
        }
    }

    fn is_function_symbol(sym: &Symbol) -> bool {
        sym.symbol_type == SymbolType::Function || sym.arity >= 0
    }

    pub fn unlink_from_list(symbol_id: usize) {
        Self::with_state(|state| {
            if let Some(kind) = state.symbols.get(symbol_id).and_then(|s| s.entry) {
                Self::unlink_from_specific_list(state, symbol_id, kind);
                if let Some(sym) = state.symbols.get_mut(symbol_id) {
                    sym.entry = None;
                }
            }
        });
    }

    pub fn append_symbol(kind: ListKind, symbol_id: usize) {
        Self::with_state(|state| {
            if symbol_id >= state.symbols.len() || !state.symbols[symbol_id].alive {
                return;
            }
            if let Some(existing) = state.symbols[symbol_id].entry {
                Self::unlink_from_specific_list(state, symbol_id, existing);
            }
            if !Self::in_list(state, kind, symbol_id) {
                Self::list_mut(state, kind).push(symbol_id);
            }
            if let Some(sym) = state.symbols.get_mut(symbol_id) {
                sym.entry = Some(kind);
            }
        });
    }

    pub fn has_her(entry: &TableEntry, n_buckets: usize) -> usize {
        if n_buckets == 0 {
            return 0;
        }
        match entry.sym {
            Some(id) => Self::with_state(|state| {
                state
                    .symbols
                    .get(id)
                    .map(|s| {
                        s.name
                            .bytes()
                            .fold(0usize, |acc, b| acc.wrapping_mul(33).wrapping_add(b as usize))
                            % n_buckets
                    })
                    .unwrap_or(id % n_buckets)
            }),
            None => 0,
        }
    }

    pub fn compare(left: &TableEntry, right: &TableEntry) -> bool {
        match (left.sym, right.sym) {
            (Some(a), Some(b)) => Self::with_state(|state| {
                state.symbols.get(a).map(|s| &s.name) == state.symbols.get(b).map(|s| &s.name)
            }),
            _ => false,
        }
    }

    pub fn find(name: &str) -> Option<usize> {
        Self::with_state(|state| {
            let mut sym = *state.symbol_table.get(name)?;
            loop {
                let current = state.symbols.get(sym)?;
                if current.symbol_type == SymbolType::Token && current.flag == SymbolFlag::Alias {
                    sym = current.alias?;
                } else {
                    return Some(sym);
                }
            }
        })
    }

    pub fn intern(name: impl Into<String>, flags: i32) -> usize {
        const INSTALL_DEFAULT: i32 = 0;
        const INSTALL_CHECK_LOCAL: i32 = 1;
        const INSTALL_OVERWRITE: i32 = 2;
        const INSTALL_UNIT_LOCAL: i32 = 4;

        let _ = INSTALL_DEFAULT;

        Self::with_state(|state| {
            let name = name.into();

            if let Some(&existing) = state.symbol_table.get(&name) {
                if (flags & INSTALL_OVERWRITE) != 0 {
                    return existing;
                }
            }

            let mark_local = (((flags & INSTALL_CHECK_LOCAL) != 0)
                && state.canonical_filename.is_some()
                && state.filename != state.canonical_filename)
                || (flags & INSTALL_UNIT_LOCAL) != 0;

            let id = state.symbols.len();
            state.symbols.push(Symbol {
                name: name.clone(),
                symbol_type: SymbolType::Undefined,
                flag: if mark_local {
                    SymbolFlag::Local
                } else {
                    SymbolFlag::None
                },
                storage: Storage::Extern,
                arity: 0,
                decl: None,
                source: None,
                def_line: 0,
                level: 0,
                ord: -1,
                owner: Some(id),
                next: None,
                alias: None,
                entry: None,
                ref_line: Vec::new(),
                caller: Vec::new(),
                callee: Vec::new(),
                active: false,
                visible: false,
                alive: true,
            });

            if mark_local {
                if !Self::in_list(state, ListKind::Static, id) {
                    state.static_symbol_list.push(id);
                }
                state.symbols[id].entry = Some(ListKind::Static);
            }

            if let Some(&existing) = state.symbol_table.get(&name) {
                if state.symbols[existing].symbol_type != SymbolType::Undefined {
                    state.symbols[id].next = Some(existing);
                }
            }
            state.symbol_table.insert(name, id);
            id
        })
    }

    pub fn ident_change_storage(symbol_id: usize, storage: Storage) {
        Self::with_state(|state| {
            if symbol_id >= state.symbols.len() || !state.symbols[symbol_id].alive {
                return;
            }
            if state.symbols[symbol_id].storage == storage {
                return;
            }

            match storage {
                Storage::Static => {
                    if let Some(existing) = state.symbols[symbol_id].entry {
                        Self::unlink_from_specific_list(state, symbol_id, existing);
                    }
                    if !Self::in_list(state, ListKind::Static, symbol_id) {
                        state.static_symbol_list.push(symbol_id);
                    }
                    state.symbols[symbol_id].entry = Some(ListKind::Static);
                }
                Storage::Auto => {
                    if let Some(existing) = state.symbols[symbol_id].entry {
                        Self::unlink_from_specific_list(state, symbol_id, existing);
                    }
                    if !Self::in_list(state, ListKind::Auto, symbol_id) {
                        state.auto_symbol_list.push(symbol_id);
                    }
                    state.symbols[symbol_id].entry = Some(ListKind::Auto);
                }
                _ => {
                    if let Some(existing) = state.symbols[symbol_id].entry {
                        Self::unlink_from_specific_list(state, symbol_id, existing);
                    }
                    state.symbols[symbol_id].entry = None;
                }
            }

            state.symbols[symbol_id].storage = storage;
        });
    }

    pub fn init_ident(symbol_id: usize, storage: Storage) {
        Self::with_state(|state| {
            if symbol_id >= state.symbols.len() || !state.symbols[symbol_id].alive {
                return;
            }
            let sym = &mut state.symbols[symbol_id];
            sym.symbol_type = SymbolType::Identifier;
            sym.arity = -1;
            sym.storage = Storage::Extern;
            sym.decl = None;
            sym.source = None;
            sym.def_line = -1;
            sym.ref_line.clear();
            sym.caller.clear();
            sym.callee.clear();
            sym.level = -1;
        });
        Self::ident_change_storage(symbol_id, storage);
    }

    pub fn install_ident(name: impl Into<String>, storage: Storage) -> usize {
        const INSTALL_DEFAULT: i32 = 0;
        const INSTALL_CHECK_LOCAL: i32 = 1;

        let id = Self::intern(
            name,
            if storage != Storage::Auto {
                INSTALL_CHECK_LOCAL
            } else {
                INSTALL_DEFAULT
            },
        );
        Self::init_ident(id, storage);
        id
    }

    pub fn unlink_symbol(symbol_id: usize) {
        Self::with_state(|state| {
            if symbol_id >= state.symbols.len() || !state.symbols[symbol_id].alive {
                return;
            }

            let name = state.symbols[symbol_id].name.clone();
            let next_symbol = state.symbols[symbol_id].next;

            if state.symbol_table.get(&name).copied() == Some(symbol_id) {
                if let Some(next) = next_symbol {
                    state.symbol_table.insert(name, next);
                } else {
                    state.symbol_table.remove(&name);
                }
            } else {
                for sym in &mut state.symbols {
                    if sym.alive && sym.next == Some(symbol_id) {
                        sym.next = next_symbol;
                        break;
                    }
                }
            }

            state.symbols[symbol_id].owner = None;
            state.symbols[symbol_id].next = None;
        });
    }

    pub fn delete_symbol(symbol_id: usize) {
        Self::with_state(|state| {
            if symbol_id >= state.symbols.len() || !state.symbols[symbol_id].alive {
                return;
            }

            let keep_for_reverse = state.reverse_tree && !state.symbols[symbol_id].callee.is_empty();

            let name = state.symbols[symbol_id].name.clone();
            let next_symbol = state.symbols[symbol_id].next;
            if state.symbol_table.get(&name).copied() == Some(symbol_id) {
                if let Some(next) = next_symbol {
                    state.symbol_table.insert(name, next);
                } else {
                    state.symbol_table.remove(&name);
                }
            } else {
                for sym in &mut state.symbols {
                    if sym.alive && sym.next == Some(symbol_id) {
                        sym.next = next_symbol;
                        break;
                    }
                }
            }

            if let Some(kind) = state.symbols[symbol_id].entry {
                Self::unlink_from_specific_list(state, symbol_id, kind);
            }

            if state.symbols[symbol_id].ref_line.is_empty() && !keep_for_reverse {
                state.symbols[symbol_id].ref_line.clear();
                state.symbols[symbol_id].caller.clear();
                state.symbols[symbol_id].callee.clear();
                state.symbols[symbol_id].alive = false;
                state.symbols[symbol_id].owner = None;
                state.symbols[symbol_id].entry = None;
                state.symbols[symbol_id].next = None;
            } else {
                state.symbols[symbol_id].owner = None;
                state.symbols[symbol_id].entry = None;
                state.symbols[symbol_id].next = None;
            }
        });
    }

    pub fn release_static_symbol(symbol_id: usize) {
        Self::with_state(|state| {
            if symbol_id >= state.symbols.len() || !state.symbols[symbol_id].alive {
                return;
            }
            if state.symbols[symbol_id].owner.is_none() {
                return;
            }

            if state.symbols[symbol_id].flag == SymbolFlag::Local {
                let include_symbol = true;
                if state.print_xref && include_symbol {
                    let name = state.symbols[symbol_id].name.clone();
                    let next_symbol = state.symbols[symbol_id].next;
                    if state.symbol_table.get(&name).copied() == Some(symbol_id) {
                        if let Some(next) = next_symbol {
                            state.symbol_table.insert(name, next);
                        } else {
                            state.symbol_table.remove(&name);
                        }
                    } else {
                        for sym in &mut state.symbols {
                            if sym.alive && sym.next == Some(symbol_id) {
                                sym.next = next_symbol;
                                break;
                            }
                        }
                    }
                    state.symbols[symbol_id].owner = None;
                    state.symbols[symbol_id].next = None;
                    if !state.unit_local_list.contains(&symbol_id) {
                        state.unit_local_list.push(symbol_id);
                    }
                } else {
                    let keep_for_reverse =
                        state.reverse_tree && !state.symbols[symbol_id].callee.is_empty();

                    let name = state.symbols[symbol_id].name.clone();
                    let next_symbol = state.symbols[symbol_id].next;
                    if state.symbol_table.get(&name).copied() == Some(symbol_id) {
                        if let Some(next) = next_symbol {
                            state.symbol_table.insert(name, next);
                        } else {
                            state.symbol_table.remove(&name);
                        }
                    } else {
                        for sym in &mut state.symbols {
                            if sym.alive && sym.next == Some(symbol_id) {
                                sym.next = next_symbol;
                                break;
                            }
                        }
                    }

                    if let Some(kind) = state.symbols[symbol_id].entry {
                        Self::unlink_from_specific_list(state, symbol_id, kind);
                    }

                    if state.symbols[symbol_id].ref_line.is_empty() && !keep_for_reverse {
                        state.symbols[symbol_id].ref_line.clear();
                        state.symbols[symbol_id].caller.clear();
                        state.symbols[symbol_id].callee.clear();
                        state.symbols[symbol_id].alive = false;
                    }
                    state.symbols[symbol_id].owner = None;
                    state.symbols[symbol_id].entry = None;
                    state.symbols[symbol_id].next = None;
                }
            } else {
                let name = state.symbols[symbol_id].name.clone();
                let next_symbol = state.symbols[symbol_id].next;
                if state.symbol_table.get(&name).copied() == Some(symbol_id) {
                    if let Some(next) = next_symbol {
                        state.symbol_table.insert(name, next);
                    } else {
                        state.symbol_table.remove(&name);
                    }
                } else {
                    for sym in &mut state.symbols {
                        if sym.alive && sym.next == Some(symbol_id) {
                            sym.next = next_symbol;
                            break;
                        }
                    }
                }
                state.symbols[symbol_id].owner = None;
                state.symbols[symbol_id].next = None;
                if Self::is_function_symbol(&state.symbols[symbol_id])
                    && !state.static_func_list.contains(&symbol_id)
                {
                    state.static_func_list.push(symbol_id);
                }
            }
        });
    }

    pub fn delete_statics() {
        let ids = Self::with_state(|state| {
            let ids = state.static_symbol_list.clone();
            state.static_symbol_list.clear();
            ids
        });
        for id in ids {
            Self::release_static_symbol(id);
        }
    }

    pub fn delete_level_autos(symbol_id: usize, level: i32) -> bool {
        let should_delete = Self::with_state(|state| {
            state
                .symbols
                .get(symbol_id)
                .map(|s| s.alive && s.level == level)
                .unwrap_or(false)
        });
        if should_delete {
            Self::delete_symbol(symbol_id);
            true
        } else {
            false
        }
    }

    pub fn delete_level_statics(symbol_id: usize, level: i32) -> bool {
        Self::with_state(|state| {
            let should_unlink = state
                .symbols
                .get(symbol_id)
                .map(|s| s.alive && s.level == level)
                .unwrap_or(false);
            if should_unlink {
                let name = state.symbols[symbol_id].name.clone();
                let next_symbol = state.symbols[symbol_id].next;
                if state.symbol_table.get(&name).copied() == Some(symbol_id) {
                    if let Some(next) = next_symbol {
                        state.symbol_table.insert(name, next);
                    } else {
                        state.symbol_table.remove(&name);
                    }
                } else {
                    for sym in &mut state.symbols {
                        if sym.alive && sym.next == Some(symbol_id) {
                            sym.next = next_symbol;
                            break;
                        }
                    }
                }
                state.symbols[symbol_id].owner = None;
                state.symbols[symbol_id].next = None;
                true
            } else {
                false
            }
        })
    }

    pub fn delete_autos(level: i32) {
        let auto_ids = Self::with_state(|state| state.auto_symbol_list.clone());
        for id in auto_ids {
            let _ = Self::delete_level_autos(id, level);
        }

        let static_ids = Self::with_state(|state| state.static_symbol_list.clone());
        for id in static_ids {
            let _ = Self::delete_level_statics(id, level);
        }
    }

    pub fn collect_processor(entry: &TableEntry, data: &mut CollectData, sel: impl Fn(&Symbol) -> bool) -> bool {
        if let Some(id) = entry.sym {
            Self::with_state(|state| {
                let mut current = Some(id);
                while let Some(sym_id) = current {
                    if let Some(sym) = state.symbols.get(sym_id) {
                        if sym.alive && sel(sym) {
                            if data.index == data.count {
                                data.count = if data.count == 0 { 1 } else { data.count * 2 };
                                data.sym.reserve(data.count.saturating_sub(data.sym.len()));
                            }
                            data.sym.push(sym_id);
                            data.index += 1;
                        }
                        current = sym.next;
                    } else {
                        break;
                    }
                }
            });
        }
        true
    }

    pub fn collect_list_entry(symbol_id: usize, data: &mut CollectData, sel: impl Fn(&Symbol) -> bool) -> bool {
        Self::with_state(|state| {
            if let Some(sym) = state.symbols.get(symbol_id) {
                if sym.alive && sel(sym) {
                    if data.index == data.count {
                        data.count = if data.count == 0 { 1 } else { data.count * 2 };
                        data.sym.reserve(data.count.saturating_sub(data.sym.len()));
                    }
                    data.sym.push(symbol_id);
                    data.index += 1;
                }
            }
        });
        false
    }

    pub fn collect_symbols(sel: impl Fn(&Symbol) -> bool, reserved_slots: usize) -> Vec<usize> {
        Self::with_state(|state| {
            let mut out = Vec::new();
            let mut seen = HashSet::new();

            for &head in state.symbol_table.values() {
                let mut current = Some(head);
                while let Some(id) = current {
                    if let Some(sym) = state.symbols.get(id) {
                        if sym.alive && sel(sym) && seen.insert(id) {
                            out.push(id);
                        }
                        current = sym.next;
                    } else {
                        break;
                    }
                }
            }

            for &id in &state.static_func_list {
                if let Some(sym) = state.symbols.get(id) {
                    if sym.alive && sel(sym) && seen.insert(id) {
                        out.push(id);
                    }
                }
            }

            for &id in &state.unit_local_list {
                if let Some(sym) = state.symbols.get(id) {
                    if sym.alive && sel(sym) && seen.insert(id) {
                        out.push(id);
                    }
                }
            }

            out.reserve(reserved_slots);
            out
        })
    }

    pub fn collect_functions() -> Vec<usize> {
        Self::with_state(|state| {
            let snum = state.static_func_list.len();
            let mut symbols = {
                let mut out = Vec::new();
                let mut seen = HashSet::new();
                for &head in state.symbol_table.values() {
                    let mut current = Some(head);
                    while let Some(id) = current {
                        if let Some(sym) = state.symbols.get(id) {
                            if sym.alive && Self::is_function_symbol(sym) && seen.insert(id) {
                                out.push(id);
                            }
                            current = sym.next;
                        } else {
                            break;
                        }
                    }
                }
                for &id in &state.static_func_list {
                    if let Some(sym) = state.symbols.get(id) {
                        if sym.alive && Self::is_function_symbol(sym) && seen.insert(id) {
                            out.push(id);
                        }
                    }
                }
                for &id in &state.unit_local_list {
                    if let Some(sym) = state.symbols.get(id) {
                        if sym.alive && Self::is_function_symbol(sym) && seen.insert(id) {
                            out.push(id);
                        }
                    }
                }
                out.reserve(snum);
                out
            };

            if snum != 0 {
                for &id in &state.static_func_list {
                    if !symbols.contains(&id) {
                        symbols.push(id);
                    }
                }
            }
            symbols
        })
    }

    pub fn delete_parms_itr(symbol_id: usize, level: i32) -> bool {
        let should_delete = Self::with_state(|state| {
            state.symbols.get(symbol_id).map(|s| {
                s.alive
                    && s.owner.is_some()
                    && s.symbol_type == SymbolType::Identifier
                    && s.storage == Storage::Auto
                    && s.flag == SymbolFlag::Parm
                    && s.level > level
            }).unwrap_or(false)
        });
        if should_delete {
            Self::delete_symbol(symbol_id);
            true
        } else {
            false
        }
    }

    pub fn delete_parms(level: i32) {
        let auto_ids = Self::with_state(|state| state.auto_symbol_list.clone());
        for id in auto_ids {
            let _ = Self::delete_parms_itr(id, level);
        }
    }

    pub fn move_parms(level: i32) {
        Self::with_state(|state| {
            let auto_ids = state.auto_symbol_list.clone();
            for id in auto_ids {
                if let Some(sym) = state.symbols.get_mut(id) {
                    if sym.alive
                        && sym.symbol_type == SymbolType::Identifier
                        && sym.storage == Storage::Auto
                        && sym.flag == SymbolFlag::Parm
                    {
                        sym.level = level;
                        sym.flag = SymbolFlag::None;
                    }
                }
            }
        });
    }

    pub fn install_starter(name: impl Into<String>) -> usize {
        let id = Self::intern(name, 0);
        Self::with_state(|state| {
            if let Some(sym) = state.symbols.get_mut(id) {
                sym.flag = SymbolFlag::Start;
            }
            if !state.start_symbol_list.contains(&id) {
                state.start_symbol_list.push(id);
            }
        });
        id
    }

    pub fn set_default_starter() {
        let empty = Self::with_state(|state| state.start_symbol_list.is_empty());
        if empty {
            let _ = Self::install_starter("main");
        }
    }

    pub fn clear_starters() {
        Self::with_state(|state| {
            state.start_symbol_list.clear();
        });
    }

    pub fn first_starter(itr: &mut Option<usize>) -> Option<usize> {
        Self::with_state(|state| {
            let mut pos = 0usize;
            while pos < state.start_symbol_list.len() {
                let id = state.start_symbol_list[pos];
                if state
                    .symbols
                    .get(id)
                    .map(|s| s.alive && s.symbol_type != SymbolType::Undefined)
                    .unwrap_or(false)
                {
                    *itr = Some(pos + 1);
                    return Some(id);
                }
                pos += 1;
            }
            *itr = None;
            None
        })
    }

    pub fn next_starter(itr: &mut Option<usize>) -> Option<usize> {
        Self::with_state(|state| {
            let Some(mut pos) = *itr else {
                return None;
            };
            while pos < state.start_symbol_list.len() {
                let id = state.start_symbol_list[pos];
                if state
                    .symbols
                    .get(id)
                    .map(|s| s.alive && s.symbol_type != SymbolType::Undefined)
                    .unwrap_or(false)
                {
                    *itr = Some(pos + 1);
                    return Some(id);
                }
                pos += 1;
            }
            *itr = None;
            None
        })
    }

    pub fn install_target(name: impl Into<String>) -> usize {
        let id = Self::intern(name, 0);
        Self::with_state(|state| {
            if let Some(sym) = state.symbols.get_mut(id) {
                sym.flag = SymbolFlag::Target;
            }
            if !state.target_symbol_list.contains(&id) {
                state.target_symbol_list.push(id);
            }
        });
        id
    }

    pub fn mark_callers(symbol_id: usize) {
        let callers = Self::with_state(|state| {
            if symbol_id >= state.symbols.len() || !state.symbols[symbol_id].alive {
                return None;
            }
            if state.symbols[symbol_id].active {
                return None;
            }
            state.symbols[symbol_id].active = true;
            Some(state.symbols[symbol_id].caller.clone())
        });

        if let Some(callers) = callers {
            for caller in callers {
                Self::mark_callers(caller);
            }
            Self::with_state(|state| {
                if let Some(sym) = state.symbols.get_mut(symbol_id) {
                    sym.visible = true;
                    sym.active = false;
                }
            });
        }
    }

    pub fn eliminate_non_targets() {
        let targets = Self::with_state(|state| state.target_symbol_list.clone());
        if targets.is_empty() {
            return;
        }
        for id in targets {
            let should_mark = Self::with_state(|state| {
                state
                    .symbols
                    .get(id)
                    .map(|s| s.alive && s.flag == SymbolFlag::Target)
                    .unwrap_or(false)
            });
            if should_mark {
                Self::mark_callers(id);
            }
        }
        Self::with_state(|state| {
            state.output_visible = true;
        });
    }

    pub fn module_src_delete_level_12(level: i32) {
        Self::delete_autos(level);
    }

    pub fn table_entry(symbol_id: Option<usize>) -> TableEntry {
        TableEntry { sym: symbol_id }
    }

    pub fn collect_data() -> CollectData {
        CollectData::default()
    }

    pub fn module_src_linked_list_entry_02(symbol_id: usize) -> ModuleSrcLinkedListEntry02 {
        ModuleSrcLinkedListEntry02 { symbol: symbol_id }
    }

    pub fn module_src_collect_data_10(symbols: Vec<usize>) -> ModuleSrcCollectData10 {
        ModuleSrcCollectData10 { symbols }
    }

    pub fn reserved_slots(count: usize) -> usize {
        count
    }

    pub fn return_sym(symbols: Vec<usize>) -> ReturnSym {
        ReturnSym { symbols }
    }

    pub fn symbol_name(symbol_id: usize) -> Option<String> {
        Self::with_state(|state| state.symbols.get(symbol_id).filter(|s| s.alive).map(|s| s.name.clone()))
    }

    pub fn compare_names(left: &str, right: &str) -> Ordering {
        left.cmp(right)
    }
}
