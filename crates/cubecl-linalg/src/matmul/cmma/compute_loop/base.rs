use cubecl_core as cubecl;
use cubecl_core::prelude::*;

use crate::matmul::cmma::{
    base::{Fragments, Ids, SharedMemories},
    compute_loop::{
        accumulators_first::AllAccumulatorsFirstComputeLoop,
        buffers_first::AllBuffersFirstComputeLoop,
    },
    config::ComptimeCmmaInfo,
    load_shared_memory::{
        base::SmemLoader,
        continous::ContinuousSmemLoader,
        load_info::{LhsLoadInfo, RhsLoadInfo},
        tiled_layout::{ColMajorTiling, RowMajorTiling},
        tilewise::TilewiseSmemLoader,
    },
};

#[cube]
pub(crate) fn compute_loop<F: Float, FC: Float>(
    shared_memories: SharedMemories<FC>,
    fragments: &mut Fragments<F, FC>,
    ids: Ids,
    #[comptime] comptime_info: ComptimeCmmaInfo,
) {
    if comptime_info.compute_loop_order_strategy == 0 {
        AllBuffersFirstComputeLoop::compute_loop::<F, FC>(
            shared_memories,
            fragments,
            ids,
            comptime_info,
        );
    } else {
        AllAccumulatorsFirstComputeLoop::compute_loop::<F, FC>(
            shared_memories,
            fragments,
            ids,
            comptime_info,
        );
    }
}

#[cube]
pub(crate) trait ComputeLoop {
    fn compute_loop<F: Float, FC: Float>(
        shared_memories: SharedMemories<FC>,
        fragments: &mut Fragments<F, FC>,
        ids: Ids,
        #[comptime] comptime_info: ComptimeCmmaInfo,
    );
}

#[cube]
pub(crate) fn load_tile_into_fragment<FC: Float>(
    nth_tile: u32,
    smem: SharedMemory<FC>,
    fragment: &cmma::Matrix<FC>,
    #[comptime] comptime_info: ComptimeCmmaInfo,
) {
    let tile_size = comptime_info.tile_size;
    let smem_stride = tile_size * tile_size;

    let smem_pos = nth_tile * smem_stride;
    let slice = smem.slice(smem_pos, smem_pos + smem_stride);
    cmma::load::<FC>(fragment, slice, 16);
}

#[cube]
pub(crate) fn get_smem_position_lhs<F: Float, FC: Float>(
    tile_row: u32,
    tile_col: u32,
    #[comptime] comptime_info: ComptimeCmmaInfo,
) -> u32 {
    if comptime_info.lhs_smem_loader_strategy == 0 {
        get_tile_smem_position::<F, FC, TilewiseSmemLoader<LhsLoadInfo, RowMajorTiling>>(
            tile_row,
            tile_col,
            comptime_info,
        )
    } else if comptime_info.lhs_smem_loader_strategy == 1 {
        get_tile_smem_position::<F, FC, TilewiseSmemLoader<LhsLoadInfo, ColMajorTiling>>(
            tile_row,
            tile_col,
            comptime_info,
        )
    } else if comptime_info.lhs_smem_loader_strategy == 2 {
        get_tile_smem_position::<F, FC, ContinuousSmemLoader<LhsLoadInfo, RowMajorTiling>>(
            tile_row,
            tile_col,
            comptime_info,
        )
    } else {
        get_tile_smem_position::<F, FC, ContinuousSmemLoader<LhsLoadInfo, ColMajorTiling>>(
            tile_row,
            tile_col,
            comptime_info,
        )
    }
}

#[cube]
pub(crate) fn get_smem_position_rhs<F: Float, FC: Float>(
    tile_row: u32,
    tile_col: u32,
    #[comptime] comptime_info: ComptimeCmmaInfo,
) -> u32 {
    if comptime_info.rhs_smem_loader_strategy == 0 {
        get_tile_smem_position::<F, FC, TilewiseSmemLoader<RhsLoadInfo, RowMajorTiling>>(
            tile_row,
            tile_col,
            comptime_info,
        )
    } else if comptime_info.rhs_smem_loader_strategy == 1 {
        get_tile_smem_position::<F, FC, TilewiseSmemLoader<RhsLoadInfo, ColMajorTiling>>(
            tile_row,
            tile_col,
            comptime_info,
        )
    } else if comptime_info.rhs_smem_loader_strategy == 2 {
        get_tile_smem_position::<F, FC, ContinuousSmemLoader<RhsLoadInfo, RowMajorTiling>>(
            tile_row,
            tile_col,
            comptime_info,
        )
    } else {
        get_tile_smem_position::<F, FC, ContinuousSmemLoader<RhsLoadInfo, ColMajorTiling>>(
            tile_row,
            tile_col,
            comptime_info,
        )
    }
}

#[cube]
fn get_tile_smem_position<F: Float, FC: Float, S: SmemLoader<F, FC>>(
    tile_row: u32,
    tile_col: u32,
    #[comptime] comptime_info: ComptimeCmmaInfo,
) -> u32 {
    S::get_tile_smem_position(tile_row, tile_col, comptime_info)
}