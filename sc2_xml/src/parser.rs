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

#[derive(Debug)]
pub enum Tag {
    /// A Tag that definitely has children, but may or may not have attributes
    Node {
        attrs: Map<&'static str, &'static str>,
        /// Key: Child tag name
        children: Map<&'static str, Tag>,
    },
    /// A Tag without children
    Leaf {
        attrs: Map<&'static str, &'static str>,
    },
    /// Multiple tags representing a hybrid between an array and a dictionary
    Array {
        /// Key: Array index (numeric or identifier), value: attributes map
        vals: Map<&'static str, Map<&'static str, &'static str>>,
    },
}

pub fn init_units() -> Map<&'static str, Tag> {
    let mut map = Map::default();

    for doc in [UNIT_WOL.deref(), &UNIT_HOTS, &UNIT_LOTV, &UNIT_MULTI] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let attrs: Map<&str, &str> = node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            let id = id.unwrap();
            if let Some(Tag::Node {
                attrs: a,
                children: _c,
            }) = map.get_mut(id)
            {
                update_attrs(&attrs, a)
            } else {
                map.insert(
                    id,
                    Tag::Node {
                        attrs,
                        children: Map::default(),
                    },
                );
            }

            let base = map.get_mut(id).unwrap();
            let child_map = match base {
                Tag::Node { attrs: _, children } => children,
                _ => panic!("Unreachable"),
            };

            init_children(node.children(), child_map)
        }
    }

    map
}

/// RECURSIVE
///
/// Traverses child nodes and populates their map entries. Recurses when encoutering a nested structure,
/// returns on leaves and arrays.
fn init_children(children: Children<'static, 'static>, map: &mut Map<&'static str, Tag>) {
    for child in children {
        let name = child.tag_name().name();
        let attrs: Map<&str, &str> = child.attributes().map(|x| (x.name(), x.value())).collect();

        // There are some arrays that don't align to these 2 conditions, but I don't need the data
        // from them
        if child.tag_name().name().ends_with("Array") || child.tag_name().name() == "Flags" {
            let entry = map.entry(name).or_insert(Tag::Array {
                vals: Map::default(),
            });

            if let Tag::Array { vals } = entry {
                // If it doesn't have an index attribute, it's definitely an insertion.
                // If it does have an index, it could be an insertion or a retrieval
                if let Some(&idx) = attrs.get("index") {
                    match idx.parse::<usize>() {
                        Ok(x) => {
                            // If the numeric index doesn't exist, we need to insert it in the
                            // proper position. The numeric index is used as a key since there isn't
                            // really a great alternative
                            if vals.get_index_entry(x).is_none() {
                                vals.insert(idx, attrs);
                            } else {
                                update_attrs(&attrs, &mut vals[x])
                            }
                        }
                        // No numeric index so it must be a retrieval
                        Err(_) => {
                            update_attrs(&attrs, vals.entry(idx).or_default());
                        }
                    }
                } else {
                    // the extra bit of nonsense is because we need &'static str and this is more
                    // "robust" than defining a const array for number -> str conversion. The leak
                    // hardly matters for such a short running program
                    vals.insert(vals.len().to_string().leak(), attrs);
                }
            }
        } else if !child.has_children() {
            if let Some(Tag::Leaf { attrs: a }) = map.get_mut(name) {
                update_attrs(&attrs, a)
            } else {
                map.insert(name, Tag::Leaf { attrs });
            }
        } else {
            // Branch
            if let Some(Tag::Node {
                attrs: a,
                children: _,
            }) = map.get_mut(name)
            {
                update_attrs(&attrs, a);
            } else {
                map.insert(
                    name,
                    Tag::Node {
                        attrs,
                        children: Map::default(),
                    },
                );
            }

            let b = map.get_mut(name).unwrap();
            if let Tag::Node {
                attrs: _,
                children: c,
            } = b
            {
                init_children(child.children(), c)
            }
        }
    }
}

fn update_attrs(src: &Map<&'static str, &'static str>, dst: &mut Map<&'static str, &'static str>) {
    for (k, v) in src {
        dst.insert(k, v);
    }
}

pub fn init_weapons() -> Map<&'static str, Tag> {
    let mut map = Map::default();

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
            if let Some(Tag::Node {
                attrs: a,
                children: _c,
            }) = map.get_mut(id)
            {
                update_attrs(&attrs, a)
            } else {
                map.insert(
                    id,
                    Tag::Node {
                        attrs,
                        children: Map::default(),
                    },
                );
            }

            let base = map.get_mut(id).unwrap();
            let child_map = match base {
                Tag::Node { attrs: _, children } => children,
                _ => panic!("Unreachable"),
            };

            init_children(node.children(), child_map);

            // base class defaults found in core.sc2mod. These only populate if they were "missing"
            // from the xml data
            child_map.entry("DisplayEffect").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", id)]),
            });
            child_map.entry("MinScanRange").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "5")]),
            });
            child_map.entry("Range").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "5")]),
            });
            child_map.entry("RangeSlop").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "1")]),
            });
            child_map.entry("ArcSlop").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "11.25")]),
            });
            child_map.entry("Period").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "0.8332")]),
            });
            child_map.entry("DamagePoint").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "0.167")]),
            });
            child_map.entry("Backswing").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "0.5")]),
            });
            child_map.entry("RandomDelayMin").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "-0.0625")]),
            });
            child_map.entry("RandomDelayMax").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", "1.25")]),
            });
            child_map.entry("Effect").or_insert(Tag::Leaf {
                attrs: Map::from_iter([("value", id)]),
            });
        }
    }

    map
}

