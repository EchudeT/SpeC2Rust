use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::thread_local;

use crate::output::{SymbolFlag, SymbolKind};
use crate::parser::StorageClass;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TableEntry {
    pub sym: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModuleSrcTableEntry06 {
    pub sym: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModuleSrcLinkedListEntry02 {
    pub data: Option<usize>,
    pub next: Option<usize>,
    pub prev: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CollectData {
    pub indices: Vec<usize>,
    pub count: usize,
    pub index: usize,
    pub reserved_slots: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModuleSrcCollectData10 {
    pub indices: Vec<usize>,
    pub count: usize,
    pub index: usize,
    pub reserved_slots: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub storage: StorageClass,
    pub flag: SymbolFlag,
    pub alias: Option<usize>,
    pub next: Option<usize>,
    pub owner: Option<usize>,
    pub list_membership: Option<SymbolListKind>,
    pub arity: i32,
    pub decl: Option<String>,
    pub source: Option<String>,
    pub def_line: i32,
    pub level: i32,
    pub ref_line: Vec<String>,
    pub callers: Vec<usize>,
    pub callees: Vec<usize>,
    pub active: bool,
    pub visible: bool,
    pub ordinal: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SymbolListKind {
    Static,
    Auto,
    Start,
    Target,
    StaticFunc,
    UnitLocal,
}

#[derive(Default)]
struct SymbolStore {
    symbols: Vec<Option<Symbol>>,
    table_entries: Vec<TableEntry>,
    buckets: HashMap<String, usize>,
    static_symbols: Vec<usize>,
    auto_symbols: Vec<usize>,
    start_symbols: Vec<usize>,
    target_symbols: Vec<usize>,
    static_funcs: Vec<usize>,
    unit_local: Vec<usize>,
    reverse_tree: bool,
    output_visible: bool,
}

thread_local! {
    static STORE: RefCell<SymbolStore> = RefCell::new(SymbolStore::default());
}

impl Symbol {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: SymbolKind::Undefined,
            storage: StorageClass::Extern,
            flag: SymbolFlag::None,
            alias: None,
            next: None,
            owner: None,
            list_membership: None,
            arity: -1,
            decl: None,
            source: None,
            def_line: -1,
            level: -1,
            ref_line: Vec::new(),
            callers: Vec::new(),
            callees: Vec::new(),
            active: false,
            visible: false,
            ordinal: -1,
        }
    }

    fn with_store<R>(f: impl FnOnce(&mut SymbolStore) -> R) -> R {
        STORE.with(|store| f(&mut store.borrow_mut()))
    }

    fn get_symbol_ref(store: &SymbolStore, index: usize) -> Option<&Symbol> {
        store.symbols.get(index).and_then(|s| s.as_ref())
    }

    fn get_symbol_mut(store: &mut SymbolStore, index: usize) -> Option<&mut Symbol> {
        store.symbols.get_mut(index).and_then(|s| s.as_mut())
    }

    fn list_mut(store: &mut SymbolStore, kind: SymbolListKind) -> &mut Vec<usize> {
        match kind {
            SymbolListKind::Static => &mut store.static_symbols,
            SymbolListKind::Auto => &mut store.auto_symbols,
            SymbolListKind::Start => &mut store.start_symbols,
            SymbolListKind::Target => &mut store.target_symbols,
            SymbolListKind::StaticFunc => &mut store.static_funcs,
            SymbolListKind::UnitLocal => &mut store.unit_local,
        }
    }

    fn list_ref(store: &SymbolStore, kind: SymbolListKind) -> &Vec<usize> {
        match kind {
            SymbolListKind::Static => &store.static_symbols,
            SymbolListKind::Auto => &store.auto_symbols,
            SymbolListKind::Start => &store.start_symbols,
            SymbolListKind::Target => &store.target_symbols,
            SymbolListKind::StaticFunc => &store.static_funcs,
            SymbolListKind::UnitLocal => &store.unit_local,
        }
    }

    fn data_in_list(store: &SymbolStore, kind: SymbolListKind, symbol_index: usize) -> bool {
        Self::list_ref(store, kind).contains(&symbol_index)
    }

    pub fn unlink_from_list(symbol_index: usize) {
        Self::with_store(|store| {
            let membership = Self::get_symbol_ref(store, symbol_index)
                .and_then(|s| s.list_membership);
            if let Some(kind) = membership {
                let list = Self::list_mut(store, kind);
                list.retain(|&idx| idx != symbol_index);
                if let Some(sym) = Self::get_symbol_mut(store, symbol_index) {
                    sym.list_membership = None;
                }
            }
        });
    }

    pub fn append_symbol(list: SymbolListKind, symbol_index: usize) {
        Self::with_store(|store| {
            let current = Self::get_symbol_ref(store, symbol_index)
                .and_then(|s| s.list_membership);
            if let Some(kind) = current {
                let old = Self::list_mut(store, kind);
                old.retain(|&idx| idx != symbol_index);
            }
            if !Self::data_in_list(store, list, symbol_index) {
                Self::list_mut(store, list).push(symbol_index);
            }
            if let Some(sym) = Self::get_symbol_mut(store, symbol_index) {
                sym.list_membership = Some(list);
            }
        });
    }

    pub fn has_her(entry: &TableEntry, n_buckets: usize) -> usize {
        if n_buckets == 0 {
            return 0;
        }
        Self::with_store(|store| {
            if let Some(sym_idx) = entry.sym {
                if let Some(sym) = Self::get_symbol_ref(store, sym_idx) {
                    let mut h: usize = 0;
                    for b in sym.name.as_bytes() {
                        h = h.wrapping_mul(33).wrapping_add(*b as usize);
                    }
                    return h % n_buckets;
                }
            }
            0
        })
    }

    pub fn compare(a: &TableEntry, b: &TableEntry) -> bool {
        Self::with_store(|store| match (a.sym, b.sym) {
            (Some(ia), Some(ib)) => {
                let sa = Self::get_symbol_ref(store, ia);
                let sb = Self::get_symbol_ref(store, ib);
                matches!((sa, sb), (Some(sa), Some(sb)) if sa.name == sb.name)
            }
            _ => false,
        })
    }

    pub fn find(name: &str) -> Option<usize> {
        Self::with_store(|store| {
            let mut current = *store.buckets.get(name)?;
            loop {
                let sym = Self::get_symbol_ref(store, current)?;
                if sym.kind == SymbolKind::Token && sym.flag == SymbolFlag::Alias {
                    current = sym.alias?;
                    continue;
                }
                return Some(current);
            }
        })
    }

    pub fn intern(name: impl Into<String>, flags: u32) -> usize {
        const INSTALL_OVERWRITE: u32 = 1 << 0;
        const INSTALL_CHECK_LOCAL: u32 = 1 << 1;
        const INSTALL_UNIT_LOCAL: u32 = 1 << 2;

        Self::with_store(|store| {
            let name = name.into();
            let mut sym = Symbol::new(name.clone());

            if (flags & INSTALL_CHECK_LOCAL) != 0 || (flags & INSTALL_UNIT_LOCAL) != 0 {
                sym.flag = SymbolFlag::Local;
            }

            let new_index = store.symbols.len();
            store.symbols.push(Some(sym));

            let entry_index = match store.buckets.get(&name).copied() {
                Some(existing_entry) => existing_entry,
                None => {
                    let entry_index = store.table_entries.len();
                    store.table_entries.push(TableEntry {
                        sym: Some(new_index),
                    });
                    store.buckets.insert(name.clone(), entry_index);
                    if let Some(s) = Self::get_symbol_mut(store, new_index) {
                        s.owner = Some(entry_index);
                    }
                    if matches!(Self::get_symbol_ref(store, new_index), Some(s) if s.flag == SymbolFlag::Local)
                    {
                        Self::append_symbol(SymbolListKind::Static, new_index);
                    }
                    return new_index;
                }
            };

            let head = store.table_entries[entry_index].sym;
            let overwrite_target = head.and_then(|idx| Self::get_symbol_ref(store, idx));
            if (flags & INSTALL_OVERWRITE) != 0
                && matches!(overwrite_target, Some(existing) if existing.kind != SymbolKind::Undefined)
            {
                store.symbols.pop();
                return head.unwrap();
            }

            if let Some(existing_head) = head {
                let existing_kind = Self::get_symbol_ref(store, existing_head)
                    .map(|s| s.kind.clone())
                    .unwrap_or(SymbolKind::Undefined);
                if existing_kind != SymbolKind::Undefined {
                    if let Some(s) = Self::get_symbol_mut(store, new_index) {
                        s.next = Some(existing_head);
                    }
                }
            }

            store.table_entries[entry_index].sym = Some(new_index);
            if let Some(s) = Self::get_symbol_mut(store, new_index) {
                s.owner = Some(entry_index);
            }
            if matches!(Self::get_symbol_ref(store, new_index), Some(s) if s.flag == SymbolFlag::Local)
            {
                Self::append_symbol(SymbolListKind::Static, new_index);
            }
            new_index
        })
    }

    pub fn ident_change_storage(symbol_index: usize, storage: StorageClass) {
        Self::with_store(|store| {
            let current = match Self::get_symbol_ref(store, symbol_index) {
                Some(sym) => sym.storage.clone(),
                None => return,
            };
            if current == storage {
                return;
            }

            match storage {
                StorageClass::Static => {
                    Self::append_symbol(SymbolListKind::Static, symbol_index);
                }
                StorageClass::Auto => {
                    Self::append_symbol(SymbolListKind::Auto, symbol_index);
                }
                _ => {
                    Self::unlink_from_list(symbol_index);
                }
            }

            if let Some(sym) = Self::get_symbol_mut(store, symbol_index) {
                sym.storage = storage;
            }
        });
    }

    pub fn init_ident(symbol_index: usize, storage: StorageClass) {
        Self::with_store(|store| {
            if let Some(sym) = Self::get_symbol_mut(store, symbol_index) {
                sym.kind = SymbolKind::Identifier;
                sym.arity = -1;
                sym.storage = StorageClass::Extern;
                sym.decl = None;
                sym.source = None;
                sym.def_line = -1;
                sym.ref_line.clear();
                sym.callers.clear();
                sym.callees.clear();
                sym.level = -1;
            }
        });
        Self::ident_change_storage(symbol_index, storage);
    }

    pub fn install_ident(name: impl Into<String>, storage: StorageClass) -> usize {
        const INSTALL_DEFAULT: u32 = 0;
        const INSTALL_CHECK_LOCAL: u32 = 1 << 1;
        let use_flags = if storage != StorageClass::Auto {
            INSTALL_CHECK_LOCAL
        } else {
            INSTALL_DEFAULT
        };
        let index = Self::intern(name, use_flags);
        Self::init_ident(index, storage);
        index
    }

    pub fn unlink_symbol(symbol_index: usize) {
        Self::with_store(|store| {
            let owner = Self::get_symbol_ref(store, symbol_index).and_then(|s| s.owner);
            let Some(owner_index) = owner else {
                return;
            };

            let mut current = store.table_entries[owner_index].sym;
            let mut previous: Option<usize> = None;

            while let Some(idx) = current {
                let next = Self::get_symbol_ref(store, idx).and_then(|s| s.next);
                if idx == symbol_index {
                    if let Some(prev) = previous {
                        if let Some(prev_sym) = Self::get_symbol_mut(store, prev) {
                            prev_sym.next = next;
                        }
                    } else {
                        store.table_entries[owner_index].sym = next;
                    }
                    break;
                }
                previous = Some(idx);
                current = next;
            }

            if let Some(sym) = Self::get_symbol_mut(store, symbol_index) {
                sym.owner = None;
            }
        });
    }

    pub fn delete_symbol(symbol_index: usize) {
        Self::with_store(|store| {
            let can_delete = match Self::get_symbol_ref(store, symbol_index) {
                Some(sym) => sym.ref_line.is_empty() && !(store.reverse_tree && !sym.callees.is_empty()),
                None => false,
            };
            if !can_delete {
                return;
            }

            let membership = Self::get_symbol_ref(store, symbol_index)
                .and_then(|s| s.list_membership);
            if let Some(kind) = membership {
                Self::list_mut(store, kind).retain(|&idx| idx != symbol_index);
            }

            let name = Self::get_symbol_ref(store, symbol_index).map(|s| s.name.clone());
            Self::unlink_symbol(symbol_index);

            if let Some(sym) = store.symbols.get_mut(symbol_index).and_then(|s| s.take()) {
                if let Some(owner_index) = sym.owner {
                    if store.table_entries.get(owner_index).and_then(|e| e.sym).is_none() {
                        if let Some(name) = name {
                            store.buckets.remove(&name);
                        }
                    }
                }
            }
        });
    }

    pub fn retain_static(symbol_index: usize, print_xref: bool, include_symbol: bool) {
        Self::with_store(|store| {
            let Some(sym) = Self::get_symbol_ref(store, symbol_index).cloned() else {
                return;
            };
            if sym.owner.is_none() {
                return;
            }

            if sym.flag == SymbolFlag::Local {
                if print_xref && include_symbol {
                    Self::unlink_symbol(symbol_index);
                    if !store.unit_local.contains(&symbol_index) {
                        store.unit_local.push(symbol_index);
                    }
                } else {
                    let _ = store;
                    Self::delete_symbol(symbol_index);
                }
            } else {
                Self::unlink_symbol(symbol_index);
                let is_function = sym.kind == SymbolKind::Identifier && sym.arity >= 0;
                if is_function && !store.static_funcs.contains(&symbol_index) {
                    store.static_funcs.push(symbol_index);
                }
            }
        });
    }

    pub fn delete_statics() {
        let items = Self::with_store(|store| store.static_symbols.clone());
        for index in items {
            Self::retain_static(index, false, false);
        }
        Self::with_store(|store| {
            store.static_symbols.clear();
        });
    }

    pub fn delete_level_autos(symbol_index: usize, level: i32) -> bool {
        let should_delete = Self::with_store(|store| {
            Self::get_symbol_ref(store, symbol_index)
                .map(|s| s.level == level)
                .unwrap_or(false)
        });
        if should_delete {
            Self::delete_symbol(symbol_index);
            return true;
        }
        false
    }

    pub fn delete_level_statics(symbol_index: usize, level: i32) -> bool {
        let should_unlink = Self::with_store(|store| {
            Self::get_symbol_ref(store, symbol_index)
                .map(|s| s.level == level)
                .unwrap_or(false)
        });
        if should_unlink {
            Self::unlink_symbol(symbol_index);
            Self::unlink_from_list(symbol_index);
            return true;
        }
        false
    }

    pub fn delete_autos(level: i32) {
        let autos = Self::with_store(|store| store.auto_symbols.clone());
        for index in autos {
            let _ = Self::delete_level_autos(index, level);
        }
        let statics = Self::with_store(|store| store.static_symbols.clone());
        for index in statics {
            let _ = Self::delete_level_statics(index, level);
        }
    }

    pub fn collect_processor(
        entry: &TableEntry,
        selector: impl Fn(&Symbol) -> bool,
        output: &mut Vec<usize>,
    ) -> bool {
        Self::with_store(|store| {
            let mut current = entry.sym;
            while let Some(idx) = current {
                if let Some(sym) = Self::get_symbol_ref(store, idx) {
                    if selector(sym) {
                        output.push(idx);
                    }
                    current = sym.next;
                } else {
                    break;
                }
            }
        });
        true
    }

    pub fn collect_list_entry(
        symbol_index: usize,
        selector: impl Fn(&Symbol) -> bool,
        output: &mut Vec<usize>,
    ) -> bool {
        Self::with_store(|store| {
            if let Some(sym) = Self::get_symbol_ref(store, symbol_index) {
                if selector(sym) {
                    output.push(symbol_index);
                }
            }
        });
        false
    }

    pub fn collect_symbols(
        selector: impl Fn(&Symbol) -> bool,
        reserved_slots: usize,
    ) -> Vec<usize> {
        let mut collected = Vec::new();
        Self::with_store(|store| {
            for entry in &store.table_entries {
                let mut current = entry.sym;
                while let Some(idx) = current {
                    if let Some(sym) = Self::get_symbol_ref(store, idx) {
                        if selector(sym) {
                            collected.push(idx);
                        }
                        current = sym.next;
                    } else {
                        break;
                    }
                }
            }

            for &idx in &store.static_funcs {
                if let Some(sym) = Self::get_symbol_ref(store, idx) {
                    if selector(sym) {
                        collected.push(idx);
                    }
                }
            }

            for &idx in &store.unit_local {
                if let Some(sym) = Self::get_symbol_ref(store, idx) {
                    if selector(sym) {
                        collected.push(idx);
                    }
                }
            }
        });
        collected.reserve(reserved_slots);
        collected
    }

    pub fn collect_functions() -> Vec<usize> {
        let snum = Self::with_store(|store| store.static_funcs.len());
        let mut symbols = Self::collect_symbols(
            |sym| sym.kind == SymbolKind::Identifier && sym.arity >= 0,
            snum,
        );
        let statics = Self::with_store(|store| store.static_funcs.clone());
        symbols.extend(statics);
        symbols
    }

    pub fn delete_parms_itr(symbol_index: usize, level: i32) -> bool {
        let should_delete = Self::with_store(|store| {
            let Some(s) = Self::get_symbol_ref(store, symbol_index) else {
                return false;
            };
            s.owner.is_some()
                && s.kind == SymbolKind::Identifier
                && s.storage == StorageClass::Auto
                && s.flag == SymbolFlag::Parm
                && s.level > level
        });
        if should_delete {
            Self::delete_symbol(symbol_index);
            return true;
        }
        false
    }

    pub fn delete_parms(level: i32) {
        let autos = Self::with_store(|store| store.auto_symbols.clone());
        for index in autos {
            let _ = Self::delete_parms_itr(index, level);
        }
    }

    pub fn move_parms(level: i32) {
        Self::with_store(|store| {
            let indices = store.auto_symbols.clone();
            for idx in indices {
                if let Some(sym) = Self::get_symbol_mut(store, idx) {
                    if sym.kind == SymbolKind::Identifier
                        && sym.storage == StorageClass::Auto
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
        let index = Self::intern(name, 0);
        Self::with_store(|store| {
            if let Some(sym) = Self::get_symbol_mut(store, index) {
                sym.flag = SymbolFlag::Start;
            }
            if !store.start_symbols.contains(&index) {
                store.start_symbols.push(index);
            }
        });
        index
    }

    pub fn set_default_starter() {
        let empty = Self::with_store(|store| store.start_symbols.is_empty());
        if empty {
            Self::install_starter("main");
        }
    }

    pub fn clear_starters() {
        Self::with_store(|store| {
            store.start_symbols.clear();
        });
    }

    pub fn first_starter(iterator: &mut Option<usize>) -> Option<usize> {
        Self::with_store(|store| {
            let mut start = 0usize;
            while start < store.start_symbols.len() {
                let sym_idx = store.start_symbols[start];
                if let Some(sym) = Self::get_symbol_ref(store, sym_idx) {
                    if sym.kind != SymbolKind::Undefined {
                        *iterator = Some(start + 1);
                        return Some(sym_idx);
                    }
                }
                start += 1;
            }
            *iterator = None;
            None
        })
    }

    pub fn next_starter(iterator: &mut Option<usize>) -> Option<usize> {
        Self::with_store(|store| {
            let mut pos = iterator.unwrap_or(0);
            while pos < store.start_symbols.len() {
                let sym_idx = store.start_symbols[pos];
                pos += 1;
                if let Some(sym) = Self::get_symbol_ref(store, sym_idx) {
                    if sym.kind != SymbolKind::Undefined {
                        *iterator = Some(pos);
                        return Some(sym_idx);
                    }
                }
            }
            *iterator = None;
            None
        })
    }

    pub fn install_target(name: impl Into<String>) -> usize {
        let index = Self::intern(name, 0);
        Self::with_store(|store| {
            if let Some(sym) = Self::get_symbol_mut(store, index) {
                sym.flag = SymbolFlag::Target;
            }
            if !store.target_symbols.contains(&index) {
                store.target_symbols.push(index);
            }
        });
        index
    }

    pub fn mark_callers(symbol_index: usize) {
        fn rec(store: &mut SymbolStore, symbol_index: usize, seen: &mut HashSet<usize>) {
            if !seen.insert(symbol_index) {
                return;
            }

            let callers = match Symbol::get_symbol_mut(store, symbol_index) {
                Some(sym) => {
                    if sym.active {
                        return;
                    }
                    sym.active = true;
                    sym.callers.clone()
                }
                None => return,
            };

            for caller in callers {
                rec(store, caller, seen);
            }

            if let Some(sym) = Symbol::get_symbol_mut(store, symbol_index) {
                sym.visible = true;
                sym.active = false;
            }
        }

        Self::with_store(|store| {
            let mut seen = HashSet::new();
            rec(store, symbol_index, &mut seen);
        });
    }

    pub fn eliminate_non_targets() {
        let targets = Self::with_store(|store| store.target_symbols.clone());
        if targets.is_empty() {
            return;
        }

        for idx in targets {
            let is_target = Self::with_store(|store| {
                Self::get_symbol_ref(store, idx)
                    .map(|s| s.flag == SymbolFlag::Target)
                    .unwrap_or(false)
            });
            if is_target {
                Self::mark_callers(idx);
            }
        }

        Self::with_store(|store| {
            store.output_visible = true;
        });
    }

    pub fn table_entry(sym: Option<usize>) -> TableEntry {
        TableEntry { sym }
    }

    pub fn module_src_delete_level_12(
        symbol_index: usize,
        level: i32,
        is_static: bool,
    ) -> bool {
        if is_static {
            Self::delete_level_statics(symbol_index, level)
        } else {
            Self::delete_level_autos(symbol_index, level)
        }
    }

    pub fn module_src_table_entry_06(sym: Option<usize>) -> ModuleSrcTableEntry06 {
        ModuleSrcTableEntry06 { sym }
    }

    pub fn module_src_linked_list_entry_02(
        data: Option<usize>,
        next: Option<usize>,
        prev: Option<usize>,
    ) -> ModuleSrcLinkedListEntry02 {
        ModuleSrcLinkedListEntry02 { data, next, prev }
    }

    pub fn module_src_collect_data_10(
        reserved_slots: usize,
    ) -> ModuleSrcCollectData10 {
        ModuleSrcCollectData10 {
            indices: Vec::with_capacity(reserved_slots),
            count: 0,
            index: 0,
            reserved_slots,
        }
    }

    pub fn compare_by_name(a: usize, b: usize) -> Ordering {
        Self::with_store(|store| {
            let an = Self::get_symbol_ref(store, a)
                .map(|s| s.name.as_str())
                .unwrap_or("");
            let bn = Self::get_symbol_ref(store, b)
                .map(|s| s.name.as_str())
                .unwrap_or("");
            an.cmp(bn)
        })
    }
}
