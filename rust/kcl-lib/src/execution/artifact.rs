use std::collections::HashMap;

use fnv::FnvHashMap;
use indexmap::IndexMap;
use kittycad_modeling_cmds::{
    self as kcmc,
    id::ModelingCmdId,
    ok_response::OkModelingCmdResponse,
    websocket::{BatchResponse, OkWebSocketResponseData, WebSocketResponse},
    EnableSketchMode, ModelingCmd,
};
use schemars::JsonSchema;
use serde::{ser::SerializeSeq, Serialize};
use uuid::Uuid;

use crate::{
    errors::KclErrorDetails,
    parsing::ast::types::{Node, Program},
    KclError, NodePath, SourceRange,
};

#[cfg(test)]
mod mermaid_tests;

/// A command that may create or update artifacts on the TS side.  Because
/// engine commands are batched, we don't have the response yet when these are
/// created.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct ArtifactCommand {
    /// Identifier of the command that can be matched with its response.
    pub cmd_id: Uuid,
    pub range: SourceRange,
    /// The engine command.  Each artifact command is backed by an engine
    /// command.  In the future, we may need to send information to the TS side
    /// without an engine command, in which case, we would make this field
    /// optional.
    pub command: ModelingCmd,
}

impl PartialOrd for ArtifactCommand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Order by the source range.
        let range = self.range.cmp(&other.range);
        if range != std::cmp::Ordering::Equal {
            return Some(range);
        }
        #[cfg(test)]
        {
            // If the ranges are equal, order by the serde variant.
            Some(
                crate::variant_name::variant_name(&self.command)
                    .cmp(&crate::variant_name::variant_name(&other.command)),
            )
        }
        #[cfg(not(test))]
        self.cmd_id.partial_cmp(&other.cmd_id)
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Ord, PartialOrd, Hash, ts_rs::TS, JsonSchema)]
#[ts(export_to = "Artifact.ts")]
pub struct ArtifactId(Uuid);

