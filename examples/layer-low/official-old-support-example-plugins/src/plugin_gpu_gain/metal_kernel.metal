#include <metal_stdlib>
using namespace metal;
kernel void GainAdjustKernel(constant int& p_Width [[buffer (11)]], constant int& p_Height [[buffer (12)]], constant float& p_GainR [[buffer (13)]],
                             constant float& p_GainG [[buffer (14)]], constant float& p_GainB [[buffer (15)]], constant float& p_GainA [[buffer (16)]],
                             const device float* p_Input [[buffer (0)]], device float* p_Output [[buffer (8)]], uint2 id [[ thread_position_in_grid ]])
{
    if ((id.x < p_Width) && (id.y < p_Height))
    {
        const int index = ((id.y * p_Width) + id.x) * 4;
        p_Output[index + 0] = p_Input[index + 0] * p_GainR;
        p_Output[index + 1] = p_Input[index + 1] * p_GainG;
        p_Output[index + 2] = p_Input[index + 2] * p_GainB;
        p_Output[index + 3] = p_Input[index + 3] * p_GainA;
    }
}