pub fn init_effects() -> Map<&'static str, Tag> {
    let mut map = Map::default();

    for doc in [
        EFFECT_WOL.deref(),
        &EFFECT_HOTS,
        &EFFECT_LOTV,
        &EFFECT_MULTI,
    ] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let mut attrs: Map<&str, &str> = node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            attrs.insert("tagname", node.tag_name().name());

            let id = id.unwrap();
            if let Some(Tag::Node {
                attrs: a,
                children: _c,
            }) = map.get_mut(id)
            {
                update_attrs(&attrs, a)
            } else {
                map.insert(
                    id,
                    Tag::Node {
                        attrs,
                        children: Map::default(),
                    },
                );
            }

            let base = map.get_mut(id).unwrap();
            let (attrs, child_map) = match base {
                Tag::Node { attrs, children } => (attrs, children),
                _ => panic!("Unreachable"),
            };

            if node.tag_name().name() == "CEffectLaunchMissile" {
                // it's either this or change all the static strs to Strings. This shouldn't cause
                // too many problems, especially for a program that's so shortlived
                let temp: &'static str = format!("{id}Weapon").leak();

                child_map.insert(
                    "AmmoUnit",
                    Tag::Leaf {
                        attrs: Map::from_iter([("value", temp)]),
                    },
                );
            }

            if let Some(&parent_class) = attrs.get("parent") {
                match parent_class {
                    "DU_WEAP" => {
                        child_map.insert(
                            "Kind",
                            Tag::Leaf {
                                attrs: Map::from_iter([("value", "Melee")]),
                            },
                        );
                    }
                    "DU_WEAP_MISSILE" => {
                        child_map.insert(
                            "Kind",
                            Tag::Leaf {
                                attrs: Map::from_iter([("value", "Ranged")]),
                            },
                        );
                    }
                    "DU_WEAP_SPLASH" => {
                        child_map.insert(
                            "Kind",
                            Tag::Leaf {
                                attrs: Map::from_iter([("value", "Splash")]),
                            },
                        );
                    }
                    _ => (),
                };
            }

            init_children(node.children(), child_map)
        }
    }

    map
}

pub fn init_abils() -> Map<&'static str, Tag> {
    let mut map = Map::default();

    for doc in [UNIT_WOL.deref(), &UNIT_HOTS, &UNIT_LOTV, &UNIT_MULTI] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let mut attrs: Map<&str, &str> = node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            attrs.insert("tagname", node.tag_name().name());

            let id = id.unwrap();
            if let Some(Tag::Node {
                attrs: a,
                children: _c,
            }) = map.get_mut(id)
            {
                update_attrs(&attrs, a)
            } else {
                map.insert(
                    id,
                    Tag::Node {
                        attrs,
                        children: Map::default(),
                    },
                );
            }

            let base = map.get_mut(id).unwrap();
            let child_map = match base {
                Tag::Node { attrs: _, children } => children,
                _ => panic!("Unreachable"),
            };

            init_children(node.children(), child_map)
        }
    }

    map
}

pub fn init_upgrades() -> Map<&'static str, Tag> {
        let mut map = Map::default();

    for doc in [UNIT_WOL.deref(), &UNIT_HOTS, &UNIT_LOTV, &UNIT_MULTI] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let attrs: Map<&str, &str> = node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            let id = id.unwrap();
            if let Some(Tag::Node {
                attrs: a,
                children: _c,
            }) = map.get_mut(id)
            {
                update_attrs(&attrs, a)
            } else {
                map.insert(
                    id,
                    Tag::Node {
                        attrs,
                        children: Map::default(),
                    },
                );
            }

            let base = map.get_mut(id).unwrap();
            let child_map = match base {
                Tag::Node { attrs: _, children } => children,
                _ => panic!("Unreachable"),
            };

            init_children(node.children(), child_map)
        }
    }

    map
}

pub fn init_movers() -> Map<&'static str, Tag> {
        let mut map = Map::default();

    for doc in [UNIT_WOL.deref(), &UNIT_HOTS, &UNIT_LOTV, &UNIT_MULTI] {
        for node in doc.root().children().next().unwrap().children() {
            let id = node.attribute("id");
            let mut attrs: Map<&str, &str> = node.attributes().map(|x| (x.name(), x.value())).collect();
            if id.is_none() || node.has_attribute("default") {
                continue;
            }

            attrs.insert("tagname", node.tag_name().name());



            let id = id.unwrap();
            if let Some(Tag::Node {
                attrs: a,
                children: _c,
            }) = map.get_mut(id)
            {
                update_attrs(&attrs, a)
            } else {
                map.insert(
                    id,
                    Tag::Node {
                        attrs,
                        children: Map::default(),
                    },
                );
            }

            let base = map.get_mut(id).unwrap();
            let child_map = match base {
                Tag::Node { attrs: _, children } => children,
                _ => panic!("Unreachable"),
            };

            if node.tag_name().name() == "CMoverMissile" {
                child_map.insert(
                    "Acceleration",
                    Tag::Leaf {
                        attrs: Map::from_iter([("value", "3200")]),
                    },
                );
                child_map.insert(
                    "MaxSpeed",
                    Tag::Leaf {
                        attrs: Map::from_iter([("value", "18.75")]),
                    },
                );
            }

            init_children(node.children(), child_map)
        }
    }

    map
}