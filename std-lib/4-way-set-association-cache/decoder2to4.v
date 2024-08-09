module decoder2to4(input [1:0] in,output reg [3:0] wayOut);
  always@(in)
  case(in)
    2'b00: wayOut=4'b0001;
    2'b01: wayOut=4'b0010;
    2'b10: wayOut=4'b0100;
    2'b11: wayOut=4'b1000;
  endcase
endmodule