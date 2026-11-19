__constant sampler_t imageSampler = CLK_NORMALIZED_COORDS_FALSE | CLK_ADDRESS_CLAMP | CLK_FILTER_NEAREST;  
                                                                                                           
__kernel void GainAdjustKernelImages(                                                                      
    int p_Width,                                                                                            
    int p_Height,                                                                                           
    float p_GainR,                                                                                          
    float p_GainG,                                                                                          
    float p_GainB,                                                                                          
    float p_GainA,                                                                                          
    __read_only  image2d_t p_Input,                                                                         
    __write_only image2d_t p_Output)                                                                        
{                                                                                                          
    const int x = get_global_id(0);                                                                         
    const int y = get_global_id(1);                                                                         
                                                                                                           
    if ((x < p_Width) && (y < p_Height))                                                                    
    {                                                                                                       
        int2 coord = (int2)(x, y);                                                                          
        float4 out = read_imagef(p_Input, imageSampler, coord);                                             
        out *= (float4)(p_GainR, p_GainG, p_GainB, p_GainA);                                                
        write_imagef(p_Output, coord, out);                                                                 
    }                                                                                                       
}       