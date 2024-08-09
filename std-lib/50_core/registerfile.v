module register_file #(parameter WIDTH = 32, DEPTH = 32)(
    input clk,
    input [4:0] rd_addr1, rd_addr2, wr_addr,
    input [WIDTH-1:0] wr_data,
    input wr_en,
    output [WIDTH-1:0] rd_data1, rd_data2
);
    reg [WIDTH-1:0] reg_file[0:DEPTH-1];

    assign rd_data1 = reg_file[rd_addr1];
    assign rd_data2 = reg_file[rd_addr2];
    
    always @(posedge clk) begin
        if (wr_en)
            reg_file[wr_addr] <= wr_data;
    end
endmodule
