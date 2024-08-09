module control_unit (
    input wire [3:0] opcode,  // 4-bit opcode from the instruction
    output reg reg_dst,       // Register destination
    output reg alu_src,       // ALU source
    output reg mem_to_reg,    // Memory to register
    output reg reg_write,     // Register write
    output reg mem_read,      // Memory read
    output reg mem_write,     // Memory write
    output reg branch,        // Branch control
    output reg [1:0] alu_op   // ALU operation
);

    always @(*) begin
        // Default values for control signals
        reg_dst = 0;
        alu_src = 0;
        mem_to_reg = 0;
        reg_write = 0;
        mem_read = 0;
        mem_write = 0;
        branch = 0;
        alu_op = 2'b00;

        case (opcode)
            4'b0000: begin // Example: R-type instruction
                reg_dst = 1;
                alu_src = 0;
                mem_to_reg = 0;
                reg_write = 1;
                alu_op = 2'b10;
            end
            4'b0001: begin // Example: Load instruction
                alu_src = 1;
                mem_to_reg = 1;
                reg_write = 1;
                mem_read = 1;
                alu_op = 2'b00;
            end
            4'b0010: begin // Example: Store instruction
                alu_src = 1;
                mem_write = 1;
                alu_op = 2'b00;
            end
            4'b0011: begin // Example: Branch instruction
                branch = 1;
                alu_op = 2'b01;
            end
            // Add more cases for other instructions as needed
            default: begin
                // Default case to handle unknown opcodes
            end
        endcase
    end

endmodule
