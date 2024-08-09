module instruction_decoder (
    input [31:0] instr,
    output reg [6:0] opcode,
    output reg [4:0] rd, rs1, rs2,
    output reg [2:0] funct3,
    output reg [6:0] funct7
);
    always @(*) begin
        opcode = instr[6:0];
        rd = instr[11:7];
        rs1 = instr[19:15];
        rs2 = instr[24:20];
        funct3 = instr[14:12];
        funct7 = instr[31:25];
    end
endmodule
