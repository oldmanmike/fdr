// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

use remote_debugger::dom::BackendNodeId;
use remote_debugger::dom::Rect;

struct LayerId(String);

struct SnapshotId(String);

struct ScrollRect {
    rect: Rect,
    scroll_type: ScrollType,
}

enum ScrollType {
    RepaintsOnScroll,
    TouchEventHandler,
    WheelEventHandler,
}

struct PictureTile {
    x: u32,
    y: u32,
    picture: String,
}

struct Layer {
    layer_id: LayerId,
    parent_layer_id: Option<LayerId>,
    backend_node_id: Option<BackendNodeId>,
    offset_x: u32,
    offset_y: u32,
    width: u32,
    height: u32,
    transform: Option<Vec<u32>>,
    anchor_x: Option<u32>,
    anchor_y: Option<u32>,
    anchor_z: Option<u32>,
    paint_count: i32,
    draws_content: bool,
    invisible: Option<bool>,
    scroll_rects: Option<Vec<ScrollRect>>,
}

struct PaintProfile<A>(Vec<A>);

struct LayerTreeObject;

enum LayerTreeEvent {
    LayerTreeDidChange(Option<Vec<Layer>>),
    LayerPainted(LayerId, Rect),
}

struct LayerTree;

impl LayerTree {
    fn enable() {
        unimplemented!()
    }
    fn disable() {
        unimplemented!()
    }
    fn compositing_reasons<'a>(layer_id: LayerId) -> io::Result<Vec<&'a str>> {
        unimplemented!()
    }
    fn make_snapshot(layer_id: LayerId) -> io::Result<SnapshotId> {
        unimplemented!()
    }
    fn load_snapshot(tiles: Vec<PictureTile>) -> io::Result<SnapshotId> {
        unimplemented!()
    }
    fn profile_snapshot<A>(snapshot_id: SnapshotId,
                           min_repeat_count: Option<i32>,
                           clip_rect: Option<u32>)
                           -> io::Result<Vec<PaintProfile<A>>> {
        unimplemented!()
    }
    fn replay_snapshot<'a>(snapshot_id: SnapshotId,
                           from_step: Option<i32>,
                           to_step: Option<i32>,
                           scale: Option<u32>)
                           -> io::Result<&'a str> {
        unimplemented!()
    }
    fn snapshot_command_log(snapshot_id: SnapshotId) -> io::Result<Vec<LayerTreeObject>> {
        unimplemented!()
    }
}
