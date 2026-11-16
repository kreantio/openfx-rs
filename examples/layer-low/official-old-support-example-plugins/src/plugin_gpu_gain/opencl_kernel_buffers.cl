__kernel void GainAdjustKernelBuffers(
    int p_Width,
    int p_Height,
    float p_GainR,
    float p_GainG,
    float p_GainB,
    float p_GainA,
    __global const float* p_Input,
    __global float* p_Output)
{
    const int x = get_global_id(0);
    const int y = get_global_id(1);

    if ((x < p_Width) && (y < p_Height))
    {
        const int index = ((y * p_Width) + x) * 4;

        p_Output[index + 0] = p_Input[index + 0] * p_GainR;
        p_Output[index + 1] = p_Input[index + 1] * p_GainG;
        p_Output[index + 2] = p_Input[index + 2] * p_GainB;
        p_Output[index + 3] = p_Input[index + 3] * p_GainA;
    }
}