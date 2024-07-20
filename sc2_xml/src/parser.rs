#![allow(unused_must_use)]
#![allow(clippy::single_match)]

use std::ops::Deref;

use roxmltree::{Children, Document, Node};

use paste::paste;
// use sc2_sim::{unit::*, *};
use static_init::dynamic;

use crate::Map;

// ---------------------------------------------------------------------------------------------- //
//                                        Data Definitions                                        //
// ---------------------------------------------------------------------------------------------- //

/// This minor abomination is necessary because .sc2mod files use a "versioning" system where only
/// subsequent changes are recorded. Thus, we need all the prior changes in all the prior files.
/// The files are grouped by data-type (e.g. weapondata, unitdata, moverdata). WoL, HotS, and LotV
/// are the game's 3 expansions, and Multi is the game's current balance patch.
///
/// This macro includes those files as static strings, creates roxmltree::Documents from them, and
/// creates Maps for each that link the top level ids to their corresponding nodes (if present) in
/// the form `[Option<Node<'static, 'static>>; 4]`, where each index refers to the WoL, HotS, LotV,
/// and Multi data in that order.
macro_rules! include_data {
    ($x: ident, $y: literal) => {
        paste!(
            pub const [<_ $x _WOL>]: &str = include_str!(concat!(r"..\..\mods\liberty.sc2mod\base.sc2data\gamedata\", $y, ".xml"));
            pub const [<_ $x _HOTS>]: &str = include_str!(concat!(r"..\..\mods\swarm.sc2mod\base.sc2data\gamedata\", $y, ".xml"));
            pub const [<_ $x _LOTV>]: &str = include_str!(concat!(r"..\..\mods\void.sc2mod\base.sc2data\gamedata\", $y, ".xml"));
            pub const [<_ $x _MULTI>]: &str = include_str!(concat!(r"..\..\mods\voidmulti.sc2mod\base.sc2data\gamedata\", $y, ".xml"));

            #[dynamic]
            static [<$x _WOL>]: Document<'static> = Document::parse([<_ $x _WOL>]).unwrap();
            #[dynamic]
            static [<$x _HOTS>]: Document<'static> = Document::parse([<_ $x _HOTS>]).unwrap();
            #[dynamic]
            static [<$x _LOTV>]: Document<'static> = Document::parse([<_ $x _LOTV>]).unwrap();
            #[dynamic]
            static [<$x _MULTI>]: Document<'static> = Document::parse([<_ $x _MULTI>]).unwrap();

            #[dynamic]
            static [<$x S>]: Map<&'static str, [Option<Node<'static, 'static>>; 4]> = {
                let mut result: Map<&str, [Option<Node<'_, '_>>; 4]> = Map::default();
                // first one needs to be .deref() for type inference to work properly
                for (i, doc) in [[<$x _WOL>].deref(), &[<$x _HOTS>], &[<$x _LOTV>], &[<$x _MULTI>]].into_iter().enumerate() {
                    // skip over the <Catalog> that contains everything
                    for node in doc.root().children().next().unwrap().children() {
                        if let Some(id) = node.attribute("id") {
                            if let Some(v) = result.get_mut(id) {
                                v[i] = Some(node);
                            } else {
                                let mut v = [None, None, None, None];
                                v[i] = Some(node);
                                result.insert(id, v);
                            }
                        }
                    }
                }
                result
            };
        );
    }
}

include_data!(UNIT, "unitdata");
include_data!(WEAPON, "weapondata");
include_data!(EFFECT, "effectdata");
include_data!(ABIL, "abildata");
include_data!(UPGRADE, "upgradedata");
include_data!(MOVER, "moverdata");

#[dynamic]
pub static UNIT_MAP: Map<&'static str, Tag> = init_units();
#[dynamic]
pub static WEAPON_MAP: Map<&'static str, Tag> = init_weapons();
#[dynamic]
pub static EFFECT_MAP: Map<&'static str, Tag> = init_effects();
#[dynamic(lazy)]
pub static ABIL_MAP: Map<&'static str, Tag> = init_abils();
#[dynamic]
pub static UPGRADE_MAP: Map<&'static str, Tag> = init_upgrades();
#[dynamic]
pub static MOVERS_MAP: Map<&'static str, Tag> = init_movers();

#[derive(Debug, Default, Clone)]
pub struct Tag {
    pub kind: &'static str,
    pub attrs: Map<&'static str, &'static str>,
    pub children: Map<&'static str, Tag>,
}

impl Tag {
    fn new(kind: &'static str, attrs: Map<&'static str, &'static str>) -> Self {
        Self {
            kind,
            attrs,
            children: Default::default(),
        }
    }

    pub fn val_of(&self, child: &str) -> &'static str {
        self.children[child].attrs["value"]
    }

    pub fn try_val_of(&self, child: &str) -> Option<&'static str> {
        self.children.get(child).map(|x| x.attrs["value"])
    }

    pub fn link_of(&self, child: &str) -> &'static str {
        self.children[child].attrs["Link"]
    }

    pub fn child_array(&self, child: &str) -> &Map<&'static str, Tag> {
        &self.children[child].children
    }

    pub fn id(&self) -> &'static str {
        self.attrs["id"]
    }
}

/// RECURSIVE
///
/// Traverses child nodes and populates their map entries. Recurses when encoutering a nested structure or array,
/// returns on leaves and non-nested arrays.
fn init_children(children: Children<'static, 'static>, map: &mut Map<&'static str, Tag>) {
    for child in children.filter(|x| x.is_element()) {
        let name = child.tag_name().name();
        let attrs: Map<&str, &str> = child.attributes().map(|x| (x.name(), x.value())).collect();

        // There are some arrays that don't align to these conditions, but I don't need the data
        // from them
        if child.tag_name().name().ends_with("Array") || name == "Flags" || name == "Attributes" || name == "CostResource" || name == "Collide" {
            // `entry` acts as the "container" for the array, the elements are stored in its
            // `children` map
            let entry = map.entry(name).or_insert(Tag {
                kind: name,
                attrs: Default::default(),
                children: Default::default(),
            });
            let elements = &mut entry.children;

            // If it doesn't have an index attribute, it's definitely an insertion.
            // If it does have an index, it could be an insertion or a retrieval
            if let Some(&idx) = attrs.get("index") {
                match idx.parse::<usize>() {
                    Ok(x) => {
                        // If the numeric index doesn't exist in the map, we insert and use the
                        // numeric index is used as a key since there isn't really an alternative
                        if elements.get_index_entry(x).is_none() {
                            elements.insert(
                                idx,
                                Tag {
                                    kind: name,
                                    attrs,
                                    children: Default::default(),
                                },
                            );
                        } else {
                            update_attrs(&attrs, &mut elements[x].attrs)
                        }
                        if child.has_children() {
                            init_children(
                                child.children(),
                                &mut elements.get_index_mut(x).unwrap().1.children,
                            )
                        }
                    }
                    // Index is not a number, so it must be a retrieval
                    Err(_) => {
                        let entry = elements
                            .entry(idx)
                            .and_modify(|x| update_attrs(&attrs, &mut x.attrs))
                            .or_insert(Tag::new(idx, attrs));
                        if child.has_children() {
                            init_children(child.children(), &mut entry.children)
                        }
                    }
                }
            } else {
                // the extra bit of nonsense is because we need &'static str and this is more
                // "robust" than defining a const array for number -> str conversion. The leak
                // hardly matters for such a short running program
                elements.insert(
                    elements.len().to_string().leak(),
                    Tag {
                        kind: name,
                        attrs,
                        children: Default::default(),
                    },
                );
            }
        } else if !child.has_children() {
            map
                .entry(name)
                .and_modify(|x| update_attrs(&attrs, &mut x.attrs))
                .or_insert(Tag::new(name, attrs));
            // if let Some(tag) = map.get_mut(name) {
            //     update_attrs(&attrs, &mut tag.attrs)
            // } else {
            //     map.insert(
            //         name,
            //         Tag {
            //             kind: name,
            //             attrs,
            //             children: Default::default(),
            //         },
            //     );
            // }
        } else {
            // Branch
            // if let Some(Tag {
            //     kind: _,
            //     attrs: a,
            //     children: _,
            // }) = map.get_mut(name)
            // {
            //     update_attrs(&attrs, a);
            // } else {
            //     map.insert(
            //         name,
            //         Tag {
            //             kind: name,
            //             attrs,
            //             children: Map::default(),
            //         },
            //     );
            // }

            let entry = map
                .entry(name)
                .and_modify(|x| update_attrs(&attrs, &mut x.attrs))
                .or_insert(Tag::new(name, attrs));

            init_children(child.children(), &mut entry.children)
        }
    }
}

fn update_attrs(src: &Map<&'static str, &'static str>, dst: &mut Map<&'static str, &'static str>) {
    for (k, v) in src {
        dst.insert(k, v);
    }
}

pub fn init_units() -> Map<&'static str, Tag> {
    let mut map: Map<&'static str, Tag> = Map::default();

    for doc in [UNIT_WOL.deref(), &UNIT_HOTS, &UNIT_LOTV, &UNIT_MULTI] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let attrs: Map<&str, &str> = node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() {
                continue;
            }

            let id = id.unwrap();
            if let Some(t) = map.get_mut(id) {
                update_attrs(&attrs, &mut t.attrs)
            } else {
                map.insert(id, Tag::new(node.tag_name().name(), attrs));
            }

            let base = map.get_mut(id).unwrap();

            init_children(node.children(), &mut base.children);
        }
    }

    map
}