impl ArtifactId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<Uuid> for ArtifactId {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl From<&Uuid> for ArtifactId {
    fn from(uuid: &Uuid) -> Self {
        Self::new(*uuid)
    }
}

impl From<ArtifactId> for Uuid {
    fn from(id: ArtifactId) -> Self {
        id.0
    }
}

impl From<&ArtifactId> for Uuid {
    fn from(id: &ArtifactId) -> Self {
        id.0
    }
}

impl From<ModelingCmdId> for ArtifactId {
    fn from(id: ModelingCmdId) -> Self {
        Self::new(*id.as_ref())
    }
}

impl From<&ModelingCmdId> for ArtifactId {
    fn from(id: &ModelingCmdId) -> Self {
        Self::new(*id.as_ref())
    }
}

pub type DummyPathToNode = Vec<()>;

fn serialize_dummy_path_to_node<S>(_path_to_node: &DummyPathToNode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Always output an empty array, for now.
    let seq = serializer.serialize_seq(Some(0))?;
    seq.end()
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct CodeRef {
    pub range: SourceRange,
    pub node_path: NodePath,
    // TODO: We should implement this in Rust.
    #[serde(default, serialize_with = "serialize_dummy_path_to_node")]
    #[ts(type = "Array<[string | number, string]>")]
    pub path_to_node: DummyPathToNode,
}

impl CodeRef {
    pub fn placeholder(range: SourceRange) -> Self {
        Self {
            range,
            node_path: Default::default(),
            path_to_node: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct CompositeSolid {
    pub id: ArtifactId,
    pub sub_type: CompositeSolidSubType,
    /// Constituent solids of the composite solid.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub solid_ids: Vec<ArtifactId>,
    /// Tool solids used for asymmetric operations like subtract.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_ids: Vec<ArtifactId>,
    pub code_ref: CodeRef,
    /// This is the ID of the composite solid that this is part of, if any, as a
    /// composite solid can be used as input for another composite solid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composite_solid_id: Option<ArtifactId>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub enum CompositeSolidSubType {
    Intersect,
    Subtract,
    Union,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct Plane {
    pub id: ArtifactId,
    pub path_ids: Vec<ArtifactId>,
    pub code_ref: CodeRef,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub id: ArtifactId,
    pub plane_id: ArtifactId,
    pub seg_ids: Vec<ArtifactId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sweep_id: Option<ArtifactId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solid2d_id: Option<ArtifactId>,
    pub code_ref: CodeRef,
    /// This is the ID of the composite solid that this is part of, if any, as
    /// this can be used as input for another composite solid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composite_solid_id: Option<ArtifactId>,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub id: ArtifactId,
    pub path_id: ArtifactId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<ArtifactId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<ArtifactId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_cut_id: Option<ArtifactId>,
    pub code_ref: CodeRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub common_surface_ids: Vec<ArtifactId>,
}

/// A sweep is a more generic term for extrude, revolve, loft, and sweep.
#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct Sweep {
    pub id: ArtifactId,
    pub sub_type: SweepSubType,
    pub path_id: ArtifactId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub surface_ids: Vec<ArtifactId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<ArtifactId>,
    pub code_ref: CodeRef,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub enum SweepSubType {
    Extrusion,
    Revolve,
    RevolveAboutEdge,
    Loft,
    Sweep,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct Solid2d {
    pub id: ArtifactId,
    pub path_id: ArtifactId,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct StartSketchOnFace {
    pub id: ArtifactId,
    pub face_id: ArtifactId,
    pub code_ref: CodeRef,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct StartSketchOnPlane {
    pub id: ArtifactId,
    pub plane_id: ArtifactId,
    pub code_ref: CodeRef,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct Wall {
    pub id: ArtifactId,
    pub seg_id: ArtifactId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_cut_edge_ids: Vec<ArtifactId>,
    pub sweep_id: ArtifactId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path_ids: Vec<ArtifactId>,
    /// This is for the sketch-on-face plane, not for the wall itself.  Traverse
    /// to the extrude and/or segment to get the wall's code_ref.
    pub face_code_ref: CodeRef,
    /// The command ID that got the data for this wall. Used for stable sorting.
    pub cmd_id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct Cap {
    pub id: ArtifactId,
    pub sub_type: CapSubType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_cut_edge_ids: Vec<ArtifactId>,
    pub sweep_id: ArtifactId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path_ids: Vec<ArtifactId>,
    /// This is for the sketch-on-face plane, not for the cap itself.  Traverse
    /// to the extrude and/or segment to get the cap's code_ref.
    pub face_code_ref: CodeRef,
    /// The command ID that got the data for this cap. Used for stable sorting.
    pub cmd_id: uuid::Uuid,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Ord, PartialOrd, Eq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub enum CapSubType {
    Start,
    End,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct SweepEdge {
    pub id: ArtifactId,
    pub sub_type: SweepEdgeSubType,
    pub seg_id: ArtifactId,
    pub cmd_id: uuid::Uuid,
    pub sweep_id: ArtifactId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub common_surface_ids: Vec<ArtifactId>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Ord, PartialOrd, Eq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub enum SweepEdgeSubType {
    Opposite,
    Adjacent,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct EdgeCut {
    pub id: ArtifactId,
    pub sub_type: EdgeCutSubType,
    pub consumed_edge_id: ArtifactId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<ArtifactId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<ArtifactId>,
    pub code_ref: CodeRef,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd, Ord, Eq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub enum EdgeCutSubType {
    Fillet,
    Chamfer,
}

impl From<kcmc::shared::CutType> for EdgeCutSubType {
    fn from(cut_type: kcmc::shared::CutType) -> Self {
        match cut_type {
            kcmc::shared::CutType::Fillet => EdgeCutSubType::Fillet,
            kcmc::shared::CutType::Chamfer => EdgeCutSubType::Chamfer,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct EdgeCutEdge {
    pub id: ArtifactId,
    pub edge_cut_id: ArtifactId,
    pub surface_id: ArtifactId,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct Helix {
    pub id: ArtifactId,
    /// The axis of the helix.  Currently this is always an edge ID, but we may
    /// add axes to the graph.
    pub axis_id: Option<ArtifactId>,
    pub code_ref: CodeRef,
}

#[derive(Debug, Clone, Serialize, PartialEq, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Artifact {
    CompositeSolid(CompositeSolid),
    Plane(Plane),
    Path(Path),
    Segment(Segment),
    Solid2d(Solid2d),
    StartSketchOnFace(StartSketchOnFace),
    StartSketchOnPlane(StartSketchOnPlane),
    Sweep(Sweep),
    Wall(Wall),
    Cap(Cap),
    SweepEdge(SweepEdge),
    EdgeCut(EdgeCut),
    #[expect(unused)]
    EdgeCutEdge(EdgeCutEdge),
    Helix(Helix),
}

impl Artifact {
    pub(crate) fn rank(&self) -> u8 {
        match self {
            Artifact::Plane(_) => 0,
            Artifact::StartSketchOnPlane(_) => 1,
            Artifact::StartSketchOnFace(_) => 2,
            Artifact::Path(_) => 3,
            Artifact::Segment(_) => 4,
            Artifact::Solid2d(_) => 5,
            Artifact::Sweep(_) => 6,
            Artifact::CompositeSolid(_) => 7,
            Artifact::Wall(_) => 8,
            Artifact::Cap(Cap { sub_type, .. }) if *sub_type == CapSubType::Start => 9,
            Artifact::Cap(Cap { sub_type, .. }) if *sub_type == CapSubType::Start => 10,
            Artifact::Cap(_) => 11,
            Artifact::SweepEdge(SweepEdge { sub_type, .. }) if *sub_type == SweepEdgeSubType::Adjacent => 12,
            Artifact::SweepEdge(SweepEdge { sub_type, .. }) if *sub_type == SweepEdgeSubType::Opposite => 13,
            Artifact::SweepEdge(_) => 14,
            Artifact::EdgeCut(_) => 15,
            Artifact::EdgeCutEdge(_) => 16,
            Artifact::Helix(_) => 17,
        }
    }
}

impl PartialOrd for Artifact {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // The only thing we want to sort is if we have two sweep edges, we want
        // to sort them by the sub_type.
        match (self, other) {
            (Artifact::SweepEdge(a), Artifact::SweepEdge(b)) => {
                if a.sub_type != b.sub_type {
                    return Some(a.sub_type.cmp(&b.sub_type));
                }
                if a.sweep_id != b.sweep_id {
                    return Some(a.sweep_id.cmp(&b.sweep_id));
                }
                if a.cmd_id != b.cmd_id {
                    return Some(a.cmd_id.cmp(&b.cmd_id));
                }
                Some(a.id.cmp(&b.id))
            }
            (Artifact::EdgeCut(a), Artifact::EdgeCut(b)) => {
                if a.code_ref.range != b.code_ref.range {
                    return Some(a.code_ref.range.cmp(&b.code_ref.range));
                }
                Some(a.id.cmp(&b.id))
            }
            (Artifact::EdgeCutEdge(a), Artifact::EdgeCutEdge(b)) => Some(a.edge_cut_id.cmp(&b.edge_cut_id)),
            (Artifact::Sweep(a), Artifact::Sweep(b)) => {
                if a.code_ref.range != b.code_ref.range {
                    return Some(a.code_ref.range.cmp(&b.code_ref.range));
                }
                Some(a.id.cmp(&b.id))
            }
            // Sort the planes by their code_ref range.
            (Artifact::Plane(a), Artifact::Plane(b)) => {
                if a.code_ref.range != b.code_ref.range {
                    return Some(a.code_ref.range.cmp(&b.code_ref.range));
                }
                Some(a.id.cmp(&b.id))
            }
            // Sort the paths by their code_ref range.
            (Artifact::Path(a), Artifact::Path(b)) => {
                if a.code_ref.range != b.code_ref.range {
                    return Some(a.code_ref.range.cmp(&b.code_ref.range));
                }
                Some(a.id.cmp(&b.id))
            }
            // Sort the segments by their code_ref range.
            (Artifact::Segment(a), Artifact::Segment(b)) => {
                if a.code_ref.range != b.code_ref.range {
                    return Some(a.code_ref.range.cmp(&b.code_ref.range));
                }
                Some(a.id.cmp(&b.id))
            }
            // Sort the solid2d by their id.
            (Artifact::Solid2d(a), Artifact::Solid2d(b)) => {
                if a.path_id != b.path_id {
                    return Some(a.path_id.cmp(&b.path_id));
                }
                Some(a.id.cmp(&b.id))
            }
            // Sort the walls by their code_ref range.
            (Artifact::Wall(a), Artifact::Wall(b)) => {
                if a.sweep_id != b.sweep_id {
                    return Some(a.sweep_id.cmp(&b.sweep_id));
                }
                if a.cmd_id != b.cmd_id {
                    return Some(a.cmd_id.cmp(&b.cmd_id));
                }
                if a.face_code_ref.range != b.face_code_ref.range {
                    return Some(a.face_code_ref.range.cmp(&b.face_code_ref.range));
                }
                if a.seg_id != b.seg_id {
                    return Some(a.seg_id.cmp(&b.seg_id));
                }
                Some(a.id.cmp(&b.id))
            }
            // Sort the caps by their code_ref range.
            (Artifact::Cap(a), Artifact::Cap(b)) => {
                if a.sub_type != b.sub_type {
                    return Some(a.sub_type.cmp(&b.sub_type));
                }
                if a.cmd_id != b.cmd_id {
                    return Some(a.cmd_id.cmp(&b.cmd_id));
                }
                if a.sweep_id != b.sweep_id {
                    return Some(a.sweep_id.cmp(&b.sweep_id));
                }
                if a.face_code_ref.range != b.face_code_ref.range {
                    return Some(a.face_code_ref.range.cmp(&b.face_code_ref.range));
                }
                Some(a.id.cmp(&b.id))
            }
            (Artifact::CompositeSolid(a), Artifact::CompositeSolid(b)) => Some(a.id.cmp(&b.id)),
            (Artifact::StartSketchOnFace(a), Artifact::StartSketchOnFace(b)) => Some(a.id.cmp(&b.id)),
            (Artifact::StartSketchOnPlane(a), Artifact::StartSketchOnPlane(b)) => Some(a.id.cmp(&b.id)),
            // Planes are first, then paths, then segments, then solids2ds, then sweeps, then
            // walls, then caps, then sweep edges, then edge cuts, then edge cut edges, then
            // helixes.
            _ => Some(self.rank().cmp(&other.rank())),
        }
    }
}

impl Artifact {
    pub(crate) fn id(&self) -> ArtifactId {
        match self {
            Artifact::CompositeSolid(a) => a.id,
            Artifact::Plane(a) => a.id,
            Artifact::Path(a) => a.id,
            Artifact::Segment(a) => a.id,
            Artifact::Solid2d(a) => a.id,
            Artifact::StartSketchOnFace(a) => a.id,
            Artifact::StartSketchOnPlane(a) => a.id,
            Artifact::Sweep(a) => a.id,
            Artifact::Wall(a) => a.id,
            Artifact::Cap(a) => a.id,
            Artifact::SweepEdge(a) => a.id,
            Artifact::EdgeCut(a) => a.id,
            Artifact::EdgeCutEdge(a) => a.id,
            Artifact::Helix(a) => a.id,
        }
    }

    #[expect(dead_code)]
    pub(crate) fn code_ref(&self) -> Option<&CodeRef> {
        match self {
            Artifact::CompositeSolid(a) => Some(&a.code_ref),
            Artifact::Plane(a) => Some(&a.code_ref),
            Artifact::Path(a) => Some(&a.code_ref),
            Artifact::Segment(a) => Some(&a.code_ref),
            Artifact::Solid2d(_) => None,
            Artifact::StartSketchOnFace(a) => Some(&a.code_ref),
            Artifact::StartSketchOnPlane(a) => Some(&a.code_ref),
            Artifact::Sweep(a) => Some(&a.code_ref),
            Artifact::Wall(_) => None,
            Artifact::Cap(_) => None,
            Artifact::SweepEdge(_) => None,
            Artifact::EdgeCut(a) => Some(&a.code_ref),
            Artifact::EdgeCutEdge(_) => None,
            Artifact::Helix(a) => Some(&a.code_ref),
        }
    }

    /// Merge the new artifact into self.  If it can't because it's a different
    /// type, return the new artifact which should be used as a replacement.
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        match self {
            Artifact::CompositeSolid(a) => a.merge(new),
            Artifact::Plane(a) => a.merge(new),
            Artifact::Path(a) => a.merge(new),
            Artifact::Segment(a) => a.merge(new),
            Artifact::Solid2d(_) => Some(new),
            Artifact::StartSketchOnFace { .. } => Some(new),
            Artifact::StartSketchOnPlane { .. } => Some(new),
            Artifact::Sweep(a) => a.merge(new),
            Artifact::Wall(a) => a.merge(new),
            Artifact::Cap(a) => a.merge(new),
            Artifact::SweepEdge(_) => Some(new),
            Artifact::EdgeCut(a) => a.merge(new),
            Artifact::EdgeCutEdge(_) => Some(new),
            Artifact::Helix(_) => Some(new),
        }
    }
}

impl CompositeSolid {
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        let Artifact::CompositeSolid(new) = new else {
            return Some(new);
        };
        merge_ids(&mut self.solid_ids, new.solid_ids);
        merge_ids(&mut self.tool_ids, new.tool_ids);
        merge_opt_id(&mut self.composite_solid_id, new.composite_solid_id);

        None
    }
}

impl Plane {
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        let Artifact::Plane(new) = new else {
            return Some(new);
        };
        merge_ids(&mut self.path_ids, new.path_ids);

        None
    }
}

impl Path {
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        let Artifact::Path(new) = new else {
            return Some(new);
        };
        merge_opt_id(&mut self.sweep_id, new.sweep_id);
        merge_ids(&mut self.seg_ids, new.seg_ids);
        merge_opt_id(&mut self.solid2d_id, new.solid2d_id);
        merge_opt_id(&mut self.composite_solid_id, new.composite_solid_id);

        None
    }
}

impl Segment {
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        let Artifact::Segment(new) = new else {
            return Some(new);
        };
        merge_opt_id(&mut self.surface_id, new.surface_id);
        merge_ids(&mut self.edge_ids, new.edge_ids);
        merge_opt_id(&mut self.edge_cut_id, new.edge_cut_id);
        merge_ids(&mut self.common_surface_ids, new.common_surface_ids);

        None
    }
}

impl Sweep {
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        let Artifact::Sweep(new) = new else {
            return Some(new);
        };
        merge_ids(&mut self.surface_ids, new.surface_ids);
        merge_ids(&mut self.edge_ids, new.edge_ids);

        None
    }
}

impl Wall {
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        let Artifact::Wall(new) = new else {
            return Some(new);
        };
        merge_ids(&mut self.edge_cut_edge_ids, new.edge_cut_edge_ids);
        merge_ids(&mut self.path_ids, new.path_ids);

        None
    }
}

impl Cap {
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        let Artifact::Cap(new) = new else {
            return Some(new);
        };
        merge_ids(&mut self.edge_cut_edge_ids, new.edge_cut_edge_ids);
        merge_ids(&mut self.path_ids, new.path_ids);

        None
    }
}

impl EdgeCut {
    fn merge(&mut self, new: Artifact) -> Option<Artifact> {
        let Artifact::EdgeCut(new) = new else {
            return Some(new);
        };
        merge_opt_id(&mut self.surface_id, new.surface_id);
        merge_ids(&mut self.edge_ids, new.edge_ids);

        None
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, ts_rs::TS)]
#[ts(export_to = "Artifact.ts")]
#[serde(rename_all = "camelCase")]
pub struct ArtifactGraph {
    map: IndexMap<ArtifactId, Artifact>,
}

impl ArtifactGraph {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Used to make the mermaid tests deterministic.
    #[cfg(test)]
    pub(crate) fn sort(&mut self) {
        self.map
            .sort_by(|_ak, av, _bk, bv| av.partial_cmp(bv).unwrap_or(std::cmp::Ordering::Equal));
    }
}

pub(super) fn build_artifact_graph(
    artifact_commands: &[ArtifactCommand],
    responses: &IndexMap<Uuid, WebSocketResponse>,
    ast: &Node<Program>,
    exec_artifacts: &IndexMap<ArtifactId, Artifact>,
) -> Result<ArtifactGraph, KclError> {
    let mut map = IndexMap::new();

    let mut path_to_plane_id_map = FnvHashMap::default();
    let mut current_plane_id = None;

    for artifact_command in artifact_commands {
        if let ModelingCmd::EnableSketchMode(EnableSketchMode { entity_id, .. }) = artifact_command.command {
            current_plane_id = Some(entity_id);
        }
        // If we get a start path command, we need to set the plane ID to the
        // current plane ID.
        // THIS IS THE ONLY THING WE CAN ASSUME IS ALWAYS SEQUENTIAL SINCE ITS PART OF THE
        // SAME ATOMIC COMMANDS BATCHING.
        if let ModelingCmd::StartPath(_) = artifact_command.command {
            if let Some(plane_id) = current_plane_id {
                path_to_plane_id_map.insert(artifact_command.cmd_id, plane_id);
            }
        }
        if let ModelingCmd::SketchModeDisable(_) = artifact_command.command {
            current_plane_id = None;
        }

        let flattened_responses = flatten_modeling_command_responses(responses);
        let artifact_updates = artifacts_to_update(
            &map,
            artifact_command,
            &flattened_responses,
            &path_to_plane_id_map,
            ast,
            exec_artifacts,
        )?;
        for artifact in artifact_updates {
            // Merge with existing artifacts.
            merge_artifact_into_map(&mut map, artifact);
        }
    }

    for exec_artifact in exec_artifacts.values() {
        merge_artifact_into_map(&mut map, exec_artifact.clone());
    }

    Ok(ArtifactGraph { map })
}

/// Flatten the responses into a map of command IDs to modeling command
/// responses.  The raw responses from the engine contain batches.
fn flatten_modeling_command_responses(
    responses: &IndexMap<Uuid, WebSocketResponse>,
) -> FnvHashMap<Uuid, OkModelingCmdResponse> {
    let mut map = FnvHashMap::default();
    for (cmd_id, ws_response) in responses {
        let WebSocketResponse::Success(response) = ws_response else {
            // Response not successful.
            continue;
        };
        match &response.resp {
            OkWebSocketResponseData::Modeling { modeling_response } => {
                map.insert(*cmd_id, modeling_response.clone());
            }
            OkWebSocketResponseData::ModelingBatch { responses } =>
            {
                #[expect(
                    clippy::iter_over_hash_type,
                    reason = "Since we're moving entries to another unordered map, it's fine that the order is undefined"
                )]
                for (cmd_id, batch_response) in responses {
                    if let BatchResponse::Success {
                        response: modeling_response,
                    } = batch_response
                    {
                        map.insert(*cmd_id.as_ref(), modeling_response.clone());
                    }
                }
            }
            OkWebSocketResponseData::IceServerInfo { .. }
            | OkWebSocketResponseData::TrickleIce { .. }
            | OkWebSocketResponseData::SdpAnswer { .. }
            | OkWebSocketResponseData::Export { .. }
            | OkWebSocketResponseData::MetricsRequest { .. }
            | OkWebSocketResponseData::ModelingSessionData { .. }
            | OkWebSocketResponseData::Debug { .. }
            | OkWebSocketResponseData::Pong { .. } => {}
        }
    }

    map
}

fn merge_artifact_into_map(map: &mut IndexMap<ArtifactId, Artifact>, new_artifact: Artifact) {
    let id = new_artifact.id();
    let Some(old_artifact) = map.get_mut(&id) else {
        // No old artifact exists.  Insert the new one.
        map.insert(id, new_artifact);
        return;
    };

    if let Some(replacement) = old_artifact.merge(new_artifact) {
        *old_artifact = replacement;
    }
}

/// Merge the new IDs into the base vector, avoiding duplicates.  This is O(nm)
/// runtime.  Rationale is that most of the ID collections in the artifact graph
/// are pretty small, but we may want to change this in the future.
fn merge_ids(base: &mut Vec<ArtifactId>, new: Vec<ArtifactId>) {
    let original_len = base.len();
    for id in new {
        // Don't bother inspecting new items that we just pushed.
        let original_base = &base[..original_len];
        if !original_base.contains(&id) {
            base.push(id);
        }
    }
}

fn merge_opt_id(base: &mut Option<ArtifactId>, new: Option<ArtifactId>) {
    // Always use the new one, even if it clears it.
    *base = new;
}

fn artifacts_to_update(
    artifacts: &IndexMap<ArtifactId, Artifact>,
    artifact_command: &ArtifactCommand,
    responses: &FnvHashMap<Uuid, OkModelingCmdResponse>,
    path_to_plane_id_map: &FnvHashMap<Uuid, Uuid>,
    ast: &Node<Program>,
    exec_artifacts: &IndexMap<ArtifactId, Artifact>,
) -> Result<Vec<Artifact>, KclError> {
    // TODO: Build path-to-node from artifact_command source range.  Right now,
    // we're serializing an empty array, and the TS wrapper fills it in with the
    // correct value based on NodePath.
    let path_to_node = Vec::new();
    let range = artifact_command.range;
    let node_path = NodePath::from_range(ast, range).unwrap_or_default();
    let code_ref = CodeRef {
        range,
        node_path,
        path_to_node,
    };

    let uuid = artifact_command.cmd_id;
    let id = ArtifactId::new(uuid);

    let Some(response) = responses.get(&uuid) else {
        // Response not found or not successful.
        return Ok(Vec::new());
    };

    let cmd = &artifact_command.command;

    match cmd {
        ModelingCmd::MakePlane(_) => {
            if range.is_synthetic() {
                return Ok(Vec::new());
            }
            // If we're calling `make_plane` and the code range doesn't end at
            // `0` it's not a default plane, but a custom one from the
            // offsetPlane standard library function.
            return Ok(vec![Artifact::Plane(Plane {
                id,
                path_ids: Vec::new(),
                code_ref,
            })]);
        }
        ModelingCmd::EnableSketchMode(EnableSketchMode { entity_id, .. }) => {
            let existing_plane = artifacts.get(&ArtifactId::new(*entity_id));
            match existing_plane {
                Some(Artifact::Wall(wall)) => {
                    return Ok(vec![Artifact::Wall(Wall {
                        id: entity_id.into(),
                        seg_id: wall.seg_id,
                        edge_cut_edge_ids: wall.edge_cut_edge_ids.clone(),
                        sweep_id: wall.sweep_id,
                        path_ids: wall.path_ids.clone(),
                        face_code_ref: wall.face_code_ref.clone(),
                        cmd_id: artifact_command.cmd_id,
                    })]);
                }
                Some(Artifact::Cap(cap)) => {
                    return Ok(vec![Artifact::Cap(Cap {
                        id: entity_id.into(),
                        sub_type: cap.sub_type,
                        edge_cut_edge_ids: cap.edge_cut_edge_ids.clone(),
                        sweep_id: cap.sweep_id,
                        path_ids: cap.path_ids.clone(),
                        face_code_ref: cap.face_code_ref.clone(),
                        cmd_id: artifact_command.cmd_id,
                    })]);
                }
                Some(_) | None => {
                    let path_ids = match existing_plane {
                        Some(Artifact::Plane(Plane { path_ids, .. })) => path_ids.clone(),
                        _ => Vec::new(),
                    };
                    return Ok(vec![Artifact::Plane(Plane {
                        id: entity_id.into(),
                        path_ids,
                        code_ref,
                    })]);
                }
            }
        }
        ModelingCmd::StartPath(_) => {
            let mut return_arr = Vec::new();
            let current_plane_id = path_to_plane_id_map.get(&artifact_command.cmd_id).ok_or_else(|| {
                KclError::Internal(KclErrorDetails {
                    message: format!(
                        "Expected a current plane ID when processing StartPath command, but we have none: {id:?}"
                    ),
                    source_ranges: vec![range],
                })
            })?;
            return_arr.push(Artifact::Path(Path {
                id,
                plane_id: (*current_plane_id).into(),
                seg_ids: Vec::new(),
                sweep_id: None,
                solid2d_id: None,
                code_ref,
                composite_solid_id: None,
            }));
            let plane = artifacts.get(&ArtifactId::new(*current_plane_id));
            if let Some(Artifact::Plane(plane)) = plane {
                let plane_code_ref = plane.code_ref.clone();
                return_arr.push(Artifact::Plane(Plane {
                    id: (*current_plane_id).into(),
                    path_ids: vec![id],
                    code_ref: plane_code_ref,
                }));
            }
            if let Some(Artifact::Wall(wall)) = plane {
                return_arr.push(Artifact::Wall(Wall {
                    id: (*current_plane_id).into(),
                    seg_id: wall.seg_id,
                    edge_cut_edge_ids: wall.edge_cut_edge_ids.clone(),
                    sweep_id: wall.sweep_id,
                    path_ids: vec![id],
                    face_code_ref: wall.face_code_ref.clone(),
                    cmd_id: artifact_command.cmd_id,
                }));
            }
            if let Some(Artifact::Cap(cap)) = plane {
                return_arr.push(Artifact::Cap(Cap {
                    id: (*current_plane_id).into(),
                    sub_type: cap.sub_type,
                    edge_cut_edge_ids: cap.edge_cut_edge_ids.clone(),
                    sweep_id: cap.sweep_id,
                    path_ids: vec![id],
                    face_code_ref: cap.face_code_ref.clone(),
                    cmd_id: artifact_command.cmd_id,
                }));
            }
            return Ok(return_arr);
        }
        ModelingCmd::ClosePath(_) | ModelingCmd::ExtendPath(_) => {
            let path_id = ArtifactId::new(match cmd {
                ModelingCmd::ClosePath(c) => c.path_id,
                ModelingCmd::ExtendPath(e) => e.path.into(),
                _ => unreachable!(),
            });
            let mut return_arr = Vec::new();
            return_arr.push(Artifact::Segment(Segment {
                id,
                path_id,
                surface_id: None,
                edge_ids: Vec::new(),
                edge_cut_id: None,
                code_ref,
                common_surface_ids: Vec::new(),
            }));
            let path = artifacts.get(&path_id);
            if let Some(Artifact::Path(path)) = path {
                let mut new_path = path.clone();
                new_path.seg_ids = vec![id];
                return_arr.push(Artifact::Path(new_path));
            }
            if let OkModelingCmdResponse::ClosePath(close_path) = response {
                return_arr.push(Artifact::Solid2d(Solid2d {
                    id: close_path.face_id.into(),
                    path_id,
                }));
                if let Some(Artifact::Path(path)) = path {
                    let mut new_path = path.clone();
                    new_path.solid2d_id = Some(close_path.face_id.into());
                    return_arr.push(Artifact::Path(new_path));
                }
            }
            return Ok(return_arr);
        }
        ModelingCmd::Extrude(kcmc::Extrude { target, .. })
        | ModelingCmd::Revolve(kcmc::Revolve { target, .. })
        | ModelingCmd::RevolveAboutEdge(kcmc::RevolveAboutEdge { target, .. })
        | ModelingCmd::Sweep(kcmc::Sweep { target, .. }) => {
            let sub_type = match cmd {
                ModelingCmd::Extrude(_) => SweepSubType::Extrusion,
                ModelingCmd::Revolve(_) => SweepSubType::Revolve,
                ModelingCmd::RevolveAboutEdge(_) => SweepSubType::RevolveAboutEdge,
                ModelingCmd::Sweep(_) => SweepSubType::Sweep,
                _ => unreachable!(),
            };
            let mut return_arr = Vec::new();
            let target = ArtifactId::from(target);
            return_arr.push(Artifact::Sweep(Sweep {
                id,
                sub_type,
                path_id: target,
                surface_ids: Vec::new(),
                edge_ids: Vec::new(),
                code_ref,
            }));
            let path = artifacts.get(&target);
            if let Some(Artifact::Path(path)) = path {
                let mut new_path = path.clone();
                new_path.sweep_id = Some(id);
                return_arr.push(Artifact::Path(new_path));
            }
            return Ok(return_arr);
        }
        ModelingCmd::Loft(loft_cmd) => {
            let OkModelingCmdResponse::Loft(_) = response else {
                return Ok(Vec::new());
            };
            let mut return_arr = Vec::new();
            return_arr.push(Artifact::Sweep(Sweep {
                id,
                sub_type: SweepSubType::Loft,
                // TODO: Using the first one.  Make sure to revisit this
                // choice, don't think it matters for now.
                path_id: ArtifactId::new(*loft_cmd.section_ids.first().ok_or_else(|| {
                    KclError::Internal(KclErrorDetails {
                        message: format!("Expected at least one section ID in Loft command: {id:?}; cmd={cmd:?}"),
                        source_ranges: vec![range],
                    })
                })?),
                surface_ids: Vec::new(),
                edge_ids: Vec::new(),
                code_ref,
            }));
            for section_id in &loft_cmd.section_ids {
                let path = artifacts.get(&ArtifactId::new(*section_id));
                if let Some(Artifact::Path(path)) = path {
                    let mut new_path = path.clone();
                    new_path.sweep_id = Some(id);
                    return_arr.push(Artifact::Path(new_path));
                }
            }
            return Ok(return_arr);
        }
        ModelingCmd::Solid3dGetInfo(request) => {
            let OkModelingCmdResponse::Solid3dGetInfo(solid_info) = response else {
                return Ok(Vec::new());
            };
            // log out entire request and response
            // println!("Solid3dGetInfo request: {:#?}", request);
            // println!("Solid3dGetInfo response: {:#?}", solid_info);
            let Some(Artifact::Path(path)) = artifacts.get(&ArtifactId::new(request.object_id)) else {
                return Ok(Vec::new());
            };
            let Some(path_sweep_id) = path.sweep_id else {
                return Ok(Vec::new());
            };

            // the logic here is that we need to use the info from this endpoint to create a series of wall, cap and sweepEdge Artifacts, as well as update segment artifacts with `common_surface_ids`
            // what we'll do is iterate over some of these hashmaps while also maintaining a some of our own hashmaps to track things

            enum ComplementaryEdgeType {
                Base,
                Adjacent,
                Opposite,
            }
            // 1 create hashmap called complementary_edge_map the shape will be {[edgeId]: {type: 'base' | 'adjacent' | 'opposite', id: baseSegmentId}}
            pub struct ComplementaryEdgeResult {
                pub base_segment_id: Uuid,
                pub sweep_edge_type: ComplementaryEdgeType,
            }
            let mut complementary_edge_map: HashMap<Uuid, ComplementaryEdgeResult> = HashMap::new();

            // Sort the hash map keys for deterministic iteration
            let mut complementary_edges: Vec<_> = solid_info.info.complementary_edges.iter().collect();
            complementary_edges.sort_by_key(|(k, _)| *k);
            for (base_segment_id, edge_info) in complementary_edges {
                if edge_info.adjacent_ids.len() < 2 {
                    continue;
                }
                complementary_edge_map.insert(
                    // second element is the "next adjacent edge"
                    edge_info.adjacent_ids[1],
                    ComplementaryEdgeResult {
                        base_segment_id: *base_segment_id,
                        sweep_edge_type: ComplementaryEdgeType::Adjacent,
                    },
                );
                if let Some(opposite_id) = edge_info.opposite_id {
                    complementary_edge_map.insert(
                        opposite_id,
                        ComplementaryEdgeResult {
                            base_segment_id: *base_segment_id,
                            sweep_edge_type: ComplementaryEdgeType::Opposite,
                        },
                    );
                }
                complementary_edge_map.insert(
                    *base_segment_id,
                    ComplementaryEdgeResult {
                        base_segment_id: *base_segment_id,
                        sweep_edge_type: ComplementaryEdgeType::Base,
                    },
                );
            }

            // 2 loop over common_edges,
            // for each faceId we find:
            //   if the id matches `top_cap_id` or `bottom_cap_id` we create a cap artifact
            //   if the edgeId matches ComplementaryEdgeType::Base from our complementary_edge_map we create a wall artifact
            //   else we ignore it

            // for each edgeId:
            //   using ComplementaryEdgeType from complementary_edge_map to determine it's sub type
            //   and use that to either create a sweepEdge or a segment
            //   we use the two face ids to populate the common_surface_ids of the segment/sweepEdge

            let mut return_arr = Vec::new();
            for (edge_id, face_ids) in solid_info.info.common_edges.iter() {
                for face_id in face_ids {
                    if solid_info.info.top_cap_id == Some(*face_id) {
                        return_arr.push(Artifact::Cap(Cap {
                            id: face_id.into(),
                            sub_type: CapSubType::End,
                            edge_cut_edge_ids: Vec::new(),
                            sweep_id: path_sweep_id,
                            path_ids: Vec::new(),
                            face_code_ref: code_ref.clone(),
                            cmd_id: artifact_command.cmd_id,
                        }));
                        if let Some(Artifact::Sweep(sweep)) = artifacts.get(&path_sweep_id) {
                            let mut new_sweep = sweep.clone();
                            new_sweep.surface_ids.push((*face_id).into());
                            return_arr.push(Artifact::Sweep(new_sweep));
                        }
                        continue;
                    }
                    if solid_info.info.bottom_cap_id == Some(*face_id) {
                        return_arr.push(Artifact::Cap(Cap {
                            id: face_id.into(),
                            sub_type: CapSubType::Start,
                            edge_cut_edge_ids: Vec::new(),
                            sweep_id: path_sweep_id,
                            path_ids: Vec::new(),
                            face_code_ref: code_ref.clone(),
                            cmd_id: artifact_command.cmd_id,
                        }));
                        if let Some(Artifact::Sweep(sweep)) = artifacts.get(&path_sweep_id) {
                            let mut new_sweep = sweep.clone();
                            new_sweep.surface_ids.push((*face_id).into());
                            return_arr.push(Artifact::Sweep(new_sweep));
                        }
                        continue;
                    }
                    if let Some(complementary_edge) = complementary_edge_map.get(edge_id) {
                        if let ComplementaryEdgeType::Base = complementary_edge.sweep_edge_type {
                            return_arr.push(Artifact::Wall(Wall {
                                id: (*face_id).into(),
                                seg_id: complementary_edge.base_segment_id.into(),
                                edge_cut_edge_ids: Vec::new(),
                                sweep_id: path_sweep_id,
                                path_ids: Vec::new(),
                                face_code_ref: code_ref.clone(),
                                cmd_id: artifact_command.cmd_id,
                            }));
                            if let Some(Artifact::Sweep(sweep)) = artifacts.get(&path_sweep_id) {
                                let mut new_sweep = sweep.clone();
                                new_sweep.surface_ids.push((*face_id).into());
                                return_arr.push(Artifact::Sweep(new_sweep));
                            }
                        }
                    }
                }

                // part 2 add sweepEdges and segments
                if let Some(complementary_edge) = complementary_edge_map.get(edge_id) {
                    let existing_artifact = artifacts.get(&ArtifactId::new(*edge_id));
                    match complementary_edge.sweep_edge_type {
                        ComplementaryEdgeType::Adjacent => {
                            if existing_artifact == None {
                                return_arr.push(Artifact::SweepEdge(SweepEdge {
                                    id: edge_id.into(),
                                    sub_type: SweepEdgeSubType::Adjacent,
                                    cmd_id: artifact_command.cmd_id,
                                    common_surface_ids: face_ids.iter().map(|id| (*id).into()).collect(),
                                    seg_id: complementary_edge.base_segment_id.into(),
                                    sweep_id: path_sweep_id,
                                }));
                                if let Some(Artifact::Segment(prev_segment)) =
                                    artifacts.get(&ArtifactId::new(complementary_edge.base_segment_id))
                                {
                                    let mut new_segment = prev_segment.clone();
                                    new_segment.edge_ids.push(edge_id.into());
                                    return_arr.push(Artifact::Segment(new_segment));
                                }
                                if let Some(Artifact::Sweep(sweep)) = artifacts.get(&path_sweep_id) {
                                    let mut new_sweep = sweep.clone();
                                    new_sweep.edge_ids.push(edge_id.into());
                                    return_arr.push(Artifact::Sweep(new_sweep));
                                }
                            }
                        }
                        ComplementaryEdgeType::Opposite => {
                            if existing_artifact == None {
                                return_arr.push(Artifact::SweepEdge(SweepEdge {
                                    id: edge_id.into(),
                                    sub_type: SweepEdgeSubType::Opposite,
                                    cmd_id: artifact_command.cmd_id,
                                    common_surface_ids: face_ids.iter().map(|id| (*id).into()).collect(),
                                    seg_id: complementary_edge.base_segment_id.into(),
                                    sweep_id: path_sweep_id,
                                }));
                                if let Some(Artifact::Segment(prev_segment)) =
                                    artifacts.get(&ArtifactId::new(complementary_edge.base_segment_id))
                                {
                                    let mut new_segment = prev_segment.clone();
                                    new_segment.edge_ids.push(edge_id.into());
                                    return_arr.push(Artifact::Segment(new_segment));
                                }
                                if let Some(Artifact::Sweep(sweep)) = artifacts.get(&path_sweep_id) {
                                    let mut new_sweep = sweep.clone();
                                    new_sweep.edge_ids.push(edge_id.into());
                                    return_arr.push(Artifact::Sweep(new_sweep));
                                }
                            }
                        }
                        ComplementaryEdgeType::Base => {
                            if let Some(Artifact::Segment(prev_segment)) =
                                artifacts.get(&ArtifactId::new(complementary_edge.base_segment_id))
                            {
                                let mut new_segment = prev_segment.clone();
                                new_segment.common_surface_ids = face_ids.iter().map(|id| (*id).into()).collect();
                                return_arr.push(Artifact::Segment(new_segment));
                            }
                        }
                    };
                }
            }
            return Ok(return_arr);
        }
        ModelingCmd::Solid3dFilletEdge(cmd) => {
            let mut return_arr = Vec::new();
            let edge_id = if let Some(edge_id) = cmd.edge_id {
                ArtifactId::new(edge_id)
            } else {
                cmd.edge_ids.first().unwrap().into()
            };
            return_arr.push(Artifact::EdgeCut(EdgeCut {
                id,
                sub_type: cmd.cut_type.into(),
                consumed_edge_id: edge_id,
                edge_ids: Vec::new(),
                surface_id: None,
                code_ref,
            }));
            let consumed_edge = artifacts.get(&edge_id);
            if let Some(Artifact::Segment(consumed_edge)) = consumed_edge {
                let mut new_segment = consumed_edge.clone();
                new_segment.edge_cut_id = Some(id);
                return_arr.push(Artifact::Segment(new_segment));
            } else {
                // TODO: Handle other types like SweepEdge.
            }
            return Ok(return_arr);
        }
        ModelingCmd::EntityMakeHelixFromParams(_) => {
            let return_arr = vec![Artifact::Helix(Helix {
                id,
                axis_id: None,
                code_ref,
            })];
            return Ok(return_arr);
        }
        ModelingCmd::EntityMakeHelixFromEdge(helix) => {
            let edge_id = ArtifactId::new(helix.edge_id);
            let return_arr = vec![Artifact::Helix(Helix {
                id,
                axis_id: Some(edge_id),
                code_ref,
            })];
            // We could add the reverse graph edge connecting from the edge to
            // the helix here, but it's not useful right now.
            return Ok(return_arr);
        }
        ModelingCmd::BooleanIntersection(_) | ModelingCmd::BooleanSubtract(_) | ModelingCmd::BooleanUnion(_) => {
            let (sub_type, solid_ids, tool_ids) = match cmd {
                ModelingCmd::BooleanIntersection(intersection) => {
                    let solid_ids = intersection
                        .solid_ids
                        .iter()
                        .copied()
                        .map(ArtifactId::new)
                        .collect::<Vec<_>>();
                    (CompositeSolidSubType::Intersect, solid_ids, Vec::new())
                }
                ModelingCmd::BooleanSubtract(subtract) => {
                    let solid_ids = subtract
                        .target_ids
                        .iter()
                        .copied()
                        .map(ArtifactId::new)
                        .collect::<Vec<_>>();
                    let tool_ids = subtract
                        .tool_ids
                        .iter()
                        .copied()
                        .map(ArtifactId::new)
                        .collect::<Vec<_>>();
                    (CompositeSolidSubType::Subtract, solid_ids, tool_ids)
                }
                ModelingCmd::BooleanUnion(union) => {
                    let solid_ids = union.solid_ids.iter().copied().map(ArtifactId::new).collect::<Vec<_>>();
                    (CompositeSolidSubType::Union, solid_ids, Vec::new())
                }
                _ => unreachable!(),
            };

            let mut new_solid_ids = vec![id];

            // Make sure we don't ever create a duplicate ID since merge_ids
            // can't handle it.
            let not_cmd_id = move |solid_id: &ArtifactId| *solid_id != id;

            match response {
                OkModelingCmdResponse::BooleanIntersection(intersection) => intersection
                    .extra_solid_ids
                    .iter()
                    .copied()
                    .map(ArtifactId::new)
                    .filter(not_cmd_id)
                    .for_each(|id| new_solid_ids.push(id)),
                OkModelingCmdResponse::BooleanSubtract(subtract) => subtract
                    .extra_solid_ids
                    .iter()
                    .copied()
                    .map(ArtifactId::new)
                    .filter(not_cmd_id)
                    .for_each(|id| new_solid_ids.push(id)),
                OkModelingCmdResponse::BooleanUnion(union) => union
                    .extra_solid_ids
                    .iter()
                    .copied()
                    .map(ArtifactId::new)
                    .filter(not_cmd_id)
                    .for_each(|id| new_solid_ids.push(id)),
                _ => {}
            }

            let mut return_arr = Vec::new();

            // Create the new composite solids and update their linked artifacts
            for solid_id in &new_solid_ids {
                // Create the composite solid
                return_arr.push(Artifact::CompositeSolid(CompositeSolid {
                    id: *solid_id,
                    sub_type,
                    solid_ids: solid_ids.clone(),
                    tool_ids: tool_ids.clone(),
                    code_ref: code_ref.clone(),
                    composite_solid_id: None,
                }));

                // Update the artifacts that were used as input for this composite solid
                for input_id in &solid_ids {
                    if let Some(artifact) = artifacts.get(input_id) {
                        match artifact {
                            Artifact::CompositeSolid(comp) => {
                                let mut new_comp = comp.clone();
                                new_comp.composite_solid_id = Some(*solid_id);
                                return_arr.push(Artifact::CompositeSolid(new_comp));
                            }
                            Artifact::Path(path) => {
                                let mut new_path = path.clone();
                                new_path.composite_solid_id = Some(*solid_id);
                                return_arr.push(Artifact::Path(new_path));
                            }
                            _ => {}
                        }
                    }
                }

                // Update the tool artifacts if this is a subtract operation
                for tool_id in &tool_ids {
                    if let Some(artifact) = artifacts.get(tool_id) {
                        match artifact {
                            Artifact::CompositeSolid(comp) => {
                                let mut new_comp = comp.clone();
                                new_comp.composite_solid_id = Some(*solid_id);
                                return_arr.push(Artifact::CompositeSolid(new_comp));
                            }
                            Artifact::Path(path) => {
                                let mut new_path = path.clone();
                                new_path.composite_solid_id = Some(*solid_id);
                                return_arr.push(Artifact::Path(new_path));
                            }
                            _ => {}
                        }
                    }
                }
            }

            return Ok(return_arr);
        }
        _ => {}
    }

    Ok(Vec::new())
}
