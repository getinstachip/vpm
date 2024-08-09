module forwarding_unit (
    input wire [4:0] ex_mem_reg_rd,  // Destination register in EX/MEM stage
    input wire [4:0] mem_wb_reg_rd,  // Destination register in MEM/WB stage
    input wire ex_mem_reg_write,     // RegWrite signal in EX/MEM stage
    input wire mem_wb_reg_write,     // RegWrite signal in MEM/WB stage
    input wire [4:0] id_ex_reg_rs,   // Source register RS in ID/EX stage
    input wire [4:0] id_ex_reg_rt,   // Source register RT in ID/EX stage
    output reg [1:0] forward_a,   
    output reg [1:0] forward_b     
);

    always @(*) begin
        // Default forwarding controls
        forward_a = 2'b00;
        forward_b = 2'b00;

        // Forwarding logic for ALU input A
        if (ex_mem_reg_write && (ex_mem_reg_rd != 0) && (ex_mem_reg_rd == id_ex_reg_rs)) begin
            forward_a = 2'b10;
        end else if (mem_wb_reg_write && (mem_wb_reg_rd != 0) && (mem_wb_reg_rd == id_ex_reg_rs)) begin
            forward_a = 2'b01;
        end

        // Forwarding logic for ALU input B
        if (ex_mem_reg_write && (ex_mem_reg_rd != 0) && (ex_mem_reg_rd == id_ex_reg_rt)) begin
            forward_b = 2'b10;
        end else if (mem_wb_reg_write && (mem_wb_reg_rd != 0) && (mem_wb_reg_rd == id_ex_reg_rt)) begin
            forward_b = 2'b01;
        end
    end

endmodule