pub fn init_weapons() -> Map<&'static str, Tag> {
    let mut map: Map<&'static str, Tag> = Map::default();

    for doc in [
        WEAPON_WOL.deref(),
        &WEAPON_HOTS,
        &WEAPON_LOTV,
        &WEAPON_MULTI,
    ] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let attrs: Map<&str, &str> = node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            let id = id.unwrap();
            if let Some(t) = map.get_mut(id) {
                update_attrs(&attrs, &mut t.attrs)
            } else {
                map.insert(id, Tag::new(node.tag_name().name(), attrs));
            }

            let base = map.get_mut(id).unwrap();

            init_children(node.children(), &mut base.children);

            // base class defaults found in core.sc2mod. These only populate if they were "missing"
            // from the xml data
            base.children
                .entry("DisplayEffect")
                .or_insert(Tag::new("DisplayEffect", Map::from_iter([("value", id)])));
            base.children
                .entry("MinScanRange")
                .or_insert(Tag::new("MinScanRange", Map::from_iter([("value", "5")])));
            base.children
                .entry("Range")
                .or_insert(Tag::new("Range", Map::from_iter([("value", "5")])));
            base.children
                .entry("RangeSlop")
                .or_insert(Tag::new("RangeSlop", Map::from_iter([("value", "1")])));
            base.children
                .entry("ArcSlop")
                .or_insert(Tag::new("ArcSlop", Map::from_iter([("value", "11.25")])));
            base.children
                .entry("Period")
                .or_insert(Tag::new("Period", Map::from_iter([("value", "0.8332")])));
            base.children.entry("DamagePoint").or_insert(Tag::new(
                "DamagePoint",
                Map::from_iter([("value", "0.167")]),
            ));
            base.children
                .entry("Backswing")
                .or_insert(Tag::new("Backswing", Map::from_iter([("value", "0.5")])));
            base.children.entry("RandomDelayMin").or_insert(Tag::new(
                "RandomDelayMin",
                Map::from_iter([("value", "-0.0625")]),
            ));
            base.children.entry("RandomDelayMax").or_insert(Tag::new(
                "RandomDelayMax",
                Map::from_iter([("value", "1.25")]),
            ));
            base.children
                .entry("Effect")
                .or_insert(Tag::new("Effect", Map::from_iter([("value", id)])));
        }
    }

    map
}

