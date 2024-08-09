module hazard_detection_unit (
    input wire [4:0] id_ex_rt, 
    input wire [4:0] if_id_rs, 
    input wire [4:0] if_id_rt,  
    input wire id_ex_mem_read,
    output reg pc_write,         
    output reg if_id_write,      
    output reg control_stall    
);

    always @(*) begin
        // Default values for control signals
        pc_write = 1;
        if_id_write = 1;
        control_stall = 0;

        // Detect load-use hazard
        if (id_ex_mem_read &&
            ((id_ex_rt == if_id_rs) || (id_ex_rt == if_id_rt))) begin
            // Hazard detected: stall the pipeline
            pc_write = 0;
            if_id_write = 0;
            control_stall = 1;
        end
    end

endmodule
