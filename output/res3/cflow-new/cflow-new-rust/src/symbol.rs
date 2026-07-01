use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use crate::output::SymbolRecord;

#[derive(Clone, Debug, Default)]
pub struct TableEntry {
    pub sym: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LinkedListEntry {
    pub symbol_name: String,
}

#[derive(Clone, Debug)]
pub struct CollectData {
    pub sym: Vec<String>,
    pub count: usize,
    pub index: usize,
    pub sel: fn(&Symbol) -> bool,
}

impl CollectData {
    pub fn new(sel: fn(&Symbol) -> bool) -> Self {
        Self {
            sym: Vec::new(),
            count: 0,
            index: 0,
            sel,
        }
    }

    fn push(&mut self, name: String) {
        if self.index == self.count {
            self.count = if self.count == 0 { 4 } else { self.count * 2 };
            self.sym.reserve(self.count.saturating_sub(self.sym.len()));
        }
        self.sym.push(name);
        self.index += 1;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolStorage {
    Extern,
    Static,
    Auto,
    Other,
}

impl SymbolStorage {
    fn from_i32(storage: i32) -> Self {
        match storage {
            0 => Self::Extern,
            1 => Self::Static,
            2 => Self::Auto,
            _ => Self::Other,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct SymbolState {
    table: HashMap<String, String>,
    storage: HashMap<String, SymbolStorage>,
    levels: HashMap<String, i32>,
    local_symbols: HashSet<String>,
    parameter_symbols: HashSet<String>,
    alias_symbols: HashMap<String, String>,
    static_symbol_list: Vec<String>,
    auto_symbol_list: Vec<String>,
    static_func_list: Vec<String>,
    unit_local_list: Vec<String>,
    start_symbol_list: Vec<String>,
    target_symbols: HashSet<String>,
    starter_iter: usize,
}

impl SymbolState {
    fn hash_string(s: &str, n_buckets: usize) -> usize {
        if n_buckets == 0 {
            return 0;
        }
        let mut h = 0usize;
        for b in s.bytes() {
            h = h.wrapping_mul(31).wrapping_add(b as usize);
        }
        h % n_buckets
    }

    fn data_in_list(list: &[String], name: &str) -> bool {
        list.iter().any(|item| item == name)
    }

    fn symbol_is_function(&self, name: &str) -> bool {
        self.static_func_list.iter().any(|n| n == name)
    }

    fn include_symbol(&self, _name: &str) -> bool {
        true
    }

    fn reset_static_caller(&mut self) {}

    fn unlink_from_lists(&mut self, name: &str) {
        self.static_symbol_list.retain(|n| n != name);
        self.auto_symbol_list.retain(|n| n != name);
    }

    fn append_to_static(&mut self, name: &str) {
        self.unlink_from_lists(name);
        if !Self::data_in_list(&self.static_symbol_list, name) {
            self.static_symbol_list.push(name.to_string());
        }
        self.storage.insert(name.to_string(), SymbolStorage::Static);
    }

    fn append_to_auto(&mut self, name: &str) {
        self.unlink_from_lists(name);
        if !Self::data_in_list(&self.auto_symbol_list, name) {
            self.auto_symbol_list.push(name.to_string());
        }
        self.storage.insert(name.to_string(), SymbolStorage::Auto);
    }

    fn unlink_symbol(&mut self, name: &str) {
        self.unlink_from_lists(name);
        self.table.retain(|_, value| value != name);
    }

    fn remove_symbol(&mut self, name: &str) {
        self.unlink_symbol(name);
        self.storage.remove(name);
        self.levels.remove(name);
        self.local_symbols.remove(name);
        self.parameter_symbols.remove(name);
        self.alias_symbols.remove(name);
        self.start_symbol_list.retain(|n| n != name);
        self.static_func_list.retain(|n| n != name);
        self.unit_local_list.retain(|n| n != name);
        self.target_symbols.remove(name);
    }

    fn release_static_symbol(&mut self, name: &str) {
        let is_local = self.local_symbols.contains(name);
        if is_local {
            if self.include_symbol(name) {
                self.unlink_symbol(name);
                if !Self::data_in_list(&self.unit_local_list, name) {
                    self.unit_local_list.push(name.to_string());
                }
            } else {
                self.remove_symbol(name);
            }
        } else {
            self.unlink_symbol(name);
            if self.symbol_is_function(name) && !Self::data_in_list(&self.static_func_list, name) {
                self.static_func_list.push(name.to_string());
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    state: SymbolState,
}

impl Symbol {
    pub fn unlink_from_list(&mut self) {
        self.state.unlink_from_lists(&self.name);
    }

    pub fn append_symbol(list: &mut Vec<Symbol>, sp: &mut Symbol) {
        sp.unlink_from_list();
        if !list.iter().any(|s| s.name == sp.name) {
            list.push(sp.clone());
        }
        match sp.state.storage.get(&sp.name).copied() {
            Some(SymbolStorage::Static) => sp.state.append_to_static(&sp.name),
            Some(SymbolStorage::Auto) => sp.state.append_to_auto(&sp.name),
            _ => {}
        }
    }

    pub fn hasher(entry: &TableEntry, n_buckets: usize) -> usize {
        match entry.sym.as_deref() {
            Some(name) => SymbolState::hash_string(name, n_buckets),
            None => 0,
        }
    }

    pub fn compare(left: &TableEntry, right: &TableEntry) -> bool {
        match (&left.sym, &right.sym) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    pub fn find(&self, name: &str) -> Option<Self> {
        let mut current = self.state.table.get(name).cloned()?;
        while let Some(alias) = self.state.alias_symbols.get(&current) {
            current = alias.clone();
        }
        Some(Self {
            name: current,
            state: self.state.clone(),
        })
    }

    pub fn create(name: &str, flags: i32) -> Self {
        const INSTALL_OVERWRITE: i32 = 1 << 0;
        const INSTALL_CHECK_LOCAL: i32 = 1 << 1;
        const INSTALL_UNIT_LOCAL: i32 = 1 << 2;

        let mut state = SymbolState::default();

        if let Some(existing_name) = state.table.get(name).cloned() {
            if (flags & INSTALL_OVERWRITE) != 0 {
                return Self {
                    name: existing_name,
                    state,
                };
            }
        }

        state.table.insert(name.to_string(), name.to_string());

        if (flags & INSTALL_CHECK_LOCAL) != 0 || (flags & INSTALL_UNIT_LOCAL) != 0 {
            state.local_symbols.insert(name.to_string());
            state.append_to_static(name);
        } else {
            state.storage.insert(name.to_string(), SymbolStorage::Extern);
        }

        Self {
            name: name.to_string(),
            state,
        }
    }

    pub fn ident_change_storage(&mut self, storage: i32) {
        let new_storage = SymbolStorage::from_i32(storage);
        if self.state.storage.get(&self.name).copied() == Some(new_storage) {
            return;
        }

        match new_storage {
            SymbolStorage::Static => self.state.append_to_static(&self.name),
            SymbolStorage::Auto => self.state.append_to_auto(&self.name),
            _ => {
                self.state.unlink_from_lists(&self.name);
                self.state.storage.insert(self.name.clone(), new_storage);
            }
        }
    }

    pub fn init_ident(&mut self, storage: i32) {
        self.state
            .storage
            .insert(self.name.clone(), SymbolStorage::Extern);
        self.state.levels.insert(self.name.clone(), -1);
        self.state.parameter_symbols.remove(&self.name);
        self.ident_change_storage(storage);
    }

    pub fn install_ident(name: &str, storage: i32) -> Self {
        const INSTALL_DEFAULT: i32 = 0;
        const INSTALL_CHECK_LOCAL: i32 = 1 << 1;

        let flags = if storage != 2 {
            INSTALL_CHECK_LOCAL
        } else {
            INSTALL_DEFAULT
        };
        let mut sp = Self::create(name, flags);
        sp.init_ident(storage);
        sp
    }

    pub fn unlink_symbol(&mut self) {
        self.state.unlink_symbol(&self.name);
    }

    pub fn delete_symbol(&mut self) {
        self.state.remove_symbol(&self.name);
    }

    fn release_static(&mut self) {
        self.state.release_static_symbol(&self.name);
    }

    pub fn delete_statics(&mut self) {
        self.state.reset_static_caller();
        let names = std::mem::take(&mut self.state.static_symbol_list);
        for name in names {
            let mut symbol = Self {
                name,
                state: self.state.clone(),
            };
            symbol.release_static();
            self.state = symbol.state;
        }
    }

    pub fn delete_level_autos(&mut self, level: i32) -> bool {
        let should_delete = self.state.levels.get(&self.name).copied() == Some(level)
            && self.state.storage.get(&self.name).copied() == Some(SymbolStorage::Auto);
        if should_delete {
            self.delete_symbol();
            true
        } else {
            false
        }
    }

    pub fn delete_level_statics(&mut self, level: i32) -> bool {
        let should_unlink = self.state.levels.get(&self.name).copied() == Some(level)
            && self.state.storage.get(&self.name).copied() == Some(SymbolStorage::Static);
        if should_unlink {
            self.unlink_symbol();
            true
        } else {
            false
        }
    }

    pub fn delete_autos(&mut self, level: i32) {
        let autos = self.state.auto_symbol_list.clone();
        let statics = self.state.static_symbol_list.clone();

        for name in autos {
            let mut sym = Self {
                name,
                state: self.state.clone(),
            };
            let _ = sym.delete_level_autos(level);
            self.state = sym.state;
        }
        for name in statics {
            let mut sym = Self {
                name,
                state: self.state.clone(),
            };
            let _ = sym.delete_level_statics(level);
            self.state = sym.state;
        }
    }

    pub fn collect_processor(&self, entry: &TableEntry, data: &mut CollectData) -> bool {
        if let Some(head) = &entry.sym {
            let candidate = Self {
                name: head.clone(),
                state: self.state.clone(),
            };
            if (data.sel)(&candidate) {
                data.push(candidate.name);
            }
        }
        true
    }

    pub fn collect_list_entry(&self, item: &LinkedListEntry, data: &mut CollectData) -> bool {
        let candidate = Self {
            name: item.symbol_name.clone(),
            state: self.state.clone(),
        };
        if (data.sel)(&candidate) {
            data.push(candidate.name);
        }
        false
    }

    pub fn collect_symbols(&self, sel: fn(&Symbol) -> bool, reserved_slots: usize) -> Vec<Symbol> {
        let mut cdata = CollectData::new(sel);

        let table_entries = self
            .state
            .table
            .values()
            .cloned()
            .map(|name| TableEntry { sym: Some(name) })
            .collect::<Vec<_>>();
        let static_funcs = self
            .state
            .static_func_list
            .iter()
            .cloned()
            .map(|symbol_name| LinkedListEntry { symbol_name })
            .collect::<Vec<_>>();
        let unit_locals = self
            .state
            .unit_local_list
            .iter()
            .cloned()
            .map(|symbol_name| LinkedListEntry { symbol_name })
            .collect::<Vec<_>>();

        for entry in &table_entries {
            let _ = self.collect_processor(entry, &mut cdata);
        }
        for entry in &static_funcs {
            let _ = self.collect_list_entry(entry, &mut cdata);
        }
        for entry in &unit_locals {
            let _ = self.collect_list_entry(entry, &mut cdata);
        }

        cdata.sym.reserve(reserved_slots);
        cdata
            .sym
            .into_iter()
            .map(|name| Self {
                name,
                state: self.state.clone(),
            })
            .collect()
    }

    pub fn collect_functions(&self) -> Vec<Symbol> {
        fn is_function(sym: &Symbol) -> bool {
            sym.state.symbol_is_function(&sym.name)
        }

        let snum = self.state.static_func_list.len();
        let mut symbols = self.collect_symbols(is_function, snum);
        for name in self.state.static_func_list.clone() {
            symbols.push(Self {
                name,
                state: self.state.clone(),
            });
        }
        symbols
    }

    pub fn delete_parms_itr(&mut self, level: i32) -> bool {
        let should_delete = self.state.storage.get(&self.name).copied() == Some(SymbolStorage::Auto)
            && self.state.parameter_symbols.contains(&self.name)
            && self.state.levels.get(&self.name).copied().unwrap_or(-1) > level;
        if should_delete {
            self.delete_symbol();
            true
        } else {
            false
        }
    }

    pub fn delete_parms(&mut self, level: i32) {
        let autos = self.state.auto_symbol_list.clone();
        for name in autos {
            let mut sym = Self {
                name,
                state: self.state.clone(),
            };
            let _ = sym.delete_parms_itr(level);
            self.state = sym.state;
        }
    }

    pub fn move_parms(&mut self, level: i32) {
        let autos = self.state.auto_symbol_list.clone();
        for name in autos {
            if self.state.storage.get(&name).copied() == Some(SymbolStorage::Auto)
                && self.state.parameter_symbols.contains(&name)
            {
                self.state.levels.insert(name.clone(), level);
                self.state.parameter_symbols.remove(&name);
            }
        }
    }

    pub fn install_starter(&mut self, name: &str) -> Symbol {
        let sp = Symbol::create(name, 0);
        if !SymbolState::data_in_list(&self.state.start_symbol_list, &sp.name) {
            self.state.start_symbol_list.push(sp.name.clone());
        }
        sp
    }

    pub fn set_default_starter(&mut self) {
        if self.state.start_symbol_list.is_empty() {
            let _ = self.install_starter("main");
        }
    }

    pub fn clear_starters(&mut self) {
        self.state.start_symbol_list.clear();
        self.state.starter_iter = 0;
    }

    pub fn first_starter(&mut self) -> Option<Symbol> {
        self.state.starter_iter = 0;
        self.state
            .start_symbol_list
            .first()
            .cloned()
            .map(|name| Symbol {
                name,
                state: self.state.clone(),
            })
    }

    pub fn next_starter(&mut self) -> Option<Symbol> {
        self.state.starter_iter = self.state.starter_iter.saturating_add(1);
        self.state
            .start_symbol_list
            .get(self.state.starter_iter)
            .cloned()
            .map(|name| Symbol {
                name,
                state: self.state.clone(),
            })
    }

    pub fn install_target(&mut self, name: &str) -> Symbol {
        let sp = Symbol::create(name, 0);
        self.state.target_symbols.insert(sp.name.clone());
        sp
    }

    pub fn mark_callers(&mut self) {
        self.state.target_symbols.insert(self.name.clone());
    }

    pub fn eliminate_non_targets(&mut self) {
        let starters = self.state.start_symbol_list.clone();
        for name in starters {
            self.state.target_symbols.insert(name.clone());
            let mut sym = Symbol {
                name,
                state: self.state.clone(),
            };
            sym.mark_callers();
            self.state = sym.state;
        }
    }

    pub fn delete_scoped(&mut self, level: i32) {
        self.delete_autos(level);
        self.delete_parms(level);
    }

    pub fn table_entry(name: Option<&str>) -> TableEntry {
        TableEntry {
            sym: name.map(str::to_string),
        }
    }

    pub fn collect_data(sel: fn(&Symbol) -> bool) -> CollectData {
        CollectData::new(sel)
    }

    pub fn linked_list_entry(symbol_name: &str) -> LinkedListEntry {
        LinkedListEntry {
            symbol_name: symbol_name.to_string(),
        }
    }

    pub fn cmp_name(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }

    pub fn collect_records<'a>(&self, records: &'a [SymbolRecord]) -> Vec<&'a SymbolRecord> {
        let known = self
            .collect_symbols(|_| true, 0)
            .into_iter()
            .map(|symbol| symbol.name)
            .collect::<HashSet<_>>();
        records
            .iter()
            .filter(|record| known.contains(&record.name))
            .collect()
    }
}