pub fn init_effects() -> Map<&'static str, Tag> {
    let mut map: Map<&'static str, Tag> = Map::default();

    for doc in [
        EFFECT_WOL.deref(),
        &EFFECT_HOTS,
        &EFFECT_LOTV,
        &EFFECT_MULTI,
    ] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let mut attrs: Map<&str, &str> =
                node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            attrs.insert("tagname", node.tag_name().name());

            let id = id.unwrap();
            if let Some(t) = map.get_mut(id) {
                update_attrs(&attrs, &mut t.attrs)
            } else {
                map.insert(id, Tag::new(node.tag_name().name(), attrs));
            }

            let base = map.get_mut(id).unwrap();

            if node.tag_name().name() == "CEffectLaunchMissile" {
                // it's either this or change all the static strs to Strings. This shouldn't cause
                // too many problems, especially for a program that's so shortlived
                let temp: &'static str = format!("{id}Weapon").leak();

                base.children.insert(
                    "AmmoUnit",
                    Tag::new("AmmoUnit", Map::from_iter([("value", temp)])),
                );
            }

            if let Some(&parent_class) = base.attrs.get("parent") {
                match parent_class {
                    "DU_WEAP" => {
                        base.children.insert(
                            "Kind",
                            Tag::new("Kind", Map::from_iter([("value", "Melee")])),
                        );
                    }
                    "DU_WEAP_MISSILE" => {
                        base.children.insert(
                            "Kind",
                            Tag::new("Kind", Map::from_iter([("value", "Ranged")])),
                        );
                    }
                    "DU_WEAP_SPLASH" => {
                        base.children.insert(
                            "Kind",
                            Tag::new("Kind", Map::from_iter([("value", "Splash")])),
                        );
                    }
                    _ => (),
                };
            }

            init_children(node.children(), &mut base.children)
        }
    }

    map
}

