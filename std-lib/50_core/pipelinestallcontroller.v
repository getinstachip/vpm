module pipeline_stall_controller (
    input wire stall,       // Stall signal
    output reg pc_write,    // Control signal to write to the program counter
    output reg if_id_write, // Control signal to write to the IF/ID pipeline register
    output reg control_stall // Control signal to stall the control unit
);

    always @(*) begin
        if (stall) begin
            pc_write = 0;
            if_id_write = 0;
            control_stall = 1;
        end else begin
            pc_write = 1;
            if_id_write = 1;
            control_stall = 0;
        end
    end

endmodule
