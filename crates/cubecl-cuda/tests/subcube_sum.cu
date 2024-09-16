typedef unsigned int uint;

extern "C" __global__ void kernel(float output_0[], uint info[]) {

  int threadIdxGlobal = threadIdx.x + threadIdx.y * blockDim.x +
                        threadIdx.z * (blockDim.x * blockDim.y);

  int warpSizeChecked = min(warpSize, blockDim.x * blockDim.y * blockDim.z);
  uint rank = info[0];
  uint rank_2 = rank * 2;
  float l_0_0;
  float l_0_1;
  bool l_0_2;
  uint l_0_3;
  bool l_0_4;
  l_0_3 = info[(1 * 2 * info[0]) + 1];
  l_0_4 = threadIdxGlobal < l_0_3;
  if (l_0_4) {
    l_0_0 = output_0[threadIdxGlobal];
  } else {
    l_0_0 = float(0.0);
  }

  l_0_1 = l_0_0;
  {
    for (int offset = warpSizeChecked / 2; offset > 0; offset /= 2) {
      l_0_1 += __shfl_down_sync(0xFFFFFFFF, l_0_1, offset);
    }
  }
  l_0_2 = threadIdxGlobal == uint(0);
  if (l_0_2) {
    uint l_1_0;
    bool l_1_1;
    l_1_0 = info[(1 * 2 * info[0]) + 1];
    l_1_1 = uint(0) < l_1_0;
    if (l_1_1) {
      output_0[uint(0)] = l_0_1;
    }
  }
}