pub fn init_abils() -> Map<&'static str, Tag> {
    let mut map: Map<&'static str, Tag> = Map::default();

    for doc in [ABIL_WOL.deref(), &ABIL_HOTS, &ABIL_LOTV, &ABIL_MULTI] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let mut attrs: Map<&str, &str> =
                node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            let id = id.unwrap();
            if let Some(t) = map.get_mut(id) {
                update_attrs(&attrs, &mut t.attrs)
            } else {
                map.insert(id, Tag::new(node.tag_name().name(), attrs));
            }

            let base = map.get_mut(id).unwrap();

            init_children(node.children(), &mut base.children)
        }
    }

    map
}

pub fn init_upgrades() -> Map<&'static str, Tag> {
    let mut map: Map<&'static str, Tag> = Map::default();

    for doc in [
        UPGRADE_WOL.deref(),
        &UPGRADE_HOTS,
        &UPGRADE_LOTV,
        &UPGRADE_MULTI,
    ] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let attrs: Map<&str, &str> = node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() {
                continue;
            }

            let id = id.unwrap();
            if let Some(parent) = attrs.get("parent").and_then(|parent| map.get(parent)) {
                let parent_children = parent.children.clone();
                map.insert(id, Tag::new(node.tag_name().name(), attrs));
                map.get_mut(id).unwrap().children = parent_children;
            } else if let Some(t) = map.get_mut(id) {
                update_attrs(&attrs, &mut t.attrs)
            } else {
                map.insert(id, Tag::new(node.tag_name().name(), attrs));
            }

            let base = map.get_mut(id).unwrap();

            init_children(node.children(), &mut base.children)
        }
    }

    map
}

pub fn init_movers() -> Map<&'static str, Tag> {
    let mut map: Map<&'static str, Tag> = Map::default();

    for doc in [MOVER_WOL.deref(), &MOVER_HOTS, &MOVER_LOTV, &MOVER_MULTI] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let mut attrs: Map<&str, &str> =
                node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            let id = id.unwrap();
            if let Some(t) = map.get_mut(id) {
                update_attrs(&attrs, &mut t.attrs)
            } else {
                map.insert(id, Tag::new(node.tag_name().name(), attrs));
            }

            let base = map.get_mut(id).unwrap();

            if node.tag_name().name() == "CMoverMissile" {
                base.children.insert(
                    "Acceleration",
                    Tag::new("Acceleration", Map::from_iter([("value", "3200")])),
                );
                base.children.insert(
                    "MaxSpeed",
                    Tag::new("MaxSpeed", Map::from_iter([("value", "18.75")])),
                );
            }

            init_children(node.children(), &mut base.children)
        }
    }

    map
}
