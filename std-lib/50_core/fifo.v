module fifo #(parameter WIDTH = 8, DEPTH = 16)(
    input clk,
    input rst,
    input wr_en,
    input rd_en,
    input [WIDTH-1:0] din,
    output [WIDTH-1:0] dout,
    output full,
    output empty
);
    reg [WIDTH-1:0] mem[0:DEPTH-1];
    reg [4:0] wr_ptr = 0;
    reg [4:0] rd_ptr = 0;
    
    assign full = (wr_ptr == (rd_ptr - 1));
    assign empty = (wr_ptr == rd_ptr);
    assign dout = mem[rd_ptr];
    
    always @(posedge clk or posedge rst) begin
        if (rst) begin
            wr_ptr <= 0;
            rd_ptr <= 0;
        end else begin
            if (wr_en && !full) begin
                mem[wr_ptr] <= din;
                wr_ptr <= wr_ptr + 1;
            end
            if (rd_en && !empty) begin
                rd_ptr <= rd_ptr + 1;
            end
        end
    end
endmodule